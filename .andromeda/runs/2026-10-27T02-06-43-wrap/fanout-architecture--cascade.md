proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 77
    section: "## Stack and Technologies"
    change: "Append to the Technology text's last line, after its closing parenthesis: «; and, since 2026-10-26-codebase-health, a cognitive-complexity reading on the dev host alone, never in CI: `rust-code-analysis-cli` 0.0.25, in `~/.cargo/bin` (installed 2026-10-03), resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15 (dev host, as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)», and append to the Role line «; a host-read complexity ceiling»."
    sidecar: "stack-and-technologies Code quality: the dev-host complexity analyser `rust-code-analysis-cli` 0.0.25 named beside the mutation tier, no lockfile, no CI leg (2026-10-26-codebase-health)."
    rationale: "The report's Dev-tool versions names `rust-code-analysis-cli` 0.0.25 as the complexity reading's analyser on the dev host, resolved by no lockfile, and Harness / gate surface says the reading is a dev-host one with no CI leg; D-arch-decisions reads a tool the stack does not name as drift, and the Code quality layer already lists the host-run mutation tier's tools in this shape."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:110"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 9
    section: "## Conventions"
    change: "Append to the Formatting and lints item: «; since 2026-10-26-codebase-health, a cognitive-complexity ceiling of 15 per function, test functions counted, read on the dev host by `rust-code-analysis-cli` 0.0.25 and run by no CI leg — 85 functions read over it at `3c04042ed`, the engine's `clear_interaction_state_for_removed_node` (17.0) and blitz-dom's `get_text_selection_ranges` (32.0) among them, left untouched (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)»."
    sidecar: "conventions Formatting and lints: the dev-host cognitive-complexity ceiling of 15, test functions counted, 85 over it at 3c04042ed (2026-10-26-codebase-health)."
    rationale: "The report's Counts / qualifiers moved and Outcome (v030-01) read the population over 15 at 88 → 85 with `clear_interaction_state_for_removed_node` 17.0 standing, and Deviations names `get_text_selection_ranges` 32.0 untouched; the ceiling the chunk worked to is a convention code is now held to, stated in no master."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:59"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 9
    section: "## Conventions"
    change: "Append to the Formatting and lints item, after the complexity ceiling's clause: «; the reading is `cognitive.sum`, which aggregates nested spaces, so a closure or nested `fn` keeps its branches counted in its parent and only a module-level sibling item lowers a function's reading»."
    sidecar: "conventions Formatting and lints: how the complexity reading counts — nested spaces aggregated, only a module-level sibling lowers a reading (2026-10-26-codebase-health)."
    rationale: "The report's Decisions & corrections records the rust-code-analysis fact the chunk's form rests on (`findings` 16 over 3 spaces, each closure 0); it says how the ceiling's reading is lowered and belongs with the ceiling."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:223"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: "## Conventions"
    change: "After «`#[track_caller]` assertion helpers (tests/blitz-tests/tests/accessibility_roles.rs:40-41)» insert: «; a table-asserting test keeps its outer assertions and row walk in the test function and moves each row's checks into private `#[track_caller]` helpers whose messages name the row and index only, so it stays under the complexity ceiling with its name, target and messages unchanged — since 2026-10-26-codebase-health escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test»."
    sidecar: "conventions Tests: a table-asserting test's row checks in private `#[track_caller]` helpers naming row and index only (2026-10-26-codebase-health)."
    rationale: "The report's Symbols / APIs shows both tests keeping their outer assertions and row walk and calling new private `#[track_caller]` helpers (`assert_described` · `assert_schema` · `assert_property`; `assert_0_2_0_row` · `assert_0_2_0_witness`), every message unchanged, and Coverage of new surfaces says their messages name row and index only."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:33"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 273
    section: "## Standard Contracts"
    change: "Replace «pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's own text by `stand_requirement_sweep`'s leg check (test-plan §1)» with «its membership held jointly by `test_stand_repeat_leg_runs_three_rounds`, which asserts the named targets among the leg's own logged words, and `stand_requirement_sweep`'s leg check (test-plan §1), which reads the script whole by `contains` and so cannot see a target dropped from `stand-repeat` alone while another leg — `a11y` — still names it, a reach gap that predates 0.3.0 (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)»."
    sidecar: "standard-contracts CI contracts: stand-repeat membership held jointly by the CI-scripts test and the sweep test's whole-script leg check, whose reach gap is stated (2026-10-26-codebase-health)."
    rationale: "The report's Spec claims disproved by measurement names this sentence: at implement's control (a) `--test select_interaction` dropped from `stand-repeat` alone left the sweep test green while the `a11y` leg still named it, and `test_stand_repeat_leg_runs_three_rounds` asserts it among the leg's logged words; this also resolves the entity's contradict class."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:133"
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "Replace «every other witness found inside `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text» with «every other witness read as inside `stand-repeat` by the `stand_` prefix or by its name in `ci-leg.sh`'s whole text, read by `contains` — so a witness dropped from `stand-repeat` alone while the `a11y` leg still names it still reads found, the leg's membership held jointly with `test_stand_repeat_leg_runs_three_rounds` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)»."
    sidecar: "existing-scopes blitz-tests: the 0.2.0 sweep's stand-repeat reach restated — whole-script `contains`, membership held jointly with the CI-scripts test (2026-10-26-codebase-health)."
    rationale: "The report's Spec claims disproved by measurement quotes this claim and measures it false at implement's control (a): removing the stand-repeat site of `select_interaction` alone left the test green; only both sites removed read red. The property predates the chunk; its owner is P5's."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:134"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "After «read by its third test» insert: «, whose target predicate is, since 2026-10-26-codebase-health, a private helper of its own, `inside_stand_repeat`, holding the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS`»."
    sidecar: "existing-scopes blitz-tests: the 0.2.0 sweep test's target predicate `inside_stand_repeat`, holding `BLITZ_TESTS` · `SEVEN_GUIS` (2026-10-26-codebase-health)."
    rationale: "The report's Symbols / APIs: the target predicate is new private `inside_stand_repeat` (head `:1097`), now holding the two path-prefix constants `BLITZ_TESTS` · `SEVEN_GUIS` the test held."
    basis: "escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:42"
    fact: F6
note: The body's broad lines that summarise these groups — Stack and Technologies (architecture.md:19, «beside a host-run mutation tier») and Conventions' Formatting and lints (architecture.md:102) — may want the dev-host complexity ceiling named if the orchestrator lifts F1/F4 to the body; the sweep test's reach gap owner is P5's per the report, and no D-arch-resources drift (no resource, crate, env var or dependency added).
