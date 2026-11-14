@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: construct (`from_html`, `from_component`, `from_vdom`;
source: ctr-fr4ocsu2 «Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none»
replace-with: construct (`from_html`, `from_component`, `from_vdom` — which, before its one pump, provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where the vdom's root holds none, a root context the builder gave standing;
