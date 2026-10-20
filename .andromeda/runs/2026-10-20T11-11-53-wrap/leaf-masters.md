group: masters
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «writes that tree as one compact text under a recorded size budget (`Snapshot::to_text`, `SNAPSHOT_TEXT_BUDGET`),»
  source: .andromeda/architecture.md:209 «beside it `Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>` (`TextPart { text: String, ids: Vec<String> }`, `Debug` · `Clone` · `Default` · `PartialEq` · `Eq`) cuts that text into parts»
  replace-with: «writes that tree as one compact text under a recorded size budget (`Snapshot::to_text`, `SNAPSHOT_TEXT_BUDGET`) and cuts that text into parts within a budget — consecutive pre-order lines, never a split line, a line longer than the budget a part alone, the parts concatenating to the whole text (`Snapshot::text_parts` → `TextPart { text, ids }`, re-exported with the budget),»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «and returns an `Outcome`: the screen's text, or whether»
  source: .andromeda/architecture.md:196 «with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text as `Outcome::Screen`; otherwise it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)`»
  replace-with: «and returns an `Outcome`: the screen's text — or, for a screen over `SNAPSHOT_TEXT_BUDGET` or a `part` asked (`snapshot`'s one optional argument, `ArgKind::Part`, the seventh kind, 1 to `MAX_PART` = 1000), one part of `text_parts(SNAPSHOT_TEXT_BUDGET)` as `Outcome::Part { text, part, parts }`, written `{"text":…,"part":n,"parts":m}`, part 1 when `part` is absent and a part past the last the empty text with the same `parts`, the record of ids fed from the whole screen — or whether»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «a `snapshot` result the screen text, a `help` result»
  source: .andromeda/security-plan.md:129 «a whole-screen `snapshot` result is `to_text`'s output, a paged one its JSON line `{"text","part","parts"}` holding the same lines»
  replace-with: «a whole-screen `snapshot` result the screen text, a paged one its JSON line `{"text","part","parts"}`, a `help` result»

- leaf: CLAUDE.md:78   class: omission
  leaf-text: «its text form and size budget · its diff | `packages/dioxus-native-dom/src/{snapshot,snapshot_text,snapshot_diff}.rs` · checks `tests/blitz-tests/tests/stand_{snapshot,snapshot_state,snapshot_text,diff}.rs`»
  source: .andromeda/test-plan.md:27 «and `stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «its text form, its parts and size budget · its diff | `packages/dioxus-native-dom/src/{snapshot,snapshot_text,snapshot_diff}.rs` · checks `tests/blitz-tests/tests/stand_{snapshot,snapshot_state,snapshot_text,snapshot_parts,diff}.rs`»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «then `snapshot` · `click --id`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «the usage line reads `snapshot [--part <part>]`»
  replace-with: «then `snapshot [--part]` · `click --id`»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «in a hosted `snapshot`'s answer;»
  source: .andromeda/architecture.md:118 «a screen over the snapshot budget crosses them in parts — `snapshot`'s optional `part` (a whole number, 1 to `MAX_PART` = 1000) in, one part's text with the closed counts `part` and `parts` out»
  replace-with: «in a hosted `snapshot`'s answer, and a screen over the budget read in parts — `Snapshot::text_parts`, consecutive pre-order lines never split, the driver's `snapshot` taking an optional `part` (a whole number, 1 to `MAX_PART` = 1000) and answering one part's text with the closed counts `part` and `parts`, part 1 by default, a part past the last the empty text, a screen within the budget answering whole as before, proven on an over-budget fixture page in both layout modes, in process and through `serve_mcp` (a ratified boundary widening);»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «a `snapshot` result the screen text itself, an accepted»
  source: .andromeda/architecture.md:196 «the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one»
  replace-with: «a `snapshot` result the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one, an accepted»
