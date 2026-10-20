group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «and, since 2026-10-19-recorded-check-replay, `stand_replay` (25 readers;»
  source: .andromeda/test-plan.md:103 «(26 readers since 2026-10-19-visual-lint added `stand_lint`, which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`; 25 before `stand_lint`,»
  replace-with: «and, since 2026-10-19-recorded-check-replay, `stand_replay` and, since 2026-10-19-visual-lint, `stand_lint` (26 readers;»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 19 checks — 18 `stand_*` files and `font_face_sniffed_source` —»
  source: .andromeda/test-plan.md:104 «20 checks declare `mod fixtures;` — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
  replace-with: «what 20 checks — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source` —»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors»
  source: .andromeda/test-plan.md:351 «since 2026-10-19-test-splits escher-driver read 4266/4350 = 98.07 %, and since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %, the 97 % floor holding»
  replace-with: «since 2026-10-19-test-splits escher-driver read 4266/4350 = 98.07 %, and since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run stand` runs the 18 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «so `boot` is the bootstrap; `run stand` runs the 19 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 19 `stand_*` readers of the shared module»
