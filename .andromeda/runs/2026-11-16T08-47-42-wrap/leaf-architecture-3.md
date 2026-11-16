@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=36 class=stale
leaf-text: `tracing` alone, ungated, in escher-driver for its `command` span
source: dec-xxvmhvjw «The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.»
replace-with: `tracing`, ungated, in escher-driver, which installs no global subscriber and sets only escher-telemetry's scoped capture default around an acting step, for its `command` span
