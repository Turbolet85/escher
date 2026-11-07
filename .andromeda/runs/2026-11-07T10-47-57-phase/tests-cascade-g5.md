@carried n=1 label=test-plan/ci-integration fact=fct-o7txuegv
why: the inverted pins must keep dioxus_runtime_probe at 10 tests, now two non-panicking focus tests one per layout mode, and this row must be re-said so it no longer records a should_panic hazard pin.
quote: its 10 tests, 8 looping `for incremental in [false, true]` and the MountedData hazard pinned as two `should_panic` tests, one per layout mode; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it
@carried n=1 run=2 label=test-plan fact=fct-glytb3w3
quote: `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change
@carried n=2 label=test-plan/test-data-fixtures fact=fct-ay5qvau6
why: the inverted pins must drive the same `hazard(spawned)` pages in dioxus_runtime_probe.rs, and this row's names and coordinates must be re-pointed where the inversion moves them.
quote: | Dioxus runtime probe pages (`rsx!`, in their check's own file) | `dioxus_runtime_probe`'s `toggled_listener` (its click listener spread in and out as a `Vec<Attribute>`), `shared_state` (`Writer`, `FirstReader` and `SecondReader` over one `GlobalSignal`) and `hazard(spawned)` (`hazard_spawned`, `hazard_synchronous`) |
@carried n=3 label=test-plan/test-data-fixtures fact=fct-6hjdlvz3
why: the buffered render must leave the stand's ids, snapshot text and diffs unchanged so the two committed checks still replay green through stand_replay and seven_guis recording with no byte of them re-recorded.
quote: the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh` (ids, names, values, snapshot text and diffs; no typed text, host path, name, secret or `screenshot` step), embedded as bytes by `stand_replay` and seven_guis `recording`
@carried n=3 run=2 label=test-plan fact=fct-wzfum7l4
quote: the two committed checks, the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh`, holding no typed text, host path, secret or `screenshot` step, embedded as bytes by `stand_replay` and seven_guis `recording`
@carried n=4 label=test-plan/ci-integration fact=fct-mrjxzany
why: every stand check boots a DioxusDocument that renders through the new buffer, so the stand-repeat leg must read three green rounds with no retry over it.
quote: stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs`
@carried n=4 run=2 label=test-plan/ci-integration fact=fct-mrjxzany
quote: the first red command ends the leg with its status — no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
@carried n=4 run=3 label=test-plan fact=fct-nymiowfp
quote: the `stand-repeat` leg runs the stand checks, seventeen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
@carried n=5 label=test-plan/ci-integration fact=fct-prdavblg
why: the ported unit test, the inverted pins and the unchanged-log-line claim must each be read green under the workspace build as well as the per-package build, since one does not stand for the other.
quote: so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
@carried n=5 run=2 label=test-plan fact=fct-he2hzpm5
quote: `cargo test --workspace --locked` and `cargo test -p blitz-tests --locked` build the same test file against different engine features — `tracing` and `writing-mode` on in the workspace build only — so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and each body of the engine's bounds reader holds a check
@carried n=6 label=test-plan/ci-integration fact=fct-qjdo4c7k
why: the ported unit test adds one passed test to dioxus-native-dom's lib line on the linux, macos, coverage and windows jobs while the a11y and stand-repeat jobs stay unmoved, and the per-OS tally record must be re-read from this chunk's CI run.
quote: Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved
@carried n=6 run=2 label=test-plan fact=fct-eu6eymmp
quote: linux runs more than macOS by the doc-test result lines, and macOS more than windows by the unix-gated binaries; ios and android build only
@carried n=7 label=test-plan/ci-integration fact=fct-kjlei6et
why: the local baseline's newest workspace re-count must be re-counted with the ported unit test's one passed test and no new result line, the inverted pins moving their outcome and not the count.
quote: Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored
@carried n=7 run=2 label=test-plan fact=fct-d74qnmiv
quote: the dev host, the dev profile with `debug = "line-tables-only"`, workspace default features, every cargo command with `--locked`
@binding n=1
**ci-integration ↔ test-data-fixtures**: the record of the legs reaching dioxus_runtime_probe ({10 tests, the hazard pinned as two should_panic tests}) and the fixture row naming its `hazard(spawned)` pages are re-said together once the pins invert: still 10 tests, the two hazard tests asserting focus moves without a panic in both layout modes, on the same hazard pages (test-plan/ci-integration fct-o7txuegv) (test-plan/test-data-fixtures fct-ay5qvau6).
@binding n=2
**ci-integration tallies ↔ local baseline ↔ agent-run proof**: the ported unit test's +1 passed, with no new result line, has to show the same way in the per-OS tallies, the local baseline re-count and `run all`'s count, which the pins keep at their number of tests (test-plan/ci-integration fct-qjdo4c7k) (test-plan/ci-integration fct-kjlei6et) (test-plan fct-glytb3w3).
@binding n=3
**ci-integration stand-repeat ↔ test-data-fixtures committed checks**: stand_replay runs in every stand-repeat round, so the buffered render must keep each recorded step's ids, snapshot text and diff byte-equal across three no-retry rounds (test-plan/ci-integration fct-mrjxzany) (test-plan/test-data-fixtures fct-6hjdlvz3).
@binding n=4
**ci-integration engine features ↔ the plan body**: the new tests are read green under the workspace leg and under `-p blitz-tests` / `-p dioxus-native-dom`, neither standing for the other (test-plan/ci-integration fct-prdavblg) (test-plan fct-he2hzpm5).
