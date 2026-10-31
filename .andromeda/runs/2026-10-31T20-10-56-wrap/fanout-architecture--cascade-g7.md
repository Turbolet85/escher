proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 32
    section: "## Occupied Resources"
    change: "In the session checks' own state directories, add `hl-popup` to the named `hl-*` directories and state after that list: since 2026-10-31-select-popup the popup tests' directories are `cc-popup` (`cli_commands.rs:442`) and `hl-popup` (`host_log.rs:296`), under `CARGO_TARGET_TMPDIR`; no `ml-popup` exists, `mcp_log`'s popup test recording nothing."
    sidecar: "res-filesystem: the select popup's test-only state directories `cc-popup` and `hl-popup` registered; no `ml-popup`."
    rationale: "Report Symbols / APIs, Test-only state directories, and Reverted / negative API facts: the two directories exist under `CARGO_TARGET_TMPDIR` and `mcp_log`'s popup test records nothing."
    fact: F28
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 57
    section: "## Occupied Resources"
    change: "Where the eight seven_guis targets are listed, `host_log` runs on `crud`, on `cells` and, since 2026-10-31-select-popup, for a select's list opened and chosen through the host, keeping the Cells part-read parenthesis on `cells`; the rest of the sentence stands."
    sidecar: "res-process-wide-state-and-threads: `host_log`'s spawned host also runs for a select's list; the count of eight targets stands."
    rationale: "Report New text, by line: `host_log.rs` 282-393 adds `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, spawning its own host at 298-308 and reaching it by `attach` at 313-321; the entity names only `crud` and `cells`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 71
    section: "## Occupied Resources"
    change: "To what the shared module `examples/seven_guis/tests/common/mod.rs` holds for the two log checks, add `option_needles`, the needles of a select's options, since 2026-10-31-select-popup."
    sidecar: "res-process-wide-state-and-threads: the shared test module's `option_needles` added to the log checks' readers."
    rationale: "Report New text, by line: `common/mod.rs` 425-453 adds `pub fn option_needles()`, reading a select's children; `host_log` and `mcp_log` each gain a test named for logging no option at trace, and the Outcome reads both at 0 needles."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 21
    section: "## Cross-cutting Patterns"
    change: "Redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing (asserted by `opening_moving_scrolling_and_closing_each_ask_a_redraw`), the Tab key asking for none of its own."
    sidecar: "crosscut-service-wiring: a select list's opening, moving, scrolling and closing added to what requests a redraw."
    rationale: "Report New text, by line: `select_popup.rs` 1045-1090 holds `opening_moving_scrolling_and_closing_each_ask_a_redraw` over a `CountingShell` counting `request_redraw`; the task's Touches names this list."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "Write «written at five kinds of site» for «written at four kinds of site», and open the closing clause «and, with no select list standing, is not written by node creation, hover, active, scroll, a viewport resize, a resource load, an animation or layout, so a `resolve` with nothing written marks nothing in either layout mode»."
    sidecar: "crosscut-invalidation-and-state-integrity: five kinds of site; the no-scroll, no-layout clause holds with no select list standing."
    rationale: "Report Spec claims disproved, `changed-set`: the sentence is false where a list stands and holds with no list open; Counts / qualifiers moved, `changed-set`: open and close marking the select is a new kind of site."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "In `BaseDocument::apply_select_choice`'s clause write «on a `multiple` select or a single select's open list, a click on a row» for «on a `multiple` select, a click on a row»."
    sidecar: "crosscut-invalidation-and-state-integrity: `apply_select_choice` also marks a select for a click on an open list's row."
    rationale: "Report Outcome, the one write path: `click_list_row` and `choose_current_and_close` call `apply_select_choice`, so a click on a single select's list row now reaches the marking the clause limits to a `multiple` select."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "As the fifth kind of site, on a continuation line so no line passes 2 000 chars: since 2026-10-31-select-popup, a select's list opening or closing marks the select; a page scroll or a move of the select's box that closes the list marks it through `reconcile_select_list` at the next resolve, a wheel elsewhere marks it through `close_select_list`, and a highlight or a wheel move inside the list marks nothing."
    sidecar: "crosscut-invalidation-and-state-integrity: the select list's marks in the changed set stated as its fifth kind of site."
    rationale: "Task fact F26; report Spec claims disproved, `changed-set`, and Counts / qualifiers moved, `changed-set`."
    fact: F26
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 275
    section: "## Cross-cutting Patterns"
    change: "The changed set is written at five kinds of site and, with no select list standing, by no node creation, hover, scroll, resize, load, animation or layout; a page scroll, a move of the select's box or a wheel elsewhere that closes a list marks its select."
    sidecar: "architecture Cross-cutting Patterns, Invalidation: the broad line's changed-set clause corrected for a standing select list."
    rationale: "Report Spec claims disproved, `changed-set`, names architecture:275 false where a list stands; Expected amendments carries §Cross-cutting Patterns → Invalidation."
    basis: .andromeda/architecture.md:275
    dependent-of: D-cascade
note: F14's clearing of the list on a removed node is standard-contracts' to carry, the Invalidation group's general rule holding as written; the body's Service wiring broad line stays broad and is not false.
