proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: `## 9. CI Integration`
    change: "Add one record after line 9, before the search record of line 10: ` Since 2026-10-24-a11y-re-read the leg runs fourteen `--test` targets, still one `cargo test --workspace --locked` line with no comment line — the six in their order, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` — on the founder's «Плюс проверки дерева (Рекомендую)» (2026-10-24, by question dialog in the overseer session, relayed verbatim by the overseer; passed over the tier-0 witnesses alone, 9 files, 59 tests), `focus_redraw`, `focus_within` and `accessibility_names` not added; its log 14 result lines · 84 passed · 0 failed · 0 ignored, the six unmoved (43) and the eight joining accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5 (41); its pin, `LegScriptTest`'s a11y tuple, names the fourteen, one `assertIn` each over the shimmed log, in no order; each joining file keeps every leg it already ran in — the workspace `test` leg and per package, `stand-repeat` for `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` (its named `--test` list) and for `stand_act_select` (its `stand_*.rs` glob); ci.yml is byte-identical — the job's slow tier, `needs`, restore-only `workspace-test` cache, failure-only log upload and no required status check unchanged — and CI#40134371519 on `4cbf0a4c9` read green, 17/17, the a11y job 14 · 84 · 0 · 0 (6 · 43 on CI#40125511019) (.github/scripts/ci-leg.sh:39; .github/scripts/test_ci_workflows.py:298-302 — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md).`"
    sidecar: "a11y-plan detail ci-integration: the a11y leg widened from six to fourteen --test targets (2026-10-24-a11y-re-read, inputs#I1), 14 · 84, pin at test_ci_workflows.py:298-302, CI#40134371519 a11y job 14 · 84 · 0 · 0."
    rationale: "Report Changes › Symbols / APIs (the fourteen in order, one line, each joining file's other legs kept), Counts / qualifiers moved (6 · 43 → 14 · 84 per file; CI per-job 14 · 84 · 0 · 0 on CI#40134371519; the pin's fourteen words, assertIn in no order, :298-302), Harness / gate surface (ci.yml byte-identical, job attributes kept), Cross-project claims (inputs#I1 the founder's word), Decisions (focus_redraw, focus_within, accessibility_names not added); Expected amendments names a11y-plan-detail/ci-integration.md:8 and :9. Entity shape: one record per fact, under 2 000 chars."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 8
    section: `## 9. CI Integration`
    change: "In line 8, date the six-file selection: `runs `bash .github/scripts/ci-leg.sh a11y` — since 2026-10-14-stand-a11y-assertions `cargo test … --test stand_accessibility_ids``, becomes `runs `bash .github/scripts/ci-leg.sh a11y` — from 2026-10-14-stand-a11y-assertions until 2026-10-24-a11y-re-read widened it to fourteen (below) `cargo test … --test stand_accessibility_ids``; and its pin cite `its pin, `LegScriptTest`'s a11y tuple, .github/scripts/test_ci_workflows.py:298-299` becomes `.github/scripts/test_ci_workflows.py:298-302` (the same tuple, now naming the fourteen). The rest of the line stays as dated; the line stays under 2 000 chars (1 882 now)."
    sidecar: "a11y-plan detail ci-integration:8: the six-file a11y selection dated until 2026-10-24-a11y-re-read; the pin's tuple cite 298-299 → 298-302."
    rationale: "Report Counts / qualifiers moved: site sweep rows `a11y-leg`, `a11y-tally` and `leg-pin` each list a11y-plan-detail/ci-integration.md:8 as changed (the six words, its log's six per-file counts, the pin at :298-299); the tuple's range 298-299 → 298-302 is a stretch the citation sweep does not re-point."
    basis: .github/scripts/test_ci_workflows.py:298-302
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: `## 9. CI Integration`
    change: "In line 9, keep the dated clause `the a11y leg's six files and its job are unchanged, CI#40114977936 reading the a11y job 6 · 43` and follow it with `; since 2026-10-24-a11y-re-read all three also run in the a11y leg, of fourteen files (below)`, so the line no longer reads the three as outside the leg."
    sidecar: "a11y-plan detail ci-integration:9: accessibility_states, inert_subtree and accessibility_active_descendant_unread now also in the a11y leg (2026-10-24-a11y-re-read)."
    rationale: "Report Counts / qualifiers moved: the site sweep lists a11y-plan-detail/ci-integration.md:9 as changed under `a11y-tally` and `outside-leg` (\"the a11y leg's six files and its job are unchanged, CI#40114977936 reading the a11y job 6 · 43\" — not on the plan's list); Symbols / APIs puts all three among the eight joining files, each keeping its `stand-repeat` place."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:9
    dependent-of: D-new-detail
note: Neither drift-base detector fires on this file (no UI element added, no violation or log schema changed); the body's own six-file sites a11y-plan.md:235 (§9) and :53-55 (§3) are outside this file and owed by the body's reconcile.
