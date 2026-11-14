@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=49 class=stale
leaf-text: re-runs its test binary twice on an ignored child that prints its shots to stdout, read by the parent into its assertions only
source: dec-4n2hhddr «Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads.»
replace-with: re-runs its test binary twice on an ignored child that prints ids or digests to stdout, read by the parent into its assertions only
