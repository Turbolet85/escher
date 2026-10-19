group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:14   class: omission
  leaf-text: «`host_timer` · `mcp_server` · `mcp_log`, the first red ending the leg»
  source: .andromeda/test-plan.md:318 «then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and, since that chunk, `recording`»
  replace-with: «`host_timer` · `mcp_server` · `mcp_log` · `recording` (since 2026-10-19-session-recording), the first red ending the leg»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`start <counter|flight-booker|timer|crud>` spawns a host for the session in `<dir>`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «since 2026-10-19-session-recording `--record <dir>` is taken at most once on `start`, the host role and the `mcp` role, and anywhere else, twice or with no value it is usage, status `2`»
  replace-with: «`start <counter|flight-booker|timer|crud> [--record <dir>]` spawns a host for the session in `<dir>` (since 2026-10-19-session-recording `--record <dir>` keeps the session's answered calls as one owner-only check in that directory, checked before the spawn — a refusal status `3` with nothing spawned — and passed to the host; taken at most once on `start`, the host role and the `mcp` role, and anywhere else, twice or with no value a usage error, status `2`)»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «the host — `serve <task> --session <dir>`, what `start` spawns —»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «the host role is `serve <task> --session <dir> [--record <dir>]`»
  replace-with: «the host — `serve <task> --session <dir> [--record <dir>]`, what `start` spawns —»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «`mcp` with anything after it is a usage error (exit `2`)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «the `mcp` role is `mcp` alone or, since 2026-10-19-session-recording, `mcp --record <dir>` (anything else after it usage)»
  replace-with: «`mcp` with anything after it but one `--record <dir>` is a usage error (exit `2`); `mcp --record <dir>` (since 2026-10-19-session-recording) keeps each session it starts as one check in that directory, checked before it serves — a refusal status `3`, its fixed message on stderr and stdout empty»

- leaf: .claude/docs/commands.md:60   class: omission
  leaf-text: «· `cargo test -p seven_guis --locked --test host_log` — the driver crate's unit tests (the session's, the command schema's and the executor's key table; `--lib refusal::` · `schema::` · `command::` · `execute::` select one module) · the host binary's own smoke · the host's stderr at `RUST_LOG=trace` (no id, no name)»
  source: .andromeda/test-plan.md:10 «since 2026-10-19-session-recording `tests/recording.rs` (3 tests, unix only, and 1 off-unix) — the command line and the `mcp` role keep the same check byte for byte»
  replace-with: «· `cargo test -p seven_guis --locked --test host_log` · `cargo test -p seven_guis --locked --test recording` — the driver crate's unit tests (the session's, the command schema's and the executor's key table; `--lib refusal::` · `schema::` · `command::` · `execute::` select one module) · the host binary's own smoke · the host's stderr at `RUST_LOG=trace` (no id, no name) · since 2026-10-19-session-recording the recorded session (3 tests, unix only, and 1 off-unix): the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`, a recording directory open to its group refused before anything runs, the `mcp` role's listing and `help` the same with and without `--record`, and off unix `--record` refused `Unsupported`»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «and a document built with no clock fading by the wall clock (not selected by `run stand`)»
  source: .andromeda/test-plan.md:27 «`virtual_clock_engine` 5 (4 until 2026-10-19-session-recording added a finger pan and its fling moving only with the clock, 3 cases in both layout modes»
  replace-with: «a finger pan and its fling moving only with the clock (since 2026-10-19-session-recording), and a document built with no clock fading by the wall clock (not selected by `run stand`)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «(4 tests, both layout modes: the scrollbar fade moving only with the clock, two clicks a double click by the clock alone — its document driven through `Harness::wrap` — a smooth scroll standing still until the clock moves, and a document built with no clock fading by the wall clock)»
  source: .andromeda/test-plan.md:27 «`virtual_clock_engine` 5 (4 until 2026-10-19-session-recording added a finger pan and its fling moving only with the clock, 3 cases in both layout modes»
  replace-with: «(5 tests — 4 until 2026-10-19-session-recording — both layout modes: the scrollbar fade moving only with the clock, two clicks a double click by the clock alone — its document driven through `Harness::wrap` — a smooth scroll standing still until the clock moves, a finger pan and its fling moving only with the clock, and a document built with no clock fading by the wall clock)»
