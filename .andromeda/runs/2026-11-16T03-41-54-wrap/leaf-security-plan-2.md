@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: `graph` and `actions` wait on none.
source: dec-bcjl3glf «The session's layout read takes its id from Rust code alone, resolves it through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings»
replace-with: `graph` and `actions` wait on none; the session's in-process layout read takes its id from Rust code alone, resolves it through the run path's id resolution (refusing `stale` or `not-found`), runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a `NodeId`, text or value.
@leaf_edit n=2 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: an actions answer's ids, owner ids, verb words and causes included
source: dec-gylmjuym «No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer and»
replace-with: an actions answer's ids, owner ids, verb words and causes and a layout answer, from a driver call or an in-process read, included
