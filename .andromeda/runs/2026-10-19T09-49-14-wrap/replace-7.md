- line: .andromeda/test-plan.md:10
old: «`tests/recording.rs` (3 tests, unix only, and 1 off-unix) —»
new: «`tests/recording.rs` (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay) —»
- line: .andromeda/test-plan.md:10
old: «the `mcp` role's listing and `help` the same with and without `--record`; off unix, `--record` refused `Unsupported` —»
new: «the `mcp` role's listing and `help` the same with and without `--record`; since 2026-10-19-recorded-check-replay the committed checks what the binary records, `record_checks.sh` run through the binary and both files compared byte for byte with the committed copies (examples/seven_guis/tests/recording.rs:205-262); off unix, `--record` refused `Unsupported` —»
- line: .andromeda/test-plan.md:22
old: «87 unit tests in fourteen files, 67 inline and,»
new: «92 unit tests in fifteen files, 72 inline and,»
- line: .andromeda/test-plan.md:22
old: «(74 in thirteen until 2026-10-19-session-recording,»
new: «(87 in fourteen until 2026-10-19-recorded-check-replay, 74 in thirteen until 2026-10-19-session-recording,»
- line: .andromeda/test-plan.md:22
old: «a check in the grammar is read; one outside it is refused `malformed`, 35 rows; a check at its bounds is read and one past them refused) and, since 2026-10-12-headless-screenshot, image.rs 5»
new: «each record keeps its answer after the digest, a shot's without `png`; a check in the grammar is read, 8 rows (6 until 2026-10-19-recorded-check-replay added an object `answer`, a refusal's `answer` and keys out of order); one outside it is refused `malformed`, 40 rows (35 until then added `answer` missing, a string, a number, an array, `null` and an unknown key beside it); a check at its bounds is read and one past them refused), since 2026-10-19-recorded-check-replay replay.rs 5 (a check replayed on a fresh session holds; an acceptance that flips is a change naming the keys that differ; a changed answer names its keys and stops the replay; a shot holds at its size whatever its pixels; a check of another app runs nothing — packages/escher-driver/src/replay.rs:141-318) and, since 2026-10-12-headless-screenshot, image.rs 5»
- line: .andromeda/test-plan.md:27
old: «a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal and keeps no screen reading at rest;»
new: «a recorded session keeps each answer whole but a shot's image — every record ending with its kept line, no `png` and no shot's first 64 base64 characters — and replays through `Check::replay` on each lean task in both layout modes, each script ending `snapshot` · `screenshot`, so the shot path's kept form is covered on each unix OS CI runs (linux, macOS; the file is unix only, as recording is);»
- line: .andromeda/test-plan.md:27
old: «a recording directory open to its group is refused with nothing written (tests/blitz-tests/tests/stand_session_recording.rs:109-302)»
new: «a recording directory open to its group is refused with nothing written (tests/blitz-tests/tests/stand_session_recording.rs:109-302), and since 2026-10-19-recorded-check-replay the committed checks replayed — `stand_replay` 3, declaring `mod session_common;`, not unix-gated: the two committed Counter checks hold in both layout modes, turn red at step 1 (the command line's) and step 3 (the MCP role's) on a test-local `Counter` variant of the task's own name with every earlier step held and `changed` among the keys, a check on another app runs nothing, and neither check holds a `png` key or a shot's first 64 base64 characters (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:206-237; tests/blitz-tests/tests/stand_replay.rs:239-261)»
- line: .andromeda/test-plan.md:27
old: «green under the per-package and the workspace build (tests/blitz-tests/tests/stand_screenshot_repeat.rs:75-146)»
new: «green under the per-package and the workspace build, byte-identical on one host only — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a committed check holds no `screenshot` step (tests/blitz-tests/tests/stand_screenshot_repeat.rs:75-146)»
- line: .andromeda/test-plan.md:51
old: «are run under plain `sh` by the Rust target `cli_flow` —»
new: «are run under plain `sh` by the Rust target `cli_flow`, and since 2026-10-19-recorded-check-replay a third, `record_checks.sh` (the binary and an existing output directory its two arguments, `rec/` made `0700`, printing only `step N` on failure), by seven_guis `recording`'s unix-gated `the_committed_checks_are_what_the_binary_records` —»
- line: .andromeda/test-plan.md:53
old: «and the parent, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`),»
new: «and the parent on one host, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot —»
- line: .andromeda/test-plan.md:67
old: «by 24 checks (23 before `stand_session_recording.rs`,»
new: «by 25 checks (24 before `stand_replay.rs`, 23 before `stand_session_recording.rs`,»
- line: .andromeda/test-plan.md:67
old: «135 `.rs` files, 134 test targets and the unbuilt `all.rs` (134 and 133 until 2026-10-19-session-recording»
new: «136 `.rs` files, 135 test targets and the unbuilt `all.rs` (135 and 134 until 2026-10-19-recorded-check-replay added `stand_replay.rs`, its own auto-discovered target; 134 and 133 until 2026-10-19-session-recording»
- line: .andromeda/test-plan.md:67
old: «holding the two `sh` flow scripts (examples/seven_guis/tests/common/mod.rs:1-6)»
new: «holding the two `sh` flow scripts and, since 2026-10-19-recorded-check-replay, `record_checks.sh`, the committed checks' recording script, with `tests/blitz-tests/checks/` a data directory beside the blitz-tests targets that is no target either (examples/seven_guis/tests/common/mod.rs:1-6)»
- line: .andromeda/test-plan.md:102
old: «`stand_session_recording`, which reads the module's new `replay(session, check)` (each kept step run again, its acceptance and digest), `kept(check)` (each kept step's acceptance and digest) and the unix-only `mode(path)` (a file's permission bits) (tests/blitz-tests/tests/session_common/mod.rs:240-273) (24 readers; 23 before»
new: «`stand_session_recording`, which reads the module's `kept(check)` (each kept step's acceptance and digest), the unix-only `mode(path)` (a file's permission bits) and, since 2026-10-19-recorded-check-replay, `shot_prefix(session)` (the first 64 base64 characters of a whole-screen shot as the driver writes it) — its `replay(session, check)` removed then for the replay `Check::replay` — and `stand_replay`, which reads `shot_prefix` too (tests/blitz-tests/tests/session_common/mod.rs:240-250) (25 readers; 24 before `stand_replay`, 23 before»
- line: .andromeda/test-plan.md:144
old: «and `Check::from_bytes`'s strict grammar (35 refusal rows, edge rows at both bounds);»
new: «and `Check::from_bytes`'s strict grammar (40 refusal rows and 8 read rows, the record keeping its answer, a shot's without `png`, edge rows at both bounds); since 2026-10-19-recorded-check-replay replay.rs that a check replayed on a fresh session holds, that an acceptance flip and a changed answer each stop the replay naming their keys, that a shot holds at its size whatever its pixels and that a check of another app runs nothing (packages/escher-driver/src/replay.rs:141-318);»
- line: .andromeda/test-plan.md:202
old: «a misplaced `--record` a usage error, status `2` (examples/seven_guis/tests/recording.rs:264-304;»
new: «a misplaced `--record` a usage error, status `2`, and since 2026-10-19-recorded-check-replay `recording`'s `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` — a command-line session (`start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click counter-increment` · `snapshot` · `stop`) and an MCP one (`initialize` … `scroll counter-increment` · `snapshot` · `stop`) — through the binary and reads both kept checks byte-equal to `tests/blitz-tests/checks/counter-{cli,mcp}.json`, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS (CI#39225375577) (examples/seven_guis/tests/recording.rs:205-262; examples/seven_guis/tests/recording.rs:264-304;»
- line: .andromeda/test-plan.md:318
old: «— 43 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording —»
new: «— 44 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording and `stand_replay` since 2026-10-19-recorded-check-replay, `test_stand_repeat_leg_runs_three_rounds` naming it (.github/scripts/test_ci_workflows.py:359) —»
- line: .andromeda/test-plan.md:327
old: «`stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build,»
new: «`stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build on one host — the shot's bytes differ across OSes (CI#39225375577, macOS aarch64 against linux x86_64) —,»
- line: .andromeda/test-plan.md:347
old: «run the 43 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 42 and seven until 2026-10-19-session-recording,»
new: «run the 44 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 43 until 2026-10-19-recorded-check-replay, whose local run read `stand-repeat: 3 of 3 rounds green` and whose CI `Stand checks repeated` ran `stand_replay` 9 times on CI#39227884248; 42 and seven until 2026-10-19-session-recording,»
