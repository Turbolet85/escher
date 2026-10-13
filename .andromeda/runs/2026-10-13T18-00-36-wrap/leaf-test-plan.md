group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:13   class: omission
  leaf-text: «a document with no font refused `FontsUnavailable`. See»
  source: .andromeda/test-plan.md:47 «and since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle;»
  replace-with: «a document with no font refused `FontsUnavailable`; and since 2026-10-13-stand-contrast-harness a contrast reader — `text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle): the text-bearing element's computed `color`, font size and weight over the background composited source-over from its ancestors' `background-color`s onto the root-else-`body` canvas over opaque white, `None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image, and `contrast_ratio` the WCAG 2.x ratio. See»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and `password_painted_masked` (an engine check with no `stand_` prefix: two password values of one length paint the same frame, a password input its bullets and a focused one its caret)»
  source: .andromeda/test-plan.md:26 «since 2026-10-13-stand-contrast-harness `stand_contrast` 2 — every stand control reads its recorded text/background pair (18 rows, one per `controls(task)` row over the four lean tasks at boot,»
  replace-with: «and `password_painted_masked` (an engine check with no `stand_` prefix: two password values of one length paint the same frame, a password input its bullets and a focused one its caret), since 2026-10-13-stand-contrast-harness `stand_contrast` (2: every stand control reads its recorded text/background pair — 18 rows, one per `controls(task)` row over the four lean tasks at boot, read through `Harness::text_contrast`; pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where the snapshot reads `enabled == Some(false)`, the record identical in both layout modes — and the painted background is the measured one within 1 per channel on all 17 measured rows; run by the workspace `test` leg and per package, not by the `a11y` leg, it records and gates nothing, SC 1.4.3's assertion owned by the route entry "Stand a11y assertions")»

- leaf: .claude/docs/tests-summary.md:23   class: stale
  leaf-text: «17 inline unit tests, 6 of them in `screenshot.rs` since 2026-10-12-headless-screenshot»
  source: .andromeda/test-plan.md:19 «22 since 2026-10-13-stand-contrast-harness, 5 more in `contrast.rs` — the WCAG 2.x ratio table (5 rows), the large-text threshold table (5 rows)»
  replace-with: «22 inline unit tests, 5 of them in `contrast.rs` since 2026-10-13-stand-contrast-harness (the WCAG 2.x ratio table of 5 rows, the large-text threshold table of 5 rows, a button reading its own text and background, a translucent ancestor composited over the page, an element with no text reading `None`), 6 in `screenshot.rs` since 2026-10-12-headless-screenshot»

- leaf: .claude/docs/tests-summary.md:30   class: omission
  leaf-text: «`cargo test --workspace` 893 · 0 · 12 over 179 result lines at 2026-10-13-upstream-sync-polish-and-ship (»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-13-stand-contrast-harness: 180 result lines, 900 passed · 0 failed · 12 ignored (+7 passed: one new result line, blitz-tests' `stand_contrast` 2, and blitz-test-harness +5»
  replace-with: «`cargo test --workspace` 900 · 0 · 12 over 180 result lines at 2026-10-13-stand-contrast-harness (+7 passed: one new result line, blitz-tests' `stand_contrast` 2, and blitz-test-harness +5, now 22, `contrast.rs`; `run stand` 141 passed · 7 ignored over 37 files, `run all` not re-read; timings not re-measured); 893 · 0 · 12 over 179 result lines at 2026-10-13-upstream-sync-polish-and-ship (»

- leaf: .claude/rules/verification-harness.md:38   class: omission
  leaf-text: «and refuses a document whose `can_paint_text` reads false with nothing painted.»
  source: .andromeda/test-plan.md:47 «and since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle;»
  replace-with: «and refuses a document whose `can_paint_text` reads false with nothing painted. Since 2026-10-13-stand-contrast-harness `text_contrast(NodeId)` returns `Option<TextContrast>` (`text`, `background`, `ratio`, `font_size_px`, `font_weight`; `required_ratio` 3.0 for large text, else 4.5), reading and changing nothing (no mutation, input, pump or settle): the text-bearing element's computed `color`, font size and weight over the background composited source-over from its ancestors' `background-color`s onto the root-else-`body` canvas over opaque white, `None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image; `contrast_ratio` is the WCAG 2.x ratio.»
