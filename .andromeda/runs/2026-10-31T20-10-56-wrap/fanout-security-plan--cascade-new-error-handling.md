proposals:
  - detector: D-new-detail
    severity: warning
    fact: F2
    file: .andromeda/security-plan-detail/error-handling.md
    near: 36
    section: "## Error Handling"
    change: "- Select list closing: a select removed, disabled, hidden or made inert with its list open closes the list with no panic (`a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`), `clear_interaction_state_for_removed_node` clearing the field for a removed select, and an option dropped with the list open is no row (`an_option_dropped_with_the_list_open_is_no_row`); this holds of the close alone: a key pressed after an ancestor gained `hidden` is the `position_in_containing_block` panic, for any control, list or none, and `select_popup` skips the key press in that one row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Error Handling: the select list's close on a removed, disabled, hidden or inert select, with no panic, and an option dropped with the list open as no row, new at 2026-10-31-select-popup."
    rationale: "The report's Outcome reads the security acceptance as met to its letter with a defect beside it, and Symbols / APIs states the removal arm clearing the field; New text lists both tests. The defect clause keeps the record from reading as covering the key press after `hidden`."
    basis: tests/blitz-tests/tests/select_popup.rs:478-520
  - detector: D-new-detail
    severity: warning
    fact: F3
    file: .andromeda/security-plan-detail/error-handling.md
    near: 30
    section: "## Error Handling"
    change: "- Select list readers: the list's public `BaseDocument` readers `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`, and its public writer `close_select_list`, go through `get_node`, a row with no styles reading `None`, and call no `layout_style()`, so they add no path to `layout_style()`'s panic on a node with no computed styles — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Error Handling: the select list's public readers and writer reach no `layout_style()` panic, new at 2026-10-31-select-popup."
    rationale: "The report's Coverage of new surfaces row for the `BaseDocument` public readers states through `get_node`, a row with no styles `None`, no `layout_style()`; Symbols / APIs names `close_select_list` the public writer."
note: F3's «add no panic path» is written as no path to `layout_style()`'s panic, the one panic the report's coverage row rules out for these readers; F2's `hidden` clause points at the panic F1 records and does not restate F1's row.
