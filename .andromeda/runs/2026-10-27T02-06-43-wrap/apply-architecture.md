# Apply — architecture

## Verdicts
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/stack-and-technologies.md:82+#1
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/conventions.md:61+#1
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:273#1
- fanout-architecture--cascade.md 6 · applied · .andromeda/architecture-detail/existing-scopes.md:115#1
- fanout-architecture--cascade.md 7 · applied · .andromeda/architecture-detail/existing-scopes.md:142+#1

## Edits
- after: .andromeda/architecture-detail/stack-and-technologies.md:82
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · stack -->»
  add: «»
  add: «- **Code quality** beside the host-run mutation tier: a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15 (dev host, as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:61
  anchor: «- A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file (tests/blitz-tests/tests/fixtures/mod.rs:17-25; tests/blitz-tests/tests/fixtures/mod.rs:87-130).»
  add: «- escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers whose messages name the row and index only (packages/escher-driver/src/mcp_tests.rs:415-496; tests/blitz-tests/tests/stand_requirement_sweep.rs:1050-1093 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md).»
- line: .andromeda/architecture-detail/standard-contracts.md:273
  old: «pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's own text by `stand_requirement_sweep`'s leg check (test-plan §1),»
  new: «its membership held jointly by `test_stand_repeat_leg_runs_three_rounds`, which asserts named targets, `select_interaction` among them, in the leg's own logged words, and `stand_requirement_sweep`'s leg check (test-plan §1), which reads the script whole by `contains` and so cannot see a target dropped from `stand-repeat` alone while the `a11y` leg still names it (.github/scripts/test_ci_workflows.py:361-366; as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md),»
- line: .andromeda/architecture-detail/existing-scopes.md:115
  old: «every other witness found inside `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text»
  new: «every other witness read as inside `stand-repeat` by the `stand_` prefix or by its name anywhere in `ci-leg.sh`'s text, which the test reads whole by `contains` — so a witness dropped from `stand-repeat` alone while the `a11y` leg still names it still reads inside, and the leg's membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`, as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
- after: .andromeda/architecture-detail/existing-scopes.md:142
  anchor: «- blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 31, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 24 (22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`); what each holds is test-plan §3's (tests/blitz-tests/tests/fixtures/mod.rs:17-130) — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md, the 31 at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md.»
  add: «- blitz-tests' `stand_requirement_sweep` reads whether a 0.2.0 witness lies inside `stand-repeat` through a private target predicate of its own, `inside_stand_repeat`, which holds the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS` (tests/blitz-tests/tests/stand_requirement_sweep.rs:1097 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md).»

## Patterns
[[pattern]]
id = "arch-01"
fixed = "read from the script's own text by `stand_requirement_sweep`'s leg check"

[[pattern]]
id = "arch-02"
fixed = "by the named `--test` list of `ci-leg.sh`'s own text"
