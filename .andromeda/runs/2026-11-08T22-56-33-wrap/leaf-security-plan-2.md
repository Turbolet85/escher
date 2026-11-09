@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: each step still validated inside `Session::run`;
source: fct-cz244xre «`escher-session coverage <app> <check>...`, the binary's coverage role — no `--session` or `--record`, no socket — reads each caller-named path as untrusted input through a bounded read»
replace-with: each step still validated inside `Session::run`; since 2026-11-08-handler-coverage the `escher-session coverage <app> <check>...` role (no `--session` or `--record`, no socket) is a second reader of caller-named checks, each read through `File::open(..).take(MAX_CHECK_BYTES + 1)` then `Check::from_bytes`, a file that cannot be read, one past the bound, one that is no check or a check of another app refused `malformed` (status 1) holding nothing of the path or the file, each accepted check replayed on a fresh in-process session, its one stdout line `{"unexercised":[{"id","owner","event"}],"checks":n,"held":m}` holding ids, owner ids, event names and the two counts only (no control value, typed text, path or `NodeId`), the role adding no span — a boundary widening, ratified;
