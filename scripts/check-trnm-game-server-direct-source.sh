#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
crate="${repo_root}/trillionnium/crates/trnm-game-server"
manifest="${crate}/Cargo.toml"
lib="${crate}/src/lib.rs"

fail() {
  printf 'direct-source gate failed: %s\n' "$*" >&2
  exit 1
}

[[ -f "${manifest}" ]] || fail "missing ${manifest}"
[[ -f "${lib}" ]] || fail "missing ${lib}"
[[ ! -e "${crate}/build.rs" ]] || fail "semantic or implicit build.rs remains"
[[ ! -e "${crate}/src/lib.rs.in" ]] || fail "template source src/lib.rs.in remains"
[[ ! -e "${crate}/src/settlement_worker.rs.in" ]] || fail "settlement worker template remains"

if grep -Eq '^\s*build\s*=\s*"build\.rs"\s*$' "${manifest}"; then
  fail "Cargo manifest still declares build.rs"
fi
if grep -Eq 'OUT_DIR|trnm_game_server_lib_generated|include!\s*\(\s*concat!' "${lib}"; then
  fail "crate root still depends on generated OUT_DIR source"
fi
if grep -R --include='*.rs' --include='*.toml' --include='*.sh' \
    -nE 'src/lib\.rs\.in|trnm_game_server_lib_generated\.rs|semantic rewrite' \
    "${crate}" "${repo_root}/scripts" | grep -v 'check-trnm-game-server-direct-source.sh'; then
  fail "legacy generated-source authority references remain"
fi
if grep -R --include='*.rs' -n 'reconcile_economy(&state\.cex' "${crate}/src"; then
  fail "legacy synchronous campaign settlement call remains"
fi
if grep -R --include='*.rs' -n 'settle_pending_matches(&settlement_state' "${crate}/src"; then
  fail "legacy in-server terminal settlement loop remains"
fi

bytes="$(wc -c < "${lib}")"
[[ "${bytes}" -gt 100000 ]] || fail "direct library source is unexpectedly small (${bytes} bytes)"
printf 'game_server_direct_source=passed lib_bytes=%s\n' "${bytes}"
