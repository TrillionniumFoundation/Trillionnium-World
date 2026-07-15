#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

require_line() {
  local file="$1" line="$2"
  if ! grep -Fqx -- "$line" "$file"; then
    echo "missing resource budget in ${file#$ROOT_DIR/}: $line" >&2
    exit 1
  fi
}

game_unit="$ROOT_DIR/deploy/systemd/trnm-game-server.service"
signer_unit="$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service"
for setting in CPUAccounting=true CPUWeight=200 CPUQuota=200% \
  MemoryAccounting=true MemoryHigh=384M MemoryMax=512M MemorySwapMax=128M \
  IOAccounting=true IOWeight=200 TasksAccounting=true TasksMax=256; do
  require_line "$game_unit" "$setting"
done
for setting in CPUAccounting=true CPUWeight=100 CPUQuota=50% \
  MemoryAccounting=true MemoryHigh=64M MemoryMax=96M MemorySwapMax=32M \
  IOAccounting=true IOWeight=100 TasksAccounting=true TasksMax=128; do
  require_line "$signer_unit" "$setting"
done

capacity_script="$ROOT_DIR/scripts/check-trnm-online-capacity-soak.sh"
bash -n "$capacity_script"
for setting in 'CPUQuota=150%' 'MemoryHigh=1536M' 'MemoryMax=2048M' \
  'MemorySwapMax=512M' 'TasksMax=512' 'TRNM_CAPACITY_MIN_AVAILABLE_MIB:-3072'; do
  rg -Fq -- "$setting" "$capacity_script"
done
rg -Fq 'register_cleanup_process "$!" group "$(command -v timeout)"' \
  "$capacity_script"
if rg -Fq 'register_cleanup_process "$!" group "$(command -v setsid)"' \
    "$capacity_script"; then
  echo "capacity worker cleanup must bind the stable timeout wrapper" >&2
  exit 1
fi
rg -q 'GAME_SERVER_DATABASE_MIN_CONNECTIONS: u32 = 12' \
  "$ROOT_DIR/trillionnium/crates/trnm-game-server/src/lib.rs"
rg -q 'GAME_SERVER_DATABASE_MAX_CONNECTIONS: u32 = 12' \
  "$ROOT_DIR/trillionnium/crates/trnm-game-server/src/lib.rs"
rg -q 'READINESS_DATABASE_MIN_CONNECTIONS: u32 = 4' \
  "$ROOT_DIR/trillionnium/crates/trnm-game-server/src/lib.rs"
rg -q 'READINESS_DATABASE_MAX_CONNECTIONS: u32 = 12' \
  "$ROOT_DIR/trillionnium/crates/trnm-game-server/src/lib.rs"
rg -q 'SIGNER_DATABASE_MAX_CONNECTIONS: u32 = 4' \
  "$ROOT_DIR/trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs"

installed=false
if [[ "${TRNM_REQUIRE_INSTALLED_RESOURCE_BUDGETS:-0}" == 1 ]]; then
  installed=true
  [[ "$(systemctl --user show trnm-game-server.service -p CPUQuotaPerSecUSec --value)" == 2s ]]
  [[ "$(systemctl --user show trnm-game-server.service -p MemoryHigh --value)" == 402653184 ]]
  [[ "$(systemctl --user show trnm-game-server.service -p MemoryMax --value)" == 536870912 ]]
  [[ "$(systemctl --user show trnm-entitlement-signer.service -p CPUQuotaPerSecUSec --value)" == 500ms ]]
  [[ "$(systemctl --user show trnm-entitlement-signer.service -p MemoryHigh --value)" == 67108864 ]]
  [[ "$(systemctl --user show trnm-entitlement-signer.service -p MemoryMax --value)" == 100663296 ]]
  probe="$(TRNM_CAPACITY_SCOPE_PROBE=1 "$capacity_script")"
  jq -e '.status == "passed"
    and .memory_high_bytes == 1610612736
    and .memory_max_bytes == 2147483648
    and .memory_swap_max_bytes == 536870912
    and .cpu_max == "150000 100000"
    and .tasks_max == 512' >/dev/null <<<"$probe"
fi

jq -n --argjson installed "$installed" \
  '{status:"passed",game_server_data_pool_min:12,game_server_data_pool_max:12,
    game_server_readiness_pool_min:4,game_server_readiness_pool_max:12,signer_pool_max:4,
    game_server_total_pool_max:24,formal_database_connection_ceiling:40,
    game_server_memory_max_mib:512,capacity_harness_memory_max_mib:2048,
    capacity_harness_min_available_memory_mib:3072,
    systemd_unit_budgets:true,installed_runtime_verified:$installed}'
