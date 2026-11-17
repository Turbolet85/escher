@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=stale
leaf-text: given only fourteen of the escher server's tools, every driver verb but `layout` (provisional)
source: ctr-enkhxjrd «True when the init tools are exactly the fifteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or»
replace-with: given only the escher server's fifteen tools, every driver verb `layout` included
@leaf_edit n=2 leaf=.claude/docs/services/blitz-test-harness.md line=16 class=omission
leaf-text: `seven_guis::stand::{boot, boot_timer, boot_cells, boot_load, options, start_in}` — `start_in`
source: ctr-emytbabz «stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.»
replace-with: `seven_guis::stand::{boot, boot_alone, boot_timer, boot_cells, boot_load, options, start_in}` — `boot_alone` booting one component alone at the stand's viewport and font, `TimerTicks` provided on the root before the build, `start_in`
@leaf_edit n=3 leaf=.claude/docs/services/blitz-test-harness.md line=19 class=omission
leaf-text: Constructors pump once; `wrap` does not.
source: ctr-xrs2rifi «Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides»
replace-with: Constructors pump once; `wrap` does not. `Harness::from_component_alone(component, props maker, HarnessOptions)` builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope, riding `from_vdom` and adding no carrier; `vdom_alone(component, props maker)` is the public mount of a component alone, a `VirtualDom` whose root context a caller provides before the harness builds.
