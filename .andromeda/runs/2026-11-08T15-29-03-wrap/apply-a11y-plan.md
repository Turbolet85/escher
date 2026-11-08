@apply_row n=1 ref_file=fanout-a11y-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=2 v=folded_into to_file=fanout-a11y-plan.md to_n=1
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=1 v=applied edit=E2
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=1 v=applied edit=E3
@apply_row n=5 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=3 v=applied edit=E4
@apply_row n=6 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=4 v=applied edit=E5
@apply_row n=7 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=5 v=applied edit=E8
@apply_row n=8 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=6 v=applied edit=E6
@apply_row n=9 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=7 v=applied edit=E10
@apply_row n=10 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=8 v=applied edit=E12
@apply_row n=11 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=9 v=applied edit=E13
@apply_row n=12 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=10 v=applied edit=E7
@apply_row n=13 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=11 v=applied edit=E9
@apply_row n=14 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=12 v=applied edit=E11
@fact id=fct-77ewqepn expect_rev=3
old: the listener mark over-includes an element whose listener was removed (conservative)
new: `unkeyed_actionable` and `DioxusDocument::actionable_nodes` read one actionable rule, its listener leg the vdom walk's exact current listeners holding an event other than `mounted`, so an element whose listener a re-render removed is read by neither, the `data-dioxus-id` mark serving event dispatch alone

@fact id=fct-d5bs73pt expect_rev=2
old: its `graph` module), while its
new: its `graph` module) and its read-only actionable-node reader (`DioxusDocument::actionable_nodes`, an `ActionableNode`, re-exported from the crate root beside `UnkeyedActionable`), which shares the one crate-private actionable rule with the check in the same `actionable` module, while its
old: packages/dioxus-native-dom/src/dioxus_document.rs:337-353)
new: packages/dioxus-native-dom/src/dioxus_document.rs:337-353; packages/dioxus-native-dom/src/actionable.rs:49-60; packages/dioxus-native-dom/src/actionable.rs:118-146; packages/dioxus-native-dom/src/lib.rs:37)

@fact id=fct-pfylw46r expect_rev=3
old: and the listener mark `data-dioxus-id` (so a hidden control is still held, and so is an element whose listener a re-render removed, its mark still standing, so the check over-includes, conservative, while a `mounted` listener writes none — read, not run), with no second role mapping
new: and the element's exact current listeners holding an event other than `mounted`, from the vdom walk's `element_facts`, through the crate-private actionable rule `Readings` it shares with `actionable_nodes` (so a hidden control is still held, and an element whose listener a re-render removed is not; the listener mark `data-dioxus-id` stays, written by the bridge and read by event dispatch alone), with no second role mapping
old: packages/dioxus-native-dom/src/actionable.rs:93-157;
new: packages/dioxus-native-dom/src/actionable.rs:62-81; packages/dioxus-native-dom/src/actionable.rs:93-157;
old: the click field at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: the click field at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the listener leg at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-cbzuzrms expect_rev=2
old: the reading `NodeState.clickable` and the graph's handlers take, so neither over-reports as the mark does
new: the reading `NodeState.clickable`, the graph's handlers and the one actionable rule `Readings`, read by `unkeyed_actionable` and `actionable_nodes`, take, so none over-reports as the mark does
old: the walk's listeners at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: the walk's listeners at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the actionable rule's readers at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-pfylw46r as=n1
- `DioxusDocument::actionable_nodes`, read-only under dioxus-native-dom's `accessibility` feature, lists the UI graph's nodes in snapshot pre-order that the actionable rule holds for or whose graph `owner` is set, each an `ActionableNode` of `id`, `owner` and `keyed`, `keyed` true where the id holds no `/` (packages/dioxus-native-dom/src/actionable.rs:49-60; packages/dioxus-native-dom/src/actionable.rs:118-146) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n1 as=n2
- The `keyed` reading on the `actions` answer is the actionable-key check's first wire form; OPEN — the `keyed` field on the carriers is PROVISIONAL · owner: the ratification of the carriers' `keyed` field — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n2 as=n3
- A fixture's unkeyed button reads `keyed` false in the `actions` answer and is the one `unkeyed_actionable` reports (`stand_actions::the_fixtures_read_covered_unkeyed_and_fonts_unavailable`); an inert or hidden element is not listed by `actionable_nodes`, whose domain is the UI graph's nodes (`actionable::tests::an_inert_or_hidden_element_is_not_listed`) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-cbzuzrms as=n4
- The crate-private `Readings` in dioxus-native-dom is the one actionable rule — `Node::is_focussable()`, or one of the 27 interactive roles `is_interactive` holds, or current listeners from the vdom walk's `element_facts` holding an event other than `mounted` — read by both `unkeyed_actionable` and `actionable_nodes` (packages/dioxus-native-dom/src/actionable.rs:62-81) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n4 as=n5
- A listener removed on a re-render is read by neither `unkeyed_actionable` nor `actionable_nodes`, and its unit test also asserts that `data-dioxus-id` outlives the listener, the in-tree witness that a reading of the mark would still count it (`actionable::tests::a_listener_removed_on_a_re_render_is_read_by_neither_reader`, packages/dioxus-native-dom/src/actionable.rs:324-374) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-gg5bgkx2 as=n6
- The `actions` verb (`Session::actions`) is one more read-only reader: for every actionable node it answers each verb of the verb table's instance rows taking an id with the cause `check_target` would refuse it with, a node resolving to no element reading `stale`; it changes nothing — no focus, snapshot or diff moves and an open list stays open — and writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId` (packages/escher-driver/src/actions.rs:40-102; packages/escher-driver/src/execute.rs:498-526) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n6 as=n7
- On Cells the `actions` answer lists 678 nodes, 461 of them reading `off-screen` for `click`, `type` and `screenshot` and all 678 taking `scroll`; the unpaged answer is 106,088 bytes, asserted above 100,000 and within 1,048,576 bytes in both layout modes (`stand_actions::the_whole_cells_answer_is_within_the_answer_bound`), a stated limit owned by residual `res-7cc5bi4k` — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-ymr6jwcn as=n8
- `ACTIONABLE_AT_BOOT` pins the actionable nodes at boot — Counter 2, Flight Booker 7, Timer 3, CRUD 11, Cells 678, the load page 2 — and the four lean tasks hold 3 `disabled` controls at boot, asserted by `stand_actions` in both layout modes (tests/blitz-tests/tests/common/mod.rs:93-102) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-vee7m5at as=n9
- In the `actions` answer the flight booker's two options read `covered` with owner `flight-type` until its list opens, `flight-return-date` reads `disabled`, and `crud-person-14` reads `off-screen` until a `scroll`, in both layout modes (`stand_actions::the_flight_bookers_options_read_covered_until_the_list_opens`, `stand_actions::the_last_crud_row_reads_off_screen_until_a_scroll`) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
@pattern id=a11y-01
fixed: the listener mark over-includes
@pattern id=a11y-02
fixed: so the check over-includes
@pattern id=a11y-03
fixed: so neither over-reports as the mark does
