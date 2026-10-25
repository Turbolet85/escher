proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the blitz-tests unit's `accessibility_states` clause, replace \"and not by the `a11y` CI leg, which runs six named files since 2026-10-14-stand-a11y-assertions, this one not among them\" with \"and, since 2026-10-24-a11y-re-read, by the `a11y` CI leg, which runs fourteen named files (six from 2026-10-14-stand-a11y-assertions until then)\"; the rest of the clause (workspace test leg, per package, `stand-repeat`'s named `--test` list) unchanged."
    sidecar: "existing-scopes: accessibility_states now run by the a11y CI leg (fourteen files) since 2026-10-24-a11y-re-read; the stale \"six named files … not among them\" retired."
    rationale: "Report Changes › Symbols / APIs: the `a11y` leg now runs fourteen `--test` targets, `accessibility_states` among the eight joining, each keeping its other legs; Counts / qualifiers moved › site sweep lists `architecture-detail/existing-scopes.md:102` as changed under `a11y-leg` / `outside-leg`; Expected amendments name `existing-scopes.md:102`."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the same line, after \"(tests/blitz-tests/tests/inert_subtree.rs:68-194), each in both layout modes\" add \", the three — `accessibility_merged_states`, `accessibility_active_descendant_unread` and `inert_subtree` — run in the `a11y` CI leg since 2026-10-24-a11y-re-read\"."
    sidecar: "existing-scopes: accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree named in the a11y CI leg since 2026-10-24-a11y-re-read."
    rationale: "Report Changes › Symbols / APIs: these three are among the eight files joining the `a11y` leg; the site sweep notes the merged-states, active-descendant and inert lines on existing-scopes.md:102 name no leg, the line counted changed."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 111
    section: "## Existing Scopes"
    change: "In the `stand_act_select` clause, after \"5 tests in both layout modes\" add \", run in the `a11y` CI leg since 2026-10-24-a11y-re-read as well as in `stand-repeat` and `run stand` by its prefix\" (before its citation tests/blitz-tests/tests/stand_act_select.rs:79-256)."
    sidecar: "existing-scopes: stand_act_select named in the a11y CI leg (beside stand-repeat and run stand) since 2026-10-24-a11y-re-read."
    rationale: "Report Changes › Symbols / APIs: `stand_act_select` joined the `a11y` leg and keeps `stand-repeat` (its `stand_*.rs` glob) and `run stand`, now in both legs as `stand_keyboard` and `stand_contrast` are — which this file already names in the leg (lines 106, 112); Expected amendments leave whether existing-scopes.md:111 gains a leg clause to the detector."
    basis: .github/scripts/ci-leg.sh:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 111
    section: "## Existing Scopes"
    change: "In the engine check `select_interaction` clause, after \"7 tests in both layout modes\" add \", run in the `a11y` CI leg since 2026-10-24-a11y-re-read and in `stand-repeat`'s named `--test` list\" (before its citation tests/blitz-tests/tests/select_interaction.rs:113-343)."
    sidecar: "existing-scopes: select_interaction named in the a11y CI leg and stand-repeat's named list since 2026-10-24-a11y-re-read."
    rationale: "Report Changes › Symbols / APIs: `select_interaction` is among the eight joining the `a11y` leg and keeps `stand-repeat` by its named `--test` list (`.github/scripts/ci-leg.sh:55-58`); existing-scopes.md:111 names no leg for it."
    basis: .github/scripts/ci-leg.sh:55-58
note: "accessibility_label and accessibility_description also joined the a11y leg but have no entry in this file (a pre-existing coverage gap, not stale text); no new resource, crate or dependency landed, so D-arch-resources and D-arch-decisions fire nothing here."
