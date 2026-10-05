# tests extract

## Relevance
partial — the chunk runs the existing `tests/blitz-tests` suite as a baseline reading and adds no test; harness, fixture and gate mandates bind only through how the run is invoked, read and recorded.

## Constraints
- test-plan §5 Integration Test Strategy (Boundaries covered) sets `tests/blitz-tests` as a workspace member whose tests are Cargo integration tests under `tests/blitz-tests/tests/`, depending only on dev-dependencies. The baseline run is therefore `cargo test -p blitz-tests` over that crate. Whether `--locked` and any other flags match `.github/workflows/ci.yml` is research's question.
- test-plan §3 Test Harness Contract (Runners and invocation) names the standard built-in Rust harness and records third-party test frameworks as absent. The baseline uses `cargo test`, not nextest, which matches the scope's no-new-tooling boundary.
- test-plan §9 CI Integration (Fonts) says font-dependent assertions rely on the `system-fonts` feature, "stated to be on by default when testing the whole workspace". A `-p blitz-tests` run may resolve a different feature set than `--workspace`. Whether it does, and whether this Arch host provides usable system fonts, is research's question.
- test-plan §2 Test Strategy (Font-dependent tests) requires font-dependent tests to skip at runtime with `eprintln!` when no usable font exists. A green exit can therefore hide skipped assertions, and libtest captures that stderr unless the run uses `--nocapture`. The baseline reading has to say whether skips occurred.
- test-plan §9 CI Integration (Benchmarks) marks the benchmark tests `#[ignore]`, run only with `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture`. A default run excludes them, and the wall-clock figure must say whether they were inside it.
- test-plan §1 Test Scope Summary (Workspace (CI); Surfaces under test) names the CI test leg as `cargo test --workspace` with default features on ubuntu, plus a `test --all --tests` matrix. This host's reading is a different OS and command scope from that leg, so the record must state its own command and features rather than claim CI parity.
- test-plan §7 Test Data & Fixtures (Seed strategies, External benchmark page) lists `PAINT_TREE_BENCH_HTML` as an optional external benchmark input. The baseline leaves it unset unless P4 decides to run the ignored benchmarks.

## Patterns to follow
- The differential oracle runs the same fixture and mutations in incremental and non-incremental documents and compares layout, children and the paint tree (per test-plan §2 Differential oracle). Run it as the suite has it; the chunk adds no `for incremental in [false, true]` case.
- Pixel tests render to a CPU buffer, and harness-driven tests go through the real event-dispatch pipeline with no window (per test-plan §2 Pixel tests; §5 Harness ↔ event pipeline). The suite is expected to run headless on the host with no display or GPU. Whether it actually needs neither is research's question.
- Randomized stress uses a deterministic LCG (per test-plan §2 Randomized stress), so a red from it should reproduce on re-run. Treat a non-reproducing red as a flakiness finding to record, not as noise.
- Regression tests are named for the bug they guard, and they document the page that exposed it (per test-plan §2 Test directory + naming conventions). Use the failing test's name and header comment to triage any red before P4 decides between a host-local fix and pinning it forward.

## Anti-patterns to avoid
- Do not install coverage tooling or nextest in this chunk. test-plan §3 contract `Bootstrap phases` (`coverage-tooling-install`) records coverage tooling as absent, and test-plan §9 (Observed absent) confirms it. That install belongs to the later "CI gate legs" / "Stand test contract" entries.
- Do not report a green exit as "all assertions passed" while font-dependent tests may have skipped through `eprintln!` (per test-plan §2 Font-dependent tests; §8 Mocking & Stubbing Discipline, Real dependencies kept).
- Do not let `--ignored` benchmarks or a `--release` build silently enter the baseline wall-clock figure (per test-plan §9 Benchmarks). If they run, record them as a separate, labelled figure.

## Contract bindings
- tests ↔ security: the build and test commands carry `--locked`, per the security rules' Dependencies section, not this plan. The test invocation recorded here has to honour the same flag.
- tests ↔ next working entry ("Fork CI reached"): this host's `cargo test -p blitz-tests` reading is the reference that the CI leg (`cargo test --workspace`, per test-plan §1 Workspace (CI)) is later compared against. The record names the command, features and host so the two readings stay distinguishable.
- tests ↔ obs: (none) — the 5-command, status-endpoint and log-format binding is unmeasured in test-plan §3, so nothing binds here.

## Acceptance criteria contributions
- `cargo test -p blitz-tests` exits 0 on this host, with every file under `tests/blitz-tests/tests/` compiled and run. Any red is recorded with the test's name as the baseline reading (per test-plan §5 Boundaries covered).
- The `incremental_oracle` differential tests pass in the baseline run (per test-plan §2 Differential oracle).
- The baseline record counts font-dependent runtime skips separately from asserted passes. They are observed through a `--nocapture` run or an equivalent, or the record states "none observed" with how that was checked (per test-plan §2 Font-dependent tests).
- The record states the ignored count, including `paint_tree_bench`, and confirms the ignored benchmarks are excluded from the recorded wall-clock or reported as a separate figure (per test-plan §9 Benchmarks).
