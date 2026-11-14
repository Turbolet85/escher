@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: or `Session::recording_into`, one `check-{n}.json` (`0600`, `create_new`, never replaced) per recorded session in a `0700` launcher-named directory
source: ctr-mlxzdu6r «Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory»
replace-with: or `Session::recording_into` or an exploration's record directory, one `check-{n}.json` (`0600`, `create_new`, never replaced) per recorded session, each explored path its own, in a `0700` launcher-named directory
@leaf_edit n=2 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: nothing written or spawned) — per answered call its verb
source: ctr-lq4urcp2 «A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.»
replace-with: nothing written or spawned) — an opening record of the app label and the format, then per answered call its verb
@leaf_edit n=3 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: read by the strict `Check::from_bytes` (closed keys,
source: ctr-kwvgqdv5 «The array opens with `{"tool":"start","arguments":{"app":…},"format":1}`; a check without format reads as format 1, and any other format value, of whatever JSON type, is refused malformed, holding nothing of the bytes.»
replace-with: read by the strict `Check::from_bytes` (closed keys, the opening `start` record's `format` 1 or absent read as 1, any other value of whatever JSON type refused `malformed` holding nothing of the bytes,
@leaf_edit n=4 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: the role adding no span — a boundary widening, ratified;
source: dec-2v6viytv «An exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, NodeId, screen text or screen digest, and it is never logged or printed.»
replace-with: the role adding no span — a boundary widening, ratified; an exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, `NodeId`, screen text or screen digest, and is never logged or printed;
