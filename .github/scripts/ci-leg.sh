#!/usr/bin/env bash
# The one definition of every linux CI leg: .github/workflows/ci.yml and the dev host both run
# `bash .github/scripts/ci-leg.sh <leg>` from the repository root. Each leg's merged output is
# written to target/ci-logs/<leg>.log (truncated at the leg's start) and the script exits with
# the leg command's own status.
set -euo pipefail

LEGS=(fmt clippy test ci-scripts build msrv counter wasm doc fast)
FAST_LEGS=(fmt clippy test ci-scripts)

usage() {
    echo "usage: $0 <leg>" >&2
    echo "legs: ${LEGS[*]}" >&2
    exit 2
}

run_leg() {
    case "$1" in
    fmt) cargo fmt --all --check ;;
    clippy) cargo clippy --workspace --locked -- -D warnings ;;
    test) cargo test --workspace --locked ;;
    ci-scripts) python3 -m unittest discover -s .github/scripts ;;
    build) cargo build --workspace --locked ;;
    msrv) cargo +1.91 build --workspace --locked ;;
    counter) cargo build -p counter --locked ;;
    wasm)
        # examples/wasm_hello is a standalone workspace with no Cargo.lock, so it cannot take --locked.
        (cd examples/wasm_hello && cargo build --target wasm32-unknown-unknown) &&
            cargo build -p seven_guis --lib --target wasm32-unknown-unknown --no-default-features --features hybrid --locked &&
            cargo build -p todomvc --lib --target wasm32-unknown-unknown --no-default-features --features hybrid --locked
        ;;
    doc) cargo doc --locked ;;
    esac
}

is_leg() {
    local l
    for l in "${LEGS[@]}"; do
        [[ "$l" == "$1" ]] && return 0
    done
    return 1
}

[[ $# -eq 1 ]] && is_leg "$1" || usage
leg="$1"

if [[ "$leg" == fast ]]; then
    for l in "${FAST_LEGS[@]}"; do
        bash "$0" "$l"
    done
    exit 0
fi

export RUSTDOCFLAGS="-D warnings"
mkdir -p target/ci-logs
run_leg "$leg" 2>&1 | tee "target/ci-logs/$leg.log"
