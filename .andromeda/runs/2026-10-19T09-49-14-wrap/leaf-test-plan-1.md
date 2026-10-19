group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 59 · «- 2026-10-19: Code compiled only off unix is unverified on t»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 87 unit tests in fourteen files, 67 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs (74 in thirteen until 2026-10-19-session-recording»
  source: .andromeda/test-plan.md:22 «**escher-driver:** 92 unit tests in fifteen files, 72 inline»
  replace-with: «`escher-driver`: 92 unit tests in fifteen files, 72 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs (87 in fourteen until 2026-10-19-recorded-check-replay, 74 in thirteen until 2026-10-19-session-recording»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «a check in the grammar read, one outside it refused `malformed`, 35 rows; a check at its bounds read and one past them refused)»
  source: .andromeda/test-plan.md:22 «a check in the grammar is read, 8 rows (6 until 2026-10-19-recorded-check-replay added an object `answer`»
  replace-with: «each record keeping its answer after the digest, a shot's without `png`; a check in the grammar read, 8 rows (6 until 2026-10-19-recorded-check-replay), one outside it refused `malformed`, 40 rows (35 until then); a check at its bounds read and one past them refused), since 2026-10-19-recorded-check-replay `replay.rs` 5 (a check replayed on a fresh session holds; an acceptance that flips is a change naming the keys that differ; a changed answer names its keys and stops the replay; a shot holds at its size whatever its pixels; a check of another app runs nothing)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «a recorded session replaying on each lean task in both layout modes with every step's acceptance and digest equal and keeping no screen reading at rest»
  source: .andromeda/test-plan.md:27 «a recorded session keeps each answer whole but a shot's image»
  replace-with: «a recorded session keeping each answer whole but a shot's image — every record ending with its kept line, no `png` and no shot's first 64 base64 characters — and replaying through `Check::replay` on each lean task in both layout modes, each script ending `snapshot` · `screenshot`, so the shot path's kept form is covered on each unix OS CI runs (linux, macOS)»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «and a recording directory open to its group refused with nothing written); seven_guis' `host_binary` (2)»
  source: .andromeda/test-plan.md:27 «since 2026-10-19-recorded-check-replay the committed checks replayed — `stand_replay` 3, declaring `mod session_common;`, not unix-gated»
  replace-with: «and a recording directory open to its group refused with nothing written) and, since 2026-10-19-recorded-check-replay, `stand_replay` (3, not unix-gated: the two committed Counter checks under `tests/blitz-tests/checks/` holding in both layout modes, red at step 1 (the command line's) and step 3 (the MCP role's) on a test-local `Counter` variant of the task's own name with every earlier step held and `changed` among the keys, a check on another app running nothing, neither check holding a `png` key or a shot's first 64 base64 characters); seven_guis' `host_binary` (2)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`recording` (3, unix only, and 1 off-unix: the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; off unix `--record` refused `Unsupported`)»
  source: .andromeda/test-plan.md:10 «`tests/recording.rs` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay)»
  replace-with: «`recording` (4, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay: the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; since 2026-10-19-recorded-check-replay the committed checks what the binary records, `record_checks.sh` run through the binary and both files compared byte for byte with the committed copies; off unix `--record` refused `Unsupported`)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «two children and the parent printing byte-identical PNGs for 12 shots, both layout modes and both builds)»
  source: .andromeda/test-plan.md:27 «byte-identical on one host only — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a committed check holds no `screenshot` step»
  replace-with: «two children and the parent printing byte-identical PNGs for 12 shots, both layout modes and both builds, byte-identical on one host only — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a committed check holds no `screenshot` step)»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «byte-identical across two child runs and the parent (`stand_screenshot_repeat`)»
  source: .andromeda/test-plan.md:54 «and the parent on one host, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`) — a shot's PNG bytes differ»
  replace-with: «byte-identical across two child runs and the parent on one host (`stand_screenshot_repeat`) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot —»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 43 `stand_*.rs` files and seven_guis' eight process checks (42 and seven until 2026-10-19-session-recording added `stand_session_recording` and `recording`)»
  source: .andromeda/test-plan.md:320 «— 44 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording and `stand_replay` since 2026-10-19-recorded-check-replay»
  replace-with: «three rounds of the 44 `stand_*.rs` files and seven_guis' eight process checks (43 until 2026-10-19-recorded-check-replay added `stand_replay`; 42 and seven until 2026-10-19-session-recording added `stand_session_recording` and `recording`)»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 43 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (42 and seven until 2026-10-19-session-recording)»
  source: .andromeda/test-plan.md:349 «run the 44 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 43 until 2026-10-19-recorded-check-replay»
  replace-with: «the 44 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (43 until 2026-10-19-recorded-check-replay; 42 and seven until 2026-10-19-session-recording)»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «`stand_virtual_clock` and, since 2026-10-19-session-recording, `stand_session_recording` (24 readers;»
  source: .andromeda/test-plan.md:68 «`tests/session_common/mod.rs`, read through `mod session_common;` by 25 checks (24 before `stand_replay.rs`»
  replace-with: «`stand_virtual_clock`, since 2026-10-19-session-recording `stand_session_recording` and, since 2026-10-19-recorded-check-replay, `stand_replay` (25 readers;»

- leaf: .claude/rules/testing.md:15   class: retired
  leaf-text: «and since 2026-10-19-session-recording the recording readers `replay` · `kept` and the unix-only `mode`)»
  source: .andromeda/test-plan.md:103 «`shot_prefix(session)` (the first 64 base64 characters of a whole-screen shot as the driver writes it) — its `replay(session, check)` removed then for the replay `Check::replay`»
  replace-with: «since 2026-10-19-session-recording the recording readers `kept` and the unix-only `mode`, and since 2026-10-19-recorded-check-replay `shot_prefix` (the first 64 base64 characters of a whole-screen shot) — its `replay` removed then for the replay `Check::replay`)»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «the two flows under `tests/flows/` are POSIX `sh` scripts run by the Rust target `cli_flow`, not a runner of their own.»
  source: .andromeda/test-plan.md:51 «and since 2026-10-19-recorded-check-replay a third, `record_checks.sh`»
  replace-with: «the two flows under `tests/flows/` are POSIX `sh` scripts run by the Rust target `cli_flow` and, since 2026-10-19-recorded-check-replay, a third, `record_checks.sh`, by seven_guis `recording`'s unix-gated `the_committed_checks_are_what_the_binary_records` — assets of a `cargo test` target, not a runner of their own.»

- leaf: .claude/rules/testing.md:35   class: omission
  leaf-text: «- Blitz-dom unit tests build DOM by hand through `DocumentMutator` — blitz-html would be a circular dev-dependency.»
  source: .andromeda/test-plan.md:52 «**Recorded-check replay (since 2026-10-19-recorded-check-replay):** a check a session recorded, committed as data under `tests/blitz-tests/checks/`»
  replace-with: «- Blitz-dom unit tests build DOM by hand through `DocumentMutator` — blitz-html would be a circular dev-dependency. Since 2026-10-19-recorded-check-replay two recorded Counter sessions are committed as data under `tests/blitz-tests/checks/` (`counter-cli.json`, `counter-mcp.json`, no target), embedded as bytes by `stand_replay`, which replays them through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs — and kept fresh by seven_guis `recording`'s byte-for-byte comparison with what `record_checks.sh` records through the binary; they hold no `screenshot` step, a shot's PNG bytes differing by OS.»
