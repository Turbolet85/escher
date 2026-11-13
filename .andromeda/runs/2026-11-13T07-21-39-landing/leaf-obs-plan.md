@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.c
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::e
@leaf_edit n=1 leaf=.claude/rules/observability.md line=26 class=stale
leaf-text: counts, booleans and closed words only, never transcript text
source: dec-4milvibf «Each cold-agent verdict field is a count, a boolean, a closed word, the task, the client's tool names, version and model, or the transcript file's path, never transcript text»
replace-with: counts, booleans, closed words, the task, the client's tool names, version and model, and the transcript file's path only, never transcript text
