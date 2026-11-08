@carried n=1 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
why: the read's new code in escher-driver (schema, executor, written forms, command line, MCP, recording, replay, self-description) and any reader P3 adds in dioxus-native-dom must leave each crate at or above its floor, and a unit-test module the chunk adds or moves into a `#[path]` test-child file moves its crate's reading with no line of code changed.
quote: three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
@carried n=1 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
quote: A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading — the coverage job's per-file report lists none, `mcp_tests.rs` and `schema_tests.rs` included — so a test module moved out of line moves its crate's reading with no line of code changed
@carried n=1 run=3 label=test-plan/quality-gates-coverage-targets fact=fct-fx4qekph
quote: read now under the same floors and `coverage_floor.py`: escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2937/3387 = 86.71 % (floor 83 %), blitz-test-harness 999/1040 = 96.06 %
@carried n=1 run=4 label=test-plan fact=fct-4t3fuckc
quote: the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
@carried n=1 run=5 label=test-plan fact=fct-qpxjumbg
quote: read across the workspace with no threshold applied; the floors bind per crate (below)
@carried n=2 label=test-plan/quality-gates-coverage-targets fact=fct-3ctmcebn
why: a stand check the chunk adds to prove the read in both layout modes joins the stand-repeat leg, which must read zero failures over three rounds with no retry, and its count of `stand_*.rs` files moves from 52.
quote: the stand checks hold a zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label): the `stand-repeat` leg and its job `Stand checks repeated` run the 52 `stand_*.rs` files, the seventeen named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
@carried n=2 run=2 label=test-plan fact=fct-temy7g7i
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)
@carried n=2 run=3 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 244 · failed 0 · ignored 7 over 52 files, `stand_graph` a file its `stand_` prefix picks up with no script change
@binding n=1
**quality-gates-coverage-targets (coverage floors) ↔ unit-test-strategy**: the escher-driver unit tests the chunk extends for a widened `VerbSpec` and the read's verb, forms, command line, MCP, recording and replay must carry the new code's lines so escher-driver reads at or above 97 % (test-plan/quality-gates-coverage-targets fct-fx4qekph), and a test module placed in a `#[path]` child counts for no line (test-plan fct-4t3fuckc); the executor's shared refusal path is read by integration checks, not unit tests (test-plan fct-optamucx)
@binding n=2
**quality-gates-coverage-targets (flakiness budget) ↔ test-strategy (agent-run proof count)**: a new `stand_` check for the read joins both the stand-repeat leg's zero-failure three rounds (test-plan/quality-gates-coverage-targets fct-3ctmcebn) and `run stand`'s recorded count of 244 over 52 files, which the chunk must re-read (test-plan fct-glytb3w3)
@binding n=3
**quality-gates-coverage-targets (flakiness budget) ↔ integration-test-strategy**: a check that reads zero failures with no retry holds no sleep or wall-clock read and bounds every wait, the covered, off-screen and owner-served cases included (test-plan fct-ubyqdssx)
