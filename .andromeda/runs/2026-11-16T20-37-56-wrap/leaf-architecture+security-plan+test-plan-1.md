@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: one item per kind (`panic`, `error`, `log`) and level (`error`, `warn`, `info`, `debug`, `trace`), with the first reached handler's id, owner id and event name where one was reached, and a count, limited to the app root boundary and tracing records, not the `log` crate;
source: ctr-wkieu5ai «contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes»
replace-with: an item per panic, error or record — its kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's scrubbed fields and the first reached handler — panics, then errors new since the boundary marks, then records, 16 kept, and `caused_past`, written after `caused` only when non-empty, one entry per kind and source counting the items past `caused`'s 16, a panic or error entry writing no source;
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=20 class=omission
leaf-text: (`busy` only when not settled, `caused` only when the step caused something)
source: ctr-t5m57o7i «contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.»
replace-with: (`busy` only when not settled, `caused` only when the step caused something and `caused_past` only when entries pass `caused`'s 16)
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=32 class=retired
leaf-text: closed kind and level words, the first reached handler's id, owner and event, and a count, never a message, payload, target or field value: a ratified boundary widening;
source: dec-vztg2sue «decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier only when the step caused something: one item per panic, boundary error and kept record, holdin»
replace-with: one item per panic, boundary error and kept record, holding its message, location, boundary, cause chain and fields, a content-named field read as redacted: a ratified boundary widening to the error text;
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=32 class=retired
leaf-text: crosses only as screen content, through Dioxus's default boundary rendering of its Display, never through `caused`, which reports it as one error item holding no byte of its text.
source: dec-chytxwvt «decision/root-boundary-err-crossing · Root-boundary Err crossing — A handler Err the app's root error boundary takes crosses as screen content, through Dioxus's default boundary rendering of its Display, and in caused as one»
replace-with: crosses as screen content, through Dioxus's default boundary rendering of its Display, and in `caused` as one error item holding its message and cause chain under the root boundary.
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: its sentinel check reading `caused` alone.
source: dec-xd4p3k72 «decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain.»
replace-with: where `caused` carries its error text and its chain.
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=49 class=omission
leaf-text: setting only escher-telemetry's scoped capture default around an acting step, which prints nothing:
source: dec-xxvmhvjw «decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default»
replace-with: setting only escher-telemetry's scoped capture default around an acting step, which prints nothing, the capture installing once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing and always runs the hook it replaced:
