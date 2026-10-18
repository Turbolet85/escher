group: test-plan
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «`aria-checked` setting none; no check reads them at a platform adapter)»
  source: .andromeda/test-plan.md:26 «two checks of the builder's merged name and description reads on a parsed page, asserted through test-that's `verify_that!`: `accessibility_label` 10»
  replace-with: «`aria-checked` setting none; no check reads them at a platform adapter; and, since 2026-10-17-upstream-sync-engine-debts's upstream merge, `accessibility_label` (10) and `accessibility_description` (3), the builder's merged name and description reads on a parsed page asserted through test-that's `verify_that!`, run by the workspace `test` leg and per package, not by the `a11y` leg; no test names the merged `aria-expanded`, `aria-disabled` or `aria-level` reads)»

- leaf: .claude/docs/tests-summary.md:30   class: omission
  leaf-text: «`cargo test --workspace` 951 · 0 · 12 over 185 result lines at 2026-10-14-stand-requirement-sweep (»
  source: .andromeda/test-plan.md:331 «re-counted at 2026-10-17-upstream-sync-engine-debts: 187 result lines, 971 passed · 0 failed · 12 ignored, from 186 · 958 (+13 passed, +2 −1 result lines»
  replace-with: «`cargo test --workspace` 971 · 0 · 12 over 187 result lines at 2026-10-17-upstream-sync-engine-debts (from 186 · 958: +13 passed, +2 −1 result lines — blitz-tests' `accessibility_description` 3 and `accessibility_label` 10, upstream's, in two new result lines, the bump crate's empty `unittests src/main.rs` line gone; `run stand` unmoved at 168 passed · 7 ignored over 40 files, `run all` 546 passed · 0 failed · 12 ignored; timings not re-measured); 951 · 0 · 12 over 185 result lines at 2026-10-14-stand-requirement-sweep (»

- leaf: .claude/docs/tests-summary.md:42   class: omission
  leaf-text: «`ci-leg.sh coverage` (cargo-llvm-cov, then `.github/scripts/coverage_floor.py`»
  source: .andromeda/test-plan.md:343 «Since 2026-10-17-upstream-sync-engine-debts both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, excluding dioxus-native-dom's crate-private `keyboard_compat.rs`»
  replace-with: «`ci-leg.sh coverage` (cargo-llvm-cov, both `cargo llvm-cov` lines taking `--ignore-filename-regex 'keyboard_compat\.rs$'` since 2026-10-17-upstream-sync-engine-debts — dioxus-native-dom's crate-private keyboard-types 0.7/0.8 bridge excluded until upstream retires it, the floors unchanged, the merged tree reading 98.19 %, 84.25 %, 91.69 % —, then `.github/scripts/coverage_floor.py`»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «No mocking library, property-test or snapshot crate is used»
  source: .andromeda/test-plan.md:83 «blitz-tests carries the assertion crate test-that 0.6.0 with test-that-macro 0.6.0 as dev-dependencies»
  replace-with: «No mocking library, property-test or snapshot crate is used (blitz-tests carries the assertion crate test-that 0.6.0 with test-that-macro 0.6.0 as dev-dependencies, 0.5.2 until 2026-10-17-upstream-sync-engine-debts, whose `verify_that!` and `TestResult<()>` upstream's merged `accessibility_label` and `accessibility_description` checks assert through under the same `#[test]` harness)»

- leaf: .claude/rules/verification-harness.md:31   class: omission
  leaf-text: «the crate re-exports `Key` and `Modifiers`, the types `press_with` takes.»
  source: .andromeda/test-plan.md:88 «the crate also re-exports keyboard-types 0.8's `Key`, `Modifiers` and, since 2026-10-17-upstream-sync-engine-debts's move to keyboard-types "0.8", `NamedKey`»
  replace-with: «the crate re-exports keyboard-types 0.8's `Key`, `Modifiers` and, since 2026-10-17-upstream-sync-engine-debts, `NamedKey` — the two types `press_with` takes and the named-key enum a 0.8 `Key::Named` holds.»
