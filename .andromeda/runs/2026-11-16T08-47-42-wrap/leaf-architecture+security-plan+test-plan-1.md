@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=19 class=omission
leaf-text: An instance that did not go quiet is a result (`settled: false` with its `Busy` class)
source: ctr-u4kw7erq «A busy class word of an instance that did not go quiet, and of an acting step whose settle panicked a second time.»
replace-with: An instance that did not go quiet, or an acting step whose settle panicked a second time (read as `render`), is a result (`settled: false` with its `Busy` class)
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=20 class=stale
leaf-text: (`busy` only when not settled)
source: ctr-t5m57o7i «An object keyed by its verb's result fields in the table's order, busy present only when not settled and caused only when the step caused something.»
replace-with: (`busy` only when not settled, `caused` only when the step caused something)
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=24 class=omission
leaf-text: `added`, `removed`, `changed` (nodes carry the `NODE_FIELDS`)
source: ctr-wkieu5ai «Present only when the step caused something: one item per kind (panic, error, log) and level (error, warn, info, debug, trace), with the first reached handler's id, owner id and event name»
replace-with: `added`, `removed`, `changed` (nodes carry the `NODE_FIELDS`), and, since 2026-11-16-errors-with-the-action, `caused`, present only when the step caused something: one item per kind (`panic`, `error`, `log`) and level (`error`, `warn`, `info`, `debug`, `trace`), with the first reached handler's id, owner id and event name where one was reached, and a count, limited to the app root boundary and tracing records, not the `log` crate
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=32 class=omission
leaf-text: no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening.
source: dec-kb2z7qfy «Each acting verb's answer (click, type, press, advance, scroll) carries caused on every ratified carrier only when the step caused something: by kind and level, closed kind and level words»
replace-with: no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening. Since 2026-11-16-errors-with-the-action each acting verb's answer (`click`, `type`, `press`, `advance`, `scroll`) carries `caused` on every ratified carrier only when the step caused something — closed kind and level words, the first reached handler's id, owner and event, and a count, never a message, payload, target or field value: a ratified boundary widening; a handler `Err` the app's root error boundary takes crosses only as screen content, through Dioxus's default boundary rendering of its Display, never through `caused`, which reports it as one error item holding no byte of its text.
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=49 class=stale
leaf-text: The crate installs no subscriber: with none the span is disabled
source: dec-xxvmhvjw «The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.»
replace-with: The crate installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, which prints nothing: with no subscriber the span is disabled
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=50 class=omission
leaf-text: give each test its own recorder
source: dec-5oq63xdv «A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder.»
replace-with: give each test its own recorder, and open an empty log capture before it, so the capture's dispatchers are registered before the recorder
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: a panicked path's check holding the steps before the panic
source: ctr-wrq2o6ky «a path whose accepted answer carries a panic item ends panicked with no payload, a re-reach step diverged, and its check holds the panicked step.»
replace-with: a path whose accepted answer carries a panic item ending panicked with no payload, a re-reach step diverged, and its check holding the panicked step
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: `stand_session_quiet`'s host installs no log sink
source: dec-xd4p3k72 «A handler Err the app's root boundary takes is checked on a session of its own, its sentinel check reading caused alone.»
replace-with: Since 2026-11-16-errors-with-the-action a handler `Err` the app's root boundary takes is checked on a session of its own (`stand_act_errors`), its sentinel check reading `caused` alone. `stand_session_quiet`'s host installs no log sink
