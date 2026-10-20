group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` runs the 21 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 22 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 22 `stand_*` readers of the shared module»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 192 · 0 · 7 with the `lint` verb at 2026-10-19-visual-lint's gated part»
  source: .andromeda/test-plan.md:132 «`run stand` 195 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
  replace-with: «`run stand` 195 · 0 · 7 over 48 files with the paged snapshot at 2026-10-20-large-screen-in-parts»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 23 checks — 21 `stand_*` files, `stand_act_select` since 2026-10-20-select-by-id,»
  source: .andromeda/test-plan.md:68 «read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (`stand_snapshot_parts`, `stand_act_select`,»
  replace-with: «(read by 24 checks — 22 `stand_*` files, `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select` since 2026-10-20-select-by-id,»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «picked up by `run stand` and the stand-repeat leg by its prefix) — asserted unconditionally»
  source: .andromeda/test-plan.md:27 «and `stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «picked up by `run stand` and the stand-repeat leg by its prefix) and, since 2026-10-20-large-screen-in-parts, `stand_snapshot_parts` (3, declaring `mod fixtures;` and `mod session_common;`, both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: the walk in parts through `Session::run` — part 1 by default, each part within the budget, the parts concatenating to `to_text()`, the empty part past the last, an id read only in the last part addressable, a password masked in every part, each part's lines equal to the whole text's — the same walk through `serve_mcp`, and a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix) — asserted unconditionally»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»
  source: .andromeda/test-plan.md:22 «**escher-driver:** 100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files»
  replace-with: «100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_tests.rs` 14 (12 until 2026-10-19-session-recording»
  source: .andromeda/test-plan.md:22 «mcp_tests.rs 15 (12 until 2026-10-19-session-recording»
  replace-with: «`mcp_tests.rs` 15 (12 until 2026-10-19-session-recording»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «the listing as the twelve verbs;»
  source: .andromeda/test-plan.md:22 «the listing is the twelve verbs of the table, `snapshot`'s tool taking an optional integer `part` and naming its two counts;»
  replace-with: «the listing as the twelve verbs, `snapshot`'s tool taking an optional integer `part` and naming its two counts;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`cli_commands` (13 — a hosted `lint` answering status `0`»
  source: .andromeda/test-plan.md:204 «`cli_commands` (14): each of the four tasks starts, answers and stops; `snapshot --part` read by number answers one uncoloured JSON line, status 0, and `--part` 0, 1001 and `x` exit 1»
  replace-with: «`cli_commands` (14 — `snapshot --part` read by number answering one uncoloured JSON line, status `0`, and `--part` 0, 1001 and `x` exiting `1` with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed written back; a hosted `lint` answering status `0`»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «a refused no-font lint's line, `cause=fonts-unavailable`;»
  source: .andromeda/test-plan.md:27 «35 calls per mode — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone —»
  replace-with: «a refused no-font lint's line, `cause=fonts-unavailable`; a `snapshot` with `part` 1 among its 35 calls per mode, its line `verb="snapshot"` alone;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and each also drives one `lint`, still reading 0 needles on stderr at `trace`»
  source: .andromeda/test-plan.md:187 «each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines — still reading 0 needles»
  replace-with: «and each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines — still reading 0 needles on stderr at `trace`»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 47 `stand_*.rs` files and seven_guis' eight process checks (46 before `stand_act_select`;»
  source: .andromeda/test-plan.md:324 «one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 48 files»
  replace-with: «three rounds of the 48 `stand_*.rs` files and seven_guis' eight process checks (47 before `stand_snapshot_parts`; 46 before `stand_act_select`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1087 · 0 · 12 over 204 result lines with the `lint` verb at 2026-10-19-visual-lint's gated part (from 204 · 1083:»
  source: .andromeda/test-plan.md:340 «re-counted with the paged snapshot: 205 result lines, 1098 passed · 0 failed · 12 ignored — +1 result line (`stand_snapshot_parts`) and +11 passed»
  replace-with: «`cargo test --workspace` 1098 · 0 · 12 over 205 result lines with the paged snapshot at 2026-10-20-large-screen-in-parts (from 204 · 1087: +1 result line — `stand_snapshot_parts` — and +11 passed — dioxus-native-dom's 6 units, escher-driver's 1, `stand_snapshot_parts` 3, `cli_commands` 1; `run stand` 195 passed · 0 failed · 7 ignored over 48 files, `run all` not re-counted; timings not re-measured); 1087 · 0 · 12 over 204 result lines with the `lint` verb at 2026-10-19-visual-lint's gated part (from 204 · 1083:»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 47 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (with the `lint` verb's four tests CI#39387656399's `Stand checks repeated` reading 165 · 672 · 0 · 21, no file added;»
  source: .andromeda/test-plan.md:353 «run the 48 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; with `stand_snapshot_parts` CI#39410865604 read 168 · 684 · 0 · 21, +3 · +12;»
  replace-with: «the 48 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (with `stand_snapshot_parts` CI#39410865604's `Stand checks repeated` reading 168 · 684 · 0 · 21; 47 with the `lint` verb's four tests, CI#39387656399's reading 165 · 672 · 0 · 21, no file added;»
