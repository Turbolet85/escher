proposals:
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `accessibility_states` 5 clause, replace 'run by the workspace `test` leg, per package and the `stand-repeat` leg (a named `--test` of its list) and not by the `a11y` leg, which runs six files since 2026-10-14-stand-a11y-assertions, this one not among them' with 'run by the workspace `test` leg, per package, the `stand-repeat` leg (a named `--test` of its list) and, since 2026-10-24-a11y-re-read, the `a11y` leg (a named `--test` of its list, which runs fourteen files, six from 2026-10-14-stand-a11y-assertions until then, on the founder's «Плюс проверки дерева (Рекомендую)», 2026-10-24)'"
    sidecar: "test-scope-summary :97 — accessibility_states now run by the a11y leg (fourteen files since 2026-10-24-a11y-re-read); 'not by the a11y leg, which runs six files … not among them' retired"
    rationale: "The report's Symbols / APIs bullet: the a11y leg (`bash .github/scripts/ci-leg.sh a11y`) now runs fourteen `--test` targets, `accessibility_states` among the eight joining, keeping every leg it already ran in; its site sweep lists this line as changed under `a11y-leg` · `a11y-tally` · `outside-leg` (report.md:9, :22-24, :44; the founder's word inputs#I1, :35)"
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 98
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `accessibility_label` 13 / `accessibility_description` 3 clause, replace 'each its own auto-discovered target, run by the workspace `test` leg and per package and not by the `a11y` leg's six;' with 'each its own auto-discovered target, run by the workspace `test` leg, per package and, since 2026-10-24-a11y-re-read, the `a11y` leg (named `--test`s of its list);'"
    sidecar: "test-scope-summary :98 — accessibility_label and accessibility_description now run by the a11y leg since 2026-10-24-a11y-re-read; 'not by the a11y leg's six' retired"
    rationale: "Both files are among the eight that joined the a11y leg's fourteen (report.md:9); the site sweep lists :98 as changed (report.md:22, :24, :44)"
    basis: .github/scripts/ci-leg.sh:39
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 99
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `focus_within` 7 / `focus_redraw` 4 clause, replace 'and not by the `a11y` leg's six or `run stand`' with 'and not by the `a11y` leg (fourteen files since 2026-10-24-a11y-re-read, neither among them) or `run stand`' — both files stay outside the leg, the count 'six' is stale"
    sidecar: "test-scope-summary :99 — focus_within and focus_redraw stay outside the a11y leg, its count restated fourteen (2026-10-24-a11y-re-read)"
    rationale: "The leg now runs fourteen files and `focus_within` / `focus_redraw` were not added (report.md:9, :61 — 'focus_redraw, focus_within and accessibility_names not added'); the sweep lists :99 as changed, the 'six' stale, NOT on the plan's list (report.md:22, :44)"
    basis: .github/scripts/ci-leg.sh:39
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `hit_test_dropped_nodes` 1 clause (the line's first occurrence), replace 'and not by the `a11y` leg's six or `run stand`' with 'and not by the `a11y` leg (fourteen files since 2026-10-24-a11y-re-read, this one not among them) or `run stand`' — the file stays outside the leg, the count 'six' is stale"
    sidecar: "test-scope-summary :100 — hit_test_dropped_nodes stays outside the a11y leg, its count restated fourteen (2026-10-24-a11y-re-read)"
    rationale: "The fourteen named in report.md:9 do not include `hit_test_dropped_nodes`; the sweep names this clause of :100 as changed, 'six' stale (report.md:22, :24, :44)"
    basis: .github/scripts/ci-leg.sh:39
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `accessibility_merged_states` 3 / `accessibility_active_descendant_unread` 1 / `inert_subtree` 4 clause (the line's last occurrence), replace '`accessibility_merged_states` not — and not by the `a11y` leg's six;' with '`accessibility_merged_states` not — and, since 2026-10-24-a11y-re-read, by the `a11y` leg (named `--test`s of its list);'"
    sidecar: "test-scope-summary :100 — accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree now run by the a11y leg since 2026-10-24-a11y-re-read; 'not by the a11y leg's six' retired"
    rationale: "All three are among the eight joining files, each keeping its other legs (stand-repeat for the active-descendant and inert files, not for merged-states — report.md:9); the sweep names the trio's clause of :100 as joined (report.md:22, :24)"
    basis: .github/scripts/ci-leg.sh:39
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `select_interaction` 7 clause, replace '(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, by name and by `run all`, not by `run stand`)' with '(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, since 2026-10-24-a11y-re-read the `a11y` leg — a named `--test` of its list —, by name and by `run all`, not by `run stand`)'"
    sidecar: "test-scope-summary :125 — select_interaction's legs gain the a11y leg (2026-10-24-a11y-re-read)"
    rationale: "`select_interaction` joined the a11y leg (report.md:9); its clause enumerates every runner and now omits one — found by reading, outside every pattern, 'now owed' (report.md:28, :44)"
    basis: .github/scripts/ci-leg.sh:39
    dependent-of: D-tests-framework
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary › tests/blitz-tests"
    change: "In the `stand_act_select` 5 clause, after 'picked up by `run stand` and the stand-repeat leg by its prefix' (this clause's occurrence, before '— as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md'), add 'and, since 2026-10-24-a11y-re-read, run by the `a11y` leg (a named `--test` of its list), in both legs as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` are'"
    sidecar: "test-scope-summary :125 — stand_act_select run by the a11y leg since 2026-10-24-a11y-re-read, beside run stand and stand-repeat"
    rationale: "Report Symbols / APIs: 'stand_act_select now runs in both the a11y and stand-repeat legs, as stand_keyboard, stand_contrast and stand_accessibility_ids already did' (report.md:9); :125 named as owed, found by reading (report.md:28, :44)"
    basis: .github/scripts/ci-leg.sh:39
note: "Sweep's no-change rows hold here — :93 (stand_accessibility_ids), :116 (stand_contrast, stand_keyboard) and :120 (the a11y-leg file focusability_updates) stay true; every per-file count this file states for the eight joining files (5 · 13 · 3 · 3 · 1 · 4 · 7 · 5) equals report.md:16; the leg's tally 14 · 84 and its pin belong to ci-integration and unit-test-strategy, not this category; the body's §1 states no leg membership, so it needs no edit."
