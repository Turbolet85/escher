proposals:
  - detector: D-arch-resources
    severity: escalate
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'Register the in-process lint: since 2026-10-19-visual-lint `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` (packages/escher-driver/src/lint.rs:190-230), in process only and no verb (`VERBS` stays eleven; no `Command`, `Outcome` variant, written form, MCP tool, recorded step, socket, CLI arm, env var or port), refuses `fonts-unavailable` before anything is read when `can_paint_text` is false, reads `harness.doc.snapshot()` (recording its ids in `SeenIds` as `snapshot` does), runs no step, settle, pump or time move and opens no span, and reports six defects — `Defect` (`ALL` in the order `Overlap` · `Clipped` · `Truncated` · `OffScreen` · `ZeroSize` · `Contrast`, `name()` `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`) — each a `Finding { id, defect, bounds: Bounds, reason: Reason }` (`Reason` `Overlap { other, area }` · `Clipped { visible }` · `Truncated { text, visible: Option<Bounds> }` · `OffScreen` · `ZeroSize` · `Contrast { ratio, required }`; `Bounds` a lint-owned `x, y, width, height: f64` rectangle in the snapshot bounds'' space, not `BoundingRect`, since the crate depends on neither blitz-dom nor stylo), all `Debug`, `Clone`, `PartialEq`, none holding a `NodeId`, page text, a colour or a value; the clip region is `visible_region` cut by the padding box of every ancestor below the root whose computed `contain` includes `paint` (the lint''s own walk; `visible_region` unchanged), computed `contain` · `overflow-x`/`-y` · `display` read through `BaseDocument::resolved_style_value` as closed keyword sets; overlap read over each box''s visible part (the operator''s ruling, inputs#I5); contrast through `Harness::text_contrast` / `required_ratio()`, exempt at `enabled == Some(false)`; findings in document order, then `Defect::ALL` order — and the crate''s re-exports move 51 → 55 names (`Bounds`, `Defect`, `Finding`, `Reason` from the private module `lint`; packages/escher-driver/src/lib.rs:82-87 · 130 · 148), F1 (in process only) and F2 (no new field) PROVISIONAL for the founder''s own word.'
    sidecar: 'arch §Standard Contracts → Driver session: Session::lint (in process, no verb) with Finding · Defect · Reason · Bounds registered; re-exports 51 → 55 (2026-10-19-visual-lint).'
    rationale: 'The report''s Changes → Symbols / APIs adds `Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds` as public escher-driver API and Crates / modules adds the private module `escher_driver::lint`; architecture.md:191 (the home the report''s Expected amendments name) lists 51 re-exported names and no lint type, so the new public API is unregistered. Expected amendments marks the pattern «Boundary widening» (escalate), F1/F2 PROVISIONAL (inputs#I2, inputs#I3 row 1); Deviations 1–3 and the Addendum (inputs#I5) give the Bounds type, the resolved_style_value reads and the overlap definition.'
    basis: 'packages/escher-driver/src/lint.rs:190-230'
  - detector: D-arch-resources
    severity: escalate
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The causes paragraph''s "`fonts-unavailable`, the last from `screenshot` alone" now reads that `fonts-unavailable` is returned by `screenshot` and, since 2026-10-19-visual-lint, by the in-process `Session::lint` (before anything is read when `can_paint_text` is false) — the executor still returning it from `screenshot` alone among the verbs.'
    sidecar: 'arch §Standard Contracts → Driver session: fonts-unavailable is also returned by Session::lint, no longer by screenshot alone (2026-10-19-visual-lint).'
    rationale: 'Report Changes → Symbols / APIs: `Session::lint` "refuses `Cause::FontsUnavailable` before anything is read when `can_paint_text` is false", and Outcome: "With no font it refuses `fonts-unavailable`" — retiring architecture.md:191''s claim that the cause comes from `screenshot` alone.'
    basis: 'packages/escher-driver/src/lint.rs:190-230'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Standard Contracts → Driver session (escher-driver)'
    change: 'The record-of-ids paragraph ("The record is fed from every snapshot a call takes once `validate` has admitted it …") adds the one feeder outside a call: since 2026-10-19-visual-lint the snapshot `Session::lint` reads records its ids in `SeenIds` as `snapshot` does.'
    sidecar: 'arch §Standard Contracts → Driver session: the session''s record of ids is also fed by Session::lint''s snapshot (2026-10-19-visual-lint).'
    rationale: 'Report Changes → Symbols / APIs: `Session::lint` "reads `harness.doc.snapshot()` and records each node''s id in the session''s `SeenIds`, as `snapshot` does" — a feeder that is no call, which architecture.md:191''s enumeration of what feeds the record does not hold.'
    basis: 'packages/escher-driver/src/lint.rs:190-230'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Scrolling, selection, tree'
    change: 'Register the new read-only reader: since 2026-10-19-visual-lint `BaseDocument::text_extent(&self, NodeId) -> Option<TextExtent>` and the crate-root re-export `pub struct TextExtent { content: BoundingRect, text: BoundingRect }` (packages/blitz-dom/src/text_extent.rs:5-15; packages/blitz-dom/src/text_extent.rs:18-54; packages/blitz-dom/src/lib.rs:69-70 · 111) — `content` the client rect less border, padding and any scrollbar, `text` from the content box''s top-left moved down by the inline layout''s `block_offset` and sized by Parley `Layout::width()` / `height()` over its scale; `None` for an id that does not resolve, a node that is no inline root or one with no inline layout data; it mutates nothing and has no log site, print, env read or panic path; its stated limits — RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect; its one non-test caller escher-driver''s `Session::lint`.'
    sidecar: 'arch §Standard Contracts → Scrolling, selection, tree: BaseDocument::text_extent and TextExtent registered, read-only, with their limits (2026-10-19-visual-lint).'
    rationale: 'The report''s Changes → Symbols / APIs adds `TextExtent` and `BaseDocument::text_extent` as "a new public read-only reader on every document", and Expected amendments names architecture.md:161 as its home; no arch registry section names it (`text-extent` control unfired).'
    basis: 'packages/blitz-dom/src/text_extent.rs:18-54'
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → escher-driver'
    change: 'The escher-driver row''s "sixteen private modules" becomes seventeen, adding, since 2026-10-19-visual-lint, lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the in-process visual lint) to the modules re-exported at the crate root.'
    sidecar: 'arch §Existing Scopes → escher-driver: private module lint added, sixteen → seventeen modules (2026-10-19-visual-lint).'
    rationale: 'Report Changes → Crates / modules: "new private modules `blitz_dom::text_extent` and `escher_driver::lint`"; the escher-driver scope row enumerates sixteen private modules and none is lint.'
    basis: 'packages/escher-driver/src/lib.rs:82-87 · 130 · 148'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Existing Scopes → blitz-dom'
    change: 'The blitz-dom row''s module list adds, since 2026-10-19-visual-lint, text_extent (the private module of `BaseDocument::text_extent`, its `TextExtent` re-exported from the crate root; packages/blitz-dom/src/lib.rs:69-70 · 111).'
    sidecar: 'arch §Existing Scopes → blitz-dom: private module text_extent added to the module list (2026-10-19-visual-lint).'
    rationale: 'Report Changes → Files and Crates / modules: `mod text_extent;` and `pub use text_extent::TextExtent;` added to packages/blitz-dom/src/lib.rs; the blitz-dom scope row enumerates its modules and lacks text_extent.'
    basis: 'packages/blitz-dom/src/lib.rs:69-70'
    dependent-of: D-arch-resources
note: 'D-arch-decisions holds (Dependencies none; the computed-style reads go through the existing public resolved_style_value, as the harness contrast reader already does). Not proposed here: architecture.md:360''s reader counts (session_common 25 → 26, fixtures 19 (18 stand_*) → 20 (19 stand_*)), which the report lists under its count patterns `session-readers` and `fixtures-readers` for the count cascade.'
