#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: run-trnm-world-endurance-24h-v1.sh OUTPUT_DIR

Required environment:
  TRNM_ENDURANCE_AUTHORIZATION     nonempty reviewed authorization file
  TRNM_ENDURANCE_WORKLOAD          executable workload driver
  TRNM_ENDURANCE_METRICS           executable bounded metrics collector
  TRNM_ENDURANCE_THRESHOLDS        machine-readable predeclared thresholds
  TRNM_WORLD_BINARY                exact World binary/package under test
  TRNM_ENDURANCE_CONFIG            non-secret effective configuration file
  TRNM_ENDURANCE_TOPOLOGY          topology description file

Optional:
  TRNM_ENDURANCE_SECONDS           default and minimum 86400
  TRNM_ENDURANCE_SAMPLE_SECONDS    default 60, range 10..900
  TRNM_ENDURANCE_STOP              executable cleanup hook

This harness never grants partial-duration credit. A source self-test may
inspect the script but cannot lower the 86400-second evidence floor.
EOF
  exit 64
}

[[ $# -eq 1 ]] || usage
out="$1"
: "${TRNM_ENDURANCE_AUTHORIZATION:?required}"
: "${TRNM_ENDURANCE_WORKLOAD:?required}"
: "${TRNM_ENDURANCE_METRICS:?required}"
: "${TRNM_ENDURANCE_THRESHOLDS:?required}"
: "${TRNM_WORLD_BINARY:?required}"
: "${TRNM_ENDURANCE_CONFIG:?required}"
: "${TRNM_ENDURANCE_TOPOLOGY:?required}"
duration="${TRNM_ENDURANCE_SECONDS:-86400}"
sample="${TRNM_ENDURANCE_SAMPLE_SECONDS:-60}"

[[ "$duration" =~ ^[0-9]+$ ]] && (( duration >= 86400 )) || {
  echo "TRNM_ENDURANCE_SECONDS must be at least 86400" >&2
  exit 1
}
[[ "$sample" =~ ^[0-9]+$ ]] && (( sample >= 10 && sample <= 900 )) || {
  echo "sample interval must be 10..900 seconds" >&2
  exit 1
}
for path in "$TRNM_ENDURANCE_AUTHORIZATION" "$TRNM_ENDURANCE_WORKLOAD" "$TRNM_ENDURANCE_METRICS" "$TRNM_ENDURANCE_THRESHOLDS" "$TRNM_WORLD_BINARY" "$TRNM_ENDURANCE_CONFIG" "$TRNM_ENDURANCE_TOPOLOGY"; do
  [[ -f "$path" && ! -L "$path" && -s "$path" ]] || { echo "missing, linked or empty input: $path" >&2; exit 1; }
done
[[ -x "$TRNM_ENDURANCE_WORKLOAD" ]] || { echo "workload driver must be executable" >&2; exit 1; }
[[ -x "$TRNM_ENDURANCE_METRICS" ]] || { echo "metrics collector must be executable" >&2; exit 1; }
if [[ -n "${TRNM_ENDURANCE_STOP:-}" ]]; then
  [[ -f "$TRNM_ENDURANCE_STOP" && ! -L "$TRNM_ENDURANCE_STOP" && -x "$TRNM_ENDURANCE_STOP" ]] || { echo "unsafe stop hook" >&2; exit 1; }
fi

umask 077
mkdir -p "$out"/{inputs,metrics,workload,summary}
[[ -z "$(find "$out" -type f -print -quit)" ]] || { echo "output directory must not contain files" >&2; exit 1; }
cp "$TRNM_ENDURANCE_AUTHORIZATION" "$out/inputs/authorization.record"
cp "$TRNM_ENDURANCE_THRESHOLDS" "$out/inputs/thresholds.record"
cp "$TRNM_ENDURANCE_CONFIG" "$out/inputs/configuration.record"
cp "$TRNM_ENDURANCE_TOPOLOGY" "$out/inputs/topology.record"
sha256sum "$TRNM_WORLD_BINARY" "$TRNM_ENDURANCE_WORKLOAD" "$TRNM_ENDURANCE_METRICS" "$TRNM_ENDURANCE_AUTHORIZATION" "$TRNM_ENDURANCE_THRESHOLDS" "$TRNM_ENDURANCE_CONFIG" "$TRNM_ENDURANCE_TOPOLOGY" > "$out/inputs/SHA256SUMS"

started_epoch="$(date -u +%s)"
started_rfc3339="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf '%s\n' "$started_rfc3339" > "$out/summary/started-at.txt"
printf '%s\n' "$duration" > "$out/summary/required-duration-seconds.txt"
printf '%s\n' "$sample" > "$out/summary/sample-interval-seconds.txt"

cleanup_needed=1
workload_pid=""
cleanup() {
  rc=$?
  set +e
  if [[ -n "$workload_pid" ]] && kill -0 "$workload_pid" 2>/dev/null; then
    kill -TERM "$workload_pid" 2>/dev/null
    for _ in $(seq 1 30); do kill -0 "$workload_pid" 2>/dev/null || break; sleep 1; done
    kill -KILL "$workload_pid" 2>/dev/null || true
  fi
  if [[ -n "${TRNM_ENDURANCE_STOP:-}" ]]; then
    timeout 300 "$TRNM_ENDURANCE_STOP" > "$out/summary/stop.stdout" 2> "$out/summary/stop.stderr" || true
  fi
  ended_epoch="$(date -u +%s)"
  printf '%s\n' "$rc" > "$out/summary/harness-exit-code.txt"
  printf '%s\n' "$((ended_epoch-started_epoch))" > "$out/summary/observed-duration-seconds.txt"
  exit "$rc"
}
trap cleanup EXIT INT TERM HUP

"$TRNM_ENDURANCE_WORKLOAD" > "$out/workload/stdout.log" 2> "$out/workload/stderr.log" &
workload_pid=$!
printf '%s\n' "$workload_pid" > "$out/workload/pid.txt"

sample_index=0
while true; do
  now="$(date -u +%s)"
  elapsed=$((now-started_epoch))
  if ! kill -0 "$workload_pid" 2>/dev/null; then
    set +e
    wait "$workload_pid"
    workload_rc=$?
    set -e
    printf '%s\n' "$workload_rc" > "$out/workload/exit-code.txt"
    echo "workload terminated before endurance duration" >&2
    exit 1
  fi
  sample_index=$((sample_index+1))
  sample_dir="$out/metrics/$(printf '%08d' "$sample_index")"
  mkdir -p "$sample_dir"
  printf '%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$sample_dir/timestamp.txt"
  printf '%s\n' "$elapsed" > "$sample_dir/elapsed-seconds.txt"
  set +e
  timeout "$((sample-1))" "$TRNM_ENDURANCE_METRICS" > "$sample_dir/metrics.json" 2> "$sample_dir/stderr.log"
  metrics_rc=$?
  set -e
  printf '%s\n' "$metrics_rc" > "$sample_dir/exit-code.txt"
  [[ "$metrics_rc" -eq 0 && -s "$sample_dir/metrics.json" ]] || {
    echo "metrics collection failed at sample $sample_index" >&2
    exit 1
  }
  if (( elapsed >= duration )); then
    break
  fi
  sleep_for=$sample
  remaining=$((duration-elapsed))
  (( remaining < sleep_for )) && sleep_for=$remaining
  sleep "$sleep_for"
done

kill -TERM "$workload_pid"
for _ in $(seq 1 60); do kill -0 "$workload_pid" 2>/dev/null || break; sleep 1; done
if kill -0 "$workload_pid" 2>/dev/null; then
  echo "workload did not terminate cleanly" >&2
  exit 1
fi
set +e
wait "$workload_pid"
workload_rc=$?
set -e
printf '%s\n' "$workload_rc" > "$out/workload/exit-code.txt"
[[ "$workload_rc" -eq 0 ]] || { echo "workload returned nonzero after endurance" >&2; exit 1; }
workload_pid=""

ended_epoch="$(date -u +%s)"
ended_rfc3339="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
observed=$((ended_epoch-started_epoch))
(( observed >= 86400 && observed >= duration )) || { echo "observed duration is below required floor" >&2; exit 1; }
printf '%s\n' "$ended_rfc3339" > "$out/summary/ended-at.txt"
printf '%s\n' "$observed" > "$out/summary/observed-duration-seconds.txt"
printf '%s\n' "$sample_index" > "$out/summary/sample-count.txt"

# The accountable environment owner provides a checker that evaluates the
# predeclared thresholds against all raw samples. It must not alter the input
# files or infer a pass from missing metrics.
: "${TRNM_ENDURANCE_EVALUATOR:?TRNM_ENDURANCE_EVALUATOR is required after the run}"
[[ -f "$TRNM_ENDURANCE_EVALUATOR" && ! -L "$TRNM_ENDURANCE_EVALUATOR" && -x "$TRNM_ENDURANCE_EVALUATOR" ]] || { echo "unsafe evaluator" >&2; exit 1; }
sha256sum "$TRNM_ENDURANCE_EVALUATOR" > "$out/inputs/evaluator.sha256"
"$TRNM_ENDURANCE_EVALUATOR" "$out/inputs/thresholds.record" "$out/metrics" > "$out/summary/evaluation.json"
[[ -s "$out/summary/evaluation.json" ]] || { echo "empty threshold evaluation" >&2; exit 1; }
python3 - "$out/summary/evaluation.json" <<'PY'
import json, pathlib, sys
value=json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
if value.get('all_thresholds_passed') is not True:
    raise SystemExit('one or more predeclared thresholds failed')
if value.get('missing_samples') not in (0, []):
    raise SystemExit('threshold evaluation reports missing samples')
PY

cleanup_needed=0
trap - EXIT INT TERM HUP
find "$out" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/summary/ARTIFACT-SHA256SUMS"
cat > "$out/summary/RESULT.txt" <<EOF
TRNM_WORLD_ENDURANCE_24H=COMPLETE
started_at_utc=$started_rfc3339
ended_at_utc=$ended_rfc3339
observed_duration_seconds=$observed
sample_count=$sample_index
production_authorization=not_granted
independent_review_required=true
strict_evidence_validation_required=true
EOF

echo "24-hour harness completed. Independent review and strict evidence-record validation remain required."
