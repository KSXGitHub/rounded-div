#! /bin/bash
set -o errexit -o pipefail
if [[ -n "$FEATURES" ]]; then
  exec cargo clippy "$FEATURES" -- -D warnings
else
  exec cargo clippy -- -D warnings
fi
