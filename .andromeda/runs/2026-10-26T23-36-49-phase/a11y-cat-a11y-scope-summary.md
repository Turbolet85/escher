# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19) The refactored `lint.rs::findings` must keep the load page's lint reading 0 findings, the served page's a11y reach its own checks pin (`stand_load_page`), so that check stays green unchanged.
  > their a11y reach is their own checks: keyed, `unkeyed_actionable` empty, the load page's lint 0 (examples/seven_guis/src/stand.rs:131-152; tests/blitz-tests/tests/stand_cells_served.rs:91-133; tests/blitz-tests/tests/stand_load_page.rs:70-84)
- (.andromeda/a11y-plan.md:158) The `findings` walk factored into helpers or a table must still read `contrast` through `Harness::text_contrast` under `stand_contrast`'s exempt rule, adding no second formula or threshold, and write a finding as ids, closed words and numbers only.
  > escher-driver's visual lint (`Session::lint`, the `lint` verb on every carrier) reads `contrast` through `Harness::text_contrast` with `stand_contrast`'s exempt rule and no second formula or threshold, a finding written as ids, closed words and numbers only.
- (.andromeda/a11y-plan.md:156) The exempt rule the lint's `contrast` reading must keep is this one, so the refactored walk exempts a node exactly where the snapshot's `enabled` reads `Some(false)` and uses the same 3.0 / 4.5 requirement.
  > `contrast_ratio` is the WCAG 2.x ratio, 3.0 required of large text, else 4.5; `stand_contrast` asserts three tables in both layout modes, a row exempt exactly where the snapshot's `enabled` reads `Some(false)`
- (.andromeda/a11y-plan.md:64) The lint stays `Harness::text_contrast`'s first non-test caller with the same exempt rule; the refactor must not fork the contrast reading out of the shared reader.
  > Since 2026-10-19-visual-lint its first non-test caller is escher-driver's visual lint (`Session::lint`, in process and as the `lint` verb on every carrier), whose `contrast` reading keeps the same exempt rule (§6).

## Bindings
- **Served pages' lint 0 ↔ body §6 Color contrast**: the load page's lint-0 reading (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19) is produced by the same `findings` walk whose `contrast` defect reads through `Harness::text_contrast` under `stand_contrast`'s exempt rule (.andromeda/a11y-plan.md:158); a refactor that moved either side would move the other.
- **Lint contrast reading ↔ body §3 Contrast verification harness**: the lint's contrast branch and `stand_contrast` share one reader and one exempt rule (.andromeda/a11y-plan.md:64; .andromeda/a11y-plan.md:156) — the chunk's "no behaviour changes" boundary holds both sides.
