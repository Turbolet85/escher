@carried n=1 label=test-plan/ci-integration fact=fct-mrjxzany
why: a probe check the plan keeps in tests/blitz-tests/tests/ under the stand_ prefix must join the stand-repeat glob, run three rounds green in both layout modes, and be re-counted in the 51-file count and the run stand and run all proof readings.
quote: stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each
  printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per
  `tests/blitz-tests/tests/stand_*.rs` — 51 files
@carried n=1 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-3ctmcebn
quote: the `stand-repeat` leg and its job `Stand checks repeated` run the 51 `stand_*.rs` files, the seventeen named
  blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
@carried n=1 run=3 label=test-plan fact=fct-nymiowfp
quote: the `stand-repeat` leg runs the stand checks, seventeen named blitz-tests targets outside the `stand_` prefix and
  seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no
  `--ignored`, no env var
@carried n=1 run=4 label=test-plan fact=fct-temy7g7i
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the
  `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry
@carried n=1 run=5 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 ·
  failed 0 · ignored 12
@carried n=2 label=test-plan/ci-integration fact=fct-prdavblg
why: each probe verdict and any kept probe check must name the build it was run under, since a green per-package run of the probe is not the workspace leg's green.
quote: the same check file read green under `-p blitz-tests` and red under the workspace leg, so a check that reads a
  sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
@carried n=2 run=2 label=test-plan fact=fct-he2hzpm5
quote: `cargo test --workspace --locked` and `cargo test -p blitz-tests --locked` build the same test file against
  different engine features — `tracing` and `writing-mode` on in the workspace build only — so a check that reads a
  sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and
  each body of the engine's bounds reader holds a check
@carried n=3 label=test-plan/ci-integration fact=fct-na5rpohe
why: probe code kept in packages/dioxus-native-dom must leave the crate's line coverage at or above its 83 % floor, the coverage leg exiting with the floor script's status.
quote: and then, chained, `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info
  packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor
  script's status
@carried n=3 run=2 label=test-plan fact=fct-k6zjxg3m
quote: the `coverage` leg runs `cargo llvm-cov` over the workspace, `keyboard_compat.rs` excluded, into
  `target/coverage/lcov.info` and a per-file report, then `coverage_floor.py` over the per-crate floors (§10), the leg
  exiting with the floor script's status
@carried n=4 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
why: a probe reading a handler from a cfg(test) child file of dioxus_document is no record of dioxus-native-dom's per-crate reading, so keeping it must not be taken as moving that crate's coverage.
quote: A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading — the coverage job's per-file
  report lists none, `mcp_tests.rs` and `schema_tests.rs` included — so a test module moved out of line moves its
  crate's reading with no line of code changed
@carried n=4 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
quote: dioxus-native-dom 2791/3241 = 86.12 %
@carried n=4 run=3 label=test-plan fact=fct-4t3fuckc
quote: the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors —
  `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo
  llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until
  escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
@carried n=5 label=test-plan/ci-integration fact=fct-qjdo4c7k
why: a probe target the plan keeps moves these per-OS and stand-repeat tallies, which its chunk must re-read on its own CI run against this newest reading.
quote: Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and
  coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 ·
  1155 · 0 · 21, unmoved
@carried n=5 run=2 label=test-plan fact=fct-eu6eymmp
quote: linux runs more than macOS by the doc-test result lines, and macOS more than windows by the unix-gated
  binaries; ios and android build only
@carried n=6 label=test-plan/ci-integration fact=fct-kjlei6et
why: a probe target the plan keeps moves the local baseline's result lines and passed count, which its chunk must re-count from this reading.
quote: Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12
  ignored
@carried n=6 run=2 label=test-plan fact=fct-d74qnmiv
quote: the dev host, the dev profile with `debug = "line-tables-only"`, workspace default features, every cargo command
  with `--locked`, no `opt-level` rewrite
@binding n=1
**ci-integration (stand-repeat leg) ↔ quality-gates-coverage-targets (flakiness budget)**: a probe check kept with the stand_ prefix joins the stand-repeat glob by its prefix alone and must hold the zero-failure budget on all three rounds in both layout modes, any kept MountedData reproduction panicking or not the same way every round; the 51-file count and the run stand 237 · run all 733 proof readings are re-counted when such a check is kept (test-plan/ci-integration fct-mrjxzany) (test-plan/quality-gates-coverage-targets fct-3ctmcebn) (test-plan fct-glytb3w3)
@binding n=2
**ci-integration (coverage leg) ↔ quality-gates-coverage-targets (coverage floors)**: the leg's 83 % floor on packages/dioxus-native-dom still binds wherever probe code is kept in that crate, and a probe kept as a cfg(test) child file of dioxus_document is no record of that crate's reading; the probe adds no non-test code to the crate (test-plan/ci-integration fct-na5rpohe) (test-plan/quality-gates-coverage-targets fct-fx4qekph)
@binding n=3
**ci-integration (engine features by runner) ↔ the chunk's measured verdicts**: a verdict resting on a run of the probe under `cargo test -p blitz-tests` names that build and does not stand for the workspace leg's green; a kept probe check is read green under both builds (test-plan/ci-integration fct-prdavblg) (test-plan fct-he2hzpm5)
@binding n=4
**ci-integration (per-OS tallies) ↔ ci-integration (local baseline)**: a kept probe target moves both the per-OS and stand-repeat tallies and the local baseline's result lines and passed count, each re-read and re-counted by the chunk that keeps it against the newest readings; a probe run and not kept moves neither (test-plan/ci-integration fct-qjdo4c7k) (test-plan/ci-integration fct-kjlei6et)
