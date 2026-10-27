# tests · cascade read · group 1 of 4

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:56) The refactor adds and removes no escher-driver unit test, so the crate's recorded count stays at 100 in sixteen files with the same inline and `#[path]` split, and the workspace `test` leg the fast gate runs stays green over them.
  > **escher-driver:** 100 unit tests in sixteen files, 64 inline and 36 in four `#[path]` test files
  (.andromeda/test-plan.md:9)
  > **Workspace (CI):** `cargo test --workspace --locked` runs on ubuntu with default features as the fast `test` leg, `bash .github/scripts/ci-leg.sh test`
- (.andromeda/test-plan-detail/test-scope-summary.md:59) The refactored `the_listing_is_the_twelve_verbs_of_the_table` must keep `mcp_tests.rs` at 15 tests and assert every row the record states, `snapshot`'s optional integer `part` and its two counts included, beside the twelve tools in `VERBS` order with `lint` after `screenshot` and `help` last.
  > mcp_tests.rs 15 (12 until 2026-10-19-session-recording, which added that each session a recording boot starts is kept as one check and that a call that ends the recording is answered as its session error; `mcp::tests`, in mcp.rs until 2026-10-16-file-splits;
  (.andromeda/test-plan-detail/test-scope-summary.md:59)
  > the listing is the twelve verbs of the table, `snapshot`'s tool taking an optional integer `part` and naming its two counts;
- (.andromeda/test-plan-detail/test-scope-summary.md:61) The factored `findings` must keep the lint's two inline unit tests passing unchanged (the six defects in order, findings in document order then defect order), with their recorded `lint.rs` line span re-stated where the move shifts it, and `stand_lint` must stay the reader of `Session::lint` over a held instance, in both layout modes.
  > lint.rs 2 (`Defect::ALL` names the six defects in order; findings follow document order, then the defect order — packages/escher-driver/src/lint.rs:526-604)
  (.andromeda/test-plan-detail/test-scope-summary.md:61)
  > `Session::lint`'s readings over a held instance by `stand_lint`, not by a unit test — the crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
  (.andromeda/test-plan.md:38)
  > **escher-driver:** unit tests, inline and in `#[path]` test files, beside off-unix tests the windows job alone runs; host.rs holds no unit test, and the executor's verbs, its refusals and `Session::lint` are read by integration checks, not unit tests — the crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The chunk re-reads `BlitzTestsTargetsTest` to hold `stand_requirement_sweep` as one target of its own under an unchanged name, and the fast gate's `ci-scripts` leg must stay green at its recorded 108 tests.
  > unit-tested with `python3 -m unittest discover -s .github/scripts` as the fast `ci-scripts` leg — 108 tests in eight test files
  (.andromeda/test-plan-detail/test-scope-summary.md:8)
  > the one test of `BlitzTestsTargetsTest`, which pins the blitz-tests manifest to one test target per file with upstream's `all` target unbuilt (.github/scripts/test_blitz_tests_targets.py:17-26)
  (.andromeda/test-plan.md:11)
  > **CI Python scripts:** unit-tested with `python3 -m unittest discover -s .github/scripts` as the fast `ci-scripts` leg (§4), among them the contract tests of the CI install script `.github/scripts/apt-install.sh`, of the coverage floor script and of the blitz-tests manifest
- (.andromeda/test-plan-detail/test-scope-summary.md:16) The `tools/list` listing and the `lint` answer the chunk re-asserts are also witnessed through the binary by seven_guis' `mcp_server`, which the workspace `test` leg runs and which must stay green, its listing of twelve verbs in order unmoved.
  > since 2026-10-10-mcp-surface `tests/mcp_server.rs` (6 tests: the binary's `mcp` role completes the Counter flow by tool calls alone — `initialize`, `tools/list` offering the verbs in order, twelve, `start`, `snapshot`, `click` — answers a `lint` tool call with its line of findings,
  (.andromeda/test-plan.md:15)
  > `examples/seven_guis` holds integration-test targets over its `escher-session` binary (host, log, command line, shell flows, timer host, MCP, recording)
- (.andromeda/test-plan.md:176) The chunk adds and removes no test, so the agent harness's `run stand` tally the record last states must read the same at the chunk's HEAD, `stand_lint` and `stand_requirement_sweep` among the files its `stand_` prefix selects.
  > re-counted with the served Cells and load page, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 205 · failed 0 · ignored 7 over 50 files
  (.andromeda/test-plan.md:164)
  > `stand` = one `--test {stem}` per `tests/blitz-tests/tests/stand_*.rs` present

## Bindings
- **test-scope-summary ↔ quality-gates-coverage-targets**: the escher-driver unit tests and `stand_lint` that must stay green over the factored `findings` are what keeps the crate at its 97 % line floor, so a helper the factoring adds is covered by those existing checks (.andromeda/test-plan-detail/test-scope-summary.md:61) ↔ (.andromeda/test-plan.md:411)
- **test-scope-summary ↔ quality-gates-coverage-targets**: `stand_lint` and `stand_requirement_sweep`, read here as the run-stand tally's files, are also held to the zero-failure budget of three rounds with no retry (.andromeda/test-plan.md:176) ↔ (.andromeda/test-plan.md:413)
- **test-scope-summary ↔ ci-integration**: the workspace `test` leg and the `ci-scripts` leg carried here are two of the fast gate's four legs, and the two touched tests are read under both builds the workspace runs (.andromeda/test-plan.md:9) ↔ (.andromeda/test-plan.md:387); (.andromeda/test-plan-detail/test-scope-summary.md:56) ↔ (.andromeda/test-plan.md:392)
