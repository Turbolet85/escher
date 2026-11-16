@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=165
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=stale
leaf-text: `caused` — one item per kind (panic, error, log) and level, with the first reached handler's id, owner id and event name and a count
source: ctr-wkieu5ai «contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes»
replace-with: `caused` — an item per panic, error or record, holding its kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's scrubbed fields and the first reached handler; panics, then errors new since the boundary marks, then records, 16 kept and the rest counted in `caused_past`
@leaf_edit n=2 leaf=CLAUDE.md line=57 class=omission
leaf-text: `request` · `error` print `[redacted]` on `escher_*` targets
source: dec-gz6acavz «decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request,»
replace-with: `request` · `error` · `password` print `[redacted]` on `escher_*` targets
@leaf_edit n=3 leaf=CLAUDE.md line=35 class=omission
leaf-text: and the scoped log capture the driver sets around an acting step, which prints nothing
source: dec-xxvmhvjw «decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped»
replace-with: and the scoped log capture the driver sets around an acting step, with its once-per-process chaining panic hook, which keeps the step's records and panics and prints nothing
