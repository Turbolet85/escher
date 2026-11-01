### visual-design-verification#4.1 · values · visual-motion · ## 6. Visual Design Verification · lines 50–50
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)
### visual-design-verification#5.1 · absence · visual-motion · ## 6. Visual Design Verification · lines 56–56
- Observed absent — prefers-reduced-motion and prefers-color-scheme rules · searched: `prefers-reduced-motion|prefers-color-scheme` over the 21 s02 files
### visual-design-verification#5.2 · absence · visual-motion · ## 6. Visual Design Verification · lines 57–57
- Observed absent — motion on an open select's list, which needs no `advance`: a screen read after a step reads the new state, pinned by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance` (tests/blitz-tests/tests/stand_act_popup.rs:408-423 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### visual-design-verification#6.1 · values · visual-state-color · ## 6. Visual Design Verification · lines 63–63
- Disabled icon buttons render at 0.35 opacity (apps/browser/assets/browser.css:191-198)
### visual-design-verification#6.2 · values · visual-state-color · ## 6. Visual Design Verification · lines 64–64
- Author styles keyed on `:open` apply to an open `details` or `dialog` in every document, `:open` reading the element's `OPEN` state (§5 Focus restoration); `open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter, and read `:open` matching (packages/blitz-dom/src/stylo.rs:510; tests/blitz-tests/tests/open_state.rs:54-143) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### visual-design-verification#6.3 · values · visual-state-color · ## 6. Visual Design Verification · lines 65–65
- An open select's list is no `:open` state: neither `:open` nor `flush_open_state` reads the list, an author's `open` on a select steers nothing and `:open` does not match it, so no author style keys on a list standing (tests/blitz-tests/tests/select_popup.rs:628-651 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### visual-design-verification#6.4 · values · visual-state-color · ## 6. Visual Design Verification · lines 66–66
- Each WPT runner result line prints its status word as text alongside its color (wpt/runner/src/main.rs:372-401)
### visual-design-verification#6.5 · values · visual-state-color · ## 6. Visual Design Verification · lines 67–67
- A group's header in an open list is heavier than a row by a synthesized bold, on plain and emboldening builds, pinned by `a_groups_header_is_darker_than_a_row_by_weight` (tests/blitz-tests/tests/select_popup_painted.rs:282-297 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### visual-design-verification#6.6 · values · visual-state-color · ## 6. Visual Design Verification · lines 68–68
- A comment in the form example notes the input color "Should be accent-color" (examples/form.rs:126-129)
### visual-design-verification#7.1 · values · visual-preferences · ## 6. Visual Design Verification · lines 74–74
- The viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed) (packages/blitz-traits/src/shell.rs:68-82; packages/blitz-traits/src/shell.rs:134-150)
### visual-design-verification#7.2 · values · visual-preferences · ## 6. Visual Design Verification · lines 75–75
- Color-scheme changes on the viewport are tracked as a device change that rebuilds the stylist device (packages/blitz-dom/src/stylo_device.rs:37-38; packages/blitz-dom/src/stylo_device.rs:55-57)
### visual-design-verification#7.3 · values · visual-preferences · ## 6. Visual Design Verification · lines 76–76
- Total scale changes from hidpi or zoom are tracked and invalidate text shaping (packages/blitz-dom/src/stylo_device.rs:33-36; packages/blitz-dom/src/stylo_device.rs:52-54)
### visual-design-verification#7.4 · values · visual-preferences · ## 6. Visual Design Verification · lines 77–77
- Page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:655-663)
### visual-design-verification#7.5 · values · visual-preferences · ## 6. Visual Design Verification · lines 78–78
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
### visual-design-verification#7.6 · values · visual-preferences · ## 6. Visual Design Verification · lines 79–79
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
### visual-design-verification#7.7 · values · visual-preferences · ## 6. Visual Design Verification · lines 80–80
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1126-1137)
### visual-design-verification#7.8 · values · visual-preferences · ## 6. Visual Design Verification · lines 81–81
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:161-207; packages/blitz-dom/src/events/pointer.rs:254-267)
### visual-design-verification#8.1 · values · visual-visibility · ## 6. Visual Design Verification · lines 87–87
- Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot — a select's option excepted from `display: none` (§4 Hidden content), so a select's options are under it in the tree and the snapshot, directly or through an `optgroup`, the exception holding under a group (packages/blitz-dom/src/node/select.rs:131-134); a hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit (the `inert` read ratified by the founder (2026-10-18)) (packages/blitz-dom/src/accessibility.rs:19-28; packages/blitz-dom/src/accessibility.rs:281-301; packages/blitz-dom/src/node/node.rs:1437-1446; tests/blitz-tests/tests/inert_subtree.rs:104-363); a hit also skips a node dropped since the last resolve (architecture §Standard Contracts → Scrolling, selection, tree)
### visual-design-verification#8.2 · values · visual-visibility · ## 6. Visual Design Verification · lines 88–88
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1448-1456)
### visual-design-verification#8.3 · values · visual-visibility · ## 6. Visual Design Verification · lines 89–89
- A transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit, an invertible twin hit where it shows, asserted by `hit_non_invertible_transform` in both layout modes (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### visual-design-verification#8.4 · values · visual-visibility · ## 6. Visual Design Verification · lines 90–90
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1458-1462; packages/blitz-dom/src/node/node.rs:1668-1676)
### visual-design-verification#8.5 · values · visual-visibility · ## 6. Visual Design Verification · lines 91–91
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87); a test asserts it paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)
### visual-design-verification#8.6 · values · visual-visibility · ## 6. Visual Design Verification · lines 92–92
- Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`hit_with_scrollbar` while `scrollbar_opacity > 0`, `packages/blitz-dom/src/events/pointer.rs`); on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer resting on the thumb keeps it shown, so a driver `click` on a cell in that band needs an `advance` past the fade first (tests/blitz-tests/tests/stand_cells_served.rs:216-232 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)
### visual-design-verification#8.7 · values · visual-visibility · ## 6. Visual Design Verification · lines 93–93
- The open list paints whole past an `opacity` ancestor and a box that clips its select, a point the list does not cover reads what paint shows, and a driver `click` on an element under the list is refused `covered` (tests/blitz-tests/tests/select_popup_painted.rs:211-235; tests/blitz-tests/tests/select_popup.rs:677-716; tests/blitz-tests/tests/stand_act_popup.rs:228-245 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### visual-design-verification#8.8 · values · visual-visibility · ## 6. Visual Design Verification · lines 94–94
- Devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)
