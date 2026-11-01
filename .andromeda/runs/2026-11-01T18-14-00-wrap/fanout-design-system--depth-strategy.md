proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 20
    section: "## Depth Strategy"
    change: "The stacking-root list now reads: A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, `isolation: isolate`, or a `mix-blend-mode` other than `normal`; the atomic paint effects sentence stays opacity, filter, clip-path and mask-image, `mix-blend-mode` not among them."
    sidecar: "Depth Strategy: `mix-blend-mode` other than `normal` joins the stacking-root conditions (upstream 6fca6308d, #1316); the atomic paint effects list is unmoved."
    rationale: "report.md:15 and report.md:45 put the new arm in the stacking-root check, depth-strategy.md:20 owed the change; report.md:100 leaves the atomic half to this detector, and report.md:275 lists node.rs's added lines as 60-61, 85, 531-538 and 1382-1384, so the arm is a stacking-root condition alone and no line was added to the atomic reader the file cites at 1390-1419."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 39
    section: "## Depth Strategy"
    change: "New bullet after this line: `mix-blend-mode` and `isolation` now paint: `render_element` pushes a viewport-wide blend layer and pops it last, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode` and `convert_blend_mode` is the module's `pub(crate)` mapping; unit tests hold the layers, no test paints a blended element to a frame, and the open select list paints last, past the blend layers (packages/blitz-paint/src/render.rs:485-506; packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/layers.rs:104-181; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "Depth Strategy: a new value row, the blend layer `mix-blend-mode` and `isolation` paint through, its frame-test gap stated and the select list recorded past it."
    rationale: "report.md:15 states the paint path and its coordinates, report.md:104 the unit tests and the missing frame test, report.md:147 the select list past the blend layers; the file's layer row at line 39 names opacity, filter and backdrop-filter alone."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 38
    section: "## Depth Strategy"
    change: "New bullet after this line: An inline box holding only block children paints no shadow, background or border of its own: `render_element` skips them (`paints_own_box`) for a box flagged `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, read through the public `Node::is_inline_with_only_block_children`; unit tests hold the flag, no test reads the skip on a frame (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/document.rs:4096-4296; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "Depth Strategy: a new value row, the inline box with only block children paints no box shadow of its own (upstream f76777766, #1319)."
    rationale: "report.md:19 states the flag, its setter and the paint skip of the box's own shadow, report.md:106 its unit tests and the missing frame test; the file's shadow rows at lines 37-38 hold no such exception."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
note: "D-design-tokens holds, every Coverage row reading tokens n/a (report.md:102-108); D-retired has no hit, the file naming none of the Retired items, `Node`'s `TODO: mix-blend-mode` included; the body's Depth Strategy text states nothing this chunk moves."
