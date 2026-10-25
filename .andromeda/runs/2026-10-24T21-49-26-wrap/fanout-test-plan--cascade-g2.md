proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary"
    change: "In the `accessibility_states` clause replace «and not by the `a11y` leg, which runs six files since 2026-10-14-stand-a11y-assertions, this one not among them» with «and, since 2026-10-24-a11y-re-read, by the `a11y` leg, which runs fourteen files (six until then)» — the workspace `test` leg, per package and `stand-repeat` kept as written."
    sidecar: "test-scope-summary §1: accessibility_states now run by the a11y leg (fourteen files since 2026-10-24-a11y-re-read); the six-file exclusion retired."
    rationale: "report.md:9 names accessibility_states among the eight files joining the a11y leg's fourteen and keeps its test, per-package and stand-repeat legs; report.md:22 disposes this line as changed under the a11y-leg and outside-leg patterns; report.md:44 carries it as an expected amendment."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:97
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 98
    section: "## 1. Test Scope Summary"
    change: "In the `accessibility_label` / `accessibility_description` clause replace «run by the workspace `test` leg and per package and not by the `a11y` leg's six» with «run by the workspace `test` leg, per package and, since 2026-10-24-a11y-re-read, the `a11y` leg (fourteen files)»."
    sidecar: "test-scope-summary §1: accessibility_label and accessibility_description now run by the a11y leg since 2026-10-24-a11y-re-read."
    rationale: "report.md:9 lists accessibility_label and accessibility_description among the eight joining files; report.md:22 disposes :98 as changed (not by the a11y leg's six); report.md:44 carries it."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:98
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 99
    section: "## 1. Test Scope Summary"
    change: "In the `focus_within` / `focus_redraw` clause replace «not by the `a11y` leg's six or `run stand`» with «not by the `a11y` leg — fourteen files since 2026-10-24-a11y-re-read, these two deliberately not added (inputs#I1) — or `run stand`»; both files stay outside the leg."
    sidecar: "test-scope-summary §1: focus_within and focus_redraw stay outside the a11y leg, its count restated six → fourteen, their exclusion the founder's choice at 2026-10-24-a11y-re-read."
    rationale: "report.md:22 disposes :99 as changed — both stay outside the leg, the count six stale; report.md:61 records focus_redraw and focus_within as not added under the founder's word inputs#I1; report.md:44 adds :99 from the sweep."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:99
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "In the `hit_test_dropped_nodes` clause replace «not by the `a11y` leg's six or `run stand`» with «not by the `a11y` leg's fourteen (since 2026-10-24-a11y-re-read) or `run stand`»; the file stays outside the leg."
    sidecar: "test-scope-summary §1: hit_test_dropped_nodes stays outside the a11y leg, its count restated six → fourteen."
    rationale: "report.md:22 disposes :100's hit_test_dropped_nodes clause as changed — stays outside, six stale; report.md:9's fourteen do not include it; report.md:44 names the clause."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:100
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "In the `accessibility_merged_states` / `accessibility_active_descendant_unread` / `inert_subtree` clause replace «— and not by the `a11y` leg's six» with «— and, since 2026-10-24-a11y-re-read, by the `a11y` leg (fourteen files)», the stand-repeat placement (`accessibility_active_descendant_unread` and `inert_subtree` named, `accessibility_merged_states` not) kept as written."
    sidecar: "test-scope-summary §1: accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree now run by the a11y leg since 2026-10-24-a11y-re-read."
    rationale: "report.md:9 lists all three among the eight joining files and keeps stand-repeat for inert_subtree and accessibility_active_descendant_unread alone; report.md:22 disposes the trio on :100 as joined."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:100
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary"
    change: "In the `select_interaction` clause replace «(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, by name and by `run all`, not by `run stand`)» with «(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, since 2026-10-24-a11y-re-read the `a11y` leg, by name and by `run all`, not by `run stand`)»."
    sidecar: "test-scope-summary §1: select_interaction now also run by the a11y leg since 2026-10-24-a11y-re-read."
    rationale: "report.md:9 names select_interaction among the eight joining files, keeping test, per-package and stand-repeat; report.md:28 finds :125 by reading, the a11y leg now owed; report.md:44 carries :125."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:125
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary"
    change: "In the `stand_act_select` clause replace «picked up by `run stand` and the stand-repeat leg by its prefix» with «picked up by `run stand` and the stand-repeat leg by its prefix and, since 2026-10-24-a11y-re-read, run by the `a11y` leg, a named `--test` of its fourteen»."
    sidecar: "test-scope-summary §1: stand_act_select now runs in both the a11y and stand-repeat legs since 2026-10-24-a11y-re-read."
    rationale: "report.md:9: stand_act_select joins the a11y leg and keeps stand-repeat (its glob) and run stand, so it runs in both legs as stand_keyboard, stand_contrast and stand_accessibility_ids do; report.md:28 finds :125 owed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:125
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary"
    change: "After the two named `aria-checked` tests of `accessibility_states` add «— the one file asserting `aria-checked`, `accessibility_merged_states` reading `aria-expanded` · `aria-disabled` · `aria-level` and not it (as read at 2026-10-24-a11y-re-read)»."
    sidecar: "test-scope-summary §1: aria-checked is asserted by accessibility_states alone, not by accessibility_merged_states (2026-10-24-a11y-re-read)."
    rationale: "report.md:59 records the witness claim corrected by reading the test names: aria-checked is asserted by accessibility_states alone, the merged-states file reading aria-expanded, aria-disabled and aria-level."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:97
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary"
    change: "After «no stand binary builds an adapter and the dev host's AT-SPI bus reads inactive» add «— the stand's normal dependency graph holding neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal` probe 0, 2026-10-24-a11y-re-read), so no check in the `a11y` leg reaches the adapter crossing»."
    sidecar: "test-scope-summary §1: the stand graph holds neither platform accessibility adapter (cargo tree probe 0), so no a11y-leg check reaches the adapter crossing (2026-10-24-a11y-re-read)."
    rationale: "report.md:11 and the Outcome's gate at report.md:85 read `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` → 0; the entity's adapter-gap clause on :97 is where the fact belongs."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:97
    fact: F5
note: "test-scope-summary.md:93, :116, :119 and :120 share the a11y-leg token and still hold (report.md:22 rules :93 and :116 no-change; stand_requirement_sweep green with stand-repeat's line unchanged, report.md:31); the test-plan body carries no broad a11y-leg line in §1 to edit."
