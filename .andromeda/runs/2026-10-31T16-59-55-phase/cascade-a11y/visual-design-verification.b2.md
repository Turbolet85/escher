### visual-design-verification#6.3 · values · visual-state-color · ## 6. Visual Design Verification · lines 60–60
- Each WPT runner result line prints its status word as text alongside its color (wpt/runner/src/main.rs:372-401)
### visual-design-verification#6.4 · values · visual-state-color · ## 6. Visual Design Verification · lines 61–61
- A comment in the form example notes the input color "Should be accent-color" (examples/form.rs:126-129)
### visual-design-verification#7.1 · values · visual-preferences · ## 6. Visual Design Verification · lines 67–67
- The viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed) (packages/blitz-traits/src/shell.rs:68-82; packages/blitz-traits/src/shell.rs:134-150)
### visual-design-verification#7.2 · values · visual-preferences · ## 6. Visual Design Verification · lines 68–68
- Color-scheme changes on the viewport are tracked as a device change that rebuilds the stylist device (packages/blitz-dom/src/stylo_device.rs:37-38; packages/blitz-dom/src/stylo_device.rs:55-57)
### visual-design-verification#7.3 · values · visual-preferences · ## 6. Visual Design Verification · lines 69–69
- Total scale changes from hidpi or zoom are tracked and invalidate text shaping (packages/blitz-dom/src/stylo_device.rs:33-36; packages/blitz-dom/src/stylo_device.rs:52-54)
### visual-design-verification#7.4 · values · visual-preferences · ## 6. Visual Design Verification · lines 70–70
- Page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:655-663)
### visual-design-verification#7.5 · values · visual-preferences · ## 6. Visual Design Verification · lines 71–71
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
### visual-design-verification#7.6 · values · visual-preferences · ## 6. Visual Design Verification · lines 72–72
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
### visual-design-verification#7.7 · values · visual-preferences · ## 6. Visual Design Verification · lines 73–73
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1115-1126)
### visual-design-verification#7.8 · values · visual-preferences · ## 6. Visual Design Verification · lines 74–74
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:161-207; packages/blitz-dom/src/events/pointer.rs:250-263)
### visual-design-verification#8.1 · values · visual-visibility · ## 6. Visual Design Verification · lines 80–80
- Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot — a select's option excepted from `display: none` (§4 Hidden content), so a select's options are under it in the tree and the snapshot, directly or through an `optgroup`, the exception holding under a group (packages/blitz-dom/src/node/select.rs:131-134); a hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit (the `inert` read ratified by the founder (2026-10-18)) (packages/blitz-dom/src/accessibility.rs:19-28; packages/blitz-dom/src/accessibility.rs:277-297; packages/blitz-dom/src/node/node.rs:1437-1446; tests/blitz-tests/tests/inert_subtree.rs:104-363); a hit also skips a node dropped since the last resolve (architecture §Standard Contracts → Scrolling, selection, tree)
### visual-design-verification#8.2 · values · visual-visibility · ## 6. Visual Design Verification · lines 81–81
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1448-1456)
### visual-design-verification#8.3 · values · visual-visibility · ## 6. Visual Design Verification · lines 82–82
- A transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit, an invertible twin hit where it shows, asserted by `hit_non_invertible_transform` in both layout modes (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### visual-design-verification#8.4 · values · visual-visibility · ## 6. Visual Design Verification · lines 83–83
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1458-1462; packages/blitz-dom/src/node/node.rs:1668-1676)
### visual-design-verification#8.5 · values · visual-visibility · ## 6. Visual Design Verification · lines 84–84
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87); a test asserts it paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)
### visual-design-verification#8.6 · values · visual-visibility · ## 6. Visual Design Verification · lines 85–85
- Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`hit_with_scrollbar` while `scrollbar_opacity > 0`, `packages/blitz-dom/src/events/pointer.rs`); on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer resting on the thumb keeps it shown, so a driver `click` on a cell in that band needs an `advance` past the fade first (tests/blitz-tests/tests/stand_cells_served.rs:216-232 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)
### visual-design-verification#8.7 · values · visual-visibility · ## 6. Visual Design Verification · lines 86–86
- Devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)
