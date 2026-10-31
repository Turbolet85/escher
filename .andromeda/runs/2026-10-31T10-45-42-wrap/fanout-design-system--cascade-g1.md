proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "| seven_guis stand | — | The stand's accent (its load page's `load-fetch` included, and since 2026-10-31-engine-debts the keyboard-focus outline of its Cells grid's current cell, `.cells-grid:focus .cell.current` with `outline: 2px solid` at `outline-offset: -2px`, examples/seven_guis/src/tasks/cells.rs:551-554), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |"
    sidecar: "2026-10-31-engine-debts: the stand row names the accent's new use, Cells' current-cell focus outline, its value held."
    rationale: "Report, Symbols / APIs F13: the CSS rule at cells.rs :551-554 uses `#3a5ce5`, the accent's held value, so no new token is introduced and the row's list of uses gains one; Coverage reads tokens stand accent `#3a5ce5` as passing."
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 28
    section: "## Color Palette › ### Core Colors"
    change: "Add a row after this one: | blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row, under `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked`: Chromium's inactive selected pair, 12.56:1 measured by `select_interaction`, hardcoded because the engine holds no tokens (packages/blitz-dom/assets/default.css:161-188) |"
    sidecar: "2026-10-31-engine-debts: the engine-default chosen-row pair `#c8c8c8` / `#000000` joins the palette's blitz-dom default rows."
    rationale: "Report, Schema / config: the default sheet adds a new engine-default chosen-row pair at :161-188, and Coverage reads it as engine default, hardcoded by design, 12.56:1 measured; the palette's rows of the sheet's default colours are incomplete without it. This is the palette's own row; the fact's statement as a component is F2, a surface fact."
    basis: packages/blitz-dom/assets/default.css:161-188
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "Add a list item after this one: - Since 2026-10-31-engine-debts a transform whose determinant is 0 or not finite answers no hit for the whole subtree, ratified by the founder, 2026-10-31 (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: a non-invertible transform takes no hit for its whole subtree."
    rationale: "Report, Symbols / APIs F2: `Node::hit_inner` answers `None` for the whole subtree at :1472-1477, and Expected amendments names this fact for design-system §Depth Strategy as new text, `non-invertible`'s control unfired; Outcome (F2) reads `hit_non_invertible_transform` 2 as met."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "Add a list item after this one: - Since 2026-10-31-engine-debts `BaseDocument::visible_region` reads each clipping box through its transforms: exact under a translate and a scale, under a rotation the bounding box of the rotated padding box, so it holds corners the box does not show, and `None` for a mapped box with a non-finite edge; with no transform on the chain it reads as before, ratified by the founder, 2026-10-31 (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: `visible_region` reads a clipping box through its transforms, a rotation reading the rotated padding box's bounding box."
    rationale: "Report, Symbols / APIs F1: `visible_region` reads each clipping box through its transforms via `padding_box_in_view` at :790-812; it sits beside this entity's clipping rule, which already reads a transformed and scrolled box's hit, and Outcome (F1, arch, design) reads `visible_region_transformed` 6 as met."
    basis: packages/blitz-dom/src/scrolling.rs:790-812
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 10
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "Add a list item after this one: - Since 2026-10-31-engine-debts the stand's Cells grid brings its current cell into view with no animation, by a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` from its key handler (tests/blitz-tests/tests/stand_keyboard.rs:293-335; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: Cells' arrow keys bring the current cell into view by a synchronous instant `scroll_to`."
    rationale: "Report, Symbols / APIs F13: arrows bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, and Reverted / negative API facts records that the async form panicked and did not ship; this entity holds programmatic scrolling's `Smooth` and `Instant`, and the session adds no animation."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:293-335
    fact: F8
note: D-design-tokens did not fire, no Coverage row reading hardcoded✗ (the chosen-row pair reads hardcoded by design, the outline the stand accent); F1 to F4 are surface's, including the select rows' 2px padding that spacing#2.5 does not carry; the outline's full literal with `#3a5ce5` stays with layout-templates by the report's Expected amendments, F7 naming its use and geometry here without the hex.
