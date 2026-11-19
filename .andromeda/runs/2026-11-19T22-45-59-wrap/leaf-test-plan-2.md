@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own
@leaf_edit n=1 leaf=.claude/rules/testing.md line=18 class=stale
leaf-text: whose `verify_that!` and `TestResult<()>` upstream's merged `accessibility_label` and `accessibility_description` checks assert through under the same `#[test]` harness
source: dec-dlncbnmi «except: upstream's merged checks that return TestResult<()> and assert with test-that's verify_that!, under the same harness»
replace-with: whose `verify_that!` and `TestResult<()>` upstream's merged checks that use them assert through under the same `#[test]` harness
