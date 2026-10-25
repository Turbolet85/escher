# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) The chunk's CI run must end the `coverage` leg with the three per-crate floors held; a document-only chunk changes no code and must leave each floor and `coverage_floor.py` as they stand.
  > - **Coverage floors:** since 2026-10-14-quality-gates the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) The newest per-crate reading is the baseline a code-free chunk must leave unmoved, so the floor verdict at the wrap must still read 3 of 3.
  > read again with the stand checks' named list, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, `coverage floor: 3 of 3 crates at or above their floors`, equal to CI#40105170961
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22) The zero-failure budget's job `Stand checks repeated` must stay green on the chunk's CI run, and any proof breadth the capability statement cites for the stand must match these counts (48 `stand_*.rs` files, ten named targets, eight process checks, three rounds, no retry).
  > - **Flakiness budget:** since the same chunk the stand checks hold a zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label): the `stand-repeat` leg and its job `Stand checks repeated` run the 48 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
- (.andromeda/test-plan.md:408) The body's broad floor line is the form the statement may cite for coverage: three per-crate floors, no workspace threshold, the `keyboard_compat.rs` exclusion standing.
  > - **Coverage floors:** the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it
- (.andromeda/test-plan.md:406) The statement must not claim a workspace coverage threshold; the floors bind per crate alone.
  > - **Workspace line coverage:** read across the workspace with no threshold applied; the floors bind per crate (below)
- (.andromeda/test-plan.md:414) The statement's stated limits must not claim a performance budget or guarantee; none is set.
  > > NOT YET MEASURED — a performance budget: none is set

## Bindings
- **Flakiness budget ↔ body §9 Legs**: the zero-failure budget's three rounds are the `stand-repeat` leg the §9 Legs line describes — "the first red command ending the leg with its status — no retry, no `--ignored`, no env var" — so the chunk's CI must pass both as one gate (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22) ↔ (.andromeda/test-plan.md:374)
- **Coverage floors ↔ body §9 Coverage**: the floor script ends the `coverage` leg §9 states, the leg exiting with the floor script's status (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) ↔ (.andromeda/test-plan.md:398)
- **Body §10 broad lines ↔ this category file**: the body's floor and flakiness lines summarise the category's groups; a statement citing either must match both (.andromeda/test-plan.md:408) ↔ (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14); (.andromeda/test-plan.md:410) ↔ (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22)
