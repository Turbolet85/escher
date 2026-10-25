proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 28
    section: '## 4. Unit Test Strategy'
    change: 'replace "the `a11y` row pinning all six test files in order, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, the last three added at 2026-10-14-stand-a11y-assertions (.github/scripts/test_ci_workflows.py:298-299)" with "the `a11y` row naming the leg''s fourteen test files, one `assertIn` per word over the shimmed log, in no order — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` (the last three added at 2026-10-14-stand-a11y-assertions), then, since 2026-10-24-a11y-re-read, `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (.github/scripts/test_ci_workflows.py:298-302)"; test_ci_workflows.py''s 33 and the leg''s 100 unchanged'
    sidecar: 'unit-test-strategy.md:28 — the a11y pin row: six words "in order" → fourteen words, one assertIn each, in no order; :298-299 → :298-302 (2026-10-24-a11y-re-read)'
    rationale: 'The report says the pin''s a11y tuple names fourteen words, one `assertIn` each over the shimmed log, in no order, and that "pinning all six test files in order" is stale on both counts. Its coordinates move from :298-299 to :298-302, a stretch the citation sweep does not re-point (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:18, :38, :45). `Ran 33 tests` and the leg''s 100 did not move (report.md:20).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: '## 9. CI Integration'
    change: 'replace "a11y (`--test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids`, six files since 2026-10-14-stand-a11y-assertions, three until then; the a11y job''s definition unchanged)" with "a11y (`--test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibility_states --test accessibility_label --test accessibility_description --test accessibility_merged_states --test accessibility_active_descendant_unread --test inert_subtree --test select_interaction --test stand_act_select`, still one `cargo test --workspace --locked` line, fourteen files since 2026-10-24-a11y-re-read (.github/scripts/ci-leg.sh:39), six since 2026-10-14-stand-a11y-assertions, three until then; each joining file keeps every leg it already ran in, so `stand_act_select` runs in both the a11y and stand-repeat legs as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do; the a11y job''s definition unchanged)"'
    sidecar: 'ci-integration.md:8 — the a11y leg: six --test words → fourteen, the eight joining after the six in the order kept (2026-10-24-a11y-re-read)'
    rationale: 'The report''s Changes say ci-leg.sh line 39 was replaced, and the leg now runs fourteen `--test` targets in the named order on one `cargo test --workspace --locked` line. Every joining file keeps its other legs, and `stand_act_select` now runs in both the a11y and stand-repeat legs. ci.yml is byte-identical (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:8, :9, :31, :43).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: '## 9. CI Integration'
    change: 'inside the a11y clause, after the fourteen words, add: "its membership set by the founder''s word of 2026-10-24 (inputs#I1 of escher-0.2.0/chunks/2026-10-24-a11y-re-read, «Плюс проверки дерева (Рекомендую)»): option (b), the tier-0 witnesses plus the five tree-reading checks, 14 files and 84 tests, chosen over (a), the tier-0 witnesses alone, 9 files and 59 tests; `focus_redraw`, `focus_within` and `accessibility_names` deliberately not added"'
    sidecar: 'ci-integration.md:8 — the a11y leg''s membership: the founder''s inputs#I1 option (b), 14 files · 84 tests, over (a) 9 · 59; three files deliberately left out (2026-10-24-a11y-re-read)'
    rationale: 'The report''s Cross-project claims and Decisions record the founder''s word on the P4 fork, option (b) over (a), with focus_redraw, focus_within and accessibility_names not added (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:35, :61).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:35
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: '## 9. CI Integration'
    change: 'inside the a11y clause, after the membership, add: "each of its files witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading the founder took, or a recorded gap — `accessibility_active_descendant_unread` the leg''s pinned-gap witness"'
    sidecar: 'ci-integration.md:8 — the a11y leg''s witness rule: SC 2.1.1 / 2.4.3, a ratified tree reading, or a recorded gap (accessibility_active_descendant_unread) (2026-10-24-a11y-re-read)'
    rationale: 'Outcome criterion: every added file witnesses an SC 2.1.1 / 2.4.3 requirement, a tree reading the founder took, or a recorded gap. MET by construction of the list, with `accessibility_active_descendant_unread` as the pinned gap (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:66).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:66
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 42
    section: '## 9. CI Integration'
    change: 'after the CI#40114977936 reading, add: "; re-read with the a11y leg''s fourteen files on CI#40134371519, the pre-CI commit `4cbf0a4c9` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 14 · 84 · 0 · 0, from 6 · 43; the stand repeat 198 · 837 · 0 · 21; ios and android build only — every other job equal to the base''s CI#40125511019 (`db585b711`), and the a11y job +8 result lines and +41 passed, the eight joining files (accessibility_states 5, accessibility_label 13, accessibility_description 3, accessibility_merged_states 3, accessibility_active_descendant_unread 1, inert_subtree 4, select_interaction 7, stand_act_select 5) — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md"'
    sidecar: 'ci-integration.md:42 — per-job tallies re-read on CI#40134371519: a11y job 6 · 43 → 14 · 84, every other job unmoved (2026-10-24-a11y-re-read)'
    rationale: 'The report measures CI#40134371519 green 17/17 on 4cbf0a4c9. The a11y job moved from 6 · 43 · 0 · 0 on CI#40125511019 to 14 · 84 · 0 · 0, and each per-file count was read from that job''s log. Every other job equals CI#40125511019''s. The site sweep names :42 as the last dated reading, after which this one is owed (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:16, :17, :22, :43, :97).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 42
    section: '## 9. CI Integration'
    change: 'after the CI#40134371519 reading, add: "A per-job tally read from a CI job log through `gh run view --log` strips ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`: the log spells the escape as the two literal characters `^[`, so a strip of `\x1b\[[0-9;]*m` alone leaves every cargo `Running tests/{name}.rs` line unattributed — as measured on CI#40134371519 (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md)"'
    sidecar: 'ci-integration.md:42 — the per-job tally read''s ANSI strip: `(\x1b|\^\[)\[[0-9;]*m`, the log spelling ESC as literal `^[` (2026-10-24-a11y-re-read)'
    rationale: 'The report''s Decisions record that a strip of `\x1b\[[0-9;]*m` alone read every row as `None` on CI#40134371519''s log, and that stripping `(\x1b|\^\[)\[[0-9;]*m`, as plan entry 21''s sed does, attributed all fourteen files (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:57).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:57
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 64
    section: '## 9. CI Integration'
    change: 'after "… stand_keyboard 14, `target/ci-logs/a11y.log`, as measured at escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md)", add: "; since 2026-10-24-a11y-re-read fourteen files, exit 0, 14 result lines · 84 passed · 0 failed · 0 ignored — the six unmoved at 43 and the eight joining at 41: accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5, `target/ci-logs/a11y.log`, as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md"'
    sidecar: 'ci-integration.md:64 — the local ci-leg.sh a11y reading: six files · 43 → fourteen · 84, the six unmoved (2026-10-24-a11y-re-read)'
    rationale: 'The report reads `ci-leg.sh a11y` at `lines 14 passed 84 failed 0 ignored 0`, the six files at their counts (43, unmoved) and the eight joining at 41, per file in evidence/counts.md. The site sweep names :64 as true as dated, with this chunk''s 14-file reading owed (escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:16, :22, :43).'
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:16
note: The +3 moves of test_ci_workflows.py cites after line 299 are left to the citation sweep (report.md:19). They sit in these entities as unit-test-strategy.md:28's `:326-374`, :29's `:381-495`, `:425-459` and `:103-499` (its end), ci-integration.md:10's `:361-365` and quality-gates-coverage-targets.md:14's `:330`. No proposal here re-points them.
