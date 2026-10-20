group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:6   class: omission
  leaf-text: «keeps its answered calls at rest as one owner-only replayable check (`std` only, unix only like the socket).»
  source: .andromeda/architecture.md:348 «and lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the in-process visual lint, PROVISIONAL) — re-exported at the crate root beside blitz-test-harness's `Busy` and `Settled`»
  replace-with: «keeps its answered calls at rest as one owner-only replayable check (`std` only, unix only like the socket), and since 2026-10-19-visual-lint the in-process **visual lint** (`Session::lint`, PROVISIONAL: in process only), which reads the held screen and returns its layout and contrast defects as typed findings, crossing nothing.»

- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «In process only. `Busy` and `Settled` are re-exported from blitz-test-harness.»
  source: .andromeda/architecture.md:198 «- The visual lint, in process only (PROVISIONAL): `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` reads the held instance's current screen and returns one `Finding { id, defect, bounds, reason }`»
  replace-with: «In process only. `Busy` and `Settled` are re-exported from blitz-test-harness. `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` (since 2026-10-19-visual-lint; in process only, PROVISIONAL) reads the held instance's current screen and returns one `Finding { id, defect, bounds, reason }` per defect it reads, in document order, then `Defect::ALL` order: it refuses `fonts-unavailable` before anything is read when `can_paint_text` reads false, reads `harness.doc.snapshot()`, recording each node's id in the session's record of ids as `snapshot` does, and runs no step, settle, pump or time move, so it changes nothing. `Defect` is closed at six (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`), `Reason` holds the measured reason (`Overlap { other, area }` · `Clipped { visible }` · `Truncated { text, visible: Option<Bounds> }` · `OffScreen` · `ZeroSize` · `Contrast { ratio, required }`), and every rectangle is the lint's own `Bounds { x, y, width, height: f64 }` in the snapshot bounds' space, the crate naming neither blitz-dom nor stylo; the four types derive `Debug`, `Clone` and `PartialEq` and hold no `NodeId`, page text, colour or value. Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint`; `truncated` reads blitz-dom's `BaseDocument::text_extent` (its one non-test caller) on a node that is no text control, and `contrast` `Harness::text_contrast` below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured. No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, and `VERBS` stays eleven.»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «`fonts-unavailable` from `screenshot` alone, checked before anything is painted.»
  source: .andromeda/architecture.md:192 «`fonts-unavailable`, the last from `screenshot` alone among the verbs, checked before anything is painted (below), and from the in-process `Session::lint` before anything is read (below).»
  replace-with: «`fonts-unavailable` from `screenshot` alone among the verbs, checked before anything is painted, and, since 2026-10-19-visual-lint, from the in-process `Session::lint` before anything is read.»

- leaf: .claude/docs/services/escher-driver.md:37   class: omission
  leaf-text: «fed from the session's own snapshots»
  source: .andromeda/architecture.md:192 «The record is fed from the snapshot the in-process `Session::lint` reads, which takes no argument, and from every snapshot a call takes once `validate` has admitted it»
  replace-with: «fed from the session's own snapshots — every snapshot a call takes once `validate` has admitted it and, since 2026-10-19-visual-lint, the one the in-process `Session::lint` reads, which takes no argument»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «- 92 unit tests (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli_tests.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 14 · `describe.rs` 2 · `image.rs` 5 · `recording_tests.rs` 9 · `replay.rs` 5;»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
  replace-with: «- 94 unit tests in sixteen files (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli_tests.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 14 · `describe.rs` 2 · `image.rs` 5 · `recording_tests.rs` 9 · `replay.rs` 5 · `lint.rs` 2, since 2026-10-19-visual-lint: `Defect::ALL` names the six defects in order, and findings follow document order, then the defect order;»

- leaf: .claude/docs/services/escher-driver.md:60   class: omission
  leaf-text: «87 until 2026-10-19-recorded-check-replay»
  source: .andromeda/test-plan.md:22 «(92 in fifteen until 2026-10-19-visual-lint, 87 in fourteen until 2026-10-19-recorded-check-replay, 74 in thirteen until 2026-10-19-session-recording»
  replace-with: «92 in fifteen until 2026-10-19-visual-lint, 87 until 2026-10-19-recorded-check-replay»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «58 tests inline and 34 in those four files»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
  replace-with: «60 tests inline and 34 in those four files»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «and neither check holds a `png` key or a shot's first 64 base64 characters).»
  source: .andromeda/test-plan.md:27 «and, since 2026-10-19-visual-lint, `stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only)»
  replace-with: «and neither check holds a `png` key or a shot's first 64 base64 characters); since 2026-10-19-visual-lint `stand_lint` (5, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on the in-process `Session::lint`: a planted page (`lint_page`) reads exactly its seven findings in document order with their reasons and its negatives read nothing; each lean task at boot reads 0 findings; the lint reads and changes nothing; after twelve Creates the last CRUD row reads `clipped` inside `crud-list` and its `click` is refused `off-screen`; an app with no font is refused `fonts-unavailable`; picked up by `run stand` and the stand-repeat leg by its prefix; a clipped or off-screen node holding text also reading `truncated` is asserted by no check).»
