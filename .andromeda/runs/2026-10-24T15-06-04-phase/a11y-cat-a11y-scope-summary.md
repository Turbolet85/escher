# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan.md:5) Every a11y-bearing 0.2.0 requirement the sweep re-proves must be measured against the tier-0 target's three criteria, and no other criterion is claimed as met.
  > **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
- (.andromeda/a11y-plan.md:9) The sweep's record must name only headless witnesses for the accessibility tree's capabilities: no stand binary builds a platform adapter, so the record must not claim an assistive-technology witness, and that owed witness stays with its residuals line, not with this chunk.
  > no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary — the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:17) The proof the sweep may cite for the tree is the headless half proven on the stand. The windowed AT witness is owed to a residuals line and must stay there, neither built nor re-owned by this chunk.
  > the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next), the headless halves, the tree the shell hands the adapter carrying stable ids only and the refresh, proven on the stand at 2026-10-14-stand-a11y-assertions (§2, §7)
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:18) The sweep's new checks and any target or leg change must keep the stand graph free of both platform adapters. The measured zero-row reading stays true at this chunk's end.
  > the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) Any fixture check the sweep adds in blitz-tests that reads the accessibility tree or the snapshot (C1's lint fixtures, a `stand_*` check carrying a fixture witness) relies on the crate building these two with the `accessibility` feature. That configuration must hold.
  > - **blitz-tests** — `blitz-dom` and `dioxus-native-dom` are built with the `accessibility` feature and `accesskit` is a dev-dependency (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/Cargo.toml:21; tests/blitz-tests/Cargo.toml:33)
- (.andromeda/a11y-plan.md:13) The sweep's a11y rows must name the tree's witnesses in the harness this line specifies (hidden-element exclusion, role mapping), whose detail lives in §3.
  > **A11y assertion harness specification:** accessibility tests cover hidden-element exclusion and HTML-to-AccessKit role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:8-137; tests/blitz-tests/tests/accessibility_roles.rs:1-7); the harness is detailed in §3.
- (.andromeda/a11y-plan.md:53) `accessibility_states` is one of the fixture witnesses the scope names as outside `stand-repeat`. If the fork brings it inside, this placement text moves with it.
  > it runs in the workspace test leg and per package, not in the `a11y` leg, which runs six files since 2026-10-14-stand-a11y-assertions (§9)
- (.andromeda/a11y-plan.md:55) `inert_subtree` and `accessibility_active_descendant_unread` are fixture witnesses outside `stand-repeat`. However the sweep brings them inside, it must keep their stated tables and counts and both layout modes.
  > `accessibility_merged_states` (3), `inert_subtree` (4) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes
- (.andromeda/a11y-plan.md:55) The leg placement of those three files is stated here. A widened `stand-repeat` selection, or a witness carried by a `stand_*` check, moves this text and its citations.
  > — in the workspace test leg and per package, outside the a11y leg's six files (§9) (tests/blitz-tests/tests/accessibility_merged_states.rs:88-157; tests/blitz-tests/tests/inert_subtree.rs:68-194; tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81)
- (.andromeda/a11y-plan.md:133) The active-descendant pin must survive the sweep unchanged. Whichever leg it ends in, it keeps asserting that the builder reads no `aria-activedescendant`.
  > the builder reads no `aria-activedescendant` (SC 4.1.2 work outside the tier-0 target), kept unread and pinned by `accessibility_active_descendant_unread`.
- (.andromeda/a11y-plan.md:156) Any lint check the sweep adds for C1 must assert findings as ids, closed words and numbers only. The lint's `contrast` reading keeps the one formula and exempt rule, with no second threshold introduced.
  > - escher-driver's visual lint (`Session::lint`, the `lint` verb on every carrier) reads `contrast` through `Harness::text_contrast` with `stand_contrast`'s exempt rule and no second formula or threshold, a finding written as ids, closed words and numbers only.
- (.andromeda/a11y-plan.md:188) A stand check the sweep adds that expects roles or names must take them from the shared module's tables, never from a second mapping.
  > the roles and names the checks expect are stated once, in the shared module's tables — never a second role or name mapping.
- (.andromeda/a11y-plan.md:235) The three a11y `stand_*` files already run in `stand-repeat` under the zero-failure budget, and they must keep running there. A change to the leg's selection moves this §9 line.
  > the three `stand_*` files also run three times in the `stand-repeat` leg under the zero-failure budget (test-plan §10).

## Bindings
- **No AT witness claimed, adapter owed ↔ `.andromeda/residuals.md` and screen-reader-support**: the sweep's record names headless witnesses only. The windowed adapter witness stays on its residuals line (target: next), and «A11y re-read» (scope Boundaries) owns any re-assertion. (.andromeda/a11y-plan-detail/a11y-scope-summary.md:17)
- **Stand graph without platform adapters ↔ security plan / arch §Occupied Resources**: adding no workspace crate or dependency keeps the zero-row reading. A dependency change would be a registration first. (.andromeda/a11y-plan-detail/a11y-scope-summary.md:18)
- **blitz-tests' `accessibility` feature ↔ test plan's target pin (`.github/scripts/test_blitz_tests_targets.py`)**: any new fixture check is one file and one target, built under the feature this line records. (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20)
- **Fixture a11y witnesses' leg placement ↔ ci-integration category and test-plan §10 (`stand-repeat`)**: if the P4 fork widens the leg's selection, the §3 placement lines and §9 line move together with `.github/scripts/ci-leg.sh` and `.github/scripts/test_ci_workflows.py`. (.andromeda/a11y-plan.md:55)
- **Lint contrast rule ↔ visual-design-verification category (`stand_contrast`'s exempt rule)**: C1's lint fixtures assert through the one reader. (.andromeda/a11y-plan.md:156)
- **Expected roles and names ↔ `tests/blitz-tests/tests/common/mod.rs` tables**: they are stated once and reused by any new stand check. (.andromeda/a11y-plan.md:188)
