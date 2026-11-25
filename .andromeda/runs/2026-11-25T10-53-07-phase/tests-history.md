@history_item n=1 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-umiuichp, §4) — `~ contract/element-id/virtual-form`: the reserved `{owner}///{key}` form now served in the accessibility tree, snapshot and diff, the walk still writing none; `+ decision/inert-guard-dropped` (PROVISIONAL, delegate). This chunk extends the form to graph, actions, coverage, layout and exploration, so the row's "served in" list grows, and a guard it retires is handled under the inert-guard rule.

@history_item n=2 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-gfskjnev, §3) — `~ contract/driver-session/stale-or-not-found` (a virtual id reads `not-found` on every verb but `click`) and `~ contract/driver-session/refusal-order` (a `click` on a virtual node refused `disabled` · `off-screen` · `covered`), both F10 B PROVISIONAL. This chunk's CARRY re-opens F10 B (`layout` reads a virtual id), so these rows move with it.

@history_item n=3 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-2bpwc464, §1) — `~ contract/driver-refusal-causes/not-found`: every id verb but `click` refuses a virtual node's id `not-found` (F10 B, PROVISIONAL). An answer for a virtual id on `layout` changes this row.

@history_item n=4 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-uboady24, §5) — `~ contract/refusal-cause/not-found` (a virtual id on any verb but `click`) and `~ contract/refusal-cause/covered` (a virtual node with an element over its region's centre, F12 A), both PROVISIONAL. `actions` must list for a virtual node the cause each verb would be refused with, so it reads these two rows, and `layout`'s carve-out moves the `not-found` row.

@history_item n=5 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-qdxfnv67, §5) — `~ decision/reserved-graph-id-form`: narrowed from "no served id holds `///`" to "no id of the UI graph, the actions answer or the lint answer". This is F9 A's exclusion, which the chunk owns and lifts for graph and actions. The row's guard (`stand_graph`) and the no-`///` guards (`stand_actions`, `virtual_nodes_declared`, `virtual_nodes_as_elements`' `the_graph_actions_and_lint_hold_no_virtual_node`) must be re-stated, not just deleted.

@history_item n=6 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (amd-p6pzkktu, §1) — `~ decision/no-screenshot-step-in-committed-checks`: repeatability of a shot holding document text is asserted on one host only, and a canvas-alone shot is held equal across hosts by digest. Relevant if a virtual-node check keeps or compares a shot.

@history_item n=7 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** (amd-ndft4ude, §1) — `~ decision/driver-executor-integration-tested`: the executor verbs, refusals and the lint's readings over a held instance are integration-tested, and the canvas label reader over recorded paint commands is unit-tested. This sets which tier tests the new virtual reads (graph, actions, coverage, layout and explore all hold an instance).

@history_item n=8 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** (amd-2h25ph2l, §4) — `~ contract/lint-findings/order`: a canvas element reads `truncated` once per label, in paint order (PROVISIONAL delegate confirm). Virtual nodes in the lint, if this chunk takes them, must fit this order contract.

@history_item n=9 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-u4oqpxyb, §4) — `~ contract/driver-verbs/verb-table` set to sixteen verbs, and `+ decision/control-leaves-earlier-target-green` (a control in a later test target needs an edit that leaves earlier targets green, since cargo stops at the first red). Both bind this chunk's controls and any verb-table pins.

@history_item n=10 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-iu5jthk4, §3) — `~ decision/socket-carries-commands` (canvas answers cross out), `~ contract/driver-session/reading-verbs` (`canvas` runs no step), `~ contract/escher-session-command-line/command-form` (sixteen verbs, `--full`) and `+ contract/escher-session-command-line/canvas`. A virtual id and name newly in graph, actions or layout answers is a crossing to read against this list and F3 A's exits.

@history_item n=11 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-7xb4smfb, §1) — `~ contract/driver-refusal-causes/fonts-unavailable` (screenshot, lint or canvas) and `~ contract/mcp-server/tools-list` (sixteen, `canvas` after `layout`). These are the refusal-cause and listing rows the `actions` causes for a virtual node read against.

@history_item n=12 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-2wj2ugrv, §5) — `~ decision/typed-password-never-returned`: the masks are held on graph, actions, layout and canvas answers and on kept and explored checks. New virtual entries in those answers stay under this rule (guard `stand_masks_on_new_answers`).

@history_item n=13 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-alveepiy, §6) — `~ contract/mcp-tools-list/order`: sixteen verbs in order. Any count or order this chunk touches sweeps this row.

@history_item n=14 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-d26vi7ce, §3) — `~ decision/no-screenshot-in-cross-os-checks` gains the canvas-alone shot exception, compared across OSes by answer digest. Any cross-OS digest table a virtual read joins follows it.

@history_item n=15 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-wsteqwoi, §1) — `~ decision/no-screenshot-step-in-committed-checks` · why scoped to a shot holding document text. This is a picked row's reason.

@history_item n=16 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (amd-n2omhba3, §2) — `~ decision/count-and-rename-sweeps`: a change of the verb count or of a CI leg's file count sweeps tests, examples and CI-script tests for literal pins. This is the scope's "literal pins to sweep" directive (counts, `VERBS.len()`, `describe()`, capabilities counts).

@history_item n=17 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (amd-stldjcwi, §4) — `~ decision/mutation-controlled-tests`: a case the control leaves green is widened until it reads red. The operator directive requires each control in the plan with its red reading.

@history_item n=18 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-gpqjgetu, §2) — `~ contract/handler-coverage-limits/bubbling-dispatch`: a stopped dispatch counts its innermost listener alone. The coverage reach of a virtual node's click must respect this counting rule.

@history_item n=19 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-lfwo5vi7, §5) — `~ decision/handler-coverage-bubbling-counts-ancestors` (stopped or unwound dispatch), `~ contract/graph-command/argument` (graph `part`) and `~ decision/graph-answer-holds-no-value` (adds `part`/`parts`). The graph's new virtual nodes must page within this part model and hold no value.

@history_item n=20 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-wudwalw5, §3) — `~ contract/driver-session/graph` (`Outcome::GraphPart`, whole nodes per part, states in part 1) and `~ contract/session-wire/graph-answer` (closed list of what crosses). A virtual graph node enters both.

@history_item n=21 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-bc5fqyab, §3) — `~ contract/escher-session-command-line/graph`, `command-form` and `usage-line` (graph `--part`, press `--ctrl`). These are the picked command-line rows a count or usage change sweeps.

@history_item n=22 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-kmmmilr5, §5) — `decision/handler-coverage-bubbling-counts-ancestors` renamed "counts its listeners until stopped".

@history_item n=23 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-wjjnfnof, §5) — `~ contract/session-run/changed-node-states` adds `highlighted`. This is a picked row on what an acting call returns, which a virtual-node click also returns.

@history_item n=24 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-rxg73dpf, §3) — `~ decision/one-contrast-formula`: lint contrast reads through the harness's readers, with no second formula. This binds a lint over virtual nodes.

@history_item n=25 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-mmdtifvl, §2) — `~ decision/cross-os-readings-by-answer-digest` (seven served tasks; digests for snapshot, lint and actions) and `~ decision/count-and-rename-sweeps` except (a count-worded test name pinned as a witness is kept). An `actions` answer that grows on a served task moves the committed digest table. The stand declares no virtual node yet, so this is expected unmoved, to be verified.

@history_item n=26 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-arfsbrh6, §5) — `~ contract/session-run/layout-answer` (equal to the in-process layout read on every node of the seven served screens) and `+ decision/caused-owner-pinned-as-graph-reads` (pin the graph's reading of an owner, not an asserted one). These are the layout-answer row the virtual `layout` answer extends, and the owner-pinning precedent for virtual graph nodes.

@history_item n=27 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-m7fw5fjq, §1) — `~ decision/capability-statement-pinned`: adds the a11y and stand-repeat leg counts to the pinned set. `docs/capabilities.md`'s "Pinned by a check" limits (F9 A, F10 B, F12 A) change with this chunk, and the pin's counts are swept.

@history_item n=28 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-zn44tekk, §1) — `~ decision/capabilities-doc-pinned-to-schema`: two leg counts held to the CI leg script. A new stand file or named target moves them.

@history_item n=29 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-aiakktz6, §2) — `~ decision/capability-check-names-held-to-schema`: only claim-section names and counts are pinned, so a renamed or moved check changes that section by hand. The renamed no-virtual-node guards must be re-mapped in the claim section by hand.

@history_item n=30 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-755msr7u, §4) — `~ decision/capability-statement-from-schema`: schema counts are read from the schema's tables, none hand-copied.

@history_item n=31 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-de7olhxu, §5) — `~ decision/typed-password-never-returned` widened to graph, actions and layout answers and to kept and explored checks. Virtual entries in those answers inherit it.

@history_item n=32 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-4oiy4oeg, §2) — `+ decision/requirement-witness-inside-stand-repeat`: every requirement names its witness inside stand-repeat, pinned by function name. v040-11's witness must sit inside stand-repeat, and a renamed witness reads red.

@history_item n=33 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-iriujbey, §5) — `decision/typed-password-never-returned` guard gains `stand_masks_on_new_answers`. This is the check that reads the answers this chunk extends.

@history_item n=34 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-3quv32td, §3) — `+ contract/stand-boot/boot-alone` and harness `from_component_alone` / `vdom_alone` (TimerTicks provided before the build). A fixture canvas a test declares may boot through this path.

@history_item n=35 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-b4vvhst7, §4) — `+ decision/harness-graph-tests-enable-accessibility`: a harness unit test of the UI graph turns `accessibility` on through its dev-dependency, since the graph is gated by it. This is a trap for any new unit test of the graph's virtual nodes.

@history_item n=36 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-cdohja6t, §5) — `~ contract/session-layout/changes-nothing` (in process or through the verb) and `+ contract/session-run/layout-answer`. These are the layout rows an answer for a virtual id extends, its no-change promise holding in both modes.

@history_item n=37 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-smsbvscr, §3) — `~ contract/escher-session-command-line/command-form`, `usage-line` and `~ contract/driver-session/reading-verbs` (adds `layout`).

@history_item n=38 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-ewa3qy3e, §3) — `~ decision/socket-carries-commands` adds layout answers out (founder-ratified). A virtual id in a layout answer crosses on this carrier.

@history_item n=39 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-h4mn5gu4, §4) — `~ contract/driver-verbs/verb-table` set to fifteen. Earlier link of the verb-count chain.

@history_item n=40 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-yfp5ovdp, §2) — `+ decision/count-and-rename-sweeps`. Its origin: a missed numeric pin and a missed renamed witness. The renamed no-`///` guards and witness strings are exactly this trap.

@history_item n=41 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (amd-fcxxug5t, §4) — `+ decision/no-change-read-control-needs-pending-work`: a control of a read that must change nothing needs pending work, since a pump on a settled screen is invisible. This binds controls of graph, actions and layout reads over virtual nodes.

@history_item n=42 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (amd-447m7y2r, §5) — `+ contract/session-layout/changes-nothing`, `+ decision/layout-reason-words-each-read` (every placing word and size cause read on a fixture in both modes, tables counting rows) and `+ decision/stated-tolerance-read-inclusively`. A layout answer for a virtual id brings any new word under the each-word rule.

@history_item n=43 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (amd-uqva53q6, §3) — `~ contract/session-recording/exploration-checks` (a panicked path's check holds the panicked step) and `~ contract/driver-session/acting-verbs` (adds `caused`). Exploration acting on a virtual node keeps one check per path that replays green.

@history_item n=44 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (amd-n3bdcrdl, §3) — `~ decision/socket-carries-commands` adds a step's caused with its error text (operator-resolved). Earlier link of the socket-crossing list.

@history_item n=45 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (amd-rlvpc7s2, §2) — `+ decision/engine-items-asserted-on-tracing-build`: engine caused items are asserted on a tracing build only, through the engine's constant, and the check runs on both builds.

@history_item n=46 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (amd-mnnwqipq, §3) — `~ overview/session-lifecycle`: the `run session` selection runs `cli_flow` flows. This is the picked overview.

@history_item n=47 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (amd-ij32pdcj) — the session binary's `coverage` role, ratified by the founder (was PROVISIONAL). This is the coverage carrier whose answer a virtual node's reach enters.

@history_item n=48 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (amd-3ieo66hz, §3) — `+ contract/session-recording/exploration-checks`: one kept check per explored path, replaying green. A virtual node as an explore candidate lands under this contract.

@history_item n=49 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (amd-4xioq6cx, §5) — `+ decision/exhaustion-asserted-on-finite-page`: the Counter was measured not to exhaust, so exhaustion is asserted only on a finite fixture page. Exploration over a virtual fixture canvas follows the same rule.

@history_item n=50 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (amd-knbkpfwu, §4) — `~ decision/mutation-controlled-tests`: a control that removes a bound loosens it to twice the budget, so the mutated run stays bounded. This matters for an exploration-budget control.

@history_item n=51 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (amd-lfmouigx, §2) — `+ decision/cross-os-readings-by-answer-digest`: served tasks read across OSes by snapshot, lint and actions digests against one committed table. An actions-answer change on a served screen moves this table.

@history_item n=52 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (amd-qq6cg6bh, §5) — `decision/handler-coverage-reads-every-replayed-screen` set to standing (founder-ratified). The coverage of a virtual node is read on every replayed screen through the UI graph.

@history_item n=53 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (amd-gkbc5lsj, §5) — `~ decision/layout-tests-in-both-layout-modes` except: upstream targets running one mode stand as named exceptions. Escher's own new checks run both modes.

@history_item n=54 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-hf3zl3jw, §2/§5) — handler coverage's three stated limits: a handler inside a canvas is not counted (deferred to 0.4.0, the canvas plans), a handler on an unreached screen is not listed, and the bubbling limit (since retired). `contract/handler-coverage-limits/canvas-handler` and `decision/handler-coverage-canvas-uncounted` are this chunk's to revise for a virtual node's reach, and the capability claim map changes with them.

@history_item n=55 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-zf5wigrz, §1/§4) — the reached record (`packages/dioxus-native-dom/src/reached.rs`): four `reached::tests` with no feature gate, and the bridge test on a page with no registered state. This is a named surface of the scope. A virtual node's reach extends this record's unit tests.

@history_item n=56 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-i7b3rmrc, §1/§2/§5/§7) — `stand_handler_coverage` (8) with `HANDLERS_AT_BOOT`, `UNEXERCISED_AFTER_DRIVES` and Cells' generated rows, and `Check::replay_each` reading the graph at boot and after each step. `handler_coverage` is covered by integration, not unit. This is where a virtual node's coverage check is held.

@history_item n=57 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-i5idzbin, §5) — handler coverage reads every screen the replays reach, through the UI graph. It was PROVISIONAL, later ratified (n=52).

@history_item n=58 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-eiwetouu, §1/§4) — the coverage line's form `{"unexercised":[{"id","owner","event"}],"checks","held"}` with `owner` left out when absent (`json.rs`), and the role's refusals (`cli_tests.rs`). These are the picked `contract/coverage-line/*` rows a virtual id in the line must meet.

@history_item n=59 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-gh4iy36p, §5) — `stand_replay` gained the replay of a kept `actions` step, a flipped `keyed` stopping at step 3. A kept actions step carrying virtual nodes replays through this check.

@history_item n=60 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-7hxia3by, session-binary) — the `coverage` role's shape, bounded read and endings `0` · `1` `malformed` · `2` · `3`. This is the picked `contract/escher-session-command-line/coverage` row.

@history_item n=61 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-pwghjeji, §1/§5/§9) — `host_log`'s coverage run reads 0 id, name, coverage, sentinel, path and recorded needles at `trace` under both builds (`coverage_needles()`). Binary-level virtual reads are out of scope, but the needle lists are the precedent.

@history_item n=62 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-nhvwrpl5, §1/§4/§6) — `cli_commands` `coverage_replays_a_kept_check_and_lists_the_handlers_it_left`, and the usage line's coverage role. This is the picked `contract/cli-coverage-role/stdout` row.

@history_item n=63 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-x5hh3g7d, §1/§4) — one actionable rule for both readers (`unkeyed_actionable` and `actionable_nodes`) read from current listeners, never the `data-dioxus-id` mark. The guard `a_listener_removed_on_a_re_render_is_read_by_neither_reader` keeps it in-tree. Listing a virtual node in `actions` must keep this one-rule decision.

@history_item n=64 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-n3udqhi7, §1/§2/§5/§7) — `stand_actions` (8) over six served screens in both modes: causes equal to `Session::run`, flight options `covered` with owner, `fonts-unavailable` named for `screenshot`, a read changing nothing and holding no typed text, `MASKED_VALUE` 0, an argument refused `malformed`. `Session::actions` and `check_target` are covered by it, not by a unit test. This is the check a virtual node's listing joins, and `stand_actions.rs` holds a no-`///` guard to sweep.

@history_item n=65 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-jjah2vzy, §5/§7) — `ACTIONABLE_AT_BOOT` (counter 2 · flight-booker 7 · timer 3 · crud 11 · cells 678 · load 2), and the Cells actions answer at 106,088 B inside the answer bound. These are measured pins a change to actions moves. The stand declares no virtual node, so they are expected unmoved, to be verified.

@history_item n=66 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-ldelf34f, §1/§4/§6) — `cli_commands` and `mcp_server` actions checks, and `stand_mcp_tools`' shared read-verb helper. These are the picked `contract/cli-actions/output` and `contract/mcp-actions/argument` rows.

@history_item n=67 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-epp6lr5n, §1/§5) — `host_log` and `mcp_log` drive one `actions` call and read 0 needles at `trace` (`actions_needles()`). `stand_act_spans` reads the `actions` command line recording `verb` alone. This is the picked `contract/command-span/actions` row.

@history_item n=68 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-dkb4oe4y, §4/§10) — `schema_tests` per-verb field-kind table (`graph.nodes`, `actions.nodes`), `describe`'s count row, the cli_tests row count, and the MCP listing test renamed with the verb count (`the_listing_is_the_fourteen_verbs_of_the_table`). This is the precedent of a count-worded test name a sweep meets.

@history_item n=69 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-poiocitr, §1/§5) — `stand_graph` (6): every graph node a snapshot node, no served id holding `{owner}///{key}`, a read changing nothing and holding no typed text, the Cells answer within the bound, the verb equal to `DioxusDocument::graph`. Its no-`///` assertion is F9 A's guard, which this chunk inverts, and "every graph node a snapshot node" (`decision/ui-graph-one-node-per-snapshot-node`) is the rule it extends.

@history_item n=70 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-a2uwkadt, §4) — `graph::tests` (each node's component, current listeners, declared writes, an option owned by its select) and the two `element_id` tests pinning the reserved `{owner}///{key}` form, `virtual_node_id` `None` for an empty or `/` key, and the walk calling it nowhere. These are the unit tests a virtual graph node extends.

@history_item n=71 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-aj6rkfgc, §1) — `NodeState.clickable` built, and `stand_non_lean_tasks`' `CLICKABLE` table (Circle Drawer 1, Cells 676). This is the picked `contract/snapshot-node/clickable` row. F12 A's "virtual `clickable` false" is this chunk's to revise, and the table may move.

@history_item n=72 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-xep3xm3z, §1/§3) — dioxus-native-dom unit scope: the reserved virtual form's two tests, `clickable` in the snapshot builder, and the one `cfg(test)` shim `dispatch_to_vdom`. This is a precedent for test-only seams in the crate.

@history_item n=73 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-ckioxq2y, §1) — the `unconfirmed` flag's stated limit: a conditional write a run skipped reads `unconfirmed`, and `GRAPH_DRIVES` drives each once. These are the picked `decision/unrun-declared-write-reads-unconfirmed` and `decision/graph-drives-confirm-every-write` rows.

@history_item n=74 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-z23dex63, §3/§7) — `GRAPH_DRIVES` in `tests/common/mod.rs`, and the session-check builders `graph()` and `graphed()`. These are the shared drives and readers the graph and coverage checks rely on.

@history_item n=75 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-tvsd3b6b, §1/§3/§5) — a graph call's `command` line records `verb` alone, and `host_log`/`mcp_log` read 0 graph needles at `trace` (`graph_needles()`). This is the picked `contract/command-span/graph` row.

@history_item n=76 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-5zqg4udh, session-wire) — a `graph` answer crosses the socket: ids, owner ids, names, flag words and booleans, never a value or `NodeId`. This is a boundary widening, so a virtual node's id and name in it are read against this ratified crossing.

@history_item n=77 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-7efxs2hi, session lifecycle) — `graph` as an instance verb, `Outcome::Graphed`, running no step. Earlier link of the reading-verbs contract.

@history_item n=78 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-afxaahjb, §1/§4) — the verb-table pins (schema_tests, `VERBS`, describe's row, mcp_tests' listing, json.rs pinning graph `nodes` and a node's `clickable`, cli_tests rows). These are the literal pins a count or node-key change sweeps.

@history_item n=79 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-halqoyxj, §1/§7) — the Cells graph answer measured at 293,869 B in one answer (stated limit "An unpaged graph", residual `res-7cc5bi4k`, since paged). A forecast of answer size was disproved before, so size claims are measured, not forecast.

@history_item n=80 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-2ho3wh7t, §5) — a node's current listeners are read from the vdom walk's dynamic attributes, not the bridge's listener attributes, which over-report. This is the picked `decision/listeners-read-from-vdom-walk`. A virtual node has no vdom listener, so its handler and click reading must be stated apart.

@history_item n=81 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-4g5qmthy, §4/§5) — `covered`'s fixed meaning adds the closed-select clause, pinned by `schema_tests`, `refusal` and `json` as `'static` text, with `covered`'s name, remedy and order unmoved. A reworded meaning for a virtual node (as at virtual-nodes-as-elements) sweeps these pins.

@history_item n=82 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-gn53juc4, §3) — `scroll_into_view` on a row of an open list moves the list's `scroll_y` alone. This is the picked `contract/harness-input/scroll-into-view`, which `scroll` on a virtual id stays outside (F10 B) unless this chunk moves it.

@history_item n=83 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-glk62rg7, §1) — the lint skips an entry in a select's box (`Entry.in_select_box`). This is the precedent for how the lint holds or skips non-element entries, relevant to F9 A's "lint holds no virtual node".

@history_item n=84 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (amd-flbkdilp, §1/§4) — `Defect::ALL` names the six defects in order, and findings follow document order, then defect order. These are the picked `contract/lint-defects/*` rows.

@history_item n=85 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-7qex4fkt, §2/§3/§7) — kept checks replay through `Session::run`, stopping at the first differing acceptance or answer, and committed checks are replayed by `stand_replay`. These are the picked `decision/recorded-checks-replayed-as-tests`, `decision/replay-through-the-call-path` and `contract/check-replay/stop` rows. Handler coverage and exploration ride this path.

@history_item n=86 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-nlavi4cc, §3/§7) — a page more than one check reads goes in `tests/fixtures/mod.rs`, booted with stand options, and a single-reader page stays in its file. This is the picked `decision/shared-fixture-pages`. The scope's declared fixture canvas, read by several reads here, belongs in `fixtures`.

@history_item n=87 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-ybvlv3ad, §1/§4) — causes are eight, `the_eight_causes_are_named_in_order`, `fonts-unavailable` the eighth. This is the picked `decision/eight-refusal-causes-in-order`. The scope's boundary is that a new cause is a fork.

@history_item n=88 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (amd-iygsbz5o, §1) — `stand_requirement_sweep` and `WITNESSES`, the requirement-to-check record. A witness renamed by this chunk reads red there (see n=32, n=40).

@history_item n=89 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (amd-472linzq, §1/§2/§4) — `test_capability_statement.py` pins every verb and cause as a code span and the counts in words, and `## Where each claim is proven` names each claim's check, with only names and counts pinned. The doc's limits for F9 A, F10 B and F12 A change with this chunk.

@history_item n=90 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-rrbdvino, §3) — a shot's PNG crosses in band as padded base64 under the unchanged answer bound (founder fork 1), and a crop of no pixel reads `off-screen`. This is the picked `decision/screenshot-in-band`, and `screenshot` with a virtual id stays `not-found` under F10 B.

@history_item n=91 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (amd-u7jdserc, §2/§3) — a socket-holding check is `cfg(unix)`-gated, while MCP stdio-child checks are not and run on windows. Trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. These are the picked `decision/socket-session-checks-unix-gated` and `decision/mcp-capture-counts-by-span` rows.

@history_item n=92 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (amd-ebg5mftg, §1/§5) — the lookup answers `stale` or `not-found`, and the refusal order is `disabled` · `off-screen` · `covered`. This is the origin of the picked refusal-order and stale/not-found rows this chunk's F10 B carve-outs amend.

@history_item n=93 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-s53lzeiq, §9) — the same check read green per package and red in the workspace leg, so a sink capture counts lines by target, never in total. This is the picked `decision/sink-capture-counted-by-target`.

@history_item n=94 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (amd-s4wow4l3, §5) — `host_log` covers a sink-installing host at `trace`, its needles read from the stand, both streams drained by threads. Trap: `start` on a piped host fills the pipe. These are the picked `decision/drain-host-pipes-while-running` and `decision/no-ids-or-names-in-stand-logs` rows.

@history_item n=95 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (amd-jdiryrjq, §4) — every table a driver test reads is stated in the test with its row count asserted, and an assertion over a call carries a row index only. Trap: the crate's count appears in two word orders. These are the picked `decision/table-tests-count-their-rows` and `decision/call-assertions-carry-row-index` rows.
