proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 63
    section: "## Established Decisions"
    change: After «`scroll_into_view` writes every scrolling box on its target's containing-block chain, innermost first, and then the viewport» add «, a row of an open select list excepted (§Standard Contracts → Scrolling, selection, tree)»; the rest of the sentence stands.
    sidecar: dec-scrolling-and-input gains the open list's row as the one exception to `scroll_into_view` writing the whole chain.
    rationale: The report's Symbols state that `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving; the ratified sentence reads every box with no exception, and the behaviour's own record is F15 in standard-contracts.
    basis: packages/blitz-dom/src/scrolling.rs:669-672
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 69
    section: "## Established Decisions"
    change: After the clause «wheel line deltas × 20» and its citation add «, a wheel over an open select list scrolling the list alone and a wheel elsewhere closing the list and then scrolling (packages/blitz-dom/src/events/pointer.rs:1194-1202)».
    sidecar: dec-scrolling-and-input states what a wheel does over and outside an open select list.
    rationale: The report's Symbols state that `handle_wheel` scrolls the list over it, else `close_select_list`, then scrolls; the user-scroll rules state neither branch.
    basis: packages/blitz-dom/src/events/pointer.rs:1194-1202
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: Make the keys' subject «a focused single-choice `select` (no `multiple`), its list closed, that is no disabled form control», and replace «every other key does nothing — Enter and Space open nothing, and a released Space activates no select» by «Enter, and a Space with no typed text standing, opens its list (below), every other key does nothing, and a released Space does nothing more».
    sidecar: Retired on the select model, Enter and Space opening nothing on a single select; they open its list.
    rationale: The report's Retired bullet names «Enter and Space open nothing on a single select», and `enter-space` lists established-decisions:78 as changed; `press_select_key` opens on Enter and on a Space with no typed text standing, and `press_open_list_key` is the open list's own key match.
    basis: tests/blitz-tests/tests/select_popup.rs:202-230
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: In «A click on a select clears the typeahead and the document's text selection and focuses it through `generate_focus_events` / `set_focus_to`» add after `set_focus_to` «, and on a single select opens its list or closes one standing»; the disabled gate's sentence stands.
    sidecar: The select model's click now opens or closes a single select's list as well as focusing it.
    rationale: The report's Symbols state that `handle_click`'s `select` arm opens a single select's list or closes one standing, a new private `focus_select` holding what the arm did; the (arch) acceptance reads two `click`s open then close, each focusing.
    basis: tests/blitz-tests/tests/stand_act_popup.rs:379-406
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "In the limits sentence delete «no popup is painted (owner: the route entry «Select popup»),» with its owner phrase, and replace «`size` is unread» by «a single select's `size` is unread»; the clause on an option of a closed single select, refused `covered` under a group too, stands."
    sidecar: Retired on the select model, the limit no popup is painted with its owner phrase, and `size` unread narrowed to a single select.
    rationale: The report's Retired bullet names «no popup is painted» and «a `multiple` select's `size` is unread»; `no-popup` and `size-unread` list established-decisions:78 as changed, `owner-title` drops the owner phrase there, and `closed-option` holds an option of a closed select still `covered`.
    basis: packages/blitz-dom/src/stylo.rs:1124-1139
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- The select list stands beside the select model, a direct engine change for every document, remote pages included — a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing (packages/blitz-dom/src/document.rs:811-844; packages/blitz-paint/src/render.rs:246-247; tests/blitz-tests/tests/select_popup.rs:202-230), on the founder's ten forks, all A — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md (its inputs#I4).»
    sidecar: New record, the select list as a direct engine change for every document and what opens it.
    rationale: F1; the report's Symbols, pointer and keys, and Decisions & corrections for the forks; the Playbook's engine behaviour change for every document is carried there.
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- An open select list closes with no event on Escape, a second click on the select, a press outside, a wheel outside the list and a page scroll or move of the select's box, and removal, `disabled`, `hidden` or `inert` on the select closes it too (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:891-907; packages/blitz-dom/src/events/pointer.rs:619-633; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520).»
    sidecar: New record, what closes an open select list.
    rationale: F2; the report's Symbols (`close_list_outside`, `reconcile_select_list`, `handle_wheel`) and its (security) acceptance on removed, disabled, hidden and made inert.
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- On an open select list Tab chooses the current row and closes before focus moves, in either direction; Enter, Space and a click on a row choose through `apply_select_choice`, and a disabled row changes nothing and leaves the list open; the move keys shift the highlight over enabled rows and choose nothing (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/select_popup.rs:377-388).»
    sidecar: New record, how an open select list chooses, Tab included, through the one path.
    rationale: F3; the report's Keys and Pointer symbols, and its (arch) acceptance that `click_list_row` and `choose_current_and_close` call `apply_select_choice` with no other writer in the diff.
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- Rejected and unbuilt for the select list — `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, and the list's colours in `default.css` (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
    sidecar: New record, the select list's rejected approaches.
    rationale: F4; the report's Reverted / negative API facts state the plan's rejected approaches stand unbuilt.
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- The select list's standing limits — the thumb is painted and not draggable, a highlighted row's contrast is unread by the lint, a larger authored font clips in the one row height, the highlight is absent from the snapshot and the tree, and a single select's `size` is unread (docs/capabilities.md:208-210; docs/capabilities.md:211-213; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
    sidecar: New record, the select list's five standing limits.
    rationale: F5; the five limits are the P5 card's row 1 as the report quotes it (inputs#I7), and capabilities' added lines state the highlight and the thumb.
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- The select list's merge surface is sixteen files, a line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md).»
    sidecar: New record, the select list's merge surface and its owner.
    rationale: F8; the report's P5 list names `evidence/merge-surface.md`, sixteen files, its line owned by Epoch 3's sync; the select model's own surface on line 79 stands apart.
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 80
    section: "## Established Decisions"
    change: After «disabled rows are never chosen;» add «a key brings the current row into view only where the select carries a valid `size` (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459);»; the rest stands.
    sidecar: A `multiple` select's keys bring the current row into view where a valid `size` stands.
    rationale: The report's Keys symbols state that `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; the semantics of `press_multiple_select_key` are otherwise kept.
    basis: tests/blitz-tests/tests/select_popup.rs:995-1032
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 88
    section: "## Established Decisions"
    change: At the end of this record add «A row of an open select list is never taken as a thumb drag, `hit_with_scrollbar` answering `select_list_hit` first and no scrollbar with it (packages/blitz-dom/src/document.rs:2404-2406); the hazard stands at every other overlay scrollbar.»
    sidecar: The overlay scrollbar's hazard gains its neighbour clause, an open list's row is never a thumb drag.
    rationale: The report's `hit-first` lists established-decisions:88's neighbour clause as changed, a row never a thumb drag, with the thumb hazard standing; its Hit symbols state the order.
    basis: packages/blitz-dom/src/document.rs:2404-2406
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 121
    section: "## Established Decisions"
    change: Replace «named by its non-empty `label`, which stays unpainted» by «named by its non-empty `label`, which an open select list paints as a header (below)»; the rest of the record stands.
    sidecar: Retired on `optgroup`, a group's `label` unpainted; an open list paints it as a header.
    rationale: The report's Retired bullet names «a group's label is unpainted», and `group-label` lists established-decisions:121 as changed.
    basis: tests/blitz-tests/tests/select_popup.rs:805-855
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 121
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- A group's `label` paints as a header heavier than a row on every build by a synthesized bold, `stroke_text_as` carrying `RunPaint`; a header chooses nothing, and a group with no label reads the whole list's box (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup.rs:805-855; tests/blitz-tests/tests/select_popup_painted.rs:282-297).»
    sidecar: New record, a group's header in an open select list.
    rationale: F7; the report's Paint symbols and its Deviations on a group with no label, the operator's direction past implement for a header heavier than a row on every build, at `715798ce2`.
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 122
    section: "## Established Decisions"
    change: "Add one new record after this line, its text «- A `multiple` select's `size` hints a `height` of `size` × 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint, with `select[multiple][size] { overflow-y: auto; }` and `line-height: 1.2` on its rows in the default sheet (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; tests/blitz-tests/tests/select_popup.rs:995-1032).»"
    sidecar: New record, `size` read on a `multiple` select, its hint and its sheet rules.
    rationale: F6; the report's `size` symbols, and its Coverage row on `size` for the hostile and bad values.
    fact: F6
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/established-decisions.md
    near: 200
    section: "## Established Decisions"
    change: Add one new record after this line, its text «- The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff — `differs` naming `bounds`, a closed set inside the ratified crossing — and as pixels in a `screenshot`, with no new carrier, exit, cause, session error, node key, log field or span; an option's and a group's label leaving as pixels in every shot of an open list is a boundary widening, recorded for the founder's answer — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md.»
    sidecar: New record, how the select list crosses the carriers, its labels as pixels a widening for the founder's answer.
    rationale: F9; the report's Schema / config and its Coverage row name the pixels a boundary widening, P2's halt, on his recorded word inputs#I4 and CARRY `cry-musoudbr`; escalated since validate halts once for his answer card and this text holds no ratification until it does.
    fact: F9
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 62
    section: "## Established Decisions"
    change: Replace «no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select, under a group too, takes no click and stays refused `covered`, `size` is unread and Ctrl+A is not modelled» by «a click, Enter or a Space with no typed text standing opens on a single select a list the engine paints over the page, a direct engine change for every document; an option of a closed single select, under a group too, takes no click and stays refused `covered`, a single select's `size` is unread, a `multiple` select's `size` hints its height and Ctrl+A is not modelled».
    sidecar: The select model's broad line retires no popup, Enter and Space opening nothing and `size` unread on a `multiple` select.
    rationale: The report's Retired bullet names the three limits; `no-popup`, `enter-space` and `size-unread` each list architecture:62 as changed, and `closed-option` holds the option's refusal.
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 56
    section: "## Established Decisions"
    change: After «user scrolls chain to the viewport» add «, a wheel over an open select list scrolling the list alone», and after «so it never animates a nested box» add «, a row of an open select list excepted from the chain»; the rest of the line stands.
    sidecar: The scrolling broad line carries the open list's wheel and its row's `scroll_into_view`.
    rationale: The broad line the two scrolling edits detail states both rules with no exception; the report's Symbols state `handle_wheel`'s branch and that `scroll_into_view` on a row moves no chain or viewport.
    basis: packages/blitz-dom/src/scrolling.rs:669-672
note: F13, F14, F15, F17 and the rest of F10 to F25 are standard-contracts' and F26 cross-cutting-patterns', the new-detail pass places them, F17 holding the `:open` clause established-decisions#17.2 was picked for; F9 waits on the founder's answer card; the `node.rs:1775` panic is security-plan's and P5's, no edit here.
