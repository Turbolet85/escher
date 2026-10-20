group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «the driver's own `escher-session mcp` with its eleven verbs»
  source: .andromeda/test-plan.md:126 «the driver's own MCP surface, `escher-session mcp` (the twelve verbs), whose stderr under escher's sink at `RUST_LOG=info` lands in `server.log`»
  replace-with: «the driver's own `escher-session mcp` with its twelve verbs»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «`isolated` — exactly the eleven `mcp__escher__*` tools»
  source: .andromeda/test-plan.md:127 «`isolated` = the init tools exactly the twelve `mcp__escher__{verb}` tools, one MCP server `escher` connected»
  replace-with: «`isolated` — exactly the twelve `mcp__escher__*` tools»

- leaf: .claude/docs/tests-summary.md:16   class: omission
  leaf-text: «The live run is never in CI.»
  source: .andromeda/test-plan.md:130 «the pipe requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired»
  replace-with: «The live run is never in CI; the pipe now requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired.»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 190 · 0 · 7 at 2026-10-20-select-by-id»
  source: .andromeda/test-plan.md:132 «`run stand` 192 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
  replace-with: «`run stand` 192 · 0 · 7 with the `lint` verb at 2026-10-19-visual-lint's gated part»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_lint` (5, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on the in-process `Session::lint` — in process only:»
  source: .andromeda/test-plan.md:27 «`stand_lint` 6, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers»
  replace-with: «`stand_lint` (6, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers what the reader reads — 7 findings on the planted page, 0 on each lean task at boot:»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the eleven verbs at their levels — `screenshot` the seventh instance verb, after `scroll`, since 2026-10-12-headless-screenshot, `help` at the tool level since 2026-10-12-self-description»
  source: .andromeda/test-plan.md:22 «eight instance-level (`screenshot` the seventh, after `scroll`, and `lint` the eighth, after `screenshot`), then `start` · `status` · `stop`, then, since 2026-10-12-self-description, `help` at the tool»
  replace-with: «the twelve verbs at their levels — `screenshot` the seventh instance verb, after `scroll`, since 2026-10-12-headless-screenshot, `lint` the eighth, after `screenshot`, `help` at the tool level since 2026-10-12-self-description»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «and session answer, a changed node with `differs` after `bounds`)»
  source: .andromeda/test-plan.md:22 «a lint's outcome as `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` then its reason's keys, every reason pinned and a ratio that is not finite written `null`»
  replace-with: «and session answer, a changed node with `differs` after `bounds`, a lint's outcome as `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` then its reason's keys, a ratio that is not finite written `null`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the listing as the eleven verbs, ten until 2026-10-12-headless-screenshot, nine until 2026-10-12-self-description»
  source: .andromeda/test-plan.md:22 «the listing is the twelve verbs of the table; the session verbs are answered by the server»
  replace-with: «the listing as the twelve verbs»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`cli_commands` (12, 11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description —»
  source: .andromeda/test-plan.md:204 «a hosted `lint` answers status `0` and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:364-385)»
  replace-with: «`cli_commands` (13 — a hosted `lint` answering status `0` and one line `{"findings":[]}`;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_server` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description:»
  source: .andromeda/test-plan.md:205 «a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:340-358), its protocol errors running nothing»
  replace-with: «`mcp_server` (6: a `lint` tool call answering its line of findings;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`tools/list` offering the eleven verbs»
  source: .andromeda/test-plan.md:205 «`mcp_server` (6): the Counter flow by tool calls alone (`initialize`, `tools/list` offering the verbs in order — twelve — `start`, `snapshot`, `click`)»
  replace-with: «`tools/list` offering the twelve verbs»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`stand_mcp_tools` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description:»
  source: .andromeda/test-plan.md:205 «a `lint` tool call answering the reader's findings and parenting its `command` span; tests/blitz-tests/tests/stand_mcp_tools.rs:706-766)»
  replace-with: «`stand_mcp_tools` (6: a `lint` tool call answering the reader's findings and parenting its `command` span;»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds;»
  source: .andromeda/test-plan.md:187 «and each also drives one `lint`, still reading 0 needles on stderr at `trace`, its count (2 each) unmoved.»
  replace-with: «also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds, and each also drives one `lint`, still reading 0 needles on stderr at `trace`, its count (2 each) unmoved;»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «no id, name or typed text, and no driver line at the default level;»
  source: .andromeda/test-plan.md:27 «it reads the eighth instance verb too, an accepted lint's line and a refused no-font lint's line, `cause=fonts-unavailable`, eight verbs read back»
  replace-with: «no id, name or typed text, and no driver line at the default level; the eighth instance verb too, an accepted lint's line and a refused no-font lint's line, `cause=fonts-unavailable`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1083 · 0 · 12 over 204 result lines at 2026-10-20-select-by-id (from 202 · 1062:»
  source: .andromeda/test-plan.md:340 «re-counted with the `lint` verb: 204 result lines, 1087 passed · 0 failed · 12 ignored, from 204 · 1083 — no result line added and +4 passed (`stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1»
  replace-with: «`cargo test --workspace` 1087 · 0 · 12 over 204 result lines with the `lint` verb at 2026-10-19-visual-lint's gated part (from 204 · 1083: no result line added and +4 passed — `stand_lint` 1, `stand_mcp_tools` 1, `cli_commands` 1, `mcp_server` 1; `run stand` 192 passed · 0 failed · 7 ignored over 47 files, `run all` not re-counted; timings not re-measured); 1083 · 0 · 12 over 204 result lines at 2026-10-20-select-by-id (from 202 · 1062:»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «(46 before `stand_act_select`, CI#39371468367's»
  source: .andromeda/test-plan.md:353 «with the `lint` verb's four tests CI#39387656399 read 165 · 672 · 0 · 21, +0 · +12, no file added; 46 before `stand_act_select`, whose CI `Stand checks repeated` on CI#39371468367 read 165 · 660»
  replace-with: «(with the `lint` verb's four tests CI#39387656399's `Stand checks repeated` reading 165 · 672 · 0 · 21, no file added; 46 before `stand_act_select`, CI#39371468367's»
