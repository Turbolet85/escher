### visual-design-verification#7.4 · values · visual-preferences · ## 6. Visual Design Verification · lines 65–65
- Page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:654-662)
### visual-design-verification#7.5 · values · visual-preferences · ## 6. Visual Design Verification · lines 66–66
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
### visual-design-verification#7.6 · values · visual-preferences · ## 6. Visual Design Verification · lines 67–67
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
### visual-design-verification#7.7 · values · visual-preferences · ## 6. Visual Design Verification · lines 68–68
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1087-1098)
### visual-design-verification#7.8 · values · visual-preferences · ## 6. Visual Design Verification · lines 69–69
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:158-204; packages/blitz-dom/src/events/pointer.rs:247-260)
### visual-design-verification#8.1 · values · visual-visibility · ## 6. Visual Design Verification · lines 75–75
- Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot — a select's option excepted from `display: none` (§4 Hidden content), so a select's options are its children in the tree and the snapshot; a hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit (the `inert` read ratified by the founder (2026-10-18)) (packages/blitz-dom/src/accessibility.rs:19-28; packages/blitz-dom/src/accessibility.rs:254-274; packages/blitz-dom/src/node/node.rs:1437-1446; tests/blitz-tests/tests/inert_subtree.rs:95-194); a hit also skips a node dropped since the last resolve (architecture §Standard Contracts → Scrolling, selection, tree)
### visual-design-verification#8.2 · values · visual-visibility · ## 6. Visual Design Verification · lines 76–76
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1448-1456)
### visual-design-verification#8.3 · values · visual-visibility · ## 6. Visual Design Verification · lines 77–77
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1458-1462; packages/blitz-dom/src/node/node.rs:1662-1670)
### visual-design-verification#8.4 · values · visual-visibility · ## 6. Visual Design Verification · lines 78–78
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87); a test asserts it paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)
### visual-design-verification#8.5 · values · visual-visibility · ## 6. Visual Design Verification · lines 79–79
- Devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)
