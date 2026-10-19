group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «holding the calls and no screen reading (ratified by the founder, 2026-10-19)»
  source: .andromeda/layout-templates.md:73 «holding per call its verb, arguments, acceptance, the answer's digest and, since 2026-10-19-recorded-check-replay, the answer itself as an `answer` key»
  replace-with: «holding per call its verb, arguments, acceptance, the answer's digest and, since 2026-10-19-recorded-check-replay, the answer itself as an `answer` key — the line the driver wrote, masked as on every carrier, a shot's kept as `{"width":…,"height":…}` without its `png` — read back and replayed by `Check::replay` (ratified by the founder, 2026-10-19: F1 A · F2 B, F3 B)»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`tests/recording.rs` spawns both the host role and the `mcp` role, with `--record`.»
  source: .andromeda/architecture.md:222 «and `recording` does both, with `--record`, and runs `tests/flows/record_checks.sh` under `sh` into `rc-fresh`, both streams drained, a `Started` guard and the bounded run's 90 s, comparing both recorded files byte for byte»
  replace-with: «`tests/recording.rs` spawns both the host role and the `mcp` role, with `--record`, and runs `tests/flows/record_checks.sh` — the third POSIX `sh` script of `tests/flows/`, no test target — under `sh` into `rc-fresh` (under `CARGO_TARGET_TMPDIR`), comparing both recorded files byte for byte with the two committed checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` (git mode `100644`, the one exception to a kept check's owner-only medium; no `screenshot` step, a shot's PNG bytes differing by OS).»

- leaf: .claude/docs/services/seven_guis.md:20   class: stale
  leaf-text: «`TaskShell` stays private.»
  source: .andromeda/architecture.md:180 «and `pub fn TaskShell`, the shell a test-local variant of a task mounts in so its unkeyed ids read as the task's»
  replace-with: «`pub fn TaskShell` — the shell a test-local variant of a task mounts in so its unkeyed ids read as the task's.»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «and its open dialog read keyed.»
  source: .andromeda/architecture.md:335 «the committed checks replayed — `stand_replay`, 3 tests in both layout modes, not unix-gated: each committed check holding on a fresh stand Counter, each red at its first activation step»
  replace-with: «and its open dialog read keyed. Since 2026-10-19-recorded-check-replay `stand_replay` (3 tests, both layout modes, not unix-gated; a reader of `tests/session_common/mod.rs`) replays the two committed checks under `tests/blitz-tests/checks/` through `Check::replay`: each holding on a fresh stand Counter, each red at its first activation step (1 for the command line's check, 3 for the MCP role's) on a test-local variant `Counter` that adds 2, mounted in `TaskShell`, and a check on a session of another app running nothing.»
