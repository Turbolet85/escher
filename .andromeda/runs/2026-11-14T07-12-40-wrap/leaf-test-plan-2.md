@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=67
last: - 2026-11-13: A control that deletes a loop's advance stateme
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=30 class=omission
leaf-text: `from_vdom` pump once; `wrap` does not.
source: ctr-fr4ocsu2 «Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context»
replace-with: `from_vdom` pump once (`from_vdom` first providing `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where the vdom's root holds none — a root context the builder gave stands); `wrap` does not.
