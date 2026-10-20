- line: .andromeda/test-plan.md:10
old: «`tests/cli_commands.rs` (12 tests, 11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot and 8 until 2026-10-12-self-description, unix only:»
new: «`tests/cli_commands.rs` (13 tests, unix only:»
- line: .andromeda/test-plan.md:10
old: «`tests/mcp_server.rs` (5 tests, 4 until 2026-10-12-headless-screenshot and 2 until 2026-10-12-self-description: the binary's `mcp` role completes the Counter flow by tool calls alone — `initialize`, `tools/list` offering the verbs in order, eleven since 2026-10-12-headless-screenshot, `start`, `snapshot`, `click` — and answers a `server/discover`,»
new: «`tests/mcp_server.rs` (6 tests: the binary's `mcp` role completes the Counter flow by tool calls alone — `initialize`, `tools/list` offering the verbs in order, twelve, `start`, `snapshot`, `click` — answers a `lint` tool call with its line of findings, and answers a `server/discover`,»
- line: .andromeda/test-plan.md:22
old: «— seven instance-level (`screenshot` the seventh, after `scroll`, since 2026-10-12-headless-screenshot), then `start` · `status` · `stop`, then, since 2026-10-12-self-description, `help` at the tool level, eleven;»
new: «— eight instance-level (`screenshot` the seventh, after `scroll`, and `lint` the eighth, after `screenshot`), then `start` · `status` · `stop`, then, since 2026-10-12-self-description, `help` at the tool level, twelve;»
- line: .andromeda/test-plan.md:22
old: «since that chunk a changed node with `differs` after `bounds`), cli_tests.rs 5»
new: «since that chunk a changed node with `differs` after `bounds`; a lint's outcome as `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` then its reason's keys, every reason pinned and a ratio that is not finite written `null`), cli_tests.rs 5»
- line: .andromeda/test-plan.md:22
old: «the listing is the eleven verbs of the table — ten until 2026-10-12-headless-screenshot, nine until 2026-10-12-self-description;»
new: «the listing is the twelve verbs of the table;»
- line: .andromeda/test-plan.md:22
old: «its count row reads `[11, 13, 12, 3, 8, 4, 15]` since 2026-10-19-session-recording)»
new: «its count row reads `[12, 13, 12, 3, 8, 4, 15]`)»
- line: .andromeda/test-plan.md:27
old: «32 calls per mode (31 before) (tests/blitz-tests/tests/stand_act_spans.rs:542-554)»
new: «34 calls per mode (tests/blitz-tests/tests/stand_act_spans.rs:542-554)»
- line: .andromeda/test-plan.md:27
old: «`cause=fonts-unavailable`, the eighth cause; and the CRUD `advance` row»
new: «`cause=fonts-unavailable`, the eighth cause; it reads the eighth instance verb too, an accepted lint's line and a refused no-font lint's line, `cause=fonts-unavailable`, eight verbs read back; and the CRUD `advance` row»
- line: .andromeda/test-plan.md:27
old: «`stand_mcp_tools` 5 (4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description) — a scripted MCP session»
new: «`stand_mcp_tools` 6 — a `lint` tool call answering the reader's findings and parenting its `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:706-766); a scripted MCP session»
- line: .andromeda/test-plan.md:27
old: «`stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (in process only): a planted page»
new: «`stand_lint` 6, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers what the reader reads — 7 findings on the planted page, 0 on each lean task at boot (`the_lint_verb_answers_what_the_reader_reads`, tests/blitz-tests/tests/stand_lint.rs:308-330): a planted page»
- line: .andromeda/test-plan.md:103
old: «`screenshot()` and `screenshot_of(id)`, `act(session, call)`»
new: «`screenshot()` and `screenshot_of(id)`, `lint()`, `act(session, call)`»
- line: .andromeda/test-plan.md:103
old: «(`None` for a refused call, for a `snapshot` and, since that chunk, for a shot;»
new: «(`None` for a refused call, for a `snapshot`, for a shot and for a lint;»
- line: .andromeda/test-plan.md:103
old: «(tests/blitz-tests/tests/session_common/mod.rs:105-144), `refused(session, call)`»
new: «(tests/blitz-tests/tests/session_common/mod.rs:105-144), `linted(session, call)` (a lint's findings, `None` for a refused call or any other outcome), `refused(session, call)`»
- line: .andromeda/test-plan.md:103
old: «which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`;»
new: «which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused`, `slug`, `lint` and `linted`;»
- line: .andromeda/test-plan.md:119
old: «`run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
new: «`run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md; re-counted with the `lint` verb, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 192 · failed 0 · ignored 7 over 47 files — +2 passed (`stand_lint`'s and `stand_mcp_tools`' lint tests, no file added); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
- line: .andromeda/test-plan.md:126
old: «`escher-session mcp` (the eleven verbs)»
new: «`escher-session mcp` (the twelve verbs)»
- line: .andromeda/test-plan.md:126
old: «`tool` ∈ the eleven verbs»
new: «`tool` ∈ the twelve verbs»
- line: .andromeda/test-plan.md:127
old: «exactly the eleven `mcp__escher__{verb}` tools»
new: «exactly the twelve `mcp__escher__{verb}` tools»
- line: .andromeda/test-plan.md:130
old: «escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json and evidence/live-run.md; before it,»
new: «escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json and evidence/live-run.md; the pipe requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired; before it,»
- line: .andromeda/test-plan.md:132
old: «`run stand` 190 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
new: «`run stand` 192 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
- line: .andromeda/test-plan.md:146
old: «— eleven verbs in a stated order»
new: «— twelve verbs in a stated order»
- line: .andromeda/test-plan.md:146
old: «: the seven instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, then `start`»
new: «: the eight instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`, then `start`»
- line: .andromeda/test-plan.md:146
old: «`cli_commands` (12 since 2026-10-14-quality-gates, 11 since 2026-10-12-headless-screenshot, 10 since 2026-10-12-self-description, 8 until then)»
new: «`cli_commands` (13)»
- line: .andromeda/test-plan.md:151
old: «(`FAKE`, which speaks the eleven tools,»
new: «(`FAKE`, which speaks the twelve tools, answering `lint` with `{"findings":[]}` (.github/scripts/test_cold_agent.py:100-101),»
- line: .andromeda/test-plan.md:187
old: «examples/seven_guis/tests/mcp_log.rs:139-152). By level,»
new: «examples/seven_guis/tests/mcp_log.rs:139-152), and each also drives one `lint`, still reading 0 needles on stderr at `trace`, its count (2 each) unmoved. By level,»
- line: .andromeda/test-plan.md:204
old: «`cli_commands` (12, 11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description):»
new: «`cli_commands` (13):»
- line: .andromeda/test-plan.md:204
old: «each of the seven instance-level verbs is accepted and answers exactly its fields»
new: «each of the eight instance-level verbs is accepted and answers exactly its fields»
- line: .andromeda/test-plan.md:204
old: «equals the PNG `Session::run` returns in process (examples/seven_guis/tests/cli_commands.rs:291-363);»
new: «equals the PNG `Session::run` returns in process (examples/seven_guis/tests/cli_commands.rs:291-363); a hosted `lint` answers status `0` and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:364-385);»
- line: .andromeda/test-plan.md:205
old: «`mcp_server` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description):»
new: «`mcp_server` (6):»
- line: .andromeda/test-plan.md:205
old: «offering the verbs in order — eleven since 2026-10-12-headless-screenshot — `start`, `snapshot`, `click`)»
new: «offering the verbs in order — twelve — `start`, `snapshot`, `click`)»
- line: .andromeda/test-plan.md:205
old: «(examples/seven_guis/tests/mcp_server.rs:308-338), its protocol errors running nothing,»
new: «(examples/seven_guis/tests/mcp_server.rs:308-338), a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:340-358), its protocol errors running nothing,»
- line: .andromeda/test-plan.md:205
old: «`stand_mcp_tools` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description)»
new: «`stand_mcp_tools` (6, its sixth a `lint` tool call answering the reader's findings and parenting its `command` span; tests/blitz-tests/tests/stand_mcp_tools.rs:706-766)»
- line: .andromeda/test-plan.md:334
old: «+3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/ci.md»
new: «+3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/ci.md; re-read with the `lint` verb on CI#39387656399, the pre-CI commit `ea3153f34` (green 17/17): linux 204 · 1087 · 0 · 12; macos and coverage 185 · 1084 · 0 · 12; windows 185 · 1041 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 165 · 672 · 0 · 21 — no result line added, +4 passed on linux, macos and coverage over CI#39371468367 (`stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1, `mcp_server` 1), +3 on windows (`cli_commands` unix-only), +0 · +12 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md»
- line: .andromeda/test-plan.md:340
old: «as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`)»
new: «as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`); re-counted with the `lint` verb: 204 result lines, 1087 passed · 0 failed · 12 ignored, from 204 · 1083 — no result line added and +4 passed (`stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1, `mcp_server` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
- line: .andromeda/test-plan.md:353
old: «(§9 Legs; 46 before `stand_act_select`,»
new: «(§9 Legs; with the `lint` verb's four tests CI#39387656399 read 165 · 672 · 0 · 21, +0 · +12, no file added; 46 before `stand_act_select`,»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
old: «while `snapshot` and `screenshot` run no step and wait on nothing»
new: «while `snapshot`, `screenshot` and `lint` run no step and wait on nothing»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
old: «`run` records each answered call of the seven instance verbs —»
new: «`run` records each answered call of the eight instance verbs, a `lint` among them —»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
old: «is the whole command line, eleven verbs — the seven instance-level ones»
new: «is the whole command line, twelve verbs — the eight instance-level ones»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
old: «`png` padded base64, status `0`), then `start <task>»
new: «`png` padded base64, status `0`; `lint` the eighth, no argument, its accepted answer one line `{"findings":[…]}`, status `0`), then `start <task>»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «seven_guis `cli_commands` 12 (11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description) ·»
new: «seven_guis `cli_commands` 13 ·»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «seven_guis `mcp_server` 5 (4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description) ·»
new: «seven_guis `mcp_server` 6 ·»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «and over MCP an image block then a text block) · since 2026-10-12-self-description `stand_mcp_masked` 1»
new: «and over MCP an image block then a text block), and a sixth, a `lint` tool call answering the reader's findings and parenting its `command` span · since 2026-10-12-self-description `stand_mcp_masked` 1»
