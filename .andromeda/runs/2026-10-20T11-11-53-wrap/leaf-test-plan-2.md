group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «(28 readers, `stand_act_select` the twenty-eighth;»
  source: .andromeda/test-plan.md:103 «(29 readers with `stand_snapshot_parts`, which reads `snapshot_part`, `part`, `scroll`, `click` and `act`; 28 with `stand_act_select`;»
  replace-with: «and, since 2026-10-20-large-screen-in-parts, `stand_snapshot_parts` (29 readers, `stand_snapshot_parts` the twenty-ninth;»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «· `lint`, the outcome reader `act`, the shot reader `shot`,»
  source: .andromeda/test-plan.md:103 «`lint()`, `snapshot_part(n)` and the part reader `part(outcome)` — the text, the part and the count, panicking with a fixed message on any other outcome —,»
  replace-with: «· `lint` · `snapshot_part`, the outcome reader `act`, the part reader `part` (a paged snapshot's text, part and count), the shot reader `shot`,»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 23 checks — 21 `stand_*` files (`stand_act_select`,»
  source: .andromeda/test-plan.md:104 «24 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source` and `select_interaction`»
  replace-with: «what 24 checks — 22 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`,»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run stand` runs the 21 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 22 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 22 `stand_*` readers of the shared module»
