# tests extract

## Relevance
relevant — the chunk's whole acceptance is that "our tests and CI prove our logic survived" an upstream merge, which rests entirely on the test surface, the agent-run contract and the CI legs.

## Constraints
- The proof is the full local gate as CI runs it: per test-plan §9 Pipeline facts (Legs, Local pre-push gate), `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) plus the `doc` leg (`cargo doc --workspace --no-deps`), with every cargo leg `--locked`. A lockfile the merge leaves unresolvable fails every leg, not just one.
- Per test-plan §2 Differential oracle, `incremental_oracle` compares every node's `final_layout()`, `layout_children` and paint tree between incremental and non-incremental documents after each step. The upstream Taffy and Parley bumps and the `align-content` and abs-pos self-alignment changes go straight through this, so it is the main proof that layout survived. Whether the oracle's fixtures cover the newly changed alignment paths is research's question.
- Per test-plan §2 Font-dependent tests and §8 Real dependencies kept, the headless-stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off and no skip path. A Parley bump that shifts shaping or line metrics shows up as a real stand failure, not a skip, and has to be dispositioned (upstream behaviour change vs our regression), never relaxed.
- Per test-plan §1 Coverage scope (tests/blitz-tests) and §2 Harness-driven tests, the surviving-logic set covers the five `stand_*` files (fresh-boot and incremental `dom_string` identity, one driven interaction per task, timer ticks), the four `telemetry_*` files and `dioxus_falsy_disabled`. `dom_string` carries geometry, so upstream inline-fragment changes (#1060/#1062) can move its output. Whether any stand check pins absolute geometry, rather than fresh-vs-incremental identity, is research's question.
- Per test-plan §4 What unit tests cover (CI workflows and leg script), `UpstreamGuardTest` pins the `DioxusLabs/blitz` repository guard on every job of publish-browser, wpt and wpt-post-results. The ci-scripts leg (64 tests, `test_agent_run.py` 14 and `test_cold_agent.py` 27 included) must stay green after the merge, even though no `.github/` file is in upstream's delta.
- Per test-plan §9 Local baseline, the last recorded workspace count is 120 result lines, 430 passed · 0 failed · 4 ignored (2026-10-06-headless-stand). After the merge the count must not lose any test of ours. Any change has to be traced to a named upstream commit. Whether upstream's 11 commits add or remove any `#[test]` (for example around `node.rs` or `attr_test.rs`) is research's question.
- Per test-plan §9 Pipeline facts (Legs), the `audit` leg (`cargo deny check advisories`) and the `a11y` leg (`accessibility_hidden`, `accessibility_roles`, `focusability_updates`) are both host-reproducible, and the scope requires the audit leg because new taffy and parley git revs enter the graph.

## Patterns to follow
- Run each gate through `bash .github/scripts/ci-leg.sh {leg}` and keep each leg's merged log in `target/ci-logs/{leg}.log` as evidence (per test-plan §9 Failure logs, Local pre-push gate).
- Drive the stand and blitz-tests proof through the 5-command discipline: `bash scripts/agent-run.sh boot`, then `run stand`, then `run all`, then `logs`, then `cleanup`. Read the outcome from the JSON-line `run.end` event (`passed`/`failed` counts, `cargo_exit`), not from raw cargo output (per test-plan §3 Agent-run contract).
- Record counts and timings as dated evidence under the chunk's `evidence/`, in the "as measured at" form that test-plan §9 Local baseline uses. A green `doc` leg recorded here is the evidence the next chunk's PREREQ can cite.
- Use the existing pinned stand options for any re-check: `seven_guis::stand::options(incremental)` (800×600, scale 1.0, Light, bundled font, no net), run for both `incremental` values (per test-plan §7 Builders and options, §3 blitz-test-harness).
- Leave the existing differential oracle as the regression net. Add no new test in this chunk unless a conflict fix or a gate failure strictly requires one, and name any such test for the bug it guards (per test-plan §2 Test function naming, and the scope's additivity rule).

## Anti-patterns to avoid
- Treating an empty or partial run as a pass. Per test-plan §3 `run {selection}`, a run is `passed` only with cargo exit 0, no failed test and at least one parsed test line. A `run stand` that reports fewer stand tests than before the merge is a red, not a green.
- Taking green at face value from tests that can pass without asserting. Per test-plan §2 Font-dependent tests, the pre-existing font tests skip with `eprintln!` when text measures 0×0. Per test-plan §4 Conditional assertions, two text-input tests assert only inside an `if`. A merge that breaks text measurement could turn these into silent passes, so check for skip lines before citing them as survival proof.
- Running or gating on WPT for the fork. Per test-plan §9 WPT workflow, WPT CI is upstream-only behind the repository guard. Upstream's `WPT_COMMIT` bump and `attr_test.rs` change are covered only by the workspace build and unit tests, never by conformance numbers.

## Contract bindings
- tests ↔ obs: the four `telemetry_*` integration files exercise the stand log format and allowlist scrub bound to obs-plan §3 (per test-plan §3 Stand log format). They must stay green after the merge because escher-telemetry rides the same workspace.
- tests ↔ security: the `audit` leg is cargo-deny over the resolved graph (per test-plan §9 Legs), bound to security §Dependencies (new git revs, `--locked`, committed `Cargo.lock`).
- tests ↔ a11y: the `a11y` leg's three files (per test-plan §9 Legs) are the a11y plan's harness. `focusability_updates` and `accessibility_roles` run on layout and DOM code that the merge touches.
- tests ↔ arch: the incremental/non-incremental layout identity invariant is enforced by `incremental_oracle` (per test-plan §2 Differential oracle).

## Acceptance criteria contributions
- After the merge, `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` both exit 0 on the dev host, as do `ci-leg.sh audit` and `ci-leg.sh a11y`, with each leg log kept as evidence (per test-plan §9 Pipeline facts).
- `bash scripts/agent-run.sh boot` exits 0. `run stand` ends `run.end outcome: passed` with at least the 13 stand `ok` events of the pre-merge proof and 0 failed. `run all` ends `passed` with 0 failed (per test-plan §3 Agent-run contract, Proof).
- The post-merge `cargo test --workspace` count is at least the recorded 430 passed · 0 failed · 4 ignored, and every difference is traced to a named upstream commit. The ci-scripts leg reports its test count OK, 64 before the merge (per test-plan §9 Local baseline and §4 CI workflows and leg script).
- Fork CI on the pushed merge commit comes back green on every job: the fast four plus the dependent jobs and the windows/macos/ios/android matrix. The run is read against the fork, not upstream (per test-plan §9 Fast/slow split, Platform).
