group: architecture+test-plan-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «every other witness found inside `stand-repeat` by the `stand_` prefix or by the ten named `--test`s of `ci-leg.sh`'s own text»
  source: .andromeda/architecture-detail/existing-scopes.md:115 «every other witness read as inside `stand-repeat` by the `stand_` prefix or by its name anywhere in `ci-leg.sh`'s text, which the test reads whole by `contains`»
  replace-with: «every other witness read as inside `stand-repeat` by the `stand_` prefix or by its name anywhere in `ci-leg.sh`'s text, which the test reads whole by `contains` — so a witness dropped from `stand-repeat` alone while the `a11y` leg still names it still reads inside, the leg's membership held jointly with `test_stand_repeat_leg_runs_three_rounds`»

- leaf: .claude/docs/services/escher-driver.md:51   class: omission
  leaf-text: «The `stand_act_*` checks keep the same rule: a message names a layout mode, a task and an index.»
  source: .andromeda/architecture-detail/conventions.md:62 «MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers»
  replace-with: «The `stand_act_*` checks keep the same rule: a message names a layout mode, a task and an index. Since 2026-10-26-codebase-health `mcp_tests.rs`' `the_listing_is_the_twelve_verbs_of_the_table` keeps its outer assertions and row walk and asserts each listed tool through three private `#[track_caller]` helpers — `assert_described`, `assert_schema`, `assert_property` — whose messages name the row and the index only.»
