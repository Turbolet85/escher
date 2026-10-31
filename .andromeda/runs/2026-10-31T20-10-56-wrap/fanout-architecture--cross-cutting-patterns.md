proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "On line 40 read «is written at five kinds of site» for «is written at four kinds of site», end line 40 with a semicolon after the `apply_select_choice` clause, and add as its own line after it, before the «not written by» clause: « and, since 2026-10-31-select-popup, the opening and closing of a single select's list, each marking its select: opened by `open_select_list` (packages/blitz-dom/src/document.rs:811-844), closed by `close_select_list`, as a wheel elsewhere closes it (packages/blitz-dom/src/document.rs:881-889), and by `reconcile_select_list` at the next `resolve`, as a page scroll or a move closes it (packages/blitz-dom/src/document.rs:891-907), a highlight or a wheel move inside the list marking nothing»"
    sidecar: "§Cross-cutting Patterns → Invalidation: the changed set gains a fifth kind of site, a select list's opening and closing, marking its select (2026-10-31-select-popup)."
    rationale: "Report Counts `changed-set` names open and close marking the select a new kind of site, and Spec claims disproved `changed-set` names `reconcile_select_list` and `close_select_list`; Expected amendments carries §Cross-cutting Patterns → Invalidation. Line 40 stands near 2 000 chars, so the new kind takes its own line; binds with .andromeda/architecture.md:275."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "In line 40's `apply_select_choice` clause read «where a key or typeahead, a click on an open list's row (packages/blitz-dom/src/events/pointer.rs:642-655) or, on a `multiple` select, a click on a row» for «where a key or typeahead or, on a `multiple` select, a click on a row», the existing citation staying with the `multiple` select's click."
    sidecar: "§Cross-cutting Patterns → Invalidation: `apply_select_choice` also marks a single select on a click on an open list's row (2026-10-31-select-popup)."
    rationale: "Report Outcome, (arch) the one write path: `click_list_row` and `choose_current_and_close` call `apply_select_choice`, so a click on a row now reaches that writer on a single select too; line 40 limits a row's click to a `multiple` select."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "Move line 40's closing clause to a line of its own after the new fifth kind, reading: « — and, with no list standing, is not written by node creation, hover, active, scroll, a viewport resize, a resource load, an animation or layout, so a `resolve` with no list standing and nothing written marks nothing in either layout mode;»"
    sidecar: "§Cross-cutting Patterns → Invalidation: the changed set's «not written by scroll, resize, load, animation or layout» now holds with no select list standing (2026-10-31-select-popup)."
    rationale: "Report Spec claims disproved `changed-set`: the claim is false where a list stands, since a page scroll or a move that closes a list marks its select at the next resolve and a wheel elsewhere marks it; with no list open it holds. Moving the clause also keeps line 40 under the 2 000-char entity limit."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:40
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 41
    section: "## Cross-cutting Patterns"
    change: "In line 41's parenthesis read «bounds-only movement changes a reading and marks nothing, but for a move that closes a list, which marks its select» for «bounds-only movement changes a reading and marks nothing»."
    sidecar: "§Cross-cutting Patterns → Invalidation: bounds-only movement marks nothing except a move that closes a select list (2026-10-31-select-popup)."
    rationale: "Report Spec claims disproved `changed-set`: a move that closes a list marks its select through `reconcile_select_list`, so the unqualified «marks nothing» on line 41 is another occurrence of the retired claim."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:41
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 41
    section: "## Cross-cutting Patterns"
    change: "Add a row after line 41: «- A single select's open list is interaction state held in `BaseDocument`'s one field `select_list`, `None` at construction (packages/blitz-dom/src/document.rs:343-344): removing its select clears the field (packages/blitz-dom/src/document.rs:1408-1410), `resolve` calls `reconcile_select_list` after layout and before `refresh_hover` (packages/blitz-dom/src/resolve.rs:133-135), and a list whose select is made inert, disabled or hidden closes (tests/blitz-tests/tests/select_popup.rs:478-520); the list's field `fresh` keeps a `:focus` rule that moves the select's box, restyled after the click that focuses and opens in one step, from reading as a move that closes it (tests/blitz-tests/tests/select_popup.rs:1105-1122)»"
    sidecar: "§Cross-cutting Patterns → Invalidation: new row, a select's open list as interaction state, cleared on removal and reconciled at resolve (2026-10-31-select-popup)."
    rationale: "Report Symbols / APIs, `BaseDocument` and The resolve, Deviations from intent on `fresh`, and Outcome (security) on the closes; the file's state-integrity group states how interaction state survives a removal and has no row for this new state."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 27
    section: "## Cross-cutting Patterns"
    change: "Add a row after line 27: «- A single select's open list asks a redraw at its opening, at a move of its highlight, at its scrolling and at its closing (tests/blitz-tests/tests/select_popup.rs:1045-1090)»"
    sidecar: "§Cross-cutting Patterns → Service wiring: new row, a select list's opening, highlight move, scrolling and closing each ask a redraw (2026-10-31-select-popup)."
    rationale: "Report New text, by line lists `opening_moving_scrolling_and_closing_each_ask_a_redraw` in `select_popup`, which its Outcome reads green; line 21 enumerates what asks a redraw and names no list. A row of its own keeps line 21 under the 2 000-char limit; binds with .andromeda/architecture.md:271."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:21
note: "D-arch-resources, D-arch-decisions and D-retired find nothing in this file; .andromeda/architecture.md:271 and :275 state the same claims in broad and move with these, and line 41's founder ratification (2026-10-07) predates the fifth kind of site, on which the report records no fresh word of his."
