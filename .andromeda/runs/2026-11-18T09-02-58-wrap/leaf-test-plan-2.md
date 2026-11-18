@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=30 class=omission
leaf-text: Constructors `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom` pump once
source: ctr-xrs2rifi «Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope;»
replace-with: Constructors `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom` and `from_component_alone(component, props maker, HarnessOptions)` (one component mounted as the one child of an anonymous root, the props maker run once in that root's scope, riding `from_vdom`; `vdom_alone` its public mount, a root context added on it before the harness builds) pump once
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=36 class=omission
leaf-text: The headless stand boots through `seven_guis::stand::{boot, boot_timer}` over `from_vdom`
source: ctr-emytbabz «stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.»
replace-with: The headless stand boots through `seven_guis::stand::{boot, boot_timer}` over `from_vdom`, a component alone through `stand::boot_alone(component, props maker, HarnessOptions)` at the stand's viewport and font with `TimerTicks` provided on the root before the build,
