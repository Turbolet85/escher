@answer n=1 entity=design-philosophy#1.1 v=no
why: the crate split and renderer-agnostic painting are not moved, since the chunk adds no crate, no engine change and paints nothing
@answer n=2 entity=design-philosophy#2.1 v=no
why: the chunk adds no provider, fetch, navigation or parser, and the graph makes no fetch, so provider injection and no-op defaults are not reached
@answer n=3 entity=design-philosophy#3.1 v=yes
why: the graph keys on the agent-visible node built by build_accessibility_tree, so the WAI-ARIA tree exclusion and HTML-AAM role mapping cited for accessibility.rs decide which nodes the graph carries
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the graph is keyed on the accesskit node with an optional owner and never on a DOM NodeId, so the versioned-id and DOM-only persistent identity rule governs the reserved virtual-node id form
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the kept checks and the headless seven_guis stand run without a window and replay through Session::run, so the headless, synthesized-input test rule binds
