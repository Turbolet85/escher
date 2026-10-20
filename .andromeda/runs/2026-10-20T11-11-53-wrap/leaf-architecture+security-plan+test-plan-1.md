group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «`dioxus-native-dom` (`DioxusDocument`, `SnapshotDiff` and,»
  source: .andromeda/architecture.md:209 «one part of `text_parts(SNAPSHOT_TEXT_BUDGET)` as `Outcome::Part`, `text_parts`' one caller outside the tests»
  replace-with: «`dioxus-native-dom` (`DioxusDocument`, `SnapshotDiff`, since 2026-10-20-large-screen-in-parts `SNAPSHOT_TEXT_BUDGET` and `Snapshot::text_parts` (the executor its one caller outside the tests) and,»

- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «Then per verb: `snapshot` returns the screen's text (`Snapshot::to_text`) and dispatches nothing;»
  source: .andromeda/architecture.md:196 «`snapshot` reads the screen once, `DioxusDocument::snapshot`, and dispatches nothing: with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text»
  replace-with: «Then per verb: `snapshot` reads the screen once and dispatches nothing — with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text as `Outcome::Screen`, otherwise (since 2026-10-20-large-screen-in-parts) it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)` and returns part `n` — `part`, or 1 when absent — as `Outcome::Part` with the count `parts`, a part past the last the empty text with the same `parts`;»

- leaf: .claude/docs/services/escher-driver.md:19   class: stale
  leaf-text: «`Screen { text }` · `Acted { settled, busy, diff }`»
  source: .andromeda/architecture.md:196 «`Outcome` is exactly `Screen { text: String }` · `Part { text: String, part: u32, parts: u32 }`»
  replace-with: «`Screen { text }` · `Part { text, part, parts }` (since 2026-10-20-large-screen-in-parts) · `Acted { settled, busy, diff }`»

- leaf: .claude/docs/services/escher-driver.md:20   class: omission
  leaf-text: «none for off-screen and zero-size; `start` answers»
  source: .andromeda/architecture.md:196 «a paged `snapshot`'s outcome `{"text":…,"part":n,"parts":m}`, a whole screen's still `{"text":…}` alone»
  replace-with: «none for off-screen and zero-size; a paged `snapshot`'s outcome `{"text":…,"part":n,"parts":m}`, a whole screen's still `{"text":…}` alone; `start` answers»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «`snapshot` (no argument; result `text`)»
  source: .andromeda/architecture.md:196 «`snapshot` (one optional `part`; result `text`, then `part` and `parts`, two counts written only when the screen is read in parts)»
  replace-with: «`snapshot` (one optional `part`; result `text`, then `part` and `parts`, two counts written only when the screen is read in parts)»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «`name` 1 to 32 bytes of `a-z`, `0-9`, `-`. The acting verbs' result»
  source: .andromeda/architecture.md:196 «`Part` (a whole number, 1 to `MAX_PART` = 1000 — `snapshot`'s `part`, the seventh kind)»
  replace-with: «`name` 1 to 32 bytes of `a-z`, `0-9`, `-` · `part` a whole number 1 to `MAX_PART` (1000), `snapshot`'s, the seventh kind. The acting verbs' result»

- leaf: .claude/docs/services/escher-driver.md:25   class: stale
  leaf-text: «`Command` is `Snapshot` · `Click`»
  source: .andromeda/architecture.md:196 «`Command` is exactly `Snapshot { part: Option<u32> }` (a `part` admitted in `1..=MAX_PART` through `u32::try_from`, any other number `OutOfBound`, a text or a flag `WrongKind`)»
  replace-with: «`Command` is `Snapshot { part: Option<u32> }` (a `part` admitted in `1..=MAX_PART` through `u32::try_from`, any other number `OutOfBound`, a text or a flag `WrongKind`) · `Click`»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «`start`'s `app` an `enum` of the apps)»
  source: .andromeda/architecture.md:196 «`start`'s `app` an `enum` of the binary's apps, `snapshot`'s `part` an `integer` from 1 to 1000)»
  replace-with: «`start`'s `app` an `enum` of the apps, `snapshot`'s `part` an `integer` from 1 to 1000)»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «an accepted `snapshot` one text block, the screen text itself;»
  source: .andromeda/architecture.md:196 «an accepted `snapshot` one text block, the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one»
  replace-with: «an accepted `snapshot` one text block, the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one;»

- leaf: .claude/docs/services/escher-driver.md:28   class: omission
  leaf-text: «`--id`, `--text`, `--key`, `--ms` each with one value, `--shift` bare;»
  source: .andromeda/architecture.md:196 «`--id`, `--text`, `--key`, `--ms` and `--part` each followed by one value, `--ms`'s and `--part`'s read as a number»
  replace-with: «`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--ms`'s and `--part`'s read as a number, `--shift` bare;»

- leaf: .claude/docs/services/escher-driver.md:28   class: omission
  leaf-text: «with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «the usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]` and `mcp role: mcp [--record <dir>]`»
  replace-with: «with `snapshot [--part <part>]`, `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`;»

- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «with no new carrier, exit, path argument, cause, session error, span field or dependency: a ratified boundary widening.»
  source: .andromeda/architecture.md:118 «a screen over the snapshot budget crosses them in parts — `snapshot`'s optional `part` (a whole number, 1 to `MAX_PART` = 1000) in, one part's text with the closed counts `part` and `parts` out»
  replace-with: «with no new carrier, exit, path argument, cause, session error, span field or dependency: a ratified boundary widening; since 2026-10-20-large-screen-in-parts a screen over the snapshot budget crosses them in parts — `snapshot`'s optional `part` (a whole number, 1 to `MAX_PART` = 1000) in, one part's text with the closed counts `part` and `parts` out, a part past the last the empty text, a screen within the budget answering whole as before, over MCP a paged answer its JSON line — with no new verb, carrier, exit, cause, session error, span field or dependency: a ratified boundary widening.»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «as does an accepted `lint`; a refused call»
  source: .andromeda/architecture.md:196 «an accepted `screenshot` record `verb` alone, as do an accepted `lint` and a part read»
  replace-with: «as do an accepted `lint` and a part read; a refused call»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «the screen text or its length, a shot's image»
  source: .andromeda/architecture.md:196 «Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image»
  replace-with: «the screen text or its length, a part's number, the part count, a shot's image»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «99 unit tests in sixteen files»
  source: .andromeda/test-plan.md:22 «**escher-driver:** 100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files»
  replace-with: «100 unit tests in sixteen files»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «`mcp_tests.rs` 14»
  source: .andromeda/test-plan.md:22 «since 2026-10-10-mcp-surface, mcp_tests.rs 15»
  replace-with: «`mcp_tests.rs` 15»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «64 tests inline and 35 in those four files»
  source: .andromeda/test-plan.md:22 «**escher-driver:** 100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files»
  replace-with: «64 tests inline and 36 in those four files»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «`stand_act_spans` (each call leaves exactly one `escher_driver` line in a sink capture, read in a re-run child: fixed words and counts only, no id, name or typed text, none at the default level;»
  source: .andromeda/test-plan.md:27 «35 calls per mode — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone —»
  replace-with: «`stand_act_spans` (each call leaves exactly one `escher_driver` line in a sink capture, read in a re-run child: fixed words and counts only, no id, name or typed text, none at the default level; 35 calls per mode, since 2026-10-20-large-screen-in-parts one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone;»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «`harness().time()` unmoved in both; picked up by `run stand` and the stand-repeat leg by its prefix).»
  source: .andromeda/test-plan.md:27 «`stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «`harness().time()` unmoved in both; picked up by `run stand` and the stand-repeat leg by its prefix). Since 2026-10-20-large-screen-in-parts `stand_snapshot_parts` (3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: the walk in parts through `Session::run` — part 1 by default, each part within the budget, the parts concatenating to `to_text()`, the empty part past the last, an id read only in the last part addressable, a password masked in every part, each part's lines equal to the whole text's — the same walk through `serve_mcp`, and a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix).»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «`cli_commands` (every driver command as a process on the four tasks: fields, refusals by status `1`, usage by `2`, session errors by `3`)»
  source: .andromeda/test-plan.md:204 «`snapshot --part` read by number answers one uncoloured JSON line, status 0, and `--part` 0, 1001 and `x` exit 1 with a line equal to `Refusal::malformed(fault).to_json()`»
  replace-with: «`cli_commands` (every driver command as a process on the four tasks: fields, refusals by status `1`, usage by `2`, session errors by `3`; since 2026-10-20-large-screen-in-parts `snapshot --part` read by number answering one uncoloured JSON line, status 0, and `--part` 0, 1001 and `x` exiting 1 with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed written back)»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «a host on `crud` at `RUST_LOG=trace` while three calls run: one span line per call»
  source: .andromeda/test-plan.md:187 «each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines»
  replace-with: «a host on `crud` at `RUST_LOG=trace` while seven calls run, one `lint` and one `snapshot` part read among them: one span line per call»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «`mcp_log` (the role at `RUST_LOG=trace`: no id, name or typed text, one `tool_call` line per call, under both builds;»
  source: .andromeda/test-plan.md:187 «each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines»
  replace-with: «`mcp_log` (the role at `RUST_LOG=trace`: no id, name or typed text, one `tool_call` line per call, under both builds; ten tool calls and 8 `command` lines, one `lint` and one `snapshot` part read among them;»
