proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 235
    section: "## 9. CI Integration"
    change: "ci.yml's `a11y` job, \"Accessibility (a11y) tests\", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over fourteen files, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (the founder's choice, 2026-10-24) — in the slow tier behind the four fast jobs, with no required status check; eight of them — the four `stand_*` files, `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` — also run three times in the `stand-repeat` leg under the zero-failure budget (test-plan §10)."
    sidecar: "§9 broad line: the a11y leg is now fourteen files, not six; the files it shares with stand-repeat are now eight (the four stand_* files plus the four on stand-repeat's named list), not the three stand_* files."
    rationale: "The report's Symbols / APIs says the leg now runs fourteen `--test` targets in the order given, and that `stand_act_select` (by its glob) and `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `select_interaction` (by its named list) run in stand-repeat. Its site sweep lists `a11y-plan.md:235` (\"over six files\") as changed under `a11y-leg` and `a11y-tally`, and Expected amendments carries §9 body :235. The founder's word is inputs#I1."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "Mark line 8's six-file command and per-file log as the leg \"from 2026-10-14-stand-a11y-assertions until 2026-10-24-a11y-re-read\", and its closing \"the a11y leg and job are unchanged, their per-file counts as above\" as true at 2026-10-14-quality-gates. Re-point the pin citation `.github/scripts/test_ci_workflows.py:298-299` → `.github/scripts/test_ci_workflows.py:298-302`: `LegScriptTest`'s a11y tuple, which now names fourteen words, one `assertIn` each over the shimmed log, in no order."
    sidecar: "ci-integration :8: the six-file leg is now dated history (until 2026-10-24-a11y-re-read); the a11y tuple's pin citation is re-pointed to :298-302."
    rationale: "The report's Changes / Files say the a11y row of `LegScriptTest` now spans lines 298-302 (was 298-299; added 299-302 per New text, by line). Counts / qualifiers moved calls the range change a stretch, not a move, so the citation sweep does not re-point it. The site sweep lists `a11y-plan-detail/ci-integration.md:8` as changed under `a11y-leg`, `a11y-tally` and `leg-pin`."
    basis: .github/scripts/test_ci_workflows.py:299-302
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Mark line 9's \"the a11y leg's six files and its job are unchanged, CI#40114977936 reading the a11y job 6 · 43\" as a reading of that chunk: \"the a11y leg's six files and its job were then unchanged, CI#40114977936 reading the a11y job 6 · 43\"."
    sidecar: "ci-integration :9: the six-file, unchanged-job statement is dated to 2026-10-24-stand-requirement-sweep."
    rationale: "The report's site sweep lists `a11y-plan-detail/ci-integration.md:9` as changed under `a11y-leg`, `a11y-tally` and `outside-leg` (Expected amendments: \"The sweep adds `a11y-plan-detail/ci-integration.md:9`\"). The leg's files moved six → fourteen; the job's `ci.yml` properties did not (Harness / gate surface: `ci.yml` byte-identical)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: " Since 2026-10-24-a11y-re-read `bash .github/scripts/ci-leg.sh a11y` runs fourteen `--test` targets, still one `cargo test --workspace --locked` line with no comment line: the six, order kept, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select`. Its log reads the six unmoved (43) and accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5 (41), so 14 result lines · 84 passed · 0 failed · 0 ignored, both locally and on the \"Accessibility (a11y) tests\" job of CI#40134371519 (6 · 43 on CI#40125511019); every other job's tally is unmoved. The job keeps its slow tier, its `needs`, its restore-only `workspace-test` cache, its failure-only log upload and no required status check. Its pin, `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`'s a11y row, names the fourteen words, one `assertIn` each over the shimmed log, in no order (.github/scripts/ci-leg.sh:39; .github/scripts/test_ci_workflows.py:298-302 — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md and its evidence/counts.md, evidence/ci.md)."
    sidecar: "ci-integration: the fourteen-file a11y leg added, with its per-file counts, its 14 · 84 tally (local and CI#40134371519) and its pin at :298-302, in no order."
    rationale: "The report's Symbols / APIs (the fourteen in order, one line, no comment line), Counts / qualifiers moved (the per-file counts, the 6 · 43 → 14 · 84 local and CI tallies, the pin's fourteen words in no order) and Harness / gate surface (`ci.yml` byte-identical, the job's properties kept). The task's Touches lines 22-27."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: " The fourteen are the founder's choice, «Плюс проверки дерева (Рекомендую)» (2026-10-24, by question dialog in the overseer session, relayed verbatim by the overseer; inputs#I1). He took option (b): the five tree-reading checks join the three tier-0 witnesses, 14 files and 84 tests. He passed over option (a), the tier-0 witnesses alone (9 files, 59 tests). The leg already ran the non-tier-0 `accessibility_hidden` and `accessibility_roles` (as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md)."
    sidecar: "ci-integration: the founder's word on the leg's composition recorded (inputs#I1, option b over option a)."
    rationale: "The report's Cross-project / external claims (inputs.py verify, inputs#I1) and Decisions & corrections (the P4 fork's answer)."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: " `focus_redraw`, `focus_within` and `accessibility_names` were deliberately left out of the `a11y` leg and run outside it: in the workspace `test` leg and per package (the plan's constraints, as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md)."
    sidecar: "ci-integration: the three a11y-adjacent files deliberately left out of the leg are named."
    rationale: "The report's Decisions & corrections: \"`focus_redraw`, `focus_within` and `accessibility_names` not added (plan.md Constraints)\"."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: " Each joining file witnesses an SC 2.1.1 or 2.4.3 requirement, a tree reading the founder took, or a recorded gap (`accessibility_active_descendant_unread`, the pinned unread `aria-activedescendant`). Each keeps every leg it already ran in: all eight stay in the workspace `test` leg and per package. `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` keep their three `stand-repeat` rounds by its named `--test` list (.github/scripts/ci-leg.sh:54-58). `stand_act_select` keeps its rounds by the `stand_*` glob and its place in `run stand`. So `stand_act_select` now runs in both the `a11y` and `stand-repeat` legs, as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` already did (as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md)."
    sidecar: "ci-integration: why each joining file belongs in the a11y leg, and that each keeps its other legs; stand_act_select is the fourth stand_* file in both a11y and stand-repeat."
    rationale: "The report's Outcome, the second a11y criterion (each added file a witness, a tree reading or a recorded gap), and Symbols / APIs (each joining file keeps its legs; `stand_act_select` in both legs)."
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: " At the same chunk the stand's Tab order, Enter/Space activation and contrast were re-read after 0.2.0's engine changes, and nothing moved: `stand_keyboard` 14, `stand_contrast` 5 and `stand_accessibility_ids` 8 green in the leg, per package and in each `stand-repeat` round, with `tab_order`, `ENABLED_TAB_ORDER`, `controls`, `TEXT_ROWS` and `STATE_ROWS` unedited and nothing re-baselined. Each 0.2.0 engine change is recorded with its witness, by test function, in escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md. `aria-checked` is witnessed by `accessibility_states` alone; `accessibility_merged_states` reads `aria-expanded`, `aria-disabled` and `aria-level`."
    sidecar: "ci-integration: the post-0.2.0 a11y re-read and its per-change witness record are cited; aria-checked's witness is named precisely."
    rationale: "The report's Outcome, the third a11y criterion (the re-read, the tables unedited, nothing re-baselined), and Decisions & corrections (the witness claim corrected: `aria-checked` asserted by `accessibility_states` alone)."
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "Replace \"— neither file in the a11y leg's six (§9), both three rounds in the `stand-repeat` leg, …\" with \"— both in the `a11y` leg since 2026-10-24-a11y-re-read, among its fourteen files (§9; .github/scripts/ci-leg.sh:39), and both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list (.github/scripts/ci-leg.sh:54-58) — the a11y leg as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md\"."
    sidecar: "keyboard-navigation :100: select_interaction and stand_act_select are now in the a11y leg; each keeps its stand-repeat place."
    rationale: "The report's Symbols / APIs: `select_interaction` and `stand_act_select` join the leg; `select_interaction` keeps stand-repeat's named list and `stand_act_select` its glob. The site sweep lists `a11y-plan-detail/keyboard-navigation.md:100` (\"neither file in the a11y leg's six\") as changed under `a11y-leg` and `outside-leg`; Expected amendments carries it."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Keep the dated \"the `a11y` leg reads 43 tests over 6 targets, unmoved\" (as measured at 2026-10-19-visual-lint) and add after the line's citation: \"; since 2026-10-24-a11y-re-read the leg reads 84 tests over 14 targets, `stand_contrast`'s 5 among them and unmoved (§9) — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md and its evidence/counts.md\"."
    sidecar: "visual-design-verification :14: the newer 84-over-14 a11y-leg reading added after the dated 43-over-6 one."
    rationale: "The report's Counts / qualifiers moved (the leg 6 · 43 → 14 · 84; `stand_contrast` 5 unmoved) and its site sweep, which lists `a11y-plan-detail/visual-design-verification.md:14` as changed under `a11y-leg` and `a11y-tally` (\"as measured at 2026-10-19-visual-lint\" — a newer reading owed). Expected amendments carries it."
    basis: .github/scripts/ci-leg.sh:39
note: "The body's §3 lines 53, 54 and 55 (\"not in the `a11y` leg, which runs six files\", \"outside the a11y leg's six files\", \"all three outside the a11y leg's six files\") are stale by the report's site sweep and are carried by its Expected amendments. §3 has no detail file, and none of its lines is the broad line of a picked entity's group, so this pass does not propose them. They need a body edit: all eight files are now in the leg."
