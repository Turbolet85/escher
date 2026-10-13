group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:35   class: omission
  leaf-text: «a document whose `can_paint_text` reads false `ScreenshotError::FontsUnavailable` with nothing painted)»
  source: .andromeda/architecture.md:255 «the private contrast (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the last two re-exported from the crate root»
  replace-with: «a document whose `can_paint_text` reads false `ScreenshotError::FontsUnavailable` with nothing painted), contrast (`Harness::text_contrast` → `Option<TextContrast>`, read-only from computed styles: the text-bearing element's `color`, font size and weight over its ancestors' `background-color`s composited onto the canvas over opaque white, `None` past an `opacity` below 1, a filter or a background image; `contrast_ratio` the WCAG 2.x ratio, `TextContrast::required_ratio` 3.0 for large text, else 4.5)»

- leaf: CLAUDE.md:74   class: omission
  leaf-text: «`packages/blitz-test-harness/src/{harness,input,inspect,settle,screenshot}.rs` (the screenshot's unit tests inline)»
  source: .andromeda/architecture.md:255 «the private contrast (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the last two re-exported from the crate root»
  replace-with: «`packages/blitz-test-harness/src/{harness,input,inspect,settle,screenshot,contrast}.rs` (the screenshot's and the contrast reader's unit tests inline) · the stand contrast check `tests/blitz-tests/tests/stand_contrast.rs`»

- leaf: CLAUDE.md:45   class: omission
  leaf-text: «focusability, focus order and painted colours are a11y surface.»
  source: .andromeda/a11y-plan.md:224 «SC 1.4.3 is therefore not met on the stand's five accent buttons. The check records and gates nothing»
  replace-with: «focusability, focus order and painted colours are a11y surface; SC 1.4.3 is measured on the stand's 18 controls by `stand_contrast`, which asserts its stated record row for row in both layout modes (a stand colour change moves that record) and gates nothing — not met on the five accent buttons (white `#ffffff` on `#4a6cf7`, 4.39 against 4.5), owned by the route entry "Stand a11y assertions".»
