@carried n=1 label=test-plan/ci-integration fact=fct-mrjxzany
why: the stand checks the chunk edits (stand_non_lean_tasks) or adds must run in the stand-repeat leg by their stand_ prefix, its file count moving with a new one, and a new non-stand_ target that must repeat joins the named --test list.
quote: stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 51 files
@carried n=1 run=2 label=test-plan fact=fct-nymiowfp
quote: the `stand-repeat` leg runs the stand checks, seventeen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
@carried n=2 label=test-plan/ci-integration fact=fct-o7txuegv
why: the chunk re-reads dioxus_runtime_probe's measured mechanisms, so the probe must stay run by the workspace test leg and run all with its 10 tests, and any test the chunk adds to it moves this record.
quote: the blitz-tests target `dioxus_runtime_probe`, auto-discovered with no manifest line, runs in the workspace `test` leg and in `agent-run.sh`'s `run all` and `run dioxus_runtime_probe`, each taking it as one more target with no line of its script naming it; its 10 tests, 8 looping `for incremental in [false, true]`
@carried n=2 run=2 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change
@carried n=3 label=test-plan/ci-integration fact=fct-prdavblg
why: the chunk's no-log checks for nodes, components, handlers, states and values read a sink's capture under the per-package and the workspace build, so they must count lines by target and be green under both.
quote: so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
@carried n=3 run=2 label=test-plan fact=fct-he2hzpm5
quote: so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and each body of the engine's bounds reader holds a check
@carried n=4 label=test-plan/ci-integration fact=fct-qjdo4c7k
why: the chunk's new tests move the per-OS tallies, which must be re-read on the chunk's CI run against this last reading, each move attributed by name.
quote: Re-read on CI#42371051720, the pre-CI commit `e26f1a207` (green 17/17): linux 220 · 1310 · 0 · 12; macos and coverage 201 · 1307 · 0 · 12; windows 201 · 1259 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved
@carried n=4 run=2 label=test-plan/ci-integration fact=fct-3kqdhqos
quote: a tally move is attributed by grepping the test's own name, beside the ANSI strip of the per-OS tallies above
@carried n=4 run=3 label=test-plan fact=fct-eu6eymmp
quote: linux runs more than macOS by the doc-test result lines, and macOS more than windows by the unix-gated binaries; ios and android build only
@carried n=5 label=test-plan/ci-integration fact=fct-3kqdhqos
why: every tally move the chunk's tests make must be attributed by grepping the test's own name, never by the cargo Running line before a result line.
quote: in a GitHub Actions job log the stderr cargo `Running …` lines and the stdout `test result:` lines do not interleave in order, so attributing a result line to the `Running` line before it mis-attributes
@carried n=6 label=test-plan/ci-integration fact=fct-kjlei6et
why: the local workspace test leg's count must be re-counted with the chunk's new unit and stand tests, from this last reading, under the local baseline's profile.
quote: Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored
@carried n=6 run=2 label=test-plan fact=fct-d74qnmiv
quote: the dev host, the dev profile with `debug = "line-tables-only"`, workspace default features, every cargo command with `--locked`, no `opt-level` rewrite
@carried n=7 label=test-plan/ci-integration fact=fct-na5rpohe
why: the new graph reader and signal registry code in dioxus-native-dom must be read by the coverage leg's floor script and leave it exiting 0.
quote: then, chained, `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status
@carried n=7 run=2 label=test-plan fact=fct-k6zjxg3m
quote: the `coverage` leg runs `cargo llvm-cov` over the workspace, `keyboard_compat.rs` excluded, into `target/coverage/lcov.info` and a per-file report, then `coverage_floor.py` over the per-crate floors (§10), the leg exiting with the floor script's status
@carried n=8 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
why: dioxus-native-dom must stay at or above its 83 % floor, and escher-driver above 97 %, with the chunk's new reader, registry and carrier code counted, read against this last reading.
quote: read now under the same floors and `coverage_floor.py`: escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2937/3387 = 86.71 % (floor 83 %), blitz-test-harness 999/1040 = 96.06 %
@carried n=8 run=2 label=test-plan fact=fct-4t3fuckc
quote: `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
@carried n=9 label=test-plan/quality-gates-coverage-targets fact=fct-3ctmcebn
why: the stand's signals move onto the registry and the chunk edits or adds stand checks, so every stand check and named target must pass three rounds with no retry and no failure.
quote: the stand checks hold a zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label): the `stand-repeat` leg and its job `Stand checks repeated` run the 51 `stand_*.rs` files, the seventeen named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
@carried n=9 run=2 label=test-plan fact=fct-temy7g7i
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)
@binding n=1
**ci-integration stand-repeat leg ↔ quality-gates flakiness budget**: the leg's three rounds with no retry are the budget's mechanism, so a stand_ file the chunk adds joins both by prefix and moves the 51-file count in both records, and a non-stand_ target that must repeat joins the seventeen named words of the leg and the budget together (test-plan/ci-integration fct-mrjxzany) (test-plan/quality-gates-coverage-targets fct-3ctmcebn)
@binding n=2
**ci-integration coverage leg ↔ quality-gates coverage floors**: the leg's floor script is where the floors are enforced, so the new dioxus-native-dom reader and registry code, kept out of a `#[path]` test-child file's count, must keep the crate at or above 83 % in the leg's own verdict (test-plan/ci-integration fct-na5rpohe) (test-plan/quality-gates-coverage-targets fct-fx4qekph)
@binding n=3
**ci-integration per-OS tallies ↔ local baseline and the agent-run proof counts**: the chunk's new tests move the CI per-job tallies, the local workspace count and the run stand / run all counts together, each move attributed by the test's own name (test-plan/ci-integration fct-qjdo4c7k) (test-plan/ci-integration fct-kjlei6et) (test-plan fct-glytb3w3)
