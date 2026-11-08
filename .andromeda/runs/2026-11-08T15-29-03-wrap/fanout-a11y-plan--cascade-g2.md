@proposal n=1 detector=D-retired severity=warning file=screen-reader-support anchor=fct-pfylw46r basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-vbjuu4aa
section: ## 7. Screen Reader Support
change: replace «and the listener mark `data-dioxus-id` (so a hidden control is still held, and so is an element whose listener a re-render removed, its mark still standing, so the check over-includes, conservative, while a `mounted` listener writes none — read, not run)» with «and, since 2026-11-08-what-can-i-do-here, the element's exact current listeners holding an event other than `mounted`, from the vdom walk's `element_facts`, through the crate-private `Readings` it shares with `actionable_nodes` (so a hidden control is still held and an element whose listener a re-render removed is not; until then it read the listener mark `data-dioxus-id`, which over-included such an element, conservative — the mark itself stays, written by the bridge and read by event dispatch)»
sidecar: a11y-plan screen-reader-support#4.1: the actionable-key check's listener leg moved from the data-dioxus-id mark to the exact current listeners (2026-11-08-what-can-i-do-here)
rationale: the report's Retired bullet retires unkeyed_actionable's reading of the data-dioxus-id mark as its listener leg, and Symbols / APIs states the leg now reads the exact current listeners from element_facts, domain, order, fields and remedy() unchanged (fct-yftgk4an)
@proposal n=2 detector=D-retired severity=warning file=body anchor=fct-77ewqepn dependent_of=D-retired basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-vbjuu4aa
section: ## 7. Screen Reader Support
change: replace «the listener mark over-includes an element whose listener was removed (conservative)» with «the check and `actionable_nodes` read one actionable rule, the listener leg on the vdom walk's exact current listeners, never the listener mark, so an element whose listener was removed is not held»
sidecar: a11y-plan §7 body: the actionable-key line's listener leg reads the exact current listeners, not the mark (2026-11-08-what-can-i-do-here)
rationale: the body's broad line still leans on the mark's over-inclusion as the check's conservative leg; the report retires that leg (Retired) and states the one rule Readings read by both readers (fct-yftgk4an, fct-7utc4dyd)
@proposal n=3 detector=D-cascade severity=warning file=screen-reader-support anchor=fct-cbzuzrms basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 7. Screen Reader Support
change: replace «the reading `NodeState.clickable` and the graph's handlers take, so neither over-reports as the mark does» with «the reading `NodeState.clickable`, the graph's handlers and, since 2026-11-08-what-can-i-do-here, the one actionable rule `Readings` — read by `unkeyed_actionable` and `actionable_nodes` — take, so none over-reports as the mark does»
sidecar: a11y-plan screen-reader-support#4.5: the exact listener reading's readers gain unkeyed_actionable and actionable_nodes (2026-11-08-what-can-i-do-here)
rationale: the report's sweep lists fct-cbzuzrms as changed under the unkeyed pattern (the exact reading's readers), and Symbols / APIs states Readings takes its listeners from element_facts and is read by both unkeyed_actionable and actionable_nodes
@proposal n=4 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-pfylw46r fact=F1 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 7. Screen Reader Support
change: add after it: «- `DioxusDocument::actionable_nodes`, read-only under `accessibility`, lists the UI graph's nodes in snapshot pre-order that the actionable rule holds for or whose graph `owner` is set, as `ActionableNode { id, owner, keyed }`, `keyed` true where the id holds no `/` (packages/dioxus-native-dom/src/actionable.rs:49-60; packages/dioxus-native-dom/src/actionable.rs:118-146) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, actionable_nodes and ActionableNode (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs states actionable_nodes, its domain, order, owner rule and keyed reading; task fact F1
@proposal n=5 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-cbzuzrms fact=F2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 7. Screen Reader Support
change: add after it: «- The crate-private `Readings` in dioxus-native-dom is the one actionable rule — `Node::is_focussable()`, or an interactive role of the unchanged 27 of `is_interactive`, or current listeners holding an event other than `mounted` — read by both `unkeyed_actionable` and `actionable_nodes` (packages/dioxus-native-dom/src/actionable.rs:62-81) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, Readings as the one actionable rule (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs names Readings the ONE actionable rule read by both readers, and the (a11y) outcome reads «One rule: Readings, read by both» (fct-7utc4dyd); task fact F2
@proposal n=6 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-pfylw46r fact=F3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-otmrcodh
section: ## 7. Screen Reader Support
change: add after it: «- The `keyed` reading on the `actions` answer is the actionable-key check's first wire form; the `keyed` field on the carriers is PROVISIONAL pending the founder's own word (the overseer's call, 2026-11-08) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, the keyed reading as the check's first wire form, provisional (2026-11-08-what-can-i-do-here)
rationale: report Expected amendments states the keyed reading as the check's first wire form, the keyed field on the carriers PROVISIONAL for the founder's own word, the authority the overseer's calls (inputs#I4); task fact F3
@proposal n=7 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-gg5bgkx2 fact=F4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 7. Screen Reader Support
change: add after it: «- The `actions` verb (`Session::actions`) is one more read-only reader: for every actionable node it answers each verb of the table's instance rows taking an id with the cause `check_target` would refuse it with; it changes nothing — no focus, list, scroll or diff change — and writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId` (packages/escher-driver/src/actions.rs:40-102; packages/escher-driver/src/execute.rs:498-526) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, the actions verb as a read-only reader (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs and Schema / config state the read and what its answer carries; the (arch) and (security) outcomes read it changes nothing and holds no sentinel or MASKED_VALUE (fct-ai45gnz6, fct-a7grs7bs); task fact F4
@proposal n=8 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-ymr6jwcn fact=F5 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-d57gqkg5
section: ## 7. Screen Reader Support
change: add after it: «- `ACTIONABLE_AT_BOOT` pins the actionable nodes at boot as measured — counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2 — and the four lean tasks hold 3 `disabled` controls at boot, asserted by `stand_actions` in both layout modes (tests/blitz-tests/tests/common/mod.rs:93-102) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, ACTIONABLE_AT_BOOT counts (2026-11-08-what-can-i-do-here)
rationale: report «Measurement recorded, not assumed» gives 2 · 7 · 3 · 11 · 678 · 2, and the outcome names the 3 disabled controls on the lean tasks (fct-ccyc2gwa); task fact F5
@proposal n=9 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-vee7m5at fact=F6 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ccyc2gwa
section: ## 7. Screen Reader Support
change: add after it: «- In the `actions` answer a flight booker option reads `covered` with owner `flight-type` until its list opens, `flight-return-date` reads `disabled`, and `crud-person-14` reads `off-screen` until a scroll (`stand_actions`, both layout modes) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, the flight booker's options covered until the list opens in the actions answer (2026-11-08-what-can-i-do-here)
rationale: report Outcome (matrix#v030-06) names the_flight_bookers_options_read_covered_until_the_list_opens and the_last_crud_row_reads_off_screen_until_a_scroll; task fact F6
@proposal n=10 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-pfylw46r fact=F7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-7utc4dyd
section: ## 7. Screen Reader Support
change: add after it: «- A fixture's unkeyed button reads `keyed` false in the `actions` answer and is the one `unkeyed_actionable` reports; an inert or hidden element is not listed by `actionable_nodes` — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, keyed false agrees with unkeyed_actionable; inert and hidden not listed (2026-11-08-what-can-i-do-here)
rationale: report (a11y) outcome names the_fixtures_read_covered_unkeyed_and_fonts_unavailable, and New text lists actionable::tests::an_inert_or_hidden_element_is_not_listed; task fact F7
@proposal n=11 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-cbzuzrms fact=F8 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-kqqs4uuc
section: ## 7. Screen Reader Support
change: add after it: «- A listener removed on a re-render is read by neither `unkeyed_actionable` nor `actionable_nodes`, and the unit test also asserts that `data-dioxus-id` outlives the listener, the in-tree witness that the old mark leg would have counted it (`actionable::tests::a_listener_removed_on_a_re_render_is_read_by_neither_reader`, packages/dioxus-native-dom/src/actionable.rs:324-374) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, the removed-listener witness read by neither reader (2026-11-08-what-can-i-do-here)
rationale: report Deviations states the removed-listener unit test also asserts data-dioxus-id outlives the listener, and the (a11y) outcome names the test; task fact F8
@proposal n=12 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-gg5bgkx2 fact=F9 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 7. Screen Reader Support
change: add after it: «- On Cells 461 of the 678 cells read `off-screen` for `click`, `type` and `screenshot` and all 678 take `scroll`; the unpaged `actions` answer is 106,088 bytes, asserted above 100,000 and within 1,048,576 bytes in both layout modes (`the_whole_cells_answer_is_within_the_answer_bound`), a stated limit owned by `res-7cc5bi4k` — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: a11y-plan screen-reader-support: new fact, the Cells actions answer's off-screen count and size (2026-11-08-what-can-i-do-here)
rationale: report Counts / qualifiers moved, «New, no prior site», gives 461 off-screen, 678 scroll and 106,088 bytes, with stand_actions' bounds; the res-7cc5bi4k widening is the operator's yes (inputs#I5); task fact F9
@remark
No detector fires (the report adds no UI element, a11y n/a, and no violation or log schema change); the body's snapshot line fct-uz4kdss3 names the graph but not the actions reader, and the owed replay test of a kept actions step (Addendum 1, a CARRY on Handler coverage) is test-plan's, not this group's.
