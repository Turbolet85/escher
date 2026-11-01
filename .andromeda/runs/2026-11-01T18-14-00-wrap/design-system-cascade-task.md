# Task — design-system · 2026-11-01-upstream-sync-ui-graph

## The session
Took twelve upstream DioxusLabs/blitz commits by no-commit cherry-pick, each an engine behaviour change for every document unless said: restyle narrowing, table damage, `<picture>`, failed stylesheets, blend and isolation painting, Stylo prefs before the UA sheet, `URLSearchParams`, `CustomWidget` attribute replay, inline decorations, mounted events, sub-pixel computed sizes, culling bounds.
Fixed escher's own key-press panic: `BaseDocument::deallocate_anonymous_block` clears `layout_parent` on the children of a dropped anonymous block; a focused control that no longer shows keeps focus and takes keys.
Made `Harness::text_contrast` read `None` past a `mix-blend-mode` other than `normal`, and made `CountedHandler` forward `NetHandler::error`, so a failed load now ends the wait.
Added the check `key_press_after_container_lost_its_box` and the `embolden` step at the end of the `fast` leg; removed `matrix_test`'s free-disk step and `select_popup`'s row-3 skip.
Measured and re-read: the tallies moved, `SelectListView`'s constants and the sheet's select rules stand as recorded, `select_popup_painted` passes on both builds, and the select list paints last, past the blend layers.

## Reached files
- packages/blitz-dom/assets/default.css · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/layout/damage.rs · merge surface
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/mutator.rs · merge surface
- packages/blitz-dom/src/net.rs · merge surface
- packages/blitz-dom/src/node/custom_widget.rs
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/node/node.rs · merge surface
- packages/blitz-dom/src/resolved_style.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-net/src/lib.rs · merge surface
- packages/blitz-paint/src/blend.rs · merge surface
- packages/blitz-paint/src/layers.rs · merge surface
- packages/blitz-paint/src/lib.rs · merge surface
- packages/blitz-paint/src/render.rs · merge surface
- packages/blitz-paint/src/render/background.rs
- packages/blitz-paint/src/render/box_shadow.rs
- packages/blitz-test-harness/src/contrast.rs
- packages/blitz-test-harness/src/settle.rs
- packages/blitz-traits/src/net.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js · merge surface
- packages/blitz-vibey-script/tests/url_search_params.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
- tests/blitz-tests/tests/render_blocking_stylesheet.rs
- tests/blitz-tests/tests/select_popup.rs
- wpt/runner/timeout-quarantine.txt · merge surface
- .github/workflows/ci.yml
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- Depth strategy's stacking roots (`depth-strategy.md:20`, the group at lines 19–20): `mix-blend-mode` other than `normal` joins them through `Node`'s stacking-context arm, and that line is changed.
- Depth strategy's atomic paint effects rule: `mix-blend-mode` is measured as a stacking root only; whether it also joins the atomic paint effects is this cascade's read.
- Any depth or paint-order statement that omits blend: `render_element` now pushes a viewport-wide blend layer and pops it last, and `isolation` now paints.
- Any depth absence, or any row stating that blending or `isolation` is unpainted or unsupported: both now paint, and `Node`'s `TODO: mix-blend-mode` is retired.
- Any row on `LayerManager` or paint layers: `maybe_with_layer` and `maybe_push_layer` now take a `BlendMode`.
- Depth's statements on hit-testing, hoisted children and paint children: a blended element is now a stacking root, so these rules read over more nodes, though the report names no edit to hit-testing.
- Depth's box-shadow rows, Border Radius's engine handling and Color Palette's engine-painted background and border rows: `render_element` skips the own shadow, background and border of an inline box holding only block children.
- Any engine row on border boxes, culling or paint bounds under scale: `render_element`'s border box and culling now stand on the scaled size, with `location` and `box_position` kept for the transform.
- The open select list's paint position in Depth Strategy: re-read, it still paints last, now past the blend layers, and `paint_select_list`'s call holds no hunk.
- Color Palette's `SelectListView` constants (`HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT`, `BORDER`): re-read, unmoved.
- The select rules of `default.css` as any category records them: re-read, every select rule stays.
- Any row stating `default.css`'s contents or line citations: upstream's `picture` rule leaves the sheet, so `<picture>` is no longer `display: contents`, and the file only lost lines, so citations past that rule shift.
- Typography's `font-embolden` rows (`typography.md:32`, `:53`, `:54`): the report reads the feature itself as no-change; any wording that no local gate builds it is retired, since `fast` now ends on `select_popup_painted` under `blitz-paint/font-embolden`.
- Any select-list row citing `select_popup`: the row-3 skip is gone, `select_popup.rs:513` is one `harness.press` for every row, and later citation digits in that file may move.
- Motion's driver load-wait row: a failed load now ends the wait where it would otherwise have run to `LOAD_WAIT`, through `CountedHandler`'s forward; `settle_waiting` is unedited and the row's clock claim stays unmoved.
- Motion's harness-settle rows that cite `settle.rs`: its inline code gained lines, so citation digits may move; the settle rule itself is unedited.
- Surface's `desktop-native-custom-widget` group: `ElementData::set_custom_widget` now replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget`; the report reads rows that only name the feature as no-change, so flag only a row stating when that call fires.
- Any spacing or size row reading computed `width` or `height`: they now read `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounded.
- Any row on restyling after a class, attribute or child-list change: `set_attribute`, `clear_attribute`, removal and insertion no longer take the blanket `RestyleHint::restyle_subtree()`.
- Any row on what a table's box repaints or rebuilds: `compute_layout_damage` takes `ALL_DAMAGE` when a table box's position, padding or border changes.
- Any Color Palette or visual-verification row stating when `text_contrast` reads `None`: the list gains an element on the walk with a `mix-blend-mode` other than `normal`; no stand row moves and the report names no row of this doc.
- Line citations into `packages/blitz-paint/src/render.rs`, `layers.rs`, `render/background.rs`, `render/box_shadow.rs` and `lib.rs`: lines were added, so later digits may move.
- Line citations into `packages/blitz-dom/src/document.rs`, `mutator.rs`, `stylo.rs`, `resolved_style.rs`, `node/node.rs`, `node/element.rs`, `node/custom_widget.rs`, `layout/construct.rs` and `layout/damage.rs`: lines were added, so later digits may move.
- Line citations into `packages/blitz-dom/src/lib.rs` past lines 96–97: a two-line comment is gone, while `pub use clock` stays.
- Line citations into `.github/workflows/ci.yml`: `matrix_test`'s free-disk step at base lines 392–398 is removed, so citations past it move and those before it do not.
- Line citations into `packages/dioxus-native-dom/src/dioxus_document.rs`: five lines were added at 235–239, so later digits may move.

## New facts
- F1 · depth-strategy · An element with `mix-blend-mode` other than `normal` is a stacking root, read in `Node`'s stacking-context arm.
- F2 · depth-strategy · `mix-blend-mode` and `isolation` now paint, through `blitz-paint`'s private `blend` module and its `pub(crate)` `convert_blend_mode`, with `render_element` pushing a viewport-wide blend layer and popping it last.
- F3 · depth-strategy · No fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; blending's witnesses are `layers.rs`'s 3 unit tests and `contrast.rs`'s fixture.
- F4 · depth-strategy · An inline box with only block children, read through `Node::is_inline_with_only_block_children`, paints no shadow, background or border of its own (`paints_own_box`), with no frame test behind it.

## Untouched
- Iconography's browser toolbar icons, tab glyphs, icon sizes and bundle icons: these live under `apps/browser`, which the session did not edit.
- Surface's `cli-output` rows on `escher-session` and its `mcp` role: `packages/escher-driver` is untouched by the take, and the JSON lines and exit classes stand.
- Typography's driver refusal `fonts-unavailable`: the refusal lives in `packages/escher-driver`, which was not edited.
- Surface's `desktop-native-stack` and `mobile-native-stack` version facts: no manifest, lock or `deny.toml` moved, and Stylo 0.22.0, taffy 0.15, Dioxus 0.7.3 and winit `=0.31.0-beta.3` stand.
- Any absence of `background-clip: text`: `ac6f9f582` was tried and left out because `BackgroundClip::Text` is `gecko`-only on Stylo 0.22.
- Example fixture rows drawn from `examples/seven_guis`: that directory is untouched by the take.
- Upstream sync readings (`e42ee1f25`, thirty-six of 57 taken, twenty-one left out): their sites are in other documents, and the report's pattern rows name none in design-system.
- The NO RECORDED INTENT and NOT YET MEASURED sections for brand identity, chosen depth approach, radius personality, expression level, anti-patterns, self-validation and project-wide scales: no founder intent or scale was recorded this session.
- Motion's reduced-motion absence: the report adds no reduced-motion handling.
