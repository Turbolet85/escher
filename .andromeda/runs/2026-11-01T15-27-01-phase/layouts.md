# layouts extract

## Relevance
partial: the chunk adds no surface, region, component or focus stop, but the take and its conflict resolutions touch the code behind the desktop-native seven_guis stand's mount, focus order, scroll geometry and select list, which must hold or be re-measured.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens requires the TaskShell mount and the pinned stand viewport to hold under every boot; `packages/dioxus-native-dom/src/dioxus_document.rs` is in the predicted conflict set, so whether a take disturbs the mount is research's question.
- layout-templates §Surface: desktop-native → IA notes requires the fixed document skeleton and the app's mount into `main`; the `dioxus_document.rs` resolution keeps both, under a take that reaches that file.
- layout-templates §Surface: desktop-native → Primary screens requires the stated Tab sequences of CRUD, the flight booker, the load page and Cells to hold; a take may not add, drop or reorder a stop. `examples/seven_guis/src/tasks/cells.rs` is in the predicted conflict set, and its grid is a single Tab stop by that section.
- layout-templates §Surface: desktop-native → Primary screens requires the select list's placement rules: painted in `paint_scene` after the root's render and before the devtools overlays, no row a layout box, no second OS window, focus staying on the select. The list's sixteen-file merge surface is re-read row by row against this.
- layout-templates §Surface: desktop-native → IA notes requires `scroll_into_view` and `visible_region` to keep their stated order and meaning, the open-list row exception included; `scrolling.rs` is in the predicted conflict set.
- layout-templates §Surface: desktop-native → IA notes records the stand's nested-scroller geometry and the `crud-list` figures; whether the Taffy, Stylo or computed-size commits (`ae774d7f4` among them) move those figures under a given take is research's question.
- layout-templates §Surface: desktop-native → IA notes requires the visual lint's stated readings on the stand, including the closed-select and open-row handling in `overlaps` and `clip_region`; whether upstream's culling and paint commits (`e42ee1f25`, `6fca6308d`, `ac6f9f582`) change them is research's question.

## Patterns to follow
- Identity and names stated by attributes alone, adding no element, class, style, wrapper or order, with no CSS selecting by id: any resolution in `examples/seven_guis/src/tasks/cells.rs` keeps this form (per layout-templates §Surface: desktop-native → Primary screens).
- A plan layout fact carries its code citation and its `as measured at` report: a layout number the take moves is re-measured and recorded in that form at the chunk's own report (per layout-templates §Surface: desktop-native → IA notes).
- Stand layout is read in both layout modes on the headless stand, and a windowed path read and not run is stated as such: the take's re-reads keep that distinction (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Moving stand markup, ids, classes or styles to absorb a layout change the take brings (per layout-templates §Surface: desktop-native → IA notes).
- Resolving a conflict so that a select row becomes a layout box or the list opens in a second OS window (per layout-templates §Surface: desktop-native → Primary screens).
- Letting the CRUD list's focus move from the list to a row, or a new Tab stop appear, as a side effect of upstream's event or restyle changes (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- Focus order ↔ a11y §Focus Order SC 2.4.3: the Tab sequences in layout-templates §Surface: desktop-native → Primary screens are re-proven by `stand_keyboard` and the a11y leg.
- Scroll and lint geometry ↔ tests: the figures in layout-templates §Surface: desktop-native → IA notes are decided by `stand_act_scroll` and `stand_lint`, in both layout modes.
- Select list bounds ↔ tests and the snapshot: the row-bounds rule in layout-templates §Surface: desktop-native → Primary screens is decided by `stand_act_popup`, `stand_keyboard` and `select_popup_painted`, the last also under the `blitz-paint/font-embolden` gate this chunk folds.

## Acceptance criteria contributions
- (layouts) After the take, `stand_keyboard` passes in both layout modes with the CRUD, flight booker, load page and Cells Tab sequences unchanged, select list open and after each close included (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) After the take, `git diff` over `examples/seven_guis/src/` from the pre-take HEAD shows no element, class, style, wrapper or order added, moved or removed, except what a named, recorded conflict resolution requires (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) After the take, `stand_act_popup` and `select_popup_painted` pass, the list painted after the root's render and before the devtools overlays, and each open row reading its own `bounds` (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) After the take, `stand_lint` and `stand_act_scroll` pass in both layout modes with every asserted figure unloosened; any moved figure is re-measured, explained and recorded in the chunk's report (per layout-templates §Surface: desktop-native → IA notes).
