@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=22 class=omission
leaf-text: a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records with closed keys,
source: ctr-kwvgqdv5 «The array opens with `{"tool":"start","arguments":{"app":…},"format":1}`; a check without format reads as format 1, and any other format value, of whatever JSON type, is refused malformed, holding nothing of the bytes.»
replace-with: a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records with closed keys, opening with a `start` record of the app and `format` 1 (a check without `format` read as format 1, any other value of whatever JSON type refused `malformed`, holding nothing of the bytes),
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=32 class=omission
leaf-text: under `--record <dir>` or `Session::recording_into`: one `check-{n}.json`
source: ctr-mlxzdu6r «Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.»
replace-with: under `--record <dir>`, `Session::recording_into` or an exploration's record directory: one `check-{n}.json` per recorded session, each explored path its own,
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=32 class=omission
leaf-text: never logged, unix only)
source: dec-2v6viytv «Exploration report holds closed content only — An exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, NodeId, screen text or screen digest, and it is never logged or printed.»
replace-with: never logged, unix only; an exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, `NodeId`, screen text or screen digest, and is never logged or printed)
