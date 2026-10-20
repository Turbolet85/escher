group: architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «(`fonts-unavailable` the eighth, returned by `screenshot` alone;»
  source: .andromeda/architecture.md:192 «the last from `screenshot` alone among the verbs, checked before anything is painted (below), and from the in-process `Session::lint` before anything is read (below).»
  replace-with: «(`fonts-unavailable` the eighth, returned by `screenshot` alone among the verbs and by the in-process `Session::lint` before anything is read;»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «returns once it has settled (in process; `NotSettled` naming the busy class when it does not go quiet), `serve`»
  source: .andromeda/architecture.md:198 «- The visual lint, in process only (PROVISIONAL): `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` reads the held instance's current screen and returns one `Finding { id, defect, bounds, reason }`»
  replace-with: «returns once it has settled (in process; `NotSettled` naming the busy class when it does not go quiet), `Session::lint` — the in-process visual lint (PROVISIONAL: in process only; no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, `VERBS` staying eleven) — reads the held screen and returns one `Finding { id, defect, bounds, reason }` per defect in document order, then `Defect::ALL` order (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`, the last through `Harness::text_contrast` under `stand_contrast`'s exempt rule), refusing `fonts-unavailable` before anything is read, recording each id it reads as `snapshot` does and running no step, so it changes nothing — its `Finding` · `Defect` · `Reason` · `Bounds` holding ids, closed words and numbers only, no `NodeId`, page text, colour or value, `serve`»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «two read-only readers, `can_paint_text`»
  source: .andromeda/architecture.md:166 «- The text extent: `BaseDocument::text_extent(&self, NodeId) -> Option<TextExtent>`, a read-only reader on every document, and `TextExtent { content, text }`, two `BoundingRect`s re-exported from the crate root»
  replace-with: «three read-only readers, `text_extent` (`TextExtent { content, text }`, the content box and the inline layout's text box, `None` for an id that does not resolve or a node that is no inline root or has no inline layout data; RTL and vertical text read as LTR horizontal lines, a transformed box at its client rect; escher-driver's `Session::lint` its one non-test caller, since 2026-10-19-visual-lint), `can_paint_text`»

- leaf: CLAUDE.md:81   class: omission
  leaf-text: «`packages/escher-driver/src/execute.rs` · the shot's PNG encoder and base64»
  source: .andromeda/test-plan.md:27 «`stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only)»
  replace-with: «`packages/escher-driver/src/execute.rs` · the in-process visual lint `packages/escher-driver/src/lint.rs` (`Session::lint`, PROVISIONAL; unit tests inline), its text-extent reader `packages/blitz-dom/src/text_extent.rs`, checks `tests/blitz-tests/tests/{stand_lint,text_extent}.rs` · the shot's PNG encoder and base64»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «so a check compared byte for byte across OSes holds no `screenshot` step).»
  source: .andromeda/architecture.md:198 «No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, and `VERBS` stays eleven; a node both clipped or off-screen and holding text also reads `truncated`, asserted by no check»
  replace-with: «so a check compared byte for byte across OSes holds no `screenshot` step), and the visual lint (built in process only, PROVISIONAL — `Session::lint` reads the held screen for six defects, `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`, through blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, and returns `Finding`s of ids, closed words and numbers, changing nothing and carried by no verb, written form, MCP tool, recorded step, span or field; proven on a planted fixture page and on the stand in both layout modes, each lean task reading 0 findings at boot and the last CRUD row after twelve Creates reading `clipped` inside `crud-list`; a node both clipped or off-screen and holding text also reads `truncated`, asserted by no check).»
