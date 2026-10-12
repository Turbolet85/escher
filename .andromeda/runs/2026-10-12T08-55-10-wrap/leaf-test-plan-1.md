group: test-plan-1
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 59 inline unit tests in eleven files (50 in ten until 2026-10-10-mcp-surface, 29 in seven until 2026-10-10-driver-cli)»
  source: .andromeda/test-plan.md:21 «66 inline unit tests in twelve files (59 in eleven until 2026-10-12-self-description»
  replace-with: «`escher-driver`: 66 inline unit tests in twelve files (59 in eleven until 2026-10-12-self-description, 50 in ten until 2026-10-10-mcp-surface, 29 in seven until 2026-10-10-driver-cli)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`error.rs` 3 (thirteen `SessionError` messages, `NotSettled`'s three included)»
  source: .andromeda/test-plan.md:21 «since 2026-10-12-self-description the thirteen errors each state a kind, a meaning and a remedy, and the state directory's message states its bound»
  replace-with: «`error.rs` 5 (thirteen `SessionError` messages, `NotSettled`'s three included; since 2026-10-12-self-description each error states a kind, a meaning and a remedy, and the state directory's message states its bound)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`schema.rs` 4 (the nine verbs at their two levels, their shapes, the lookup, the kinds, bounds and key names)»
  source: .andromeda/test-plan.md:21 «schema.rs 6 (the verbs are named in order, each at its level — six instance-level, then `start` · `status` · `stop`, then, since 2026-10-12-self-description, `help` at the tool level, ten;»
  replace-with: «`schema.rs` 6 (the ten verbs at their levels — `help` at the tool level since 2026-10-12-self-description — their shapes, the lookup, the kinds, bounds and key names, the thirteen node keys in order each with its help, the four exit statuses in status order)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «each outcome, refusal, session error and session answer)»
  source: .andromeda/test-plan.md:21 «each session error with its kind and its message and, since 2026-10-12-self-description, its remedy;»
  replace-with: «each outcome, refusal, session error — with its remedy since 2026-10-12-self-description — and session answer, a changed node with `differs` after `bounds`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp.rs` 9 (each line read by its rules — the byte bound, JSON, the envelope; a line read whole across reads; the id echoed only in its two forms; each method's reply; each argument value's kind or its refusal; the listing as the nine verbs; the session verbs answered by the server;»
  source: .andromeda/test-plan.md:21 «the listing is the ten verbs of the table — nine until 2026-10-12-self-description;»
  replace-with: «`mcp.rs` 10 (each line read by its rules — the byte bound, JSON, the envelope; a line read whole across reads; the id echoed only in its two forms; each method's reply; each argument value's kind or its refusal; the listing as the ten verbs, nine until 2026-10-12-self-description; the session verbs answered by the server; since that chunk `help` answered by the server with or without a session;»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «a result over the answer bound replaced); `host.rs` holds none.»
  source: .andromeda/test-plan.md:21 «describe.rs 2 (the document holds `help`'s seven fields in order; every text in it is its tables');»
  replace-with: «a result over the answer bound replaced) and, since 2026-10-12-self-description, `describe.rs` 2 (the document holds `help`'s seven fields in order; every text in it is its tables'); `host.rs` holds none.»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «seven_guis' `cli_commands` (8)»
  source: .andromeda/test-plan.md:200 «`cli_commands` (10, 8 until 2026-10-12-self-description):»
  replace-with: «seven_guis' `cli_commands` (10, 8 until 2026-10-12-self-description — since then `help` answering `describe()`'s line and reaching no session, every provoked cause's `remedy` equal to its cause's, and a 94-byte state directory starting while one of 95 ends `state-dir-too-long`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_server` (2: the Counter flow by tool calls alone through the binary's `mcp` role, protocol errors running nothing and leaking nothing; unix only, a snapshot equal through MCP and the command line)»
  source: .andromeda/test-plan.md:201 «`mcp_server` (4, 2 until 2026-10-12-self-description):»
  replace-with: «`mcp_server` (4, 2 until 2026-10-12-self-description: the Counter flow by tool calls alone through the binary's `mcp` role, `tools/list` offering the ten verbs, protocol errors running nothing and leaking nothing; unix only, a snapshot equal through MCP and the command line; since that chunk the newest revision `2025-11-25` spoken and `help` answered before any `start`, and, unix only, `help` the same bytes through MCP and the command line)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «in process by `stand_mcp_tools` (2: the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`; a held Timer's step through `start`)»
  source: .andromeda/test-plan.md:26 «`stand_mcp_tools` 4 (2 until 2026-10-12-self-description)»
  replace-with: «in process by `stand_mcp_tools` (4, 2 until 2026-10-12-self-description: the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`; a held Timer's step through `start`; since that chunk every cause a tool call reaches served with its remedy, and `help` answered by the server, its `tool_call` span parenting no `command`) and, since that chunk, `stand_mcp_masked` (1: a password typed over MCP reads `MASKED_VALUE` and never the typed text, both layout modes)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the two seven_guis targets not unix-gated but the comparison, run by the windows job too»
  source: .andromeda/test-plan.md:49 «not unix-gated but `mcp_server`'s command-line comparisons, two since 2026-10-12-self-description»
  replace-with: «the two seven_guis targets not unix-gated but `mcp_server`'s two command-line comparisons, run by the windows job too»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «equal across the two layout modes, with a hidden element, a masked password and a rewritten label on in-file fixtures —»
  source: .andromeda/test-plan.md:26 «`stand_diff` gains the Flight Booker's Book naming eight nodes that only moved, each `differs` = `[bounds]` in both modes»
  replace-with: «equal across the two layout modes, with a hidden element, a masked password and a rewritten label on in-file fixtures, and, since 2026-10-12-self-description, each changed node's `differs` checked — the Flight Booker's Book naming eight nodes that only moved, each `differs` = `[bounds]` —»

- leaf: .claude/docs/tests-summary.md:17   class: omission
  leaf-text: «`tool` and `ending` only, the parent of the call's `command` line —»
  source: .andromeda/test-plan.md:103 «a `help` call, answered by the server, writes one `tool_call` line, `tool="help" ending="accepted"`, with no `command` line beneath it»
  replace-with: «`tool` and `ending` only, the parent of the call's `command` line, and since 2026-10-12-self-description a `help` call, answered by the server, one `tool_call` line with no `command` line beneath it —»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «(70 CI-script tests)»
  source: .andromeda/test-plan.md:8 «78 tests in six test files»
  replace-with: «(78 CI-script tests in six files)»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 805 · 0 · 11 over 170 result lines at 2026-10-10-mcp-surface»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-12-self-description: 171 result lines, 821 passed · 0 failed · 11 ignored»
  replace-with: «`cargo test --workspace` 821 · 0 · 11 over 171 result lines at 2026-10-12-self-description (+16 passed: one new result line — `stand_mcp_masked` 1 — escher-driver +7, now 66, dioxus-native-dom +1, `stand_mcp_tools` +2, `stand_diff` +1, seven_guis' `cli_commands` +2 and `mcp_server` +2; `run stand` 131 passed · 6 ignored over 34 files, `run all` not re-read); 805 · 0 · 11 over 170 result lines at 2026-10-10-mcp-surface»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «the session checks, `stand_settle` and the twelve driver-action checks `stand_act_*` —»
  source: .andromeda/test-plan.md:101 «so do the twelve driver-action checks `stand_act_*` and, since 2026-10-10-mcp-surface, `stand_mcp_tools` (18 readers»
  replace-with: «the session checks, `stand_settle`, the twelve driver-action checks `stand_act_*` and `stand_mcp_tools` (18 readers; `stand_mcp_masked` declares none) —»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «is not unix-gated but `mcp_server`'s command-line comparison;»
  source: .andromeda/test-plan.md:49 «not unix-gated but `mcp_server`'s command-line comparisons, two since 2026-10-12-self-description»
  replace-with: «is not unix-gated but `mcp_server`'s two command-line comparisons;»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`status` each run one command against it, and `stop` ends it;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «`help` takes no argument and at most one `--session <dir>`, not read, and answers `describe()`'s line on stdout, status `0`, reaching no session»
  replace-with: «`status` each run one command against it, `stop` ends it, and `help` (since 2026-10-12-self-description; no argument, at most one `--session <dir>`, not read) answers `describe()`'s line on stdout, status `0`, reaching no session;»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`<dir>` must be short, since a socket address has about a hundred bytes for the whole path»
  source: .andromeda/test-plan.md:200 «a state directory of 94 bytes as given starts and stops while one of 95 ends `state-dir-too-long`, status `3`»
  replace-with: «`<dir>` must be short, since a socket address has about a hundred bytes for the whole path — a state directory of 94 bytes as given starts, one of 95 ends `state-dir-too-long`, status `3`»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «its tools the nine driver verbs»
  source: .andromeda/test-plan.md:201 «`tools/list` offering the verbs in order — ten since that chunk —»
  replace-with: «its tools the ten driver verbs (`help` since 2026-10-12-self-description, answered before any `start`)»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «its one standing check is the driver's `press backspace` on the macOS CI leg»
  source: .andromeda/test-plan.md:91 «since 2026-10-10-driver-cli a second standing check reaches the same path there»
  replace-with: «exercised on the macOS CI leg only, by the driver's `press backspace` in `stand_act_keys` and, since 2026-10-10-driver-cli, by `stand_act_filled`'s clearing test (a `type` with an empty text deleting through the driver's own Backspace press)»
