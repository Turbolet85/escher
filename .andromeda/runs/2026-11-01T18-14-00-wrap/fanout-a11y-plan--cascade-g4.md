proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 11
    section: "## 6. Visual Design Verification"
    change: "In the `None` clause, read: it reads `None` where an element on that walk has `opacity` below 1, a non-empty `filter`, a `background-image` layer or a `mix-blend-mode` other than `normal`."
    sidecar: "visual-design-verification: `text_contrast`'s `None` conditions gain a `mix-blend-mode` other than `normal`."
    rationale: "report.md:24 has `Harness::text_contrast` read `None` past an element on the walk with a `mix-blend-mode` other than `normal`, and report.md:44 marks this entity changed where it states its `None` conditions; the list sits on the entity's first line, so that line is the one edited."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 160
    section: "## 6. Visual Design Verification"
    change: "Change the parenthesis to: (`None` under opacity below 1, a filter, a background image or a `mix-blend-mode` other than `normal`)."
    sidecar: "a11y-plan §6: the broad SC 1.4.3 line's `None` conditions gain a `mix-blend-mode` other than `normal`."
    rationale: "report.md:44 marks `a11y-plan.md:160` changed where the line states its `None` conditions, and report.md:99 carries the amendment for §6; the line's closing claim on measured stand rows holds, no stand row moving (report.md:24)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:44
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 13
    section: "## 6. Visual Design Verification"
    change: "Add after the entity: - Since 2026-11-01-upstream-sync-ui-graph `Harness::text_contrast` reads `None` past an element on its walk with a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), pinned by `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`; no stand row moves (packages/blitz-test-harness/src/contrast.rs:154-156; packages/blitz-test-harness/src/contrast.rs:304-315 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "visual-design-verification: new detail, `text_contrast`'s `None` past a blend, its reader and pin, no stand row moving."
    rationale: "F1 belongs beside the entity that defines the reader's `None`; report.md:24 states the reader and that no stand row moves, and report.md:146 has `stand_contrast` passing with every table and row unedited."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 13
    section: "## 6. Visual Design Verification"
    change: "Add after the entity: - Since 2026-11-01-upstream-sync-ui-graph `mix-blend-mode` and `isolation` paint on every document, `render_element` pushing a viewport-wide blend layer through blitz-paint's `convert_blend_mode`, and no fork or upstream test paints a blended element to a frame; the absence takes its owner at that wrap's P5 (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/render.rs:485-506 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "visual-design-verification: new detail, blending paints on every document with no frame test, the absence's owner due at this wrap's P5."
    rationale: "F3 sits beside the contrast reader whose `None` answers this paint: report.md:15 states the paint, report.md:104 the ✗ on a frame test and the `None` as its a11y coverage, and report.md:172 has that ✗ take an owner at P5."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "Add after the entity: - Since 2026-11-01-upstream-sync-ui-graph `Session::lint`'s `contrast` reads the same `None` past a blend, through the one reader, so a blended text's pair is unread by the lint (packages/blitz-test-harness/src/contrast.rs:154-156 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "visual-design-verification: new detail, the visual lint's `contrast` reads the same `None` past a blend, a blended text's pair unread."
    rationale: "F2 belongs beside the entity on the lint's `contrast` reading, whose `None` is already not measured; report.md:24 has `Session::lint`'s `contrast` read the same, with no second formula."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
    fact: F2
note: "D-a11y-surface and D-a11y-obs-schema do not fire: the report adds no interactive UI element and moves no violation schema or scrub set (report.md:31); the contract key file `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3`, a `contrast-none` row (report.md:99), lies outside this group's categories."
