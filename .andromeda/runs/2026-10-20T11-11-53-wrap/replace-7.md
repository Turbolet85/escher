- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «ends at the bound `settled` false with `busy` `Loads`, no time moved) and seven_guis `recording` 4»
new: «ends at the bound `settled` false with `busy` `Loads`, no time moved) · `stand_snapshot_parts` 3 (`Session::run` in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: `snapshot` with no argument answering part 1 of `parts` ≥ 2, each part within the budget, the parts concatenating to `to_text()` with every node id on exactly one line, `parts + 1` the empty text with the same `parts` and the screen unchanged, an id read only in the last part taking `scroll` then `click`, a password reading `MASKED_VALUE` in every part and the typed sentinel 0 times; the same walk through `serve_mcp` answering the JSON lines `Session::run` writes; a screen inside the budget read whole as before) and seven_guis `recording` 4»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «seven_guis `cli_commands` 13 · `cli_flow` 2»
new: «seven_guis `cli_commands` 14 · `cli_flow` 2»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «escher-driver's 99 unit tests (»
new: «escher-driver's 100 unit tests (»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «lint.rs 2 · mcp_tests.rs 14 · recording_tests.rs 9»
new: «lint.rs 2 · mcp_tests.rs 15 · recording_tests.rs 9»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
old: «(`--id`, `--text`, `--key`, `--ms` each with one value, `--shift` bare;»
new: «(`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare; `snapshot`'s `--part` a whole number 1 to 1000, a paged answer one line `{"text":…,"part":n,"parts":m}`, status `0`, a whole screen's still `{"text":…}` alone;»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
old: «the usage line reads `start <app> [--record <dir>]`,»
new: «the usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`,»
- line: .andromeda/test-plan.md:27
old: «picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
new: «picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md; and `stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: the walk in parts through `Session::run` — part 1 by default, each part within the budget, the parts concatenating to `to_text()`, the empty part past the last, an id read only in the last part addressable, a password masked in every part, each part's lines equal to the whole text's — the same walk through `serve_mcp`, and a screen inside the budget read whole (tests/blitz-tests/tests/stand_snapshot_parts.rs:82-333), picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
- line: .andromeda/test-plan.md:27
old: «exactly one line carried a busy class, 34 calls per mode»
new: «exactly one line carried a busy class, 35 calls per mode — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone —»
- line: .andromeda/test-plan.md:22
old: «99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»
new: «100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files»
- line: .andromeda/test-plan.md:22
old: «mcp_tests.rs 14 (12 until»
new: «mcp_tests.rs 15 (12 until»
- line: .andromeda/test-plan.md:22
old: «the listing is the twelve verbs of the table;»
new: «the listing is the twelve verbs of the table, `snapshot`'s tool taking an optional integer `part` and naming its two counts;»
- line: .andromeda/test-plan.md:25
old: «75 unit tests in nine files»
new: «81 unit tests in nine files»
- line: .andromeda/test-plan.md:25
old: «(packages/dioxus-native-dom/src/snapshot_text.rs:137-610), and nine on»
new: «(packages/dioxus-native-dom/src/snapshot_text.rs:137-610), six on its parts `Snapshot::text_parts` — the parts concatenating to the whole text each within the budget, each node id in exactly one part in pre-order, a line longer than the budget a part alone, an empty snapshot one empty part, a budget holding the whole text one part, a part keeping each line's absolute indent (packages/dioxus-native-dom/src/snapshot_text.rs:493-609), and nine on»
- line: .andromeda/test-plan.md:68
old: «read through `mod session_common;` by 28 checks (27 before»
new: «read through `mod session_common;` by 29 checks (`stand_snapshot_parts.rs` the latest; 27 before»
- line: .andromeda/test-plan.md:68
old: «read through `mod fixtures;` by 23 checks — 21 `stand_*.rs` (`stand_act_select`,»
new: «read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (`stand_snapshot_parts`, `stand_act_select`,»
- line: .andromeda/test-plan.md:103
old: «`lint()`, `act(session, call)`»
new: «`lint()`, `snapshot_part(n)` and the part reader `part(outcome)` — the text, the part and the count, panicking with a fixed message on any other outcome —, `act(session, call)`»
- line: .andromeda/test-plan.md:103
old: «(`None` for a refused call, for a `snapshot`, for a shot and for a lint;»
new: «(`None` for a refused call, for a `snapshot` whole or in parts, for a shot and for a lint;»
- line: .andromeda/test-plan.md:103
old: «(28 readers with `stand_act_select`;»
new: «(29 readers with `stand_snapshot_parts`, which reads `snapshot_part`, `part`, `scroll`, `click` and `act`; 28 with `stand_act_select`;»
- line: .andromeda/test-plan.md:104
old: «23 checks declare `mod fixtures;` — 21 `stand_*` files (`stand_act_select`;»
new: «24 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`;»
- line: .andromeda/test-plan.md:119
old: «re-counted with the `lint` verb, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 192 · failed 0 · ignored 7 over 47 files — +2 passed (`stand_lint`'s and `stand_mcp_tools`' lint tests, no file added); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
new: «re-counted with the paged snapshot, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 195 · failed 0 · ignored 7 over 48 files — +3 passed (`stand_snapshot_parts`, a new file its `stand_` prefix picks up); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
- line: .andromeda/test-plan.md:132
old: «runs the 21 `stand_*` readers of the shared module»
new: «runs the 22 `stand_*` readers of the shared module»
- line: .andromeda/test-plan.md:132
old: «`run stand` 192 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
new: «`run stand` 195 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
- line: .andromeda/test-plan.md:146
old: «the six argument kinds with their bounds (the sixth, `ArgKind::Name`, 1 to 32 bytes of `a-z`, `0-9`, `-`)»
new: «the seven argument kinds with their bounds (the sixth, `ArgKind::Name`, 1 to 32 bytes of `a-z`, `0-9`, `-`; the seventh, `ArgKind::Part`, a whole number 1 to `MAX_PART` = 1000)»
- line: .andromeda/test-plan.md:146
old: «`cli_commands` (13), `cli_flow` (2)»
new: «`cli_commands` (14), `cli_flow` (2)»
- line: .andromeda/test-plan.md:187
old: «and each also drives one `lint`, still reading 0 needles on stderr at `trace`, its count (2 each) unmoved.»
new: «and each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines — still reading 0 needles on stderr at `trace`, its count (2 each) unmoved.»
- line: .andromeda/test-plan.md:204
old: «`cli_commands` (13): each of the four tasks starts, answers and stops;»
new: «`cli_commands` (14): each of the four tasks starts, answers and stops; `snapshot --part` read by number answers one uncoloured JSON line, status 0, and `--part` 0, 1001 and `x` exit 1 with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed written back (examples/seven_guis/tests/cli_commands.rs:294-339);»
- line: .andromeda/test-plan.md:324
old: «`tests/blitz-tests/tests/stand_*.rs` — 47 files,»
new: «`tests/blitz-tests/tests/stand_*.rs` — 48 files,»
- line: .andromeda/test-plan.md:324
old: «`stand_load_wait` since 2026-10-20-load-wait and `stand_act_select`,»
new: «`stand_load_wait` since 2026-10-20-load-wait, `stand_act_select` and `stand_snapshot_parts`,»
- line: .andromeda/test-plan.md:334
old: «re-read with the `lint` verb on CI#39387656399, the pre-CI commit `ea3153f34` (green 17/17): linux 204 · 1087 · 0 · 12; macos and coverage 185 · 1084 · 0 · 12; windows 185 · 1041 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 165 · 672 · 0 · 21 — no result line added, +4 passed on linux, macos and coverage over CI#39371468367 (`stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1, `mcp_server` 1), +3 on windows (`cli_commands` unix-only), +0 · +12 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md»
new: «re-read with the paged snapshot on CI#39410865604, the pre-CI commit `191e66fdb` (green 17/17): linux 205 · 1098 · 0 · 12; macos and coverage 186 · 1095 · 0 · 12; windows 186 · 1051 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 168 · 684 · 0 · 21 — +1 result line and +11 passed on linux, macos and coverage over CI#39387656399 (dioxus-native-dom's 6 units, escher-driver's 1, `stand_snapshot_parts` 3, `cli_commands` 1), +10 on windows (`cli_commands` unix-only), +3 · +12 on the stand repeat (`stand_snapshot_parts` 9, `cli_commands` 3) — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/evidence/ci.md»
- line: .andromeda/test-plan.md:340
old: «re-counted with the `lint` verb: 204 result lines, 1087 passed · 0 failed · 12 ignored, from 204 · 1083 — no result line added and +4 passed (`stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1, `mcp_server` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
new: «re-counted with the paged snapshot: 205 result lines, 1098 passed · 0 failed · 12 ignored — +1 result line (`stand_snapshot_parts`) and +11 passed (dioxus-native-dom's 6 units, escher-driver's 1, `stand_snapshot_parts` 3, `cli_commands` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
- line: .andromeda/test-plan.md:353
old: «run the 47 `stand_*.rs` files»
new: «run the 48 `stand_*.rs` files»
- line: .andromeda/test-plan.md:353
old: «(§9 Legs; with the `lint` verb's four tests CI#39387656399 read 165 · 672 · 0 · 21, +0 · +12, no file added;»
new: «(§9 Legs; with `stand_snapshot_parts` CI#39410865604 read 168 · 684 · 0 · 21, +3 · +12;»
