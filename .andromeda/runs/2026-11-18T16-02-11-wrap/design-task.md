@task_prose part=session
The session added a seventh served stand app, order: an order page whose controls return an error, log and panic, so a driver and a cold agent meet what an action caused.
The page holds a quantity field, a Submit button, an Export button and a status line; Submit refuses a quantity that is not a whole number from 10 to 20 and shows Order failed with a Retry button under the page's own error boundary; a good Submit logs order placed; Export panics with export is not built yet.
escher-session's served-task set went from six to seven (order after load), so the usage line's apps and the MCP start enum name it; the stand's boot, actions, handler, graph, layout, keyboard, contrast and cross-OS tables each gained the order page.
The cold-agent pipe now counts what each acting answer caused by kind (panic, error, log) into a new verdict field caused_by_kind, and capability-unused also fires when any of the three reads 0; its prompt names seven apps and a goal 7.
Measured: the order page's controls, keyed by HTML ids under no keyed element, read no owner in the UI graph, so each caused item carries no owner.
After implement, CI found the page did not compile for android because the dioxus-native prelude brings a one-parameter Result alias there; Submit's return type is now spelled std::result::Result.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: new, then edited after implement — the order page served as the order task: quantity field, Submit under its own error boundary with Order failed and Retry, Export that panics, the status line, the page's styles and the fixed rule, log and panic texts
@task_file n=2 path=tests/blitz-tests/tests/stand_order_served.rs
what: new — the order page's stand checks: keyed within budget with no finding at boot, each control answering what it caused, the same caused over the MCP role, a kept check replaying
@task_file n=3 path=examples/seven_guis/Cargo.toml
what: edited — the seven_guis example app's manifest, gaining the tracing dependency
@task_file n=4 path=Cargo.lock
what: edited — the workspace lock, one dependency edge added for seven_guis
@task_file n=5 path=examples/seven_guis/src/tasks/mod.rs
what: edited — the list of the 7GUIs stand task modules, gaining order
@task_file n=6 path=examples/seven_guis/src/stand.rs
what: edited — the headless stand's boot functions, gaining boot_order rendering the order page in the task shell
@task_file n=7 path=examples/seven_guis/src/session_host.rs
what: edited — escher-session's served-task table, six to seven with order after load, which the usage line and the MCP start enum read
@task_file n=8 path=examples/seven_guis/tests/cli_commands.rs
what: edited — escher-session command line checks, the served-task list and the usage line's apps now seven
@task_file n=9 path=examples/seven_guis/tests/host_log.rs
what: edited — escher-session host log checks, gaining that the served order page logs neither its message nor its rule on stderr
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: edited — the stand's shared tables: actionable and handlers at boot, graph drives, unexercised after drives, gaining order rows, and the new order tab order
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: edited — the actions check over every served app, now seven
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: edited — the stand contrast check, gaining the order page's ratios
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited — the stand readings same on every CI OS table, 48 to 54 rows with the order page
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: edited — the explore check over every served app, now seven
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: edited — the UI graph check, gaining the order page drive
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: edited — the handler coverage check's docs, now seven apps
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited — the stated tab order walk, gaining the order page and its failed state with Retry
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: edited — the layout check over every served app, now seven
@task_file n=19 path=scripts/cold-agent.sh
what: edited — the cold-agent run and its verdict: seven apps, goal 7, the caused_by_kind field and capability-unused reading it
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: edited — the cold-agent contract tests and their fake escher-session, gaining the order app and a caused scenario
@task_file n=21 path=docs/capabilities.md
what: edited — the capability statement: seven served apps, the boot actionable counts, the order page as where caused is met
@task_file n=22 path=README.md
what: edited — the README's 0.3.0 status, its served app slugs now seven
@task_item n=1 part=touches
The escher-session tasks contract rows stand at six served tasks (cells, counter, crud, flight-booker, load, timer); a seventh, order, is now served and owes its row beside them.
@task_item n=2 part=touches
The escher-session tasks unknown-task row: any row naming the set of known tasks or the six now reads seven, order among them.
@task_item n=3 part=touches
escher-session exit status 2 and the cli surface's exit code 2: the usage line that names the apps now names seven, order after load.
@task_item n=4 part=touches
escher-session commands help and the MCP role's tool results: the start tool's task enum gains order.
@task_item n=5 part=touches
The actions line and the decision that actions are decided by the verbs' own checks: a new served page answers actions at boot with 4 actionable nodes and a disabled Export or Retry state to read.
@task_item n=6 part=touches
The UI graph line content: controls keyed by HTML ids under no keyed element read no owner, measured on the order page; a row saying a caused item is tied to its owner does not hold there.
@task_item n=7 part=touches
The driver verbs click, type and press and their answers: on the order page a click or Enter answers caused items of kind error (boundary own), log (info, target the order module, no fields) and panic (file and line).
@task_item n=8 part=touches
The decision that escher-session is read by an agent, not a person, and that driver command line output is read by agents: the cold agent now must read what an action caused, counted by kind.
@task_item n=9 part=touches
escher-session stdout command line output and mcp role output: the same caused answer crosses both roles for the order page.
@task_item n=10 part=touches
The colour palette's not-yet-measured project token set and the absence of engine colour tokens: the stand page uses fixed app colours (accent 3a5ce5, 2a4cd3, 1f3fc1, page f5f5f5, text 1a1a1a, failure c53030 on fff5f5), not engine tokens.
@task_item n=11 part=touches
Any surface row stating six served screens or six apps, or a boot list of the served apps, now reads seven.
@task_fact n=1 slug=surface
fact: escher-session serves the task order: an order page with a quantity field, Submit, Export and a status line; Submit with a quantity not a whole number from 10 to 20 answers an error item and shows Order failed with Retry under the page's own error boundary, a good Submit logs order placed and shows Order placed, Export panics with export is not built yet.
@task_fact n=2 slug=surface
fact: A control keyed by an HTML id under no keyed element reads no owner in the UI graph, so a caused item it answers carries no owner; the order page's controls all read none.
@task_fact n=3 slug=surface
fact: The order page's element ids order-quantity, order-submit, order-retry, order-failed, order-export and order-status are occupied stand ids, each an HTML id.
@task_fact n=4 slug=color-palette
fact: The stand's order page shows its failure state as text c53030 on fff5f5 beside the stand accent 3a5ce5 (hover 2a4cd3, active 1f3fc1), its ratios asserted by the stand contrast check.
@task_item n=1 part=untouched
The select list rows (events, accessibility, colours, overlay, open gestures, motion): no select was added or changed.
@task_item n=2 part=untouched
Typography and font rows (system fonts, bundled font context on WASM, fallback families, font element hints): no font handling moved.
@task_item n=3 part=untouched
Motion and virtual clock rows (advance, settle, scroll instant, overlay scrollbar fades, smooth scroll): no clock or animation behaviour changed.
@task_item n=4 part=untouched
WPT runner output and result colour rows: the WPT runner was not touched.
@task_item n=5 part=untouched
Shell provider, cursor, favicon and window rows (maximize, minimize, drag, decorations, set_cursor, alpha mode) and the JS window rows: no shell or window surface changed.
@task_item n=6 part=untouched
Spacing, depth, hit-testing, stacking and paint-order rows, and the web-spa surface: no engine layout or paint behaviour and no WASM page changed.
@task_item n=7 part=untouched
escher-session's screenshot, snapshot, coverage, graph and layout arguments, its session error remedy and stderr shapes, and exit statuses 0, 1 and 3: no verb, argument, cause, session error or exit changed.
