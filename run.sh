#! /bin/bash
set -o errexit -o pipefail -o nounset
exec cargo run --features=cli --bin="$1" -- "${@:2}"
