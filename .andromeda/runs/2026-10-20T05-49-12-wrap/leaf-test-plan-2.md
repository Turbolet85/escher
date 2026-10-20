group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «the fourteen driver-action checks `stand_act_*`»
  source: .andromeda/test-plan.md:68 «`stand_settle.rs`, which declares it and not `mod common;`, the fifteen `stand_act_*.rs`»
  replace-with: «the fifteen driver-action checks `stand_act_*`»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «(27 readers;»
  source: .andromeda/test-plan.md:103 «(28 readers with `stand_act_select`; 27 since 2026-10-20-load-wait added `stand_load_wait`»
  replace-with: «(28 readers, `stand_act_select` the twenty-eighth;»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 21 checks — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source` — share»
  source: .andromeda/test-plan.md:104 «23 checks declare `mod fixtures;` — 21 `stand_*` files (`stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source` and `select_interaction`»
  replace-with: «what 23 checks — 21 `stand_*` files (`stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source` and `select_interaction` — share»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run stand` runs the 20 `stand_*` readers of the shared module, `run font_face_sniffed_source` its one non-stand reader»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 21 `stand_*` readers of the shared module, `run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`)»
  replace-with: «`run stand` runs the 21 `stand_*` readers of the shared module, `run font_face_sniffed_source` and `run select_interaction` its two non-stand readers»
