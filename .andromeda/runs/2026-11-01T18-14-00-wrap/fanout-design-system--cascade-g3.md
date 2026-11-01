proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 20
    section: "## Depth Strategy"
    change: "In the stacking-root list replace «atomic paint effects, or `isolation: isolate`» with «atomic paint effects, `isolation: isolate`, or `mix-blend-mode` other than `normal`»; the atomic paint effects clause stands as it reads, opacity, filter, clip-path and mask-image."
    sidecar: "depth-strategy: `mix-blend-mode` other than `normal` joins the stacking-root list; it does not join the atomic paint effects."
    rationale: "The report marks depth-strategy.md:20 changed for `mix-blend-mode` among the stacking roots (report.md:45), the arm at node/node.rs:1382-1384 (report.md:15) sitting in the stacking-root check, so stacking is the measured half (report.md:100); nothing in the report measures it as an atomic paint effect, so that list is left as it reads."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:45
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 36
    section: "## Depth Strategy"
    change: "After «so it stands above every box of the page» add «, past the blend layers as well»."
    sidecar: "depth-strategy: the open select list's paint-last row now says it paints past the blend layers."
    rationale: "The report's design criterion reads the list as painting last, past the blend layers, `paint_select_list`'s call holding no hunk (report.md:147); `render_element` now pushes a viewport-wide blend layer (report.md:15), which the row's «every box of the page» does not name."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:147
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 38
    section: "## Depth Strategy"
    change: "- An inline box with only block children, read through `Node::is_inline_with_only_block_children`, paints no shadow, background or border of its own (`paints_own_box`), with no frame test behind it (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/document.rs:4096-4296; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "depth-strategy: new row, an inline box holding only block children paints no shadow, background or border of its own, with no frame test."
    rationale: "`f76777766` #1319 sets the flag and `render_element` skips such a box's own shadow, background and border (report.md:19); the Coverage row reads unit tests in `document.rs` and frame ✗ (report.md:106). Placed after the shadow rows."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 39
    section: "## Depth Strategy"
    change: "- `mix-blend-mode` and `isolation` paint, through `blitz-paint`'s private `blend` module and its `pub(crate)` `convert_blend_mode`, with `render_element` pushing a viewport-wide blend layer and popping it last (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/render.rs:485-506; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "depth-strategy: new row, `mix-blend-mode` and `isolation` now paint through a viewport-wide blend layer."
    rationale: "`6fca6308d` #1316 makes both paint (report.md:15) and `blend` is a new private module (report.md:29); this follows the opacity, filter and backdrop-filter layer row, whose border-box clip this viewport-wide layer does not share."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:15
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 39
    section: "## Depth Strategy"
    change: "- No fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; blending's witnesses are `layers.rs`'s 3 unit tests and `contrast.rs`'s fixture (packages/blitz-paint/src/layers.rs:104-181; packages/blitz-test-harness/src/contrast.rs:304-315; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "depth-strategy: new row, blending has no frame test; its witnesses are the `layers.rs` unit tests and the `contrast.rs` fixture."
    rationale: "The Coverage row for blend and isolation reads unit `layers.rs` 3 and ✗ no test painting a blended element to a frame, with only `contrast.rs`'s fixture naming it (report.md:104). Follows F2's new row, itself placed after this line."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
    fact: F3
note: "Coverage's `tokens` flag reads n/a on every new surface, so D-design-tokens fires nothing; a failed load ending the driver's wait is owed to the harness and driver contracts (report.md:25, report.md:49), not to motion#4.1, whose clock claim holds; on the report's open question (report.md:100) this pass reads `mix-blend-mode` as stacking only, the atomic list unedited."
