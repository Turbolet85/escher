@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own 
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=32 class=omission
leaf-text: each acting verb one settled step returning the diff of the snapshots before and after.
source: ctr-bujbd4ha «`click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something»
replace-with: each acting verb one settled step returning the diff of the snapshots before and after, with `caused` where the step caused something.
