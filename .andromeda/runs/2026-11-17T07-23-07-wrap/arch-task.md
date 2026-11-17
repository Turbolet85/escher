@task_prose part=session
The session added a way to render one component alone, with given props, in a headless document at the stand's pinned viewport and font, read as it reads inside its app.
The test harness gained a public constructor from_component_alone and a public function vdom_alone: the component mounted as the one child of an anonymous root, its props made by a props maker run once in that root's scope, never a props value.
The headless stand gained boot_alone, which provides the TimerTicks root context on that mount before the harness builds; no existing stand boot changed.
The cold-agent pipe's verb list went from fourteen to fifteen with layout after actions, so its isolated tool set is the fifteen mcp__escher__ tools; layout is an uncounted read like graph and actions, and the verdict shape is unchanged.
It measured that a component alone has no shell CSS, so its root element sizes to its content while the document's viewport still reads 800 x 600 at scale 1; it confirmed that a vdom rebuilt in place reads one Root boundary mark through the fallback.
No driver verb, argument, carrier, MCP tool, session socket call, kept-check key, span, log field, crate, dependency or schema was added or retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: a new blitz-tests integration target, one test file, the stand checks of a component alone in both layout modes (a stand_ file, so it joins the stand run and the stand-repeat leg)
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the blitz-test-harness Harness and its constructors; edited to add from_component_alone, vdom_alone and its private anonymous root, and a unit-test module at the file's end
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the blitz-test-harness crate root and its re-exports; edited to re-export vdom_alone
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the blitz-test-harness manifest; edited to add a dev-dependency on dioxus-native-dom with its accessibility feature, a workspace dependency, for the test build only
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand and its boots; edited to add boot_alone
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the dioxus-native-dom boundary marks and root error mark; edited to add one unit test, the root fallback of a document built after the first build
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe's launcher and its verb list VERBS; edited to add layout after actions
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and their fake MCP server, with their own VERBS list; edited to fifteen verbs and a test that a layout call is an uncounted read
@task_file n=9 path=docs/capabilities.md
what: the capability statement; edited to add a subsection A component alone with its Witnesses line
@task_item n=1 part=touches
The test harness contract (blitz-test-harness Harness) gains a constructor from_component_alone and a public function vdom_alone; a row listing the Harness constructors or its from_vdom seed is now incomplete.
@task_item n=2 part=touches
The seven_guis headless stand contract gains boot_alone beside its boots, start_in and timer_step; the stand's boot rows name no isolated boot yet.
@task_item n=3 part=touches
The absence of a fixture state on the driver's carriers gains a sibling: a component alone is reached in process only and on no carrier, command line, MCP, session socket or kept check.
@task_item n=4 part=touches
The dioxus-native-dom root error mark contract: a vdom rebuilt in place now reads one Root mark through the fallback, a behaviour its row can add.
@task_item n=5 part=touches
The cold-agent isolated session decision: its tool set is now the fifteen mcp__escher__ tools, layout among them; any row holding fourteen as the pipe's verb or tool count is stale.
@task_item n=6 part=touches
The cold-agent verdict rows (counted calls, capability-unused, wrong-call bar): layout joins graph and actions as an uncounted read, COUNTED and the verdict fields unchanged.
@task_item n=7 part=touches
The cold-agent MCP server being the workspace's own and the cold-agent judge on stdlib Python: their fake server and contract tests now answer layout as a read.
@task_item n=8 part=touches
The capability statement decision and its sections contract: the statement takes a new subsection A component alone with a Witnesses line.
@task_item n=9 part=touches
The stand pinned environment and the stand's DejaVu Sans typography: an isolated document keeps the stand's 800 x 600 viewport at scale 1 and its font, but its root element sizes to its content, since the shell's CSS is absent.
@task_item n=10 part=touches
Dioxus services as root contexts: boot_alone provides TimerTicks as a root context on the anonymous root before the harness builds, since a context added after the build reaches no render.
@task_item n=11 part=touches
Stable element ids by component path and the owning component read through Dioxus public API: a component alone reads its ids with the shell's component prefix removed, the anonymous root named by its type path, and its owning component named as in the app.
@task_item n=12 part=touches
One blitz-tests target per file, integration tests in blitz-tests and scenarios in both layout modes: a new target runs its cases in both modes.
@task_item n=13 part=touches
The stand-repeat zero-failure budget and the stand-repeat leg: the new stand_ target joins that leg by its glob.
@task_item n=14 part=touches
Unit tests at the end of the source file and unit-test placement: blitz-test-harness takes its first unit-test module at the end of its harness source.
@task_item n=15 part=touches
Accessibility feature gating in dioxus-native-dom and in-repo crates without default features: a per-package harness test of the UI graph needs that feature turned on through a dev-dependency.
@task_item n=16 part=touches
Committed stand-Counter checks: the stand's Counter is now also read alone and as the shell given the stand's props, node for node.
@task_item n=17 part=touches
UI graph readers and owner: a Counter alone names Counter as the component and sole reader of count, as in the app.
@task_fact n=1 slug=standard-contracts
fact: The test harness builds a document from one component alone with given props: the component is the one child of an anonymous root, its props come from a props maker run once in that root's scope (a props value cannot carry an event handler made outside a Dioxus runtime), and the document reads as the component reads inside its app.
@task_fact n=2 slug=standard-contracts
fact: The headless stand boots a component alone at its pinned viewport and font through its isolated boot, which provides the stand's timer ticks as a root context on the mount before the harness builds.
@task_fact n=3 slug=standard-contracts
fact: A component alone on the driver's carriers is absent: it is reached in process only, and no verb, role, argument, MCP tool, session socket call or kept check reaches it.
@task_fact n=4 slug=standard-contracts
fact: A component alone carries no shell CSS, so its root element sizes to its content; a boot-condition read of the pinned viewport reads the document's viewport, never the root element's bounds.
@task_fact n=5 slug=standard-contracts
fact: The cold-agent pipe offers the agent fifteen driver verbs, layout after actions, and its isolation check's tool set is the fifteen mcp__escher__ tools; layout is an uncounted read like graph and actions.
@task_item n=1 part=untouched
The driver's verbs, arguments, answers, refusal causes and the driver verb table: no verb or argument was added or changed.
@task_item n=2 part=untouched
The MCP server's tool listing, the mcp role and the session socket's wire, hosts and errors: no tool, call or reply was added.
@task_item n=3 part=untouched
Kept check files, recording directories and replay: no kept-check key changed.
@task_item n=4 part=untouched
Telemetry, spans, log fields and the escher-telemetry sink: the new checks print nothing and install no subscriber.
@task_item n=5 part=untouched
The engine's style, layout, paint, hit testing, select list, scrolling, script and form-control rows: no engine behaviour changed.
@task_item n=6 part=untouched
Occupied names and resources: no crate, binary, listener, port, env var or state area was added or freed.
@task_item n=7 part=untouched
The browser app, WPT runner, examples other than the seven_guis stand, CI workflow jobs and the stack's dependency choices: none changed.
