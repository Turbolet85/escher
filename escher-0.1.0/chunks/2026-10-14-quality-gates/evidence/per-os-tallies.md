# Per-OS test tallies — CARRY 8, attributed

Source: CI#38649787488 on `eb5ba90c` (green, 16/16), the job logs of `Test [default features]` (linux),
`Test (macos)`, `Test (windows)`, `Coverage report`, `Test (ios)` and `Test (android)`, each read whole
through `gh run view -R Turbolet85/escher --job {id} --log` and tallied per test binary by a stdlib parser
(`$TMPDIR/{scratchpad}/tally.py`: the escaped colour codes stripped, each `test result:` line attributed to
the `Running … deps/{binary}-{hash}` or `Doc-tests {crate}` line before it). Re-derived at implement,
2026-10-15; the figures equal research.md's Readings.

## Totals (result lines · passed · failed · ignored)

| job | result lines | passed | failed | ignored | what it runs |
|---|---|---|---|---|---|
| linux (`Test [default features]`) | 185 | 951 | 0 | 12 | `cargo test --workspace --locked` — doctests included |
| macos (`Test (macos)`) | 166 | 948 | 0 | 12 | `cargo test --all --tests` (`ci.yml` matrix `args`) — no doctests |
| coverage (`Coverage report`) | 166 | 948 | 0 | 12 | `cargo llvm-cov --workspace` — no doctests |
| windows (`Test (windows)`) | 166 | 926 | 0 | 9 | `cargo test --all --tests` — no doctests, no unix-gated checks |
| ios (`Test (ios)`) | 0 | 0 | 0 | 0 | `cargo build --all` — no test step |
| android (`Test (android)`) | 0 | 0 | 0 | 0 | `cargo build --all` (cross) — no test step |

## linux − macos = linux − coverage: the 19 doctest lines

Every one of the 19 differing result lines is a `Doc-tests {crate}` line; 3 of them run a doctest that
passes (`accesskit_xplat` 1, `blitz_vibey_script` 1, `dioxus_native_dom` 1), the other 16 run none. The
per-binary tallies of every other target are equal across the three jobs.

| | result lines | passed | failed | ignored |
|---|---|---|---|---|
| difference | 19 | 3 | 0 | 0 |

## macos − windows: 22 passed and 3 ignored, every one unix-gated

| test binary | macos (lines · passed · failed · ignored) | windows | cause |
|---|---|---|---|
| `stand_session_lifecycle` | 1 · 2 · 0 · 2 | 1 · 0 · 0 · 0 | `cfg(unix)`: holds a session over the socket |
| `stand_session_quiet` | 1 · 1 · 0 · 1 | 1 · 0 · 0 · 0 | `cfg(unix)` |
| `escher_driver` (lib) | 1 · 72 · 0 · 0 | 1 · 71 · 0 · 0 | one unix-only unit test |
| seven_guis `cli_commands` | 1 · 11 · 0 · 0 | 1 · 0 · 0 · 0 | `#![cfg(unix)]` |
| seven_guis `cli_flow` | 1 · 2 · 0 · 0 | 1 · 0 · 0 · 0 | unix-gated |
| seven_guis `host_binary` | 1 · 2 · 0 · 0 | 1 · 1 · 0 · 0 | one unix-gated check |
| seven_guis `host_log` | 1 · 1 · 0 · 0 | 1 · 0 · 0 · 0 | unix-gated |
| seven_guis `host_timer` | 1 · 1 · 0 · 0 | 1 · 0 · 0 · 0 | unix-gated |
| seven_guis `mcp_server` | 1 · 5 · 0 · 0 | 1 · 3 · 0 · 0 | its two command-line comparisons are unix-gated |
| **sum** | | | 0 lines · 22 passed · 0 failed · 3 ignored |

## The 2026-10-10 gap, restated

CI#38013740580 on `9462a7e4` read linux 158 result lines against 139 for `Coverage report`, `Test (macos)`
and `Test (windows)`: 158 − 139 = 19, the same 19 `Doc-tests` lines (the matrix and `cargo llvm-cov` run
no doctests). Its 716 − 712 passed and 10 − 8 ignored between macos and windows are the same second
cause at that run's counts: the unix-gated session and command-line checks.

## After this chunk

This chunk adds one unix-only unit test to `escher_driver` (`client::imp::tests`) and one check to the
unix-gated `cli_commands`, so the macos − windows difference grows by 2 passed at the next run; it adds
`font_face_sniffed_source` (2 tests, no gate) to every job that runs tests.
