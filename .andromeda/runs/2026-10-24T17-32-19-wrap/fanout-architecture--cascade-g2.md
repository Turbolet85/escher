proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 54
    section: '## Standard Contracts'
    change: 'In the text extent row, the limits sentence reads: Its limits: RTL and vertical text are read as LTR horizontal lines, and a transformed box is read at its client rect — each pinned since 2026-10-24-stand-requirement-sweep by a `text_extent` test in both layout modes: `right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box` (RTL text read as an LTR line from the content box), `vertical_text_reads_as_a_horizontal_line` (vertical text read as a horizontal line) and `a_transformed_box_reads_its_lines_at_its_client_rect` (the content box equal to `get_client_bounding_rect`, the lines unscaled) (tests/blitz-tests/tests/text_extent.rs:127-152; tests/blitz-tests/tests/text_extent.rs:154-174; tests/blitz-tests/tests/text_extent.rs:176-205). The rest of the row stands.'
    sidecar: 'standard-contracts — the text extent row: its three stated limits now pinned by text_extent tests in both layout modes (2026-10-24-stand-requirement-sweep)'
    rationale: 'Report Changes, Symbols / APIs (text_extent.rs: three tests, both layout modes, green under -p blitz-tests and --workspace) and Counts fact 5 (extent-limits: standard-contracts.md:54 — the limits stand as limits; a pointer to their check may be added); Expected amendments name this row (→ Scrolling, selection, tree, text_extent limits asserted). The limits themselves are unchanged — no engine source changed.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:46
    fact: F4
note: 'Two moved rows the report names fall outside this group''s picked entities and are not proposed here — standard-contracts.md:181 (the co-reading "asserted by no check" → asserted by stand_lint, fact 4) and standard-contracts.md:256 (the stand-repeat selection clauses, fact 1, with F1 and F2 beside it); and standard-contracts.md:165''s stand_act_spans.rs:555-567 citation may have shifted (lines added at 6-8 and 243, net shift not stated) — a cites re-point, not a content edit.'
