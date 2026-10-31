proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 12
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: An open select's list has no motion, and a screen read after a step reads the new state with no `advance`; `scroll_into_view` on one of its rows is the exception to the line above, writing the list's `scroll_y` at once and returning, with no chain or viewport moving (packages/blitz-dom/src/scrolling.rs:669-672; tests/blitz-tests/tests/stand_act_popup.rs:408-423; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
    sidecar: Motion gains a row for the select list, no motion and no `advance` needed, a row's into-view scroll written at once to the list alone (2026-10-31-select-popup)
    rationale: The report's Expected amendments owe §Motion a new row with no master site to find, its Outcome (design) records no motion and no `advance` needed as met, and its Symbols / APIs state the row's into-view write, which line 12's every-box claim does not cover
    basis: packages/blitz-dom/src/scrolling.rs:669-672
note: D-design-tokens' `hardcoded✗` on the select list owes rows in §Color Palette, none here; line 15 stands, the report's `hit-first` reading it no-change; no passage here describes a Retired item as standing; design-system body line 121 repeats the every-box claim of line 12, with no carve-out for a list's row.
