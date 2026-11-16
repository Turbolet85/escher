@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=164
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=35 class=omission
leaf-text: stderr `tracing` subscriber, service identity, target allowlist and scrub, panic hook
source: dec-xxvmhvjw «The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.»
replace-with: stderr `tracing` subscriber, service identity, target allowlist and scrub, panic hook, and the scoped log capture the driver sets around an acting step, which prints nothing
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=omission
leaf-text: and each returns once the instance has settled, with the diff of the screen before and after;
source: ctr-wkieu5ai «Present only when the step caused something: one item per kind (panic, error, log) and level (error, warn, info, debug, trace), with the first reached handler's id, owner id and event name where one»
replace-with: and each returns once the instance has settled, with the diff of the screen before and after and, only where the step caused something, `caused` — one item per kind (panic, error, log) and level, with the first reached handler's id, owner id and event name and a count;
@leaf_edit n=3 leaf=CLAUDE.md line=33 class=omission
leaf-text: the reached record and the run's seeded randomness
source: ctr-22yz2gro «dioxus-native-dom public API · root_error_mark and RootErrorMark — Public and re-exported: root_error_mark reads the root error boundary's error context; RootErrorMark compares by identity and its Debug»
replace-with: the reached record, the root error boundary's mark (`root_error_mark`, `RootErrorMark`) and the run's seeded randomness
