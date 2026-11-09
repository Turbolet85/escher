@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=21 class=omission
leaf-text: whose end writes nothing on stdout — status 0, or 3 with a fixed message on stderr.
source: fct-jwfmxwld «The first word `coverage` selects the coverage role, its one shape `coverage <app> <check>...` — an app the binary boots, then one or more paths, no `--session` and no `--record` word among them»
replace-with: whose end writes nothing on stdout — status 0, or 3 with a fixed message on stderr; since 2026-11-08-handler-coverage the first word `coverage` selects the coverage role, its one shape `coverage <app> <check>...` (no `--session` or `--record`, no socket; any other shape usage, status 2), each caller-named path read through `File::open(..).take(MAX_CHECK_BYTES + 1)` and `Check::from_bytes`, a file that cannot be read, one past the bound, one that is no check or a check of another app refused `malformed`, status 1, holding nothing of the path or the file, a failed boot the session error, status 3, and an accepted run one line `{"unexercised":[…],"checks":n,"held":m}` on stdout, status 0, holding ids, owner ids, event names and the two counts only — a boundary widening, ratified.
