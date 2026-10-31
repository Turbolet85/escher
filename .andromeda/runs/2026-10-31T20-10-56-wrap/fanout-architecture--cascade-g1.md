proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 51
    section: "## Design Philosophy"
    change: "After the clause that user-initiated and programmatic scrolls share one scrolling primitive, differing only in the `ScrollRequest`, add: an open select list's rows stand apart from it, a wheel over the list and `scroll_into_view` on a row moving the list's own `scroll_y`, the latter writing it at once and returning with no chain or viewport moving."
    sidecar: "design-philosophy: the one scrolling primitive gains its exception, an open select list's own `scroll_y`."
    rationale: "Report Changes, Symbols / APIs: `handle_wheel` scrolls the list over it, `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving (`scrolling.rs` 669-672), with crate-private `scroll_select_list_to` and `scroll_select_list_by` on `BaseDocument`; the universal claim no longer holds for these scrolls."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
note: "No proposal, but for the new-detail pass: F16's rows laid out at paint (`select_row_layout`) sits beside design-philosophy:13's painting assumption and :47's Parley phase, and F25's no local gate building `blitz-paint/font-embolden` beside stack-and-technologies:76's pre-push gate; both facts are standard-contracts', and those lines stay true as they stand."
