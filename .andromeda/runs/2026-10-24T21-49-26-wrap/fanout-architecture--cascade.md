proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 255
    section: "## Standard Contracts"
    change: "In the `a11y` leg's parenthetical, replace \"the `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` integration tests — six since 2026-10-14-stand-a11y-assertions, three until then, widened on the founder's F4 «Расширить задание a11y (Рекомендую)» with no new job, no required status check and no `ci.yml` change\" with \"fourteen integration tests — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` — fourteen since 2026-10-24-a11y-re-read, six until then since 2026-10-14-stand-a11y-assertions, three before that, each widening with no new job, no required status check and no `ci.yml` change (the six on the founder's F4 «Расширить задание a11y (Рекомендую)»)\", and re-point its pin's citation `.github/scripts/test_ci_workflows.py:298-299` to `.github/scripts/test_ci_workflows.py:298-302`."
    sidecar: "standard-contracts CI contracts: the `a11y` leg six → fourteen named targets (2026-10-24-a11y-re-read), its pin cited at test_ci_workflows.py:298-302."
    rationale: "Report Changes → Symbols / APIs: the leg's selection now runs fourteen `--test` targets in the stated order; Counts / qualifiers moved: six → fourteen, 6 · 43 → 14 · 84; the pin's coordinates `:298-299` → `:298-302`, a stretch, not a move (so the citation sweep does not re-point it); the sweep names standard-contracts.md:255 under leg-pin as changed."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 255
    section: "## Standard Contracts"
    change: "Inside the `a11y` leg's parenthetical, after the fourteen names, add: \"the eight joining on the founder's «Плюс проверки дерева (Рекомендую)» (2026-10-24) — the five tree-reading checks `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states` and `accessibility_active_descendant_unread` beside the three tier-0 witnesses `inert_subtree`, `select_interaction` and `stand_act_select`, the six earlier files kept in their order\"."
    sidecar: "standard-contracts CI contracts: the founder's «Плюс проверки дерева (Рекомендую)» (2026-10-24) recorded as the source of the a11y leg's eight joining targets."
    rationale: "Report Cross-project / external claims: inputs#I1, the founder's word on the P4 fork, option (b), the leg 14 files, 84 tests, over (a) the tier-0 witnesses alone; Decisions & corrections: the five tree-reading checks join beside the three tier-0 witnesses."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 255
    section: "## Standard Contracts"
    change: "Inside the `a11y` leg's parenthetical, add: \"still one `cargo test --workspace --locked` line with no comment line; `focus_redraw`, `focus_within`, `accessibility_names` and `hit_test_dropped_nodes` stay outside it, run by the workspace `test` leg\"."
    sidecar: "standard-contracts CI contracts: the a11y leg stays one cargo line with no comment line; four a11y-adjacent checks named as outside it."
    rationale: "Report Changes → Symbols / APIs: it stays one `cargo test --workspace --locked` line with no comment line; Decisions & corrections: `focus_redraw`, `focus_within` and `accessibility_names` not added; the site sweep: `hit_test_dropped_nodes` stays outside the leg."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 255
    section: "## Standard Contracts"
    change: "After \"its pin `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`\" add \", whose a11y row asserts each of the fourteen target names by one `assertIn` over the shimmed log, in no order\"."
    sidecar: "standard-contracts CI contracts: the a11y pin's form stated — one assertIn per target name, in no order."
    rationale: "Report Counts / qualifiers moved: `LegScriptTest`'s a11y tuple names fourteen words, one `assertIn` each over the shimmed log, in no order; Spec claims disproved: the pin is one `assertIn` per word, in no order."
    basis: .github/scripts/test_ci_workflows.py:299-302
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the accessibility_states clause, replace \"and not by the `a11y` CI leg, which runs six named files since 2026-10-14-stand-a11y-assertions, this one not among them\" with \"and, since 2026-10-24-a11y-re-read, by the `a11y` CI leg, one of its fourteen named files (outside the leg's six until then)\"."
    sidecar: "existing-scopes blitz-tests: accessibility_states now runs in the a11y CI leg (2026-10-24-a11y-re-read), the \"not among the six\" clause retired."
    rationale: "Report Changes → Symbols / APIs: accessibility_states is among the eight joining the leg, keeping the workspace `test` leg, per package and `stand-repeat` by name; the site sweep names existing-scopes.md:102 changed under a11y-leg and outside-leg."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "After the inert_subtree clause's \"4 tests (tests/blitz-tests/tests/inert_subtree.rs:68-194)\" add \"— these three, with `accessibility_states`, `accessibility_label` and `accessibility_description`, run in the `a11y` CI leg since 2026-10-24-a11y-re-read, beside the workspace `test` leg and, for `accessibility_states`, `accessibility_active_descendant_unread` and `inert_subtree`, the `stand-repeat` leg\"."
    sidecar: "existing-scopes blitz-tests: the merged-states, active-descendant and inert checks (with accessibility_states, _label, _description) named as a11y-leg targets."
    rationale: "Report Changes → Symbols / APIs: the eight joining files keep every leg they already ran in — the workspace `test` leg for all eight, `stand-repeat` for accessibility_states, inert_subtree and accessibility_active_descendant_unread by name; the Touches note the merged-states, active-descendant and inert lines name no leg."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 111
    section: "## Existing Scopes"
    change: "After stand_act_select's \"5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:79-256)\" add \", run in both the `a11y` CI leg (since 2026-10-24-a11y-re-read) and the `stand-repeat` leg (by its `stand_*` prefix)\", and after select_interaction's \"7 tests in both layout modes (tests/blitz-tests/tests/select_interaction.rs:113-343)\" add \", in the `a11y` CI leg since 2026-10-24-a11y-re-read\"."
    sidecar: "existing-scopes blitz-tests: select_interaction and stand_act_select named as a11y-leg targets, stand_act_select in both a11y and stand-repeat."
    rationale: "Report Changes → Symbols / APIs: select_interaction and stand_act_select join the leg; stand_act_select now runs in both the `a11y` and `stand-repeat` legs, as stand_keyboard, stand_contrast and stand_accessibility_ids already did; Expected amendments: existing-scopes.md:111 holds no leg clause."
    fact: F4
note: "The +3 moves of standard-contracts.md citations into test_ci_workflows.py past line 299 (line 256's `:330`, line 257's `:103-499`, line 259's `:381-495`) are left to the citation sweep (`cites.py apply`), as the report assigns them; line 259's `:218-235` and the body's `:186-199` lie before the change and stand; no D-arch-resources or D-arch-decisions drift (no resource, crate, dependency or decision changed)."
