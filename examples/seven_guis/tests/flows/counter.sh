#!/bin/sh
# A stand flow in shell alone, on the Counter: start a session, read the screen, click the
# button the screen lists, read the change in the answer and on the screen, see a click on an
# id no screen reads refused, stop, and see a command after the stop fail as a session error.
#
# Arguments: the escher-session binary, and a state directory that does not exist yet.
# Exits 0 when every step read as expected; otherwise prints the failing step and exits 1.
# Uses nothing beyond the shell's own `case`, `$(...)` and functions.

bin=$1
dir=$2

fail() {
    echo "step $1"
    "$bin" stop --session "$dir" >/dev/null 2>&1
    exit 1
}

# 1: start answers the session's label and its idle expiry
out=$("$bin" start counter --session "$dir") || fail 1
case $out in
    '{"label":"counter",'*'"idle_expiry_s":1800}') ;;
    *) fail 1 ;;
esac

# 2: the screen reads 0
out=$("$bin" snapshot --session "$dir") || fail 2
case $out in
    *'Paragraph \"0\" id=\"counter-value\"'*) ;;
    *) fail 2 ;;
esac

# 3: a click on the button, and the answer names the value it changed
out=$("$bin" click --id counter-increment --session "$dir") || fail 3
case $out in
    '{"settled":true,"added":[],"removed":[],"changed":[{"id":"counter-value",'*'"name":"1",'*) ;;
    *) fail 3 ;;
esac

# 4: the screen reads 1
out=$("$bin" snapshot --session "$dir") || fail 4
case $out in
    *'Paragraph \"1\" id=\"counter-value\"'*) ;;
    *) fail 4 ;;
esac

# 5: a click on an id no screen reads is refused, by status 1 and by its cause
out=$("$bin" click --id flow-names-no-element --session "$dir")
case $? in
    1) ;;
    *) fail 5 ;;
esac
case $out in
    '{"refused":{"cause":"not-found",'*) ;;
    *) fail 5 ;;
esac

# 6: the refused click changed nothing
out=$("$bin" snapshot --session "$dir") || fail 6
case $out in
    *'Paragraph \"1\" id=\"counter-value\"'*) ;;
    *) fail 6 ;;
esac

# 7: stop
out=$("$bin" stop --session "$dir") || fail 7
case $out in
    '{"stopped":true}') ;;
    *) fail 7 ;;
esac

# 8: a command after the stop is a session error, by status 3
out=$("$bin" snapshot --session "$dir" 2>/dev/null)
case $? in
    3) ;;
    *) fail 8 ;;
esac
case $out in
    '{"error":{"kind":"no-session",'*) ;;
    *) fail 8 ;;
esac

exit 0
