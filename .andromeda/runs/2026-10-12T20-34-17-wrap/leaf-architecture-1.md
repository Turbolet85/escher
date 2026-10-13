group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/conventions.md:6   class: tree
  leaf-text: «with the stand checks' shared tables and helpers stated once in `tests/common/mod.rs` (no test target).»
  source: .andromeda/architecture.md:115 «the seven_guis stand checks state the tables and helpers they share once, in the module `tests/blitz-tests/tests/common/mod.rs`, which a check reads by declaring `mod common;`»
  replace-with: «with the stand checks' shared tables and helpers stated once in `tests/blitz-tests/tests/common/mod.rs`, and what the checks that hold a `Session` share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs` (neither a test target).»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`BoundingRect`, CSSOM View accessors, `Widget` trait.»
  source: .andromeda/architecture.md:121 «since 2026-10-12-headless-screenshot two public read-only methods: `BaseDocument::can_paint_text(&self) -> bool` — whether the font context resolves a family for the `sans-serif` generic»
  replace-with: «`BoundingRect`, CSSOM View accessors, `Widget` trait, and since 2026-10-12-headless-screenshot two public read-only methods: `BaseDocument::can_paint_text(&self) -> bool` (whether the font context resolves a family for the `sans-serif` generic, through fontique's `generic_families`) and `BaseDocument::layout_as_text_input(&self, node_id, text) -> Option<parley::Layout<TextBrush>>` (`text` laid out with a text input's editor styles, scale and direction in the document's own fonts, one unbroken line, a fresh `LayoutContext` per call, `None` for a node that is no text input — its one caller is blitz-paint's password mask).»

- leaf: .claude/docs/services/blitz-paint.md:18   class: omission
  leaf-text: «- Paint mirrors named engines (Chrome contrast ratio and dash ratios, WebKit/Blink darken/lighten, Firefox per-box decoration).»
  source: .andromeda/architecture.md:129 «since 2026-10-12-headless-screenshot a password `input` (its `type` reading `password`) paints its value masked, in every document and through every renderer that paints with blitz-paint»
  replace-with: «- Paint mirrors named engines (Chrome contrast ratio and dash ratios, WebKit/Blink darken/lighten, Firefox per-box decoration). Since 2026-10-12-headless-screenshot a password `input` paints its value masked, in every document and through every renderer that paints with blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, a bullet per character laid out by blitz-dom's `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, the caret at the bullet of its index in the input's caret colour and the field scrolled on the bullets — a direct engine change for every document, ratified by the founder (2026-10-12); a click still places the caret by the real characters' widths (blitz-dom's pointer path, unchanged, not measured by any check).»

- leaf: .claude/docs/services/blitz-paint.md:32   class: omission
  leaf-text: «- **Headless screenshots:** `examples/screenshot.rs` renders through `VelloCpuImageRenderer`»
  source: .andromeda/architecture.md:79 «and since 2026-10-12-headless-screenshot so does blitz-test-harness's `screenshot` (`anyrender_vello_cpu` with no `multithreading`), which composites the frame over opaque white after paint»
  replace-with: «- **Headless screenshots:** `examples/screenshot.rs` renders through `VelloCpuImageRenderer`, and since 2026-10-12-headless-screenshot so does blitz-test-harness's `Harness::screenshot` / `screenshot_rect` (`anyrender_vello_cpu` with no `multithreading`), which composites the frame over opaque white after paint where the example fills before `paint_scene`»
