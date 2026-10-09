#!/usr/bin/env bash
# The one definition of fork CI's package install: every job of .github/workflows/ci.yml that
# needs a package runs `bash .github/scripts/apt-install.sh <package>...`. A stalled mirror holds
# `apt-get update` with no error line (actions/runner-images#14594, #14823), so each apt-get call
# is bounded and a failed attempt is tried again: up to ATTEMPTS attempts of update then install,
# each call ended BOUND seconds in (killed GRACE seconds later if it has not ended), PAUSE seconds
# between attempts. Exits 0 when an attempt completes, 1 when every attempt failed, 2 on usage.
set -euo pipefail

ATTEMPTS=3
BOUND=120
PAUSE=10
GRACE=10

usage() {
    echo "usage: $0 [--attempts N] [--bound SECONDS] [--pause SECONDS] <package>..." >&2
    exit 2
}

is_count() {
    [[ "$1" =~ ^(0|[1-9][0-9]{0,5})$ ]]
}

packages=()
while [[ $# -gt 0 ]]; do
    case "$1" in
    --attempts | --bound | --pause)
        [[ $# -ge 2 ]] && is_count "$2" || usage
        case "$1" in
        --attempts) ATTEMPTS="$2" ;;
        --bound) BOUND="$2" ;;
        --pause) PAUSE="$2" ;;
        esac
        shift 2
        ;;
    *)
        [[ "$1" =~ ^[a-z0-9][a-z0-9+.-]+$ ]] || usage
        packages+=("$1")
        shift
        ;;
    esac
done
[[ ${#packages[@]} -ge 1 && "$ATTEMPTS" -ge 1 && "$BOUND" -ge 1 ]] || usage

# The bound sits inside the privilege, so the process it signals is the package manager itself.
bounded() {
    sudo timeout --kill-after="$GRACE" "$BOUND" apt-get "$@"
}

attempt_once() {
    phase=update
    bounded update || return
    phase=install
    bounded install -y "${packages[@]}"
}

for ((attempt = 1; attempt <= ATTEMPTS; attempt++)); do
    attempt_once && exit 0
    status=$?
    echo "apt-install: attempt $attempt of $ATTEMPTS failed in $phase (exit $status)" >&2
    if ((attempt < ATTEMPTS)); then
        sleep "$PAUSE"
    fi
done
exit 1
