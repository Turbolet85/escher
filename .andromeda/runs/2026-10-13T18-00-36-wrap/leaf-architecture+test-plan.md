group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:6   class: omission
  leaf-text: «composited over opaque white, a paint with no display.»
  source: .andromeda/test-plan.md:47 «and since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle»
  replace-with: «composited over opaque white, a paint with no display — and, since 2026-10-13-stand-contrast-harness, a contrast reader, `Harness::text_contrast`, reading a control's text colour against its composited background from computed styles and changing nothing.»

- leaf: .claude/docs/services/blitz-test-harness.md:11   class: omission
  leaf-text: «`anyrender` and `anyrender_vello_cpu` (no `multithreading`).»
  source: .andromeda/architecture.md:130 «for it the harness depends on `color` (`{ workspace = true }`, already in the graph through blitz-dom — `Cargo.lock` gained one line, `"color",` in the harness's entry, and no package)»
  replace-with: «`anyrender` and `anyrender_vello_cpu` (no `multithreading`) and, since 2026-10-13-stand-contrast-harness, for the contrast reader `color` (`{ workspace = true }`, already in the graph through blitz-dom; the harness still takes no `tracing`).»

- leaf: .claude/docs/services/blitz-test-harness.md:14   class: omission
  leaf-text: «`Raster` and `ScreenshotError` (`FontsUnavailable` · `EmptyRect`) from the private module `screenshot`.»
  source: .andromeda/architecture.md:130 «since 2026-10-13-stand-contrast-harness, `TextContrast` and `contrast_ratio` from its private module `contrast`, re-exported at the crate root (packages/blitz-test-harness/src/lib.rs:28)»
  replace-with: «`Raster` and `ScreenshotError` (`FontsUnavailable` · `EmptyRect`) from the private module `screenshot` and, since 2026-10-13-stand-contrast-harness, `TextContrast` and `contrast_ratio` from the private module `contrast`, both re-exported at the crate root.»

- leaf: .claude/docs/services/blitz-test-harness.md:15   class: omission
  leaf-text: «a document whose `can_paint_text` reads false is `FontsUnavailable` with nothing painted; no path prints, panics or logs.»
  source: .andromeda/architecture.md:130 «`Harness::text_contrast(&self, NodeId) -> Option<TextContrast>` reads and changes nothing — no `doc.mutate()`, input, pump or settle, every node reached by `get_node`»
  replace-with: «a document whose `can_paint_text` reads false is `FontsUnavailable` with nothing painted; no path prints, panics or logs. Since 2026-10-13-stand-contrast-harness `Harness::text_contrast(&self, NodeId) -> Option<TextContrast>` reads and changes nothing (no `doc.mutate()`, input, pump or settle; every node by `get_node`): the text-bearing element — the node itself when it is a text input, else the parent element of the first text node under it holding a character other than whitespace — gives its computed `color`, `font-size` and `font-weight`, over a background composited source-over from each element's `background-color` up through its ancestors to the first opaque one, else onto the canvas (the root element's background, else `body`'s) over opaque white; it reads `None` for no such text, for an element on the walk without primary styles, and for one up to the first opaque whose `opacity` is below 1, whose `filter` list is not empty or whose `background-image` holds a layer other than `none`; `TextContrast { text, background, ratio, font_size_px, font_weight }` has `is_large_text()` (at least 24 px, or 14 pt at weight 700 or more) and `required_ratio()` (3.0 for large text, else 4.5); `contrast_ratio(a, b) -> f32` is the WCAG 2.x ratio `(L1 + 0.05) / (L2 + 0.05)` over the `color` crate's relative luminance, alpha ignored; it opens no span and prints or logs nothing.»

- leaf: .claude/docs/services/blitz-test-harness.md:30   class: omission
  leaf-text: «`ScreenshotError`, since 2026-10-12-headless-screenshot)»
  source: .andromeda/architecture.md:255 «the private contrast (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the last two re-exported from the crate root; packages/blitz-test-harness/src/lib.rs:21;»
  replace-with: «`ScreenshotError`, since 2026-10-12-headless-screenshot) · `src/contrast.rs` (the contrast reader: `Harness::text_contrast`, `TextContrast`, `contrast_ratio`, since 2026-10-13-stand-contrast-harness)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «(17 in the crate since 2026-10-12-headless-screenshot:»
  source: .andromeda/test-plan.md:19 «17 since 2026-10-12-headless-screenshot, 6 more in `screenshot.rs` (packages/blitz-test-harness/src/screenshot.rs:140-244); 22 since 2026-10-13-stand-contrast-harness, 5 more in `contrast.rs`»
  replace-with: «(22 in the crate since 2026-10-13-stand-contrast-harness, 17 before it; the six:»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: omission
  leaf-text: «`cargo test -p blitz-tests --test stand_settle` — settle on the stand and on fixtures, both layout modes.»
  source: .andromeda/test-plan.md:19 «22 since 2026-10-13-stand-contrast-harness, 5 more in `contrast.rs` — the WCAG 2.x ratio table (5 rows), the large-text threshold table (5 rows), a button reading its own text and background»
  replace-with: «`cargo test -p blitz-tests --test stand_settle` — settle on the stand and on fixtures, both layout modes; `cargo test -p blitz-test-harness --lib contrast` — its 5 contrast unit tests (since 2026-10-13-stand-contrast-harness: the WCAG 2.x ratio table, the large-text threshold table, a button reading its own text and background, a translucent ancestor composited over the page, an element with no text reading `None`); `cargo test -p blitz-tests --test stand_contrast` — the 18 `controls(task)` rows of the four lean tasks at boot read through `text_contrast` against a stated record, and the painted background matching the measured one, both layout modes; it records and gates nothing.»

- leaf: .claude/docs/commands.md:28   class: omission
  leaf-text: «--test stand_id_edits --test stand_actionable_keys` — the headless-stand checks, package-alone»
  source: .andromeda/test-plan.md:26 «run by the workspace `test` leg and per package, not by the `a11y` leg, which stays three files; it records and gates nothing, SC 1.4.3's assertion owned by the route entry "Stand a11y assertions"»
  replace-with: «--test stand_id_edits --test stand_actionable_keys --test stand_contrast` — the headless-stand checks, package-alone»
