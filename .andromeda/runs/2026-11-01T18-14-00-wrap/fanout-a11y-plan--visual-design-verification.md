proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 11
    section: "## 6. Visual Design Verification"
    change: In line 11's clause on `Harness::text_contrast`, the `None` list should read «it reads `None` where an element on that walk has `opacity` below 1, a non-empty `filter`, a `background-image` layer or a `mix-blend-mode` other than `normal`»; nothing else on the line moves.
    sidecar: a11y-plan §6, `text_contrast`'s `None` list gains a `mix-blend-mode` other than `normal` (2026-11-01-upstream-sync-ui-graph).
    rationale: report.md:24 adds the blend to the `None` conditions and report.md:99 carries it to a11y-plan §6; report.md:44 names `visual-design-verification.md:13` as changed where the line states its conditions, read by its excerpt alone (report.md:99), and in this file the conditions stand on line 11, line 13 stating none.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: After line 15, add one record «- Since 2026-11-01-upstream-sync-ui-graph `mix-blend-mode` and `isolation` paint in every document, and `text_contrast` reads `None` past an element on its walk with a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, read through `resolved_style_value`), asserted by `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`; `Session::lint`'s `contrast` reads the same with no second formula, no row of `stand_contrast` moves, and no test paints a blended element to a frame (packages/blitz-test-harness/src/contrast.rs:154-156; packages/blitz-test-harness/src/contrast.rs:304-315 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)».
    sidecar: a11y-plan §6, a record of blending now painted, what `text_contrast` and the lint read past it, no stand row moved and no frame test of a blended element (2026-11-01-upstream-sync-ui-graph).
    rationale: report.md:15 has blending paint on every document; report.md:24 gives the reader, the lint and the unmoved rows; report.md:104 gives the coverage row with its missing frame test; report.md:310 lists the new test's lines.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 93
    section: "## 6. Visual Design Verification"
    change: After line 93, add one record «- The open list paints last, past the blend layers `render_element` pushes for `mix-blend-mode` and `isolation`, `paint_select_list`'s call unedited and `select_popup_painted` green on both builds; no test paints a blended element to a frame, so the order under a blend is read and not measured (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)».
    sidecar: a11y-plan §6, the open list paints past the new blend layers, read and not measured under a blend (2026-11-01-upstream-sync-ui-graph).
    rationale: report.md:147 records the list painting last past the blend layers as met, with `paint_select_list`'s call holding no hunk; report.md:104 states no test paints a blended element to a frame; line 93 holds the list's paint past an `opacity` ancestor and a clipping box and says nothing of the blend layers this chunk adds.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:147
note: D-a11y-surface and D-a11y-obs-schema hold (no interactive element added, report.md:102-108; no violation schema moved, report.md:31); body `a11y-plan.md:160` owes the same `None` edit and lies outside this file; unmeasured inference, not proposed since report.md:106 reads a11y n/a, that `text_contrast` may still composite the background of an inline box with only block children, which `paints_own_box` now skips.
