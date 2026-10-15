group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:44   class: stale
  leaf-text: «The workspace binds no network port; its one listener is»
  source: .andromeda/architecture.md:146 «bound at the tree's one served listener site (beside it, since 2026-10-14-quality-gates, one test-only bind: escher-driver's unix-only unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`»
  replace-with: «The workspace binds no network port; beside one test-only bind (since 2026-10-14-quality-gates escher-driver's unix-only client unit test binds a `UnixListener` that never answers in `eab-{pid}` under the system temporary directory and removes it — served by no binary, reached by no caller, no crossing), its one served listener is»

- leaf: CLAUDE.md:47   class: omission
  leaf-text: «(measured on the `escher-session` host and its `mcp` role while they run calls: 0 id needles»
  source: .andromeda/security-plan.md:399 «since 2026-10-14-quality-gates three more readings are taken by build and level, 28 runs with 0 needles in every run — a client's own stderr, the host on flows beyond the three CRUD commands»
  replace-with: «(measured on the `escher-session` host and its `mcp` role while they run calls and, since 2026-10-14-quality-gates, on a client's own stderr, the host on flows beyond the three CRUD commands and the windowed `seven_guis_native` built with the workspace on the host's display — 28 runs by build and level, 0 needles in every run: 0 id needles»

- leaf: CLAUDE.md:88   class: omission
  leaf-text: «the blitz-tests target pin `.github/scripts/test_blitz_tests_targets.py` |»
  source: .andromeda/architecture.md:66 «since 2026-10-14-quality-gates escher's `coverage_floor.py`, the coverage leg's floor, and its tests `test_coverage_floor.py`, stdlib only, no package added»
  replace-with: «the blitz-tests target pin `.github/scripts/test_blitz_tests_targets.py` · the coverage leg's per-crate floor `.github/scripts/coverage_floor.py` (its tests `.github/scripts/test_coverage_floor.py`) |»

- leaf: CLAUDE.md:98   class: omission
  leaf-text: «`audit` · `a11y` · `coverage` · …)»
  source: .andromeda/architecture.md:141 «· `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests -»
  replace-with: «`audit` · `a11y` · `coverage` · `stand-repeat` — the stand checks three rounds under a zero-failure budget, not a fast leg · …)»
