# design extract

## Relevance
partial — no new screen or token; design binds through engine depth and hit rules, the default sheet's select values, the stand's colour record for the contrast rows, and the agent-read output of the driver; every fix below is conditional on the founder's word at P4.

## Constraints
- design-system §Depth Strategy → Engine depth behavior requires hit-testing to stop at the padding box of a node paint clips, and its detail (`.andromeda/design-system-detail/depth-strategy.md`) requires a box both transformed and scrolled to be hit where its rows show. If the transformed-scrolling-box fix is taken, `visible_region` must answer the same place; whether the two readings already diverge only at `visible_region` is research's question.
- design-system §Depth Strategy → Engine depth behavior (detail: `.andromeda/design-system-detail/depth-strategy.md`, the stacking-root and paint-order groups) requires any transform, scale or translate to root a stacking context and `z-index: auto` positioned descendants to share one paint level in tree order. A pan/zoom fixture's absolutely positioned nodes are read under those rules, with a clipped container's hit limit in force.
- design-system §Surface: desktop-native › Tokens (platform-specific) requires a select's look and its chosen-option-alone display to stay engine defaults of the default sheet, with no design token. A `multiple`, `optgroup` or popup fix that edits those rules changes every document's select, so it belongs to its own fork; whether the current sheet alone explains the empty `multiple` box is research's question.
- design-system §Color Palette › Core Colors requires the stand's accent, `load-fetch` included, to be held in layout-templates §Surface: desktop-native → Primary screens, not as design-system values. A served-page contrast row reads that pair from there; a red row moves that record, and the move is a further fork.
- design-system §Typography → Loading requires `blitz-dom` built with `system-fonts` for the blitz-tests crate, since text otherwise measures 0x0 and font-dependent assertions pass vacuously, and requires a driver `screenshot` with no `sans-serif` family to be refused `fonts-unavailable`. Option-bounds, select-label and contrast proofs stand on a resolved font.
- design-system §Motion → This project's values requires a programmatic into-view scroll never to animate a nested scrolling box and the driver's `scroll` to be instant in every box and in the viewport. A `visible_region` or scrolling fix keeps both.
- design-system §Surface: cli › Component Patterns requires `escher-session` and its `mcp` role to hold only JSON on stdout, with no colour and no ESC byte. Moved `off-screen`, `covered` or `lint` text keeps that form and the exit classes unchanged.

## Patterns to follow
- Stand values stay in their one record: design-system §Color Palette › Core Colors, §Depth Strategy → Observed values and §Border Radius name layout-templates as the holder of the stand's colours, shadows and radii, so checks cite that record and restate no literal here.
- Engine-painted control colours are declared per control in `packages/blitz-paint/src/render/form_controls.rs` (per design-system §Color Palette › Semantic Colors and §Color Palette › Border Progression). The checkedness debt is recorded on the tree side; whether paint already reads the checked state, and whether a fix touches it, is research's question.
- Time in a check moves through the harness's `VirtualClock`, and settle only reads it (per design-system §Motion → Animation runtime). A served-page check that clicks a Cells cell in the scrollbar band right after a `scroll` does an `advance` past the fade first (detail: `.andromeda/design-system-detail/motion.md`).
- `escher-session` serves Cells and the load page in the same `TaskShell` as the lean four (per design-system §Surface: cli › Component Patterns, detail: `.andromeda/design-system-detail/surface.md`), so the served-page rows reach them through the task names already listed, with no new app entry.
- If a painted popup is taken, the nearest recorded floating-list precedent is the browser chrome's menu and suggestions layer (per design-system §Depth Strategy → Observed values in the apps and examples), a precedent only; it is browser-app CSS, not an engine default, and form-control text starts from the default sheet's stack (per design-system §Typography).

## Anti-patterns to avoid
- Adding `--color-*` tokens or CSS custom properties to the engine for a fix or a row: design-system §Color Palette (detail: `.andromeda/design-system-detail/color-palette.md`, the absence group) records none in the engine and integration crates, and adding a set is a design decision of its own, not this entry's.
- Pinning a contrast or colour check on a copied hex, so that it stays green when the stand's value moves (per design-system §Color Palette › Core Colors).
- Introducing a transition on a popup or on a focus change without a fork: design-system §Motion → Reduced motion records reduced-motion handling as observed absent, so an animated surface here would ship with no override.

## Contract bindings
- Contrast: `load-fetch`'s colour pair (layout-templates §Surface: desktop-native → Primary screens) ↔ a11y-plan §Contrast SC 1.4.3 and the OPEN gap in a11y-plan §5; witness `stand_contrast`.
- State colour and label: checkbox and radio state (design-system §Color Palette › Semantic Colors, §Color Palette › Border Progression) ↔ a11y-plan §Use of Color SC 1.4.1 and the HTML-AAM `toggled` reading of the checkedness debt.
- Focus: the default input outline (design-system §Color Palette › Border Progression) ↔ a11y SC 2.4.3, where an `inert` focus fixup moves focus and what shows it; witness `inert_subtree`.
- Motion: any animated popup or focus change (design-system §Motion → Reduced motion) ↔ a11y SC 2.3.3, the override being mandatory.
- Depth ↔ architecture §Standard Contracts → Scrolling, selection, tree, which the hit walk's dropped-node rule cites; fonts ↔ architecture §Standard Contracts → Headless stand, which holds the stand's font registration; both ↔ test-plan through the `for incremental in [false, true]` rule the chunk states.

## Acceptance criteria contributions
- If the transformed-scrolling-box fix is taken, a fixture with a CSS `transform` on a container holding absolutely positioned nodes asserts, in both layout modes, that a shown target's visible region agrees with where it is hit; a scale is asserted too if P3 finds it in the fix's reach (per design-system §Depth Strategy → Engine depth behavior).
- A served-page contrast row for `load-fetch` reads the colours the page resolves, and fails when the stand's recorded pair is changed to one below 4.5:1; no hex is copied into the check (per design-system §Color Palette › Core Colors).
- `packages/blitz-dom/assets/default.css`'s select and option rules show no diff unless the founder's word took the fork that moves them (per design-system §Surface: desktop-native › Tokens (platform-specific)).
- Every new check reading text bounds or painting text first asserts a non-zero text measure or an accepted `screenshot`, so it cannot pass vacuously (per design-system §Typography → Loading).
