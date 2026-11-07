@carried n=1 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
why: the coverage leg must still end at or above `packages/dioxus-native-dom`'s 83 % line floor with the new buffer module counted and the two render sites rewritten, `coverage_floor.py` and the floors unchanged.
quote: `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
@carried n=1 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
quote: Upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), a commit escher has not taken, left out of the take of upstream/main `ebacd68bd`, so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand
@carried n=1 run=3 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
quote: read now under the same floors and `coverage_floor.py`: escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3241 = 86.12 %
@carried n=1 run=4 label=test-plan fact=fct-4t3fuckc
quote: both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
@carried n=2 label=test-plan/quality-gates-coverage-targets fact=fct-3ctmcebn
why: the stand checks and the seventeen named targets that witness unchanged behaviour must pass three rounds with no retry and zero failures over the buffered render path.
quote: the stand checks hold a zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label): the `stand-repeat` leg and its job `Stand checks repeated` run the 51 `stand_*.rs` files, the seventeen named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
@carried n=2 run=2 label=test-plan fact=fct-temy7g7i
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry
@binding n=1
**quality-gates-coverage-targets (coverage floor) ↔ the chunk's scope (the partial port of `286874cc1`)**: the port takes the buffered render of `286874cc1` and not its `keyboard_compat.rs` removal, so both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines must stand and the floors stay unchanged (test-plan/quality-gates-coverage-targets fct-fx4qekph), while the new `mutation_buffer.rs` counts toward dioxus-native-dom's 83 % floor and the ported `mod tests` in `dioxus_document.rs` drives its lines (test-plan fct-4t3fuckc).
@binding n=2
**quality-gates-coverage-targets (flakiness budget) ↔ integration-test-strategy (the stand checks as witness)**: the `stand_*` checks re-asserting unchanged snapshots, diffs, element ids and mounted-event order over the buffered `poll` must stay green three rounds with no retry in `stand-repeat` (test-plan/quality-gates-coverage-targets fct-3ctmcebn), and the inverted `dioxus_runtime_probe` pins must not bring in a new failure in any round where that leg runs them (test-plan fct-temy7g7i).
