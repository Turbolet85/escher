group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «a recorded session's calls — the ids they named among them — rest in a kept check, with its answers' digests and no screen reading (ratified by the founder, 2026-10-19)»
  source: .andromeda/architecture.md:119 «each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels — so a kept check holds the screens' ids, names, values»
  replace-with: «a recorded session's calls — the ids they named among them — rest in a kept check with its answers' digests and, since 2026-10-19-recorded-check-replay, each answer kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels — so the screens' ids, names, values, snapshot text and diffs rest there too, never logged (ratified by the founder, 2026-10-19, F1 A · F2 B · F3 B); a kept check replays through `Session::run` alone (`Check::replay`), and the repository's own two scripted stand checks are committed in git, the one exception to owner-only»

- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «the reader of MCP requests and, since 2026-10-19-session-recording, of kept checks (`Check::from_bytes`), writing nothing»
  source: .andromeda/security-plan.md:255 «-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing»
  replace-with: «the reader of MCP requests, since 2026-10-19-session-recording of kept checks (`Check::from_bytes`) and, since 2026-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing»

- leaf: .claude/docs/services/escher-driver.md:12   class: omission
  leaf-text: «`serde_json` reads MCP requests and, since 2026-10-19-session-recording, kept checks, and writes nothing.»
  source: .andromeda/architecture.md:115 «a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing»
  replace-with: «`serde_json` reads MCP requests, since 2026-10-19-session-recording kept checks and, since 2026-10-19-recorded-check-replay, the live answer line a replay compares, and writes nothing.»

- leaf: .claude/docs/services/escher-driver.md:16   class: omission
  leaf-text: «`masked` (`true`, on a `type` alone) · `accepted` · `digest`, and rewrites the check whole»
  source: .andromeda/architecture.md:186 «`digest` · `answer` — the answer's line as the driver wrote it (`Outcome::to_json` or `Refusal::to_json`, private `written`) inlined as its own JSON object, masked as on every carrier»
  replace-with: «`masked` (`true`, on a `type` alone) · `accepted` · `digest` · `answer` (since 2026-10-19-recorded-check-replay: the answer's line as the driver wrote it, `Outcome::to_json` or `Refusal::to_json`, inlined as its own JSON object, masked as on every carrier, a shot's written by `json::shot_without_image(width, height)` as `{"width":…,"height":…}` with no `png` — crate-private `kept_line`, computed once by `Recording::record`), and rewrites the check whole»

- leaf: .claude/docs/services/escher-driver.md:16   class: stale
  leaf-text: «every other `type` its text as it crossed; no screen reading is kept.»
  source: .andromeda/architecture.md:186 «every other `type` its text as it crossed; each answer is kept whole, a shot's pixels excepted.»
  replace-with: «every other `type` its text as it crossed; each answer is kept whole, a shot's pixels excepted.»

- leaf: .claude/docs/services/escher-driver.md:16   class: omission
  leaf-text: «`Check::from_bytes(&[u8]) -> Result<Check, Refusal>` reads a kept check strictly, anything else refused»
  source: .andromeda/architecture.md:186 «`answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, as for every key)»
  replace-with: «`Check::from_bytes(&[u8]) -> Result<Check, Refusal>` reads a kept check strictly — since 2026-10-19-recorded-check-replay `answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, as for every key) — anything else refused»

- leaf: .claude/docs/services/escher-driver.md:16   class: omission
  leaf-text: «`Check::{app, steps}` and `Step::{call, accepted, digest, masked}` read it.»
  source: .andromeda/architecture.md:187 «The replay: `Check::replay(&self, session: &mut Session) -> Replay` runs nothing on a session whose `label()` is not the check's `app()`; otherwise it runs each step's `call()` through `Session::run`»
  replace-with: «`Check::{app, steps}` and `Step::{call, accepted, digest, masked}` read it, the kept answer held in a private field read by `pub(crate) fn answer`, no public accessor returning a `serde_json` type. Since 2026-10-19-recorded-check-replay `Check::replay(&self, session: &mut Session) -> Replay` runs nothing on a session whose `label()` is not the check's `app()`; otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs — its acceptance differs, or the live answer's kept line, read by `serde_json::from_str`, is not equal to the kept `answer`; `Replay` (`holds()` · `another_app()` · `ran()` · `change() -> Option<&Change>`) and `Change` (`step()` · `accepted_then()` · `accepted_now()` · `keys() -> &[&'static str]`, the step verb's result fields that differ in table order, then `refused`) hold no reading and no `serde_json` type and derive `Debug, Clone, PartialEq, Eq`; the module opens no span, each replayed step leaving its own `command` span.»

- leaf: .claude/docs/services/escher-driver.md:37   class: stale
  leaf-text: «only the optional crate-private recording `recording_into` sets, which keeps calls and answers' digests, never a screen reading»
  source: .andromeda/architecture.md:186 «every other `type` its text as it crossed; each answer is kept whole, a shot's pixels excepted.»
  replace-with: «only the optional crate-private recording `recording_into` sets, which keeps calls, their answers' digests and, since 2026-10-19-recorded-check-replay, each answer whole, a shot's pixels excepted»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «87 unit tests (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 14 · `describe.rs` 2 · `image.rs` 5 · `recording.rs` 9; `host.rs` none; beside three off-unix tests the windows job alone runs; 74 until 2026-10-19-session-recording»
  source: .andromeda/test-plan.md:22 «**escher-driver:** 92 unit tests in fifteen files, 72 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files»
  replace-with: «92 unit tests (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 3 · `mcp_tests.rs` 14 · `describe.rs` 2 · `image.rs` 5 · `recording.rs` 9 · `replay.rs` 5; `host.rs` none; beside three off-unix tests the windows job alone runs; 87 until 2026-10-19-recorded-check-replay, 74 until 2026-10-19-session-recording»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «a shot of one state byte-identical in two child runs and in the parent, 12 shots in both layout modes»
  source: .andromeda/test-plan.md:54 «parent on one host, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577)»
  replace-with: «a shot of one state byte-identical in two child runs and in the parent on one host, 12 shots in both layout modes — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no `screenshot` step»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal and keeps no screen reading at rest»
  source: .andromeda/test-plan.md:27 «a recorded session keeps each answer whole but a shot's image — every record ending with its kept line, no `png` and no shot's first 64 base64 characters — and replays through `Check::replay`»
  replace-with: «a recorded session keeps each answer whole but a shot's image — every record ending with its kept line, no `png` and no shot's first 64 base64 characters — and replays through `Check::replay` on each lean task in both layout modes, each script ending `snapshot` · `screenshot`, so the shot path's kept form is covered on each unix OS CI runs (linux, macOS; the file is unix only, as recording is)»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «a recording directory open to its group is refused with nothing written).»
  source: .andromeda/test-plan.md:27 «the committed checks replayed — `stand_replay` 3, declaring `mod session_common;`, not unix-gated: the two committed Counter checks hold in both layout modes»
  replace-with: «a recording directory open to its group is refused with nothing written); since 2026-10-19-recorded-check-replay `stand_replay` (3, not unix-gated: the two committed Counter checks under `tests/blitz-tests/checks/` hold in both layout modes, turn red at step 1 (the command line's) and step 3 (the MCP role's) on a test-local `Counter` variant of the task's own name with every earlier step held and `changed` among the keys, a check on another app runs nothing, and neither check holds a `png` key or a shot's first 64 base64 characters).»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «`recording` (3 tests, unix only, and 1 off-unix: the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`;»
  source: .andromeda/test-plan.md:10 «`tests/recording.rs` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay) — the command line and the `mcp` role keep the same check byte for byte»
  replace-with: «`recording` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay: the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; since 2026-10-19-recorded-check-replay the committed checks what the binary records, `tests/flows/record_checks.sh` run through the binary and both files compared byte for byte with the committed copies;»
