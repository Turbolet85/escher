@task_prose part=session
The session added a seventh served stand app, the order page (OrderPage, served as order), whose controls panic, return an error and log, so a driver and a cold agent meet what each action caused.
The order page holds a quantity input, a Submit button under its own error boundary that shows the text Order failed and a Retry button on a bad quantity, an Export button that panics, and a status line Order placed on a good Submit.
The stand's keyboard check walks the order page's stated Tab order forward and back, and again with its failure shown (Retry in Submit's place); the contrast check asserts its ratios at boot, with the failure, Retry and the placed status, in both render modes.
The served-task set moved six to seven across the stand's tables (actionable nodes at boot, handlers at boot, graph drives, the shared Tab order table gains the order page's row), and the cross-OS readings table moved 48 to 54 rows.
The cold-agent verdict gains caused_by_kind (counts of panic, error and log), which capability-unused also reads; the prompt names seven apps and a seventh goal.
Measured: no order-page control reads an owner in the UI graph, so a caused item from it carries no owner; after implement, the Submit handler's return type was spelled std::result::Result so the app builds on android.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: new; the order page, a seventh stand task component — its quantity input, Submit under its own error boundary with the Order failed text and Retry, the Export control, the status line and its stand colours (the stand accent, the invalid pair); edited after implement (the Submit handler's return type spelled whole)
@task_file n=2 path=examples/seven_guis/src/tasks/mod.rs
what: edited; the stand's task module list, now naming the order page
@task_file n=3 path=examples/seven_guis/src/stand.rs
what: edited; the headless stand's boot functions, gaining the order page's boot in the task shell
@task_file n=4 path=examples/seven_guis/src/session_host.rs
what: edited; the served-task set of the session host, six to seven with order
@task_file n=5 path=examples/seven_guis/Cargo.toml
what: edited; the stand app crate's manifest, gaining the tracing dependency
@task_file n=6 path=Cargo.lock
what: edited; the workspace lock, one dependency edge for the stand app crate
@task_file n=7 path=examples/seven_guis/tests/cli_commands.rs
what: edited; the session CLI check over the served tasks and the usage line, now seven
@task_file n=8 path=examples/seven_guis/tests/host_log.rs
what: edited; the served host's log check, gaining that the order page's message and rule are not written
@task_file n=9 path=tests/blitz-tests/tests/stand_order_served.rs
what: new; the order page's stand check — keyed within budget and no lint finding at boot, what each control caused, the same over MCP, a kept check replaying
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: edited; the stand's shared expectation tables — actionable nodes at boot, handlers at boot, graph drives, unexercised handlers, and the shared Tab order table gaining the order page's row
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: edited; the stand's actionable-nodes check over the served apps, now seven
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: edited; the stand contrast check (SC 1.4.3 asserted on the stand), gaining the order page's ratios
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited; the stand readings compared on every CI OS by answer digest, 48 to 54 rows
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: edited; the stand's explore check over the served apps, now seven
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: edited; the stand's UI graph check, gaining the order page's drive
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: edited; the stand's handler coverage check, its doc naming seven apps
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited; the stand's stated Tab order walk, gaining the order page and its failure-shown order (a local table)
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: edited; the stand's layout check over the served apps, now seven
@task_file n=19 path=scripts/cold-agent.sh
what: edited; the cold-agent run and its verdict — caused_by_kind, capability-unused, the prompt's seven apps and seventh goal
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: edited; the cold-agent contract tests and their fake session host
@task_file n=21 path=docs/capabilities.md
what: edited; the capability statement page — seven served apps, the boot actionable counts, the order page as where caused is met
@task_file n=22 path=README.md
what: edited; the README's served-app slugs, now seven
@task_item n=1 part=touches
decision/stated-tab-order-walked: the stand's stated Tab order walk now covers the order page, forward and back, and a second order with its failure shown, Retry in Submit's place.
@task_item n=2 part=touches
decision/stand-shared-tab-order-table: the shared Tab order table gains the order page's row (back, quantity, Submit, Export), while the failure-shown order is a local table of the keyboard check alone — an exception to stating focus order once in the shared table.
@task_item n=3 part=touches
decision/stand-contrast-asserted: SC 1.4.3 on the stand now asserts the order page, at boot and with the failure text, Retry and the placed status shown, both render modes.
@task_item n=4 part=touches
decision/stand-contrast-measured and decision/text-contrast-required-ratio: the order page's pairs are measured from computed styles against the required ratio, including the invalid pair red on pale red.
@task_item n=5 part=touches
decision/stand-accent-colour: the order page's buttons use the stand accent and its hover and active shades.
@task_item n=6 part=touches
decision/stand-lint-clean-at-boot: a seventh stand page that reads no lint finding at boot.
@task_item n=7 part=touches
decision/actionable-key-rule and decision/one-actionable-rule: the order page's actionable nodes are keyed by HTML id, none unkeyed, four actionable at boot.
@task_item n=8 part=touches
decision/stand-expectation-tables and decision/role-name-tables-stated-once: the stand's shared tables gain the order page's rows (actionable at boot, handlers at boot, graph drives, unexercised after drives, Tab order).
@task_item n=9 part=touches
contract/actions-answer/causes: the order page is where the caused answer is met with an error, a log and a panic item; measured, no item carries an owner, as the UI graph reads none for the page's controls.
@task_item n=10 part=touches
decision/ui-graph-one-node-per-snapshot-node and decision/ui-graph-carries-no-values: the order page's controls read no owner in the UI graph (all None), pinned by the order page's stand check.
@task_item n=11 part=touches
contract/ui-graph-state/use-graph-state: the order page registers two graph states, quantity (written by the quantity input) and placed (written by Submit's click).
@task_item n=12 part=touches
contract/handler-coverage/answer: handlers at boot gain the order page's four (back, quantity input, Submit click, Export click), and two stay unexercised after drives (back, Export).
@task_item n=13 part=touches
decision/keyboard-activation-enter-space and decision/tab-and-shift-tab-move-focus: the order page is driven by Tab to Submit and Enter on it, and its walk runs Shift+Tab back.
@task_item n=14 part=touches
decision/invalid-state-not-colour-alone and decision/invalid-state-not-by-class-alone: the order page's failure carries the invalid colour pair and is stated as text, Order failed, with a Retry button.
@task_item n=15 part=touches
decision/interactive-nodes-named: the order page adds four named interactive controls and a Retry shown on failure.
@task_item n=16 part=touches
decision/contrast-exempt-disabled-controls: Retry is enabled once shown, so its ratio is asserted, not exempt.
@task_item n=17 part=touches
decision/stand-checks-repeated-three-rounds: the repeated stand round now takes the order page's stand check by its prefix.
@task_item n=18 part=touches
any row that counts the served stand apps as six or names the lean tasks beside the served set: the served set is now seven, order the seventh.
@task_item n=19 part=touches
decision/capability-statement-reach-by-hand: the capability statement page was edited (served apps, boot actionable counts, where caused is met); its accessibility reach text was not.
@task_fact n=1 slug=keyboard-navigation
fact: The order page's stated Tab order is back, quantity, Submit, Export; with its failure shown Retry takes Submit's place, and both orders are walked forward and back in both render modes.
@task_fact n=2 slug=cognitive-accessibility
fact: A Tab order reached only after a state change (the order page with its failure shown) is stated as a local table of the keyboard check, not in the shared Tab order table — the exception to stating focus order once.
@task_fact n=3 slug=visual-design-verification
fact: The order page's contrast is asserted at boot and with its failure text, Retry and placed status shown, the invalid pair red on pale red among its pairs, in both render modes.
@task_fact n=4 slug=screen-reader-support
fact: A control keyed by HTML ids under no keyed element reads no owner in the UI graph, so a caused item from it carries no owner.
@task_item n=1 part=untouched
The platform accessibility adapter, its reach and the tier 0 scope: no adapter was added to any stand binary and no assistive technology reach moved.
@task_item n=2 part=untouched
ARIA state and property reads, role mapping by HTML-AAM and accessible name precedence: the engine's reads did not change; the order page uses plain native controls.
@task_item n=3 part=untouched
The single and multiple select contracts, open list keys, colours, motion and row heights: the order page has no select.
@task_item n=4 part=untouched
The range input, Cells grid and CRUD list keyboard contracts: none of those tasks changed.
@task_item n=5 part=untouched
Hidden and inert content, hit-testing, focusability rules and focus-visible: no engine focus or hit rule moved.
@task_item n=6 part=untouched
The snapshot node, snapshot text form, snapshot diff, password masking and screenshot contracts: no driver schema, node key, verb or cause changed.
@task_item n=7 part=untouched
The lint's contrast finding contract (bounds, defect, id, ratio, required) and the single contrast formula: no lint or ratio helper changed.
@task_item n=8 part=untouched
The accessibility feature defaults and the session library always enabling accessibility: no feature or crate changed.
