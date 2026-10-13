group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «78 CI-script tests in six files»
  source: .andromeda/test-plan.md:8 «as the fast `ci-scripts` leg — 79 tests in six test files (§4; 78 until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added one to test_wpt_area_changes.py)»
  replace-with: «79 CI-script tests in six files — 78 until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added one to test_wpt_area_changes.py»

- leaf: .claude/docs/tests-summary.md:30   class: omission
  leaf-text: «`cargo test --workspace` 849 · 0 · 12 over 174 result lines at 2026-10-12-headless-screenshot (»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-13-upstream-sync-polish-and-ship (the upstream merge of 434e0661, 16 commits): 179 result lines, 893 passed · 0 failed · 12 ignored (+44 passed: five new result lines, every one upstream's»
  replace-with: «`cargo test --workspace` 893 · 0 · 12 over 179 result lines at 2026-10-13-upstream-sync-polish-and-ship (the upstream merge of 434e0661: +44 passed — five new result lines, every one upstream's: blitz-tests' `anonymous_block_restyle` 10, `button_alignment` 16, `checked_attribute` 2, `pointer_leave` 7 and `ua_stylesheet_order` 3 — and dioxus-native-dom +6, now 67; the workspace's test executables 123 → 128, the unbuilt `all` none of them; the CI-scripts leg 79 tests, +1 upstream; timings not re-measured); 849 · 0 · 12 over 174 result lines at 2026-10-12-headless-screenshot (»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the clearing of the bridge's 27 boolean attributes when falsy»
  source: .andromeda/test-plan.md:26 «and of each of the 27 boolean attributes the check names — the bridge's own list until 2026-10-13-upstream-sync-polish-and-ship, its predicate since then upstream's 28-name `is_bool_attr` (`inert` added)»
  replace-with: «the clearing of the 27 boolean attributes the check names when falsy — the bridge's own list until 2026-10-13-upstream-sync-polish-and-ship, its predicate since then upstream's 28-name `is_bool_attr` (`inert` added), whether a check reads a falsy `inert` not measured»
