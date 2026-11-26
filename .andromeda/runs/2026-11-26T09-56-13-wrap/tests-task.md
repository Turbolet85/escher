@task_prose part=session
The session gave Circle Drawer's canvas a keyboard path: one Tab stop, arrows move a selection in declared order (clamped), Enter opens the dialog on the selected circle, a press with the dialog open changes nothing; a click on a circle now selects it and adds none.
A virtual node declaration gained a selected state (None states nothing), written to the accessibility tree and read by the snapshot's virtual branch as the element branch reads it; a flip reads differs selected.
The driver's covered refusal now also covers a canvas's node with a node its canvas declares later whose region (bounding box) holds the target's centre, PROVISIONAL; actions and the run path stay one check; the cause's name, remedy and order are unchanged, its fixed meaning text restated.
The engine was fixed so a focused custom-widget element no longer traps Tab: the widget hears the key, then Tab and Shift+Tab move focus (PROVISIONAL, flagged upstreamable), checked at engine level with red control C7; the app's own focus call on the canvas click was measured inert (control C6 green) and dropped, focus being the engine's on pointer down.
Retired two stand readings (a click on a circle node adds a circle; another circle never covers one), each replaced by its restated case; GRAPH_DRIVES for Circle Drawer 9 to 13 drives, UNEXERCISED_AFTER_DRIVES 4 to 2 handlers; the a11y leg reading 20 · 199 · 0 · 0 to 20 · 201 · 0 · 0.
Measured: a data-circles write sets no app state, so click or key cases add circles by harness clicks off the centre; with the canvas kept at 700 x 400 Close lies below the 600 px view and is refused off-screen until scrolled.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget's virtual node declaration (behind custom-widget) — the declared region, disabled and now selected state of a canvas's virtual nodes
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: blitz-dom's event dispatch (upstream's handle_dom_event and its widget arm) — edited to make one call to the Tab pass-through so a focused widget element does not trap Tab
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: escher's own pointer and widget additions to the engine's dispatch — widget click synthesis and the new widget Tab pass-through
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the dioxus document's accessibility tree with virtual nodes — now writes a declared selected state on a virtual node
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot node reading — its virtual node branch now reads selected from the tree node
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node reading and its unit declaration builder
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver executor — click target checks and the covered refusal reading, now with the later-declared node check
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: the driver's actions verb — its listing of each target's refusal, kept one check with the run path
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: the driver refusal causes, their fixed meanings and remedies served by help and MCP, with the unit test pinning the meaning text
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer, the seven_guis stand app's canvas task — its circles as virtual nodes, graph state registration and declared writes, canvas keys and click
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: the seven_guis host binary checks
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand tables module — graph drives, handlers unexercised after the drives, stated tab orders
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixture pages module — its virtual node declarations
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of virtual nodes a widget declares
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check that virtual nodes read as elements in the tree, the snapshot and a diff — now with a declared selected state and its own fixture page
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand check of Circle Drawer's circle nodes — select by click, covered by a later node, disabled while the dialog is open, keys, the focus events of a canvas click
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard check — stated tab order walks, now Circle Drawer's walk holding the canvas across an arrow
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: the stand canvas click check — now the engine-level check that Tab and Shift+Tab leave a focused widget which still hears them
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas and its dialog slider — re-read with Close after a scroll
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of acting error text
@task_file n=21 path=docs/capabilities.md
what: the capability statement — the a11y leg reading, the covered meaning, the current circle read from selected alone, a click landing at the target's centre
@task_file n=22 path=.andromeda/runs/2026-11-26T07-21-03-implement/gate-2026-11-26-canvas-keyboard-path.json
what: the implement run's gate trail, bookkeeping rewritten by the pre-push gate firing — no code
@task_item n=1 part=touches
contract/refusal-cause/covered: a node a canvas declares now also reads covered by a node its canvas declares later whose region (the declared bounding box) holds the target's centre, so a later square's corner covers where its disc does not — PROVISIONAL; name, remedy and order unchanged.
@task_item n=2 part=touches
decision/circles-apart-written-through-attribute: a centre click on a circle now selects it rather than adding one; a data-circles write sets no app state, so a check whose click or key acts on circles apart adds them by harness clicks off the centre, while widget-level readings (covered, disabled) still write data-circles.
@task_item n=3 part=touches
decision/graph-drives-confirm-every-write and the shared drive tables: Circle Drawer's drives 9 to 13 (Press tab, Press enter, Click circle-canvas, Press arrow-right, Press enter in place of Click circle-close) and its unexercised handlers 4 to 2; new graph state selected writes circle-canvas click and keydown; dialog_idx and dialog_r declare circle-canvas keydown.
@task_item n=4 part=touches
Handler coverage rows (contract/handler-coverage-limits/canvas-handler, decision/handler-coverage-reads-every-replayed-screen): the canvas gains a keydown handler and the unexercised-after-drives list now holds back-btn click and circle-canvas///c1 click.
@task_item n=5 part=touches
decision/inert-guard-dropped and decision/mutation-controlled-tests: control C6 read the app's canvas focus call inert (case green with it removed), so the call was dropped and the fact pinned by behaviour; C1–C5 and the engine control C7 hold red.
@task_item n=6 part=touches
decision/focus-test-pins-where-focus-lands: a canvas click focuses the canvas within its own step by the engine's pointer-down focus, blur and focusout on the prior node then focus and focusin on the canvas; after Close focus drops to the root.
@task_item n=7 part=touches
Capability statement rows (decision/capabilities-doc-pinned-to-schema, decision/capability-statement-pinned, decision/capability-claims-name-their-checks): the a11y leg reading moved to 20 · 201 · 0 · 0, the covered meaning restated, new claims on the current circle and the click landing at centre.
@task_item n=8 part=touches
decision/a11y-leg-membership: the a11y leg's test count moved 199 to 201, its file count twenty unmoved.
@task_item n=9 part=touches
Refusal cause text served by help and MCP (decision/mcp-causes-served-with-remedy, contract/cli-help/output, contract/mcp-help/parity, decision/eight-refusal-causes-in-order, contract/refusal-causes/cause-set): covered's fixed meaning text changed; set, names, remedies and order unchanged.
@task_item n=10 part=touches
decision/expected-value-pinned-as-literal and decision/count-and-rename-sweeps: the covered meaning's pinned copy in its unit test restated; a sweep of the fixed text by its opening clause found two pins (recurrence).
@task_item n=11 part=touches
contract/driver-verbs/actions and decision/actions-answer-holds-no-value: actions lists covered for a later-declared node exactly as the run path refuses it — one check.
@task_item n=12 part=touches
contract/refusal-cause/off-screen (indirect): with the canvas at full size Circle Drawer's Close is refused off-screen in the 600 px view until scrolled; checks scroll it first or reach it by tab and enter.
@task_item n=13 part=touches
contract/stand-boot/boot-circle-drawer (indirect): the booted Circle Drawer now has a Tab-stop canvas with keys, a selected circle, data-selected and data-adjusting attributes, and every other circle disabled while the dialog is open.
@task_item n=14 part=touches
decision/shared-stand-tables-in-one-module: Circle Drawer's stated tab order (back-btn, circle-canvas) joins the shared tables.
@task_item n=15 part=touches
decision/shared-fixtures-in-one-module and decision/shared-fixture-pages: the selected-state fixture page sits in its one check's file (a page with one reader); the shared fixtures' declarations take selected None.
@task_item n=16 part=touches
Snapshot and element-id rows (contract/snapshot-node/clickable, contract/element-id/virtual-form): a virtual node's snapshot entry now carries selected; circle node ids unchanged.
@task_item n=17 part=touches
decision/graph-signal-registration and decision/unrun-declared-write-reads-unconfirmed: Circle Drawer registers one more graph state, selected, and adds keydown writes to two declared states.
@task_item n=18 part=touches
decision/regression-tests-document-bug and decision/regression-tests-named-for-bug: the engine Tab trap fix carries its engine-level regression check named for the behaviour, with red control C7.
@task_item n=19 part=touches
decision/upstream-oversize-files-kept (indirect): an upstream engine dispatch file gained one call into escher's own function, the fork addition kept out of upstream's body.
@task_item n=20 part=touches
decision/canvas-commands-table-across-hosts and decision/canvas-shots-cross-os-by-digest: canvas pins read unchanged — no committed case selects a circle, so the selected fill is pinned by none.
@task_item n=21 part=touches
Retired stand readings: a click on a circle node adds a circle, and another circle never covers one — each replaced by its restated case (selects and adds none; a circle under a later one is refused covered).
@task_fact n=1 slug=integration-test-strategy
fact: A focused custom-widget element passes Tab and Shift+Tab on after the widget hears the key; this engine behaviour is checked at engine level on a fixture canvas in both layout modes with its red control, PROVISIONAL and upstreamable.
@task_fact n=2 slug=integration-test-strategy
fact: Canvas keyboard checks read three focus readings — the engine's, the snapshot's and the tree's — and require them to agree, with no virtual node ever the tree's focus.
@task_fact n=3 slug=test-data-fixtures
fact: Circle Drawer's Close lies below the 600 px view with the canvas at 700 x 400, so a check scrolls it into view before clicking or reaches it by tab and enter, and re-clicks the canvas to refocus it before its keys since closing the dialog drops focus to the root.
@task_fact n=4 slug=integration-test-strategy
fact: The engine focuses a custom-widget element on pointer down, so a check pins a canvas click's focus by its focus events in one step, never by an app focus call.
@task_item n=1 part=untouched
Cold-agent rows (events, verdict, exit codes, isolation, prompt, replay): no cold-agent surface moved.
@task_item n=2 part=untouched
agent-run verbs, selections, events and exit codes: unchanged.
@task_item n=3 part=untouched
Session wire, session host, session errors, exit status, state directory and recording contracts: no verb, argument, session error, exit, carrier, log field or span added.
@task_item n=4 part=untouched
WPT runner, conformance, expectations and status set rows: untouched.
@task_item n=5 part=untouched
CI integration rows — jobs, caches, package installs, MSRV, clippy legs, artifacts, coverage floors, mutation testing, quality gates: no harness or gate surface and no dependency changed.
@task_item n=6 part=untouched
Lint defect and lint finding contracts: the lint's checks on virtual nodes stay with another owner.
@task_item n=7 part=untouched
Mocking, stand-in, virtual clock, timer and advance rows: untouched.
@task_item n=8 part=untouched
Harness construction, input, settle, screenshot and contrast contracts: used as they stand, none changed.
@task_item n=9 part=untouched
Driver verb table and argument kinds, log line and command span contracts: no new verb, argument or key name.
