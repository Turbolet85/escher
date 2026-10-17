#!/usr/bin/env bash
# The one definition of every linux CI leg: .github/workflows/ci.yml and the dev host both run
# `bash .github/scripts/ci-leg.sh <leg>` from the repository root. Each leg's merged output is
# written to target/ci-logs/<leg>.log (truncated at the leg's start) and the script exits with
# the leg command's own status.
set -euo pipefail

LEGS=(fmt clippy test ci-scripts build msrv counter wasm doc audit a11y coverage stand-repeat fast)
FAST_LEGS=(fmt clippy test ci-scripts)
SCRIPTS="$(dirname "$0")"

usage() {
    echo "usage: $0 <leg>" >&2
    echo "legs: ${LEGS[*]}" >&2
    exit 2
}

run_leg() {
    case "$1" in
    fmt) cargo fmt --all --check ;;
    clippy)
        # Upstream's own blitz-dom line beside the workspace's: its tests and benches, without text-transform-icu.
        cargo clippy --workspace --locked -- -D warnings &&
            cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings
        ;;
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
    doc) cargo doc --workspace --no-deps --locked ;;
    audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
    a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids ;;
    coverage)
        # keyboard_compat.rs is upstream's temporary keyboard-types 0.7/0.8 bridge in dioxus-native-dom,
        # excluded by filename until upstream retires it.
        mkdir -p target/coverage &&
            cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' --lcov --output-path target/coverage/lcov.info &&
            cargo llvm-cov report --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' &&
            python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
        ;;
    stand-repeat)
        # The flakiness budget is zero failures: the first red command ends the leg, and nothing is retried.
        local f round stand=()
        for f in "$SCRIPTS"/../../tests/blitz-tests/tests/stand_*.rs; do
            stand+=(--test "$(basename "$f" .rs)")
        done
        for round in 1 2 3; do
            echo "stand-repeat: round $round of 3"
            cargo test -p blitz-tests --locked "${stand[@]}" || return
            cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test host_timer --test mcp_server --test mcp_log || return
        done
        echo "stand-repeat: 3 of 3 rounds green"
        ;;
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
