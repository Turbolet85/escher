@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`: never log or field them.
source: dec-bcjl3glf «The session's layout read takes its id from Rust code alone, resolves it through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings»
replace-with: Since 2026-11-16-layout-explained the session's in-process layout read takes its id from Rust code alone, resolves it through the run path's id resolution refusing `stale` or `not-found`, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a `NodeId`, text or value. `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`: never log or field them.
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: verb words and causes reaching no log).
source: dec-gylmjuym «No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer»
replace-with: verb words and causes reaching no log; an in-process layout read's answer reaching no log line at any level either).
