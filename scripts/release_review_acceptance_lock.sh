#!/usr/bin/env bash

trnm_acquire_release_review_acceptance_lock() {
  local acceptance_dir="$1"
  if [[ "${TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_HELD:-0}" == "1" ]]; then
    return 0
  fi

  mkdir -p "$acceptance_dir"
  local lock_file="${TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_FILE:-$acceptance_dir/.release-review-acceptance.lock}"
  exec {TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_FD}>"$lock_file"
  flock "$TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_FD"
  export TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_FILE="$lock_file"
  export TRNM_RELEASE_REVIEW_ACCEPTANCE_LOCK_HELD=1
}
