# P5 review — 2026-10-19-test-splits · «Test splits» (Epoch 4)

Plan: `escher-0.2.0/chunks/2026-10-19-test-splits/plan.md` (415 lines, 26 gates, 12 criteria) · research and scope
beside it · validation-1: **aligned**. Mechanical checks pass; planlint's one HIT (check 4 (13), the per-OS tallies
job-log read) is accepted, because its note names the read and carries its control fired on CI#39312031677.

## Goal
A test-only close of the Epoch 3 audit's M2 and the `host_log` ↔ `mcp_log` half of its M1. No production line, wire,
verb, log field, env read, dependency or target moves.
- **escher-driver** — `cli.rs` and `recording.rs` move their inline test modules (`tests` and `off_unix_tests`) out to
  `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs`. They use the `#[path]` seam
  2026-10-16-file-splits used, so the test paths are unchanged. The files drop from 1,032 and 981 lines to 478 and 459
  (measured in a scratch clone).
- **seven_guis** — the six fragments `host_log` and `mcp_log` share are stated once in `tests/common/mod.rs`:
  `recorded_needles`, the stand-needle read and its asserts, the stderr stamp check, and the needle counter. jscpd
  measures them at 6 clones and 95 lines today. Every needle class, level, build and exact span count stays asserted.

## Top acceptance criteria
- `cargo test -p escher-driver` reads 92 passed, and its `--list` is byte-identical to the record step 1 takes.
- The off-unix side checks for `x86_64-pc-windows-gnu`.
- The three moved bodies are token-equal to their base spans, and the production lines (`cli.rs` 1-475,
  `recording.rs` 1-452) are byte-identical.
- jscpd reads 0 clones among `host_log.rs`, `mcp_log.rs` and `common/mod.rs`.
- `host_log` and `mcp_log` keep their 4 test names and pass under both builds. The workspace tally stays at
  `lines 199 passed 1045 failed 0 ignored 12`.
- The silence census reads 0, the env-read set is unchanged, and no file outside the eight moves.
- CI is green on the pushed sha, the coverage floors read 3 of 3 and the stand-repeat leg is green.

## Touchpoints
- New: `packages/escher-driver/src/{cli_tests,recording_tests,recording_off_unix_tests}.rs`.
- Modified: `packages/escher-driver/src/{cli,recording}.rs` and `examples/seven_guis/tests/{host_log,mcp_log,common/mod}.rs`.

## Test commands (26 entries)
- The two crates' tests and their `--list` comparisons.
- The windows check.
- Eight guards over the move: inline modules gone, `#[path]` lines, the production head, three token comparisons and
  the size.
- jscpd, the env-read set, the silence census, the span count, and the tracked and untracked file guards.
- `ci-leg.sh fast` with the workspace tally and the log-target read, and `ci-leg.sh doc`.
- The operator pass: hygiene, push, the CI read, and the per-OS tallies.

Every new entry is baselined. Guards were checked with must-pass and must-fail controls in a scratch clone with the
move applied (`.andromeda/runs/2026-10-19T20-32-57-phase/p5-controls.md`).

## Capabilities
None claimed — concretization preview: n/a. The unclaimed six (v020-05 · 07 · 08 · 09 · 10 · 11) each name a surface
this chunk does not build.

## Leans taken (decisive, per artifact)
- `off_unix_tests` moves out too, into its own file, keeping its path. This follows the working entry ("inline test
  modules out of … `recording.rs`"). Nesting it in `recording_tests.rs` would rename its path.
- The fragment goes into `tests/common/mod.rs`, per test-plan §2 Directory pattern (the crate's one shared module).
- No playbook escalate pattern applies: no boundary widens and no PROVISIONAL mark is touched.

## Expected amendments (wrap)
- test-plan §1, §2, §4 and the §3 Session lifecycle key file: the inline/`#[path]` split goes from 72/20 to 58/34, and
  out-of-line modules from five to eight.
- architecture §Conventions → Tests and the escher-driver row.
- Line-citation re-points in security-plan, obs-plan and test-plan.
- No layout-templates or design-system change is expected.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | The audit's three other directions stay unowned audit directions for the founder's judgment. None is on the working entry, and this chunk does not take them: M1's reuse of `command.rs`/`wire.rs` readers in the recording module's field parsing, M1's `stand_replay` Counter variant vs `counter.rs`, and M2's per-verb helpers for `cli.rs::read`. | deferral (no owner proposed) |

Apply? (yes / review / cancel)
