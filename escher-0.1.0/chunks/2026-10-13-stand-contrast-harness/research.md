# Codebase Research — 2026-10-13-stand-contrast-harness

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the `timeout` exit-124 gate trap; this chunk names no bounded boot smoke, so it applies to nothing planned) · `.claude/rules/testing.md` — auto-loaded, 7 Session Additions read (the `run stand` agent-run contract; the both-builds rule; the table-pin sweep of 2026-10-10)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup 5a read one green run)
- **External inputs:** `inputs#I1` — the operator relaying the founder: no mutation runs (the mutation tier stays unstarted); pipeline tool output redirected to a file and read whole

## Files inspected
- `tests/blitz-tests/tests/common/mod.rs` (1-100) — `controls(task)` (16-45): 18 rows over the four lean tasks, `back-btn` first in each; `INPUT_NAMES` (48-55); `rendered(task)` (60-97); `boot(task, incremental)` (99-101). The CRUD rows (`crud-person-{id}`) are in neither `controls` nor `rendered`.
- `examples/seven_guis/src/stand.rs` (15-95, by grep) — `LeanTask::ALL` (24), the pins `VIEWPORT_WIDTH` 800 · `VIEWPORT_HEIGHT` 600 · `HIDPI_SCALE` 1.0 · `COLOR_SCHEME` Light (43-49), `font_ctx()` (52), `options(incremental)` (58), `boot` (73).
- `examples/seven_guis/src/app.rs` (183-340) — `SHELL_CSS`: `#back-btn` 13px, `color: #333`, `background: #f0f0f0` (308-317), `:hover` `#e4e4e4` (319-321); `html, body, #main` state no background in the shell sheet (`#task-shell` paints `#f5f5f5`).
- `examples/seven_guis/src/tasks/counter.rs` (8-14, 23-73) — `.counter-btn` 16px · 500 · `#ffffff` on `#4a6cf7` (55-64), hover `#3a5ce5`, active `#2a4cd3`.
- `examples/seven_guis/src/tasks/flight_booker.rs` (45-114, 121-241) — boot state: `flight_type` "one-way" (45), both dates "01.01.2026" (46-47); `flight-one-way` carries `type-btn type-btn-active` at boot (68-70) → 14px · 600 · `#ffffff` on `#4a6cf7` (161-175); `flight-return` `.type-btn` 14px · `#444` on `#fafafa`; `.date-input` 15px · `#1a1a1a` on `#ffffff` (177-184), `:disabled` `#999999` on `#ececec` (191-195), `.invalid` `#c53030` on `#fff5f5` (197-201); `flight-return-date` is `disabled: !is_return` → disabled at boot (95); `.flight-btn` 15px · 500 · `#ffffff` on `#4a6cf7` (203-217), `disabled: !dates_ok` (102), `:disabled` `#a0aec0` (227-230).
- `examples/seven_guis/src/tasks/timer.rs` (69-97, 106-196) — `timer-duration` is the range input (80-86, role Slider in `controls`); `.timer-reset-btn` 16px · 500 · `#ffffff` on `#4a6cf7` (177-187).
- `examples/seven_guis/src/tasks/crud.rs` (25-147, 148-256) — `selected` is `None` at boot (32), so `crud-update` / `crud-delete` carry `disabled` and `btn-off` at boot (90-107 of the rsx excerpt, `disabled: !has_selection`); `.crud-buttons button` 14px · 500 · `#ffffff` on `#4a6cf7` (230-239), `.btn-off` `#b0b8d8` (245-248); the three inputs state no `color` or `background` in the task sheet (217-222, 250-255), so both come from inheritance and the UA sheet; `.list-item.selected` `#ffffff` on `#4a6cf7` (199-202).
- `packages/blitz-dom/assets/default.css` (76-109) — `input, textarea { background-color: white }` (76-81); `input, select, button { font-family: system-ui, sans-serif }` (83-87); `button { color: black; background-color: #EFEFEF }` (98-109). No UA `color` on `input`, so an input's text colour is inherited.
- `packages/blitz-paint/src/render.rs` (185-229) — the canvas: the root element's `background-color`, else `body`'s when the root's is transparent, filled over the viewport; nothing when both are transparent, and a raster then shows the renderer's opaque white.
- `packages/blitz-paint/src/render/background.rs` (334-336) — a box's background colour is its computed `background-color` resolved against its computed `color`.
- `packages/blitz-paint/src/text.rs` (150-167, 565-640) — `stroke_text` strokes each glyph run in the colour of the run's own brush node, the top of its ancestor stack (616-638), read by `resolve_decoration_entry` as that node's inherited-text `color` (156-167; black when the node has no styles); (25-48) is the inline-background path, not glyph colour; `packages/blitz-dom/src/layout/construct.rs` (875-910) — a text input's editor takes the input's own text style, its font size and weight included, and `StyleProperty::Brush(parley_style.brush)` (910), so an input's value paints in the input's computed `color` at its computed `font-size`.
- `packages/blitz-paint/src/color.rs` (1-66) — `contrast_ratio(a, b)` is `pub(crate)` (29), not feature-gated, the WCAG ratio over `OpaqueColor::relative_luminance` of the `color` crate (`split().0.relative_luminance() + 0.05`, larger over smaller); `blend_for_contrast` is `#[cfg(feature = "scrollbars")]` (43).
- `packages/blitz-paint/Cargo.toml` (`[features]`) — `default = ["svg"]`; `scrollbars` opt-in; blitz-test-harness takes blitz-paint with `default-features = false`.
- `packages/blitz-dom/src/util.rs` (166-179) — `pub trait ToColorColor { fn as_color_color(&self) -> Color }` for `AbsoluteColor` (sRGB conversion); `blitz_dom::util` is `pub mod` (lib.rs:81).
- `packages/blitz-dom/src/node/node.rs` (1107) — `Node::primary_styles()`; the integration pattern `doc.get_node(id).and_then(|n| n.primary_styles())` (`tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:125-129`).
- `packages/blitz-test-harness/Cargo.toml` and `src/{lib,inspect,screenshot}.rs` (by grep) — modules harness · input · inspect · settle · screenshot; deps blitz-dom (accessibility, system-fonts), blitz-html, blitz-traits, blitz-paint (no default features), dioxus-native-dom, dioxus-core, anyrender, anyrender_vello_cpu, keyboard-types, smol_str — no `color` and no `tracing`; inspect offers `query`, `node`, `layout_rect_of`, `focused`, `hovered`; screenshot offers `screenshot` / `screenshot_rect(PixelRect)` → `Raster`.
- `packages/dioxus-native-dom/src/snapshot.rs` (34-52, 96, 154) — `SnapshotNode.state.enabled: Option<bool>`, `Some(false)` on the presence of `disabled` whatever its value.
- `Cargo.toml` (132) — `color = "0.3"` in `[workspace.dependencies]`; blitz-dom already takes it (`packages/blitz-dom/Cargo.toml:75`), so `color` is in the lock.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T17-00-57-phase/tree-query-2026-10-13-stand-contrast-harness.json`)
- **contrast_ratio** — 4 call sites, all inside blitz-paint: `blend_for_contrast` (`packages/blitz-paint/src/color.rs:49`, `:59`), the `border` module's import (`packages/blitz-paint/src/render/border.rs:12`) and `beveled_edge_color` (`packages/blitz-paint/src/render/border.rs:74`) — no caller outside the crate; widening it would be a public-API change with no consumer but this chunk.
- **controls** (the `common` table) — read by 6 stand checks (`stand_accessibility_ids`, `stand_act_ids`, `stand_act_obstructed`, `stand_session_fresh`, `stand_session_ids`, `stand_snapshot`); this chunk reads it and does not change it, so none of them moves.
- **primary_styles** — 16 reads across 11 blitz-tests targets, the established computed-style read in integration tests.
- **crate edges of blitz-test-harness** — out: blitz-dom, blitz-html, blitz-paint, blitz-traits, dioxus-native-dom; in: blitz-tests, escher-driver, seven_guis. An additive module has no cross-crate blast radius; a new public item is visible to the three inbound crates.

## Patterns detected
- **A harness surface added as a private module, re-exported at the root** (`packages/blitz-test-harness/src/lib.rs`, the `screenshot` module of 2026-10-12-headless-screenshot): the types re-exported, `///` on every public item, unit tests inline.
- **Paint's colour sources are computed styles** (`packages/blitz-paint/src/text.rs:156-167, 616-638`, `packages/blitz-paint/src/render/background.rs:334-336`, `packages/blitz-paint/src/render.rs:193-219`): text from its brush node's `color`, a box from its `background-color`, the canvas from the root's else body's background — so a computed-style reading of a control whose ancestors paint opaque backgrounds equals what is painted, provided no ancestor between the control and the first opaque background paints a background image, gradient, opacity or filter. The stand's four lean sheets and the shell sheet state none of those (grep `gradient|background-image|opacity|filter|mix-blend` over the five files: 0 style hits; `rgba` appears only in three `box-shadow`s).
- **A stated table with its row count asserted, in both layout modes** (`tests/blitz-tests/tests/accessibility_states.rs:128-220`).

## Conventions to follow
- **Stand checks boot through the stand** — `common::boot(task, incremental)` (`tests/blitz-tests/tests/common/mod.rs:99-101`), `for incremental in [false, true]`, no font skip, no print, no subscriber, no env read.
- **A failure message names a layout mode, a task and an index, never an id or anything a screen reads** (test-plan, 2026-10-07-act-by-id); the security rule's no id, name or value in an assertion message.
- **No `NodeId` indexing** — resolve through `get_node` / `query`, as `hit_clipped_at_scrolling_box.rs:125-129` does.

## New files to create
- `packages/blitz-test-harness/src/contrast.rs` — the contrast reader: a WCAG ratio, a control's text/background pair read from computed styles with the canvas rule, the SC 1.4.3 threshold of its text, and its unit tests
- `tests/blitz-tests/tests/stand_contrast.rs` — the stand check: every row of `controls(task)` over `LeanTask::ALL`, measured in both layout modes, compared with the stated per-pair table, and the painted-background witness
- `escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/evidence/` — the per-pair record as measured, and the gate captures

## Files to modify
- `packages/blitz-test-harness/src/lib.rs` — declare the `contrast` module and re-export its public items
- `packages/blitz-test-harness/Cargo.toml` — take `color = { workspace = true }`, the colour type the reader's public items name
- `Cargo.lock` — blitz-test-harness's dependency list gains `color`; no package enters

## Sweeps
- `contrast` (case-insensitive) over `packages/blitz-test-harness`, `tests/blitz-tests/tests/common`, `tests/blitz-tests/tests/session_common`, `examples/seven_guis/src`: 0 hits · 0 changed · 0 no-change (the name is free there)
- `controls` callers by the graph (`refs`, `callee_name = 'controls'`, `file LIKE '%tests/blitz-tests%'`): 15 sites in 6 files · 0 changed · 15 no-change (the table is read, never edited)
- `gradient|background-image|opacity|filter|mix-blend|rgba|hsla|transparent|placeholder` over the four lean task files and `app.rs`: 3 `rgba` hits (box-shadows) and crud's `filter` signal names · 0 changed · all no-change (no translucent or image background on any control's ancestor chain)

## Open questions
- Which colour-changing states beyond the boot state are measured (hover, active, a pressed toggle's alternate, an invalid date, a selected CRUD row)? → blocks: plan-decision
- Whether the CRUD rows enter the measured set (they are keyed and clickable, role `option`, absent from `controls`)? → blocks: plan-decision
