#!/bin/sh
# A stand flow in shell alone, on the Flight Booker: start a session, type a text that is no
# date over the date the departure field holds, see the booking refused as disabled, type a
# date, book, read the confirmation in the answer, and stop.
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

# 1: start
out=$("$bin" start flight-booker --session "$dir") || fail 1
case $out in
    '{"label":"flight-booker",'*) ;;
    *) fail 1 ;;
esac

# 2: the departure field holds a date, and the booking is enabled
out=$("$bin" snapshot --session "$dir") || fail 2
case $out in
    *'id=\"flight-start\" enabled value=\"01.01.2026\"'*'id=\"flight-book\" enabled'*) ;;
    *) fail 2 ;;
esac

# 3: typing replaces what the field holds; a text that is no date disables the booking
out=$("$bin" type --id flight-start --text 'next week' --session "$dir") || fail 3
case $out in
    '{"settled":true,'*'"id":"flight-start",'*'"value":"next week",'*'"id":"flight-book",'*'"enabled":false,'*) ;;
    *) fail 3 ;;
esac

# 4: the booking is refused, by status 1 and by its cause
out=$("$bin" click --id flight-book --session "$dir")
case $? in
    1) ;;
    *) fail 4 ;;
esac
case $out in
    '{"refused":{"cause":"disabled",'*) ;;
    *) fail 4 ;;
esac

# 5: typing a date replaces the text and enables the booking
out=$("$bin" type --id flight-start --text 24.12.2026 --session "$dir") || fail 5
case $out in
    '{"settled":true,'*'"id":"flight-start",'*'"value":"24.12.2026",'*'"id":"flight-book",'*'"enabled":true,'*) ;;
    *) fail 5 ;;
esac

# 6: the booking is accepted, and the answer adds the confirmation
out=$("$bin" click --id flight-book --session "$dir") || fail 6
case $out in
    '{"settled":true,"added":[{"id":"flight-booked",'*'"name":"Booked one-way flight on 24.12.2026",'*) ;;
    *) fail 6 ;;
esac

# 7: stop
out=$("$bin" stop --session "$dir") || fail 7
case $out in
    '{"stopped":true}') ;;
    *) fail 7 ;;
esac

exit 0
