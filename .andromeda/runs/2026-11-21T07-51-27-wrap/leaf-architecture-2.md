@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.cl
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=22 class=omission
leaf-text: it writes the reached record (`record_reached`, `reached_handlers`) only where that was switched on
source: dec-uxsddcdx «Where a reached record is switched on, the chain's nodes whose current listeners hold the event's name are resolved by stable id before the handlers run and written after: all while it propagates, the innermost»
replace-with: it resolves the reached record (`record_reached`, `reached_handlers`) by stable id before the handlers run and writes it after — every listening node of the chain while the event propagates, the innermost alone when a listener stops it or a handler's panic unwinds, a non-bubbling event's target alone — only where that was switched on
