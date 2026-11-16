@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: `reached_handlers`, `ReachedHandler`)
source: ctr-4y6274wj «contract/reached-record/take-step-reached · Reached record · take_step_reached — Drains the handlers reached since the last drain, each once, in first-reached order.»
replace-with: `reached_handlers`, `ReachedHandler`, and `take_step_reached`, which drains the handlers reached since the last drain, each once, in first-reached order)
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: `SizeKeyword`) and `BoundingRect`.
source: ctr-22yz2gro «contract/dioxus-native-dom-api/root-error-mark · dioxus-native-dom public API · root_error_mark and RootErrorMark — Public and re-exported: root_error_mark reads the root error boundary's error context; Root»
replace-with: `SizeKeyword`) and `BoundingRect`; public and re-exported, `root_error_mark`, which reads the root error boundary's error context, and `RootErrorMark`, compared by identity, its `Debug` printing its name alone.
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=38 class=omission
leaf-text: events,reached,seed,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`seed`, like `reached` and `signal_registry`, outside every `accessibility` gate
source: dec-wdtb3kpq «decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `»
replace-with: events,reached,seed,boundary,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`seed`, like `reached`, `boundary` and `signal_registry`, outside every `accessibility` gate
