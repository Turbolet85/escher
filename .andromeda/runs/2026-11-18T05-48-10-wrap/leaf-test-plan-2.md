@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own
@leaf_edit n=1 leaf=.claude/rules/testing.md line=21 class=omission
leaf-text: and only its names and counts are pinned, so a check renamed or moved changes that section by hand.
source: dec-7uqcvocj «Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script»
replace-with: and only its names and counts are pinned, the names and schema counts to the driver schema's tables and the a11y leg's file count and `stand-repeat`'s named-target count to the CI leg script, so a check renamed or moved changes that section by hand.
