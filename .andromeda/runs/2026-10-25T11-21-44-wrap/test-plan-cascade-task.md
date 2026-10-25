# Task — test-plan · 2026-10-25-capability-statement

## The session
Added `docs/capabilities.md`, a 304-line statement of what escher 0.2.0 can do: five sections, its stated limits, its accessibility reach, what an MCP client carries onward, and where each claim is proven.
Added `.github/scripts/test_capability_statement.py`, a CI-script pin with 4 tests that ties the statement's verb and cause names and its five table counts to the schema's tables, and checks the README's MCP tool count.
Changed one line of README.md: the MCP sentence now says the twelve driver commands as tools (it said eleven) and links `docs/capabilities.md`.
Measured: the `ci-scripts` leg went from 100 to 104 tests and from seven to eight test files. The workspace Rust tally did not move (206 result lines, 1105 passed, 0 failed, 12 ignored). CI#40184674449 was green, 17/17.
Corrected one disproved claim. The windowed assistive-technology witness is owned by its `.andromeda/residuals.md` line, which reads `re-carried:0.2.0`, not `(target: next)`, and 0.2.0 does not reach an active assistive technology.
Added no Rust symbol, verb, argument, field, cause, session error, exit, carrier, log field, span, crate, dependency, leg, job or workflow line.

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md

## Touches
- `.andromeda/test-plan-detail/test-scope-summary.md:8`: «100 tests in seven test files» becomes 104 tests in eight test files, since `test_capability_statement.py` adds 4.
- `.andromeda/test-plan-detail/unit-test-strategy.md:29` (around offset 1002 and 1017): «the CI-scripts leg runs 100 tests over seven test files» becomes 104 tests over eight files. Its per-file list gains `test_capability_statement.py` with 4 tests.
- `.andromeda/test-plan-detail/unit-test-strategy.md:29` (around offset 1303): the dated measurement at 2026-10-15-cold-agent-test (`Ran 100 tests`) stays as written, because it is a dated reading.
- The body `.andromeda/test-plan.md:48` (tests/blitz-tests bullet): «owner: its `.andromeda/residuals.md` line (target: next)» for the assistive-technology states check is disproved. The line reads `re-carried:0.2.0`, and 0.2.0 does not reach an active assistive technology. Any category entity that names this witness's owner as `(target: next)` changes the same way.
- Any test-plan count of the test files under `.github/scripts`, or of the tests the `ci-scripts` leg discovers, moves by one file and 4 tests.
- §1 CI Python scripts scope (the contract tests the `ci-scripts` leg runs): its list of what the leg tests now includes the capability statement pin over `docs/capabilities.md` and README.md.
- §4 CI Python script coverage: the test files of the `ci-scripts` leg gain a fourth escher-written kind beside the workflow invariants, the blitz-tests target pin and the contract tests. This one pins a document's counts to the Rust schema's tables.
- §9 the `ci-scripts` leg and the local pre-push gate `ci-leg.sh fast`: the new file joins through `python3 -m unittest discover -s .github/scripts` (`.github/scripts/ci-leg.sh:27`) with no leg, job, workflow or `ci-leg.sh` line changed. CI's «Test CI scripts» job runs it (CI#40184674449 green, 17/17).
- Any test-plan text that says the README or front page counts eleven MCP tools now reads twelve, and the README links `docs/capabilities.md`.
- The cold-agent live run's eleven-tool reading (2026-10-15) is restated in `docs/capabilities.md` as not re-fired since `lint`. The test-plan claim that it was not re-fired is re-read and has not moved.
- The select model's limits, asserted by no check, and the boolean model's two limits, pinned at unit and engine level only, are re-stated by the statement. The test-plan claims are re-read and have not moved.
- The cross-OS shot limit (a PNG's bytes differ between linux x86_64 and macOS aarch64, CI#39225375577) and the rule that a check compared across OSes holds no `screenshot` step are re-stated. They have not moved.
- Windowed end-to-end runs are still unmeasured, and the statement now says so too. §6's NOT YET MEASURED has not moved.
- The `stand-repeat` leg's reach (the 48 `stand_*` files, ten named blitz-tests targets and seven_guis' eight process checks) and the `a11y` leg's fourteen files at 14 · 84 · 0 · 0 on CI#40134371519 are re-stated by the statement. Neither has moved.
- The lint readings (0 findings at boot per lean task, `crud-person-14` reading `clipped` inside `crud-list` after twelve Creates) are re-stated. They have not moved.
- The workspace test tally (206 result lines, 1105 passed, 0 failed, 12 ignored) was re-read and has not moved. No Rust test target, unit test or stand check was added.

## New facts
- F1 · unit-test-strategy · `.github/scripts/test_capability_statement.py` holds `CapabilityStatementTest`, 4 tests. They check that every `VERBS` verb (`packages/escher-driver/src/schema.rs`) and every `Cause::name` (`refusal.rs`) appears in `docs/capabilities.md` as a code span. They check that the statement gives five counts as number words read from the schema's tables: verbs, refusal causes, session errors (`SESSION_ERRORS` in `error.rs`), node keys (`NODE_FIELDS`) and exit statuses (`EXIT_STATUSES`). They also check that README.md holds «the {word} driver commands as tools».
- F2 · unit-test-strategy · The capability-statement pin reads by path from the repository root, writes nothing and reads no env var, and no helper carries a `test_` prefix. It folds markdown whitespace so that a phrase which wraps a source line still matches. It asserts the phrase is in the text, with the phrase as the failure message. Its red-before-green controls are recorded in the chunk's `evidence/pin-controls.md`.
- F3 · test-scope-summary · `docs/capabilities.md` is a document with no runtime surface. Its counts and names are in test scope through the CI-script pin alone, and README.md's MCP tool count is pinned by the same file.
- F4 · test-strategy · `docs/capabilities.md` has a `## Where each claim is proven` section that maps each claim to the check that proves it. Each pinned limit names its check, and a limit no check asserts is stated as asserted by no check.

## Untouched
- Integration test strategy (session checks, the session host, the driver's log line): no Rust code, check or session behaviour changed.
- Mocking and stubbing discipline (`HeldNet`, `TimerTicks`, the time stand-ins): no stand-in was added or changed.
- Test data and fixtures: no fixture page, no shared fixture module, and the two committed Counter checks under `tests/blitz-tests/checks/` are unchanged.
- Quality gates and coverage targets: the coverage floors, the flakiness budget and the mutation tier are unchanged. A CI python test is outside the per-crate floors.
- E2E strategy (command line, MCP and shell flows): the binary, its roles and its flows are unchanged. The tool count twelve has stood since 2026-10-19-visual-lint.
- The agent-run contract counts (`run stand` 197 · 0 · 7 over 48 files, `run all`): no blitz-tests file or test was added, and `scripts/agent-run.sh` is unchanged.
- The cold-agent pipe's contract tests `test_cold_agent.py` and its script: unchanged.
- The other `.github/scripts` test files (`test_ci_workflows.py`, `test_blitz_tests_targets.py`, `test_agent_run.py`, `test_coverage_floor.py` and the two WPT script tests): their contents and counts are unchanged. Only the leg total moves.
- The CI pipeline's jobs, caches, failure logs, package installs and fast/slow split: no workflow or leg line changed.
- The respelled cell in a phase run dir (`p5-controls.md:15`) and the Viola pointer outside the repository: both are outside test-plan's subject.
