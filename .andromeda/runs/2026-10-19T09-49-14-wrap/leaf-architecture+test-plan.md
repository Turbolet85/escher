group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:60   class: stale
  leaf-text: «the recorded session (3 tests, unix only, and 1 off-unix): the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`, a recording directory open to its group refused before anything runs, the `mcp` role's listing and `help` the same with and without `--record`, and off unix `--record` refused `Unsupported`»
  source: .andromeda/test-plan.md:10 «`tests/recording.rs` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay)»
  replace-with: «the recorded session (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay): the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`, a recording directory open to its group refused before anything runs, the `mcp` role's listing and `help` the same with and without `--record`, since 2026-10-19-recorded-check-replay the committed checks what the binary records — `tests/flows/record_checks.sh` run through the binary and both files compared byte for byte with `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` — and off unix `--record` refused `Unsupported`»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «a shot of one state byte-identical in two child runs and the parent;»
  source: .andromeda/test-plan.md:54 «the parent on one host, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577)»
  replace-with: «a shot of one state byte-identical in two child runs and the parent on one host — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step;»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_requirement_sweep --test stand_virtual_clock`»
  source: .andromeda/test-plan.md:52 «run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs»
  replace-with: «--test stand_requirement_sweep --test stand_virtual_clock --test stand_replay`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and two clicks with no `advance` between them a double click)»
  source: .andromeda/architecture.md:335 «the committed checks replayed — `stand_replay`, 3 tests in both layout modes, not unix-gated: each committed check holding on a fresh stand Counter, each red at its first activation step (1 for the command line's check, 3 for the MCP role's)»
  replace-with: «and two clicks with no `advance` between them a double click; since 2026-10-19-recorded-check-replay `stand_replay`, 3 tests, not unix-gated: the two committed checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` read through `Check::from_bytes` and replayed through `Check::replay` on a fresh stand Counter in both layout modes, each red at its first activation step on a test-local `Counter` variant that adds 2, and a check on a session of another app running nothing)»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «keeps the session's answered calls as one owner-only check in that directory»
  source: .andromeda/architecture.md:119 «each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels»
  replace-with: «keeps the session's answered calls as one owner-only check in that directory, each answer kept whole beside its digest (since 2026-10-19-recorded-check-replay; masked as on every carrier, a shot's without its pixels)»
