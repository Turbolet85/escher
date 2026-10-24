# arch · Design Philosophy — category read

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:59) The sweep's proof of every 0.2.0 capability must run headless, on the stand or a fixture beside it, with no window, matching the principle this category states.
  > - **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks;
- (.andromeda/architecture-detail/design-philosophy.md:60) Each check the sweep adds, a C1 fixture check among them, must drive the screen through the headless harness, with input synthesized through the real event-dispatch pipeline, and must live in blitz-tests' `tests/` directory.
  > - The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19); integration tests live in the `tests/` directory of the blitz-tests crate (tests/blitz-tests/lib.rs:1)
- (.andromeda/architecture-detail/design-philosophy.md:46) Each new fixture check, the C1 readings in "both layout modes" included, must run in both layout modes, and any engine change a fork brings must keep the two modes' layouts identical.
  > - Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2223-2240; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
- (.andromeda/architecture.md:11) The body's broad line holds the same two-mode identity that every new check and any engine change in this chunk must keep.
  > both layout modes produce identical layouts
- (.andromeda/architecture-detail/design-philosophy.md:33) If C1's fork judges a `text_extent` reading (RTL or vertical text read as LTR lines, a transformed box at its client rect) to be a defect and the founder answers "change", the fixed reading must be written against a named specification or a named browser engine.
  > - **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.

## Bindings
- **Headless proof ↔ body §Standard Contracts (Test harness)**: the sweep's checks run through the headless harness (.andromeda/architecture-detail/design-philosophy.md:60), whose `Harness` contract governs how they pump, settle and read the screen (.andromeda/architecture.md:159)
- **Two layout modes ↔ body §Conventions (Tests)**: the incremental identity (.andromeda/architecture-detail/design-philosophy.md:46) is enforced in each new check because its scenarios run in both layout modes (.andromeda/architecture.md:121)
- **Cite the spec ↔ body §Established Decisions (engine changes)**: a `text_extent` change that a C1 fork brings would cite its spec (.andromeda/architecture-detail/design-philosophy.md:33) and would also be a direct engine change for every document, flagged upstreamable (.andromeda/architecture.md:60)
