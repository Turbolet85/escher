@row_set n=1 row=contract/cold-agent-check/step cell=promise seen=1 detector=none
value: A step holds `tool` among the fifteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
basis: fct-vl5f6he5
because: The pipe's verb list moved from fourteen to fifteen with `layout` after `actions`, so a check's step may name any of the fifteen verbs.
@row_set n=2 row=contract/cold-agent-verdict/isolated cell=promise seen=1 detector=none
value: True when the init tools are exactly the fifteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
basis: fct-vl5f6he5
because: The isolation check's tool set follows the pipe's verb list, now the fifteen `mcp__escher__*` tools.
@row_retire n=1 row=decision/cold-agent-verbs-without-layout seen=1 heir=decision/cold-agent-verb-list detector=none
why: The provisional choice to leave `layout` out of the cold-agent pipe is ended by its ratification: the verb list now holds all fifteen driver verbs.
basis: fct-dawkr2ev
because: Both `VERBS` lists now hold fifteen verbs with `layout` after `actions`, so the provisional fourteen-verb decision no longer holds.
@row_add n=1 row=decision/cold-agent-verb-list area=3-test-harness-contract fact=F6 detector=none
name: Cold-agent pipe's verb list
rule: The cold-agent pipe's verb list holds the fifteen driver verbs, `layout` after `actions`; `layout` is an uncounted read like `graph` and `actions`, and a `layout` call is no act.
why: Ratified: `layout` joins the pipe.
guard: test_a_layout_read_is_an_uncounted_read_in_the_run_and_in_the_check
basis: fct-vl5f6he5
because: The pipe's verb list moved to fifteen with `layout` uncounted, and this row replaces the retired fourteen-verb decision.
@row_add n=2 row=contract/harness-construction/from-component-alone area=3-test-harness-contract fact=F1 detector=none
boundary: Harness construction
item: Harness::from_component_alone
promise: Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
basis: fct-xmlfpf6t
because: The harness gained a new public constructor for a component alone, a construction promise no row states.
@row_add n=3 row=contract/harness-construction/vdom-alone area=3-test-harness-contract fact=F2 detector=none
boundary: Harness construction
item: vdom_alone
promise: vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
basis: fct-xmlfpf6t
because: The harness crate now exports a public mount of a component alone that the stand's boot builds on.
@row_add n=4 row=contract/stand-boot/boot-alone area=3-test-harness-contract fact=F3 detector=none
boundary: seven_guis headless stand
item: stand::boot_alone
promise: stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
basis: fct-xmlfpf6t
because: The headless stand gained a new boot for a component alone beside its existing boots, none of which changed.
@row_add n=5 row=decision/component-alone-props-as-maker area=3-test-harness-contract fact=F4 detector=none
name: Props of a component alone given as a maker
rule: Props of a component alone are given as a maker run once in the mount's root scope, never as a value.
why: An EventHandler prop can only be made inside a Dioxus runtime.
basis: fct-q4nj3g5q
because: A props-value signature was replaced by a props maker because a value cannot carry an EventHandler made outside a runtime.
@row_add n=6 row=decision/component-alone-root-context-before-build area=3-test-harness-contract fact=F5 detector=none
name: Root context of a component alone provided before the build
rule: A root context for a component alone is provided on the mount before the harness builds.
why: A context added after the build reaches no render.
basis: fct-pok4ivsq
because: A boot that provided TimerTicks after the build was replaced since the context reached no render, which binds every caller of the mount.
@row_add n=7 row=decision/pinned-viewport-read-from-document area=test-strategy fact=F7 detector=none
name: Pinned viewport read from the document
rule: A boot-condition check of the pinned viewport reads the document viewport, never the root element's bounds.
why: A component alone has no shell CSS, so its root element sizes to its content.
basis: fct-jms6y2th
because: The root element of a component alone was measured at its content height, so only the document viewport reads the pinned size.
@row_add n=8 row=decision/harness-graph-tests-enable-accessibility area=unit-test-strategy fact=F8 detector=none
name: Harness graph unit tests turn accessibility on
rule: A harness unit test of the UI graph turns the accessibility feature on through its dev-dependency, for the test build only.
why: A per-package test build leaves the accessibility feature off, and the UI graph is gated by it.
basis: fct-j3cdj2ux
because: The harness's first unit test reads the UI graph, which a per-package test build compiles out unless the dev-dependency turns the feature on.
@row_add n=9 row=decision/timer-context-read-by-delivered-ticks area=mocking-stubbing-discipline fact=F9 detector=none
name: Timer context checked by delivered ticks
rule: A check that the Timer's TimerTicks context is present reads delivered ticks, never the Timer's elapsed text at boot.
why: The Timer falls back to wall time without the context, so its elapsed text at boot reads the same either way.
basis: fct-jrlxprzv
because: The planned check by elapsed text at boot could not fail, so the check of the context reads delivered ticks instead.
