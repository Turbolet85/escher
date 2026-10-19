#!/bin/sh
# Records the two committed Counter checks through the binary: one session driven through the
# command line and one through the `mcp` role, each kept by `--record`, then moved into the
# output directory as `counter-cli.json` and `counter-mcp.json`.
#
# To re-record after a deliberate change to the Counter, from the repository root:
#   cargo build --locked -p seven_guis --bin escher-session
#   sh examples/seven_guis/tests/flows/record_checks.sh target/debug/escher-session tests/blitz-tests/checks
#
# Arguments: the escher-session binary, and an existing output directory.
# Exits 0 when both checks were written; otherwise prints the failing step and exits 1.
# Prints nothing a screen reads.

bin=$1
out=$2
scratch=$out/rec

fail() {
    echo "step $1"
    "$bin" stop --session "$scratch/s" >/dev/null 2>&1
    rm -rf "$scratch"
    exit 1
}

# Short relative names keep the socket's path under its bound.
rm -rf "$scratch"
mkdir -m 700 "$scratch" || fail 1

# The command line: two clicks an `advance` apart, so they read as two single clicks.
"$bin" start counter --session "$scratch/s" --record "$scratch/c" >/dev/null || fail 2
"$bin" snapshot --session "$scratch/s" >/dev/null || fail 3
"$bin" click --id counter-increment --session "$scratch/s" >/dev/null || fail 4
"$bin" advance --ms 600 --session "$scratch/s" >/dev/null || fail 5
"$bin" click --id counter-increment --session "$scratch/s" >/dev/null || fail 6
"$bin" snapshot --session "$scratch/s" >/dev/null || fail 7
"$bin" screenshot --session "$scratch/s" >/dev/null || fail 8
"$bin" stop --session "$scratch/s" >/dev/null || fail 9
mv "$scratch/c/check-1.json" "$out/counter-cli.json" || fail 10

# The `mcp` role: the second Tab focuses the Counter's button, which Enter and Space press.
replies=$(printf '%s\n' \
    '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"record-checks","version":"1"}}}' \
    '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
    '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"start","arguments":{"app":"counter"}}}' \
    '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"snapshot","arguments":{}}}' \
    '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"press","arguments":{"key":"tab"}}}' \
    '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"press","arguments":{"key":"tab"}}}' \
    '{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"press","arguments":{"key":"enter"}}}' \
    '{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"press","arguments":{"key":"space"}}}' \
    '{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"advance","arguments":{"ms":100}}}' \
    '{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"scroll","arguments":{"id":"counter-increment"}}}' \
    '{"jsonrpc":"2.0","id":10,"method":"tools/call","params":{"name":"screenshot","arguments":{}}}' \
    '{"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"name":"snapshot","arguments":{}}}' \
    '{"jsonrpc":"2.0","id":12,"method":"tools/call","params":{"name":"stop","arguments":{}}}' |
    "$bin" mcp --record "$scratch/m") || fail 11
case $replies in
    *'"isError":true'* | *'"error":'*) fail 12 ;;
    *'"id":12,"result":'*) ;;
    *) fail 12 ;;
esac
mv "$scratch/m/check-1.json" "$out/counter-mcp.json" || fail 13

rm -rf "$scratch"
exit 0
