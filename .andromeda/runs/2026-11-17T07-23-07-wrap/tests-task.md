@task_prose part=session
The session added a way to render one component alone, with given props, at the headless stand's viewport and font, and read it as it reads inside its app.
The harness gained a public constructor Harness::from_component_alone and a public mount vdom_alone; the props are a maker run once in an anonymous root's scope, not a value, because an EventHandler prop needs a Dioxus runtime to be made.
The seven_guis headless stand gained stand::boot_alone, which provides TimerTicks on the root before the harness builds; no existing boot changed.
The cold-agent pipe's verb list moved from fourteen to fifteen, layout after actions, uncounted like graph and actions; its isolation tool set is now the fifteen escher MCP tools; the verdict shape is unchanged.
A new stand_* integration target, stand_component_alone, runs in both layout modes; a boundary unit test confirms a vdom rebuilt in place reads one Root error mark through the fallback.
Measured: the root element of a component alone sizes to its content (no shell CSS), so a boot-condition check reads the document viewport (800 x 600, scale 1), never the root's box.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: new blitz-tests integration target, a stand check of a component alone (the shell and the Counter alone against the stand's boot, a click, the viewport and font, the Timer's ticks from the root context), in both layout modes
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness's Harness type and its construction; edited to add from_component_alone, vdom_alone and its first inline unit test module
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the harness crate's root and its public re-exports; edited to re-export vdom_alone
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the harness crate's manifest; edited to add a dev-dependency feature (accessibility on dioxus-native-dom) for the test build only
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand and its boots; edited to add stand::boot_alone
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the error boundary registry and its root error mark; edited to add a unit test of the root fallback for a document built after the first build
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe script, its verb list VERBS; edited from fourteen verbs to fifteen
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's offline unittest suite and its fake server, its own VERBS list; edited to fifteen verbs, a layout read in the fake, one test added
@task_file n=9 path=docs/capabilities.md
what: the public capability statement; edited to add a subsection A component alone with its Witnesses line
@task_item n=1 part=touches
decision/cold-agent-verbs-without-layout is ended by the founder's word: the pipe's verb list now holds layout; the provisional decision retires.
@task_item n=2 part=touches
contract/cold-agent-check/step names the fourteen tools an agent-written check's step may use; it is now fifteen, layout among them.
@task_item n=3 part=touches
contract/cold-agent-verdict/isolated names exactly the fourteen escher MCP tools as the isolation set; it is now the fifteen.
@task_item n=4 part=touches
contract/cold-agent-verdict/capability-unused and decision/cold-agent-passes-on-positive-evidence: layout joins as an uncounted read, COUNTED unchanged (lint, screenshot, advance), and a layout call is no act; check whether a row lists the counted or uncounted tools.
@task_item n=5 part=touches
decision/cold-agent-session-isolated, decision/cold-agent-isolation and decision/cold-agent-tests-offline: the isolation tool set and the offline fake now answer layout; any row holding the count fourteen is stale.
@task_item n=6 part=touches
contract/harness-construction rows (from-component, from-vdom, wrap): a new constructor from_component_alone joins them and rides from_vdom; vdom_alone is a new public mount of the harness.
@task_item n=7 part=touches
decision/harness-reexports-key-types: the harness crate's public re-exports gain vdom_alone.
@task_item n=8 part=touches
contract/stand-boot rows (boot, boot-timer, boot-cells, boot-load, options): a new boot stand::boot_alone joins them, at the stand's options, viewport and font.
@task_item n=9 part=touches
decision/stand-timer-ticks and decision/timer-driven-by-delivered-ticks: boot_alone provides TimerTicks on the root context, and the Timer alone is read by delivered ticks (3 ticks read Elapsed 0.3s).
@task_item n=10 part=touches
decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-behaviour: blitz-tests gains one target, stand_component_alone.
@task_item n=11 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: the new stand check runs every case in both layout modes.
@task_item n=12 part=touches
contract/agent-run-selections/stand, contract/agent-run-selection/stand, decision/stand-repeat-three-rounds and decision/zero-failure-stand-repeat: the new stand_* target joins run stand and the stand-repeat leg by its glob.
@task_item n=13 part=touches
decision/stand-checks-bundled-font, decision/stand-pinned-real-font and decision/stand-boots-offline: a component alone keeps the stand's pinned font and reads can_paint_text; it boots as the stand does.
@task_item n=14 part=touches
decision/stated-tolerance-read-inclusively: the node-for-node comparison reads bounds within 1/64 px inclusive.
@task_item n=15 part=touches
decision/no-ids-or-names-in-stand-logs and decision/test-output-never-reaches-events: the new stand check prints nothing and installs no subscriber.
@task_item n=16 part=touches
decision/mutation-controlled-tests: three controls held (a named child, the timer ticks, the root fallback), each naming one case.
@task_item n=17 part=touches
decision/root-boundary-err-on-own-session and the error-boundary rows: a vdom rebuilt in place now reads one Root mark through the fallback, confirmed by a new boundary unit test.
@task_item n=18 part=touches
decision/inline-cfg-test-modules, decision/inline-unit-test-modules and decision/unit-tests-in-source-file: the harness crate gains its first inline test module for the constructor.
@task_item n=19 part=touches
decision/capability-claims-name-their-checks, decision/capabilities-doc-pinned-to-schema and decision/capability-check-names-held-to-schema: the capability statement gains a subsection with a Witnesses line naming the new checks.
@task_item n=20 part=touches
decision/ci-scripts-unit-tested and decision/python-script-tests-under-unittest: the cold-agent suite grows from 37 to 38 tests; a touch only where a row holds that count.
@task_item n=21 part=touches
decision/count-and-rename-sweeps: the verb count fourteen to fifteen is a sweep across the pipe's two VERBS lists and the fake's comment.
@task_item n=22 part=touches
decision/session-checks-share-one-module, decision/shared-stand-tables-in-one-module and decision/shared-test-modules-not-targets: the new target holds its own table of the Counter's four rows; check whether that table belongs in the shared stand module.
@task_fact n=1 slug=3-test-harness-contract
fact: Harness::from_component_alone(component, props maker, options) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
@task_fact n=2 slug=3-test-harness-contract
fact: vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
@task_fact n=3 slug=3-test-harness-contract
fact: stand::boot_alone(component, props maker, options) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
@task_fact n=4 slug=3-test-harness-contract
fact: Props of a component alone are given as a maker, never a value: an EventHandler prop can only be made inside a Dioxus runtime.
@task_fact n=5 slug=3-test-harness-contract
fact: A root context for a component alone is provided on the mount before the harness builds; a context added after the build reaches no render.
@task_fact n=6 slug=3-test-harness-contract
fact: The cold-agent pipe's verb list holds the fifteen driver verbs, layout after actions; layout is an uncounted read like graph and actions, and a layout call is no act.
@task_fact n=7 slug=test-strategy
fact: A component alone has no shell CSS, so its root element sizes to its content; a boot-condition check of the pinned viewport reads the document viewport, never the root element's bounds.
@task_fact n=8 slug=unit-test-strategy
fact: A harness unit test of the UI graph needs the accessibility feature on its dev-dependency, since a per-package test build leaves it off.
@task_fact n=9 slug=mocking-stubbing-discipline
fact: The Timer's TimerTicks context cannot be read by its elapsed text at boot, since it falls back to wall time; a check of the context reads delivered ticks.
@task_item n=1 part=untouched
WPT conformance, the WPT runner and its status set: no WPT suite, runner or expectation changed.
@task_item n=2 part=untouched
The session wire, socket protocol, session host and driver session contracts: no verb, argument, carrier, socket call or answer changed, and no driver file moved.
@task_item n=3 part=untouched
The MCP server's tools list, protocol and tool results, and the escher-session command line and its exit codes: no MCP tool or command was added.
@task_item n=4 part=untouched
Driver refusal causes, lint defects and findings, snapshot nodes and the layout verb's own answer: none changed.
@task_item n=5 part=untouched
Session recording, committed checks and handler coverage: no recording, check file or coverage line changed.
@task_item n=6 part=untouched
CI workflow rows: action pinning, package installs, caches, trigger branches, MSRV, clippy, audit, coverage floors and mutation runs; no CI step or floor changed.
@task_item n=7 part=untouched
agent-run events, exit codes, verbs and state: no agent-run step changed.
