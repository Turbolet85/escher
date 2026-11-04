@carried n=1 label=test-plan/ci-integration fact=fct-mrjxzany
why: any probe check or example code the chunk keeps must build and run in every linux leg with `--locked` and stay reproducible on the dev host through the same leg script.
quote: every cargo leg `--locked` but `fmt` (.github/scripts/ci-leg.sh:20) and `examples/wasm_hello` (no `Cargo.lock`); the dev host runs the same script, so every linux leg is host-reproducible (.github/scripts/ci-leg.sh:18-71; .github/workflows/ci.yml:47)
@carried n=2 label=test-plan/ci-integration fact=fct-mrjxzany
why: a probe check kept under `tests/blitz-tests/tests/stand_*.rs` joins `Stand checks repeated` through its prefix, and it must then pass three rounds with no retry, `--ignored` or env var.
quote: stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 51 files,
@carried n=2 run=2 label=test-plan/ci-integration fact=fct-mrjxzany
quote: the first red command ends the leg with its status — no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
@carried n=3 label=test-plan/ci-integration fact=fct-mrjxzany
why: a probe target with no `stand_` prefix reaches `stand-repeat` only by being added as a literal `--test` word, and then `ci-leg.sh` and its pin in `test_ci_workflows.py` must both name it.
quote: each file keeping its name (.github/scripts/ci-leg.sh:56-62), `test_stand_repeat_leg_runs_three_rounds` naming the glob and the seventeen words (.github/scripts/test_ci_workflows.py:384-390)
@carried n=4 label=test-plan/ci-integration fact=fct-nrwelml5
why: this is the precedent for a probe check with no `stand_` prefix over the key-handler path. It reaches CI through the workspace `test` leg and `run all` with no script change, and its readings must cover both layout modes.
quote: the blitz-tests target `key_press_after_container_lost_its_box` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; its 3 tests run in both layout modes; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it
@carried n=5 label=test-plan/ci-integration fact=fct-prdavblg
why: a probe verdict measured with `-p blitz-tests` must not be taken as the workspace leg's verdict. Any probe check that reads a sink capture must count lines by target.
quote: so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
@carried n=6 label=test-plan/ci-integration fact=fct-kjlei6et
why: this is the newest local baseline that a probe check kept by the chunk moves. The record must be re-counted at the wrap if any test is added.
quote: Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored
@carried n=7 label=test-plan/ci-integration fact=fct-qjdo4c7k
why: these are the newest per-OS tallies the chunk's CI run is read against. A kept probe check moves them, and they must be read through the stated ANSI strip.
quote: Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved
@carried n=7 run=2 label=test-plan/ci-integration fact=fct-qjdo4c7k
quote: A per-job tally read from a CI job log through `gh run view --log` strips the ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`
@carried n=8 label=test-plan/ci-integration fact=fct-na5rpohe
why: `packages/dioxus-native-dom/` is a surface the probe touches. Any probe or test code landing there must keep the crate at or above its 83 floor in the `coverage` leg.
quote: `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status
@carried n=9 label=test-plan/ci-integration fact=fct-pmze6kz2
why: a probe check that reproduces the MountedData panic must not be left as a red target in a leg. One red target hides every later target in that job's run.
quote: `cargo` stops at the first failing target, so a red job's later targets go unread on that run
@carried n=10 label=test-plan fact=fct-ovlcl3hy
why: the chunk's code must pass the local pre-push gate before it is pushed.
quote: `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden
@carried n=11 label=test-plan fact=fct-42zxrqel
why: a probe check run on the stand must boot through the stand's bundled font with system fonts off, so its reading holds on any runner and in the package-alone build.
quote: except the headless-stand checks, which use only the bundled DejaVu Sans with system fonts off and so measure the same on any runner, in the `-p blitz-tests` package-alone build included
@binding n=1
**stand-repeat leg ↔ body §10 flakiness budget**: a probe check placed in `stand-repeat` (by prefix or by a named word) falls under the zero-failure, three-round, no-retry budget (test-plan/ci-integration fct-mrjxzany) (test-plan fct-temy7g7i)
@binding n=2
**coverage floor ↔ body §10 coverage floors and the scope's bridge surface**: the `packages/dioxus-native-dom=83` floor binds any code the probe lands in the bridge crate, with `keyboard_compat.rs` excluded on both `cargo llvm-cov` lines (test-plan/ci-integration fct-na5rpohe) (test-plan fct-4t3fuckc)
@binding n=3
**engine features by runner ↔ the scope's "Nothing crosses" rule (values masked, never logged)**: a probe check that reads a sink capture counts lines by target under both builds, and a per-package green is not taken for the workspace leg's green (test-plan/ci-integration fct-prdavblg) (test-plan fct-he2hzpm5)
@binding n=4
**local baseline and per-OS tallies ↔ body §3 agent-run proof**: a kept probe target moves the 219 · 1299 · 0 · 12 linux baseline and the `run all` 733 · 0 · 12 reading together, and the wrap re-reads both (test-plan/ci-integration fct-kjlei6et) (test-plan fct-glytb3w3)
