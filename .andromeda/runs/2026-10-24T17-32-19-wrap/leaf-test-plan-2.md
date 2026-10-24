group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «the pages more than one check reads — `password_labelled`, `password_named`, `covered` —»
  source: .andromeda/test-plan.md:151 «the pages `password_labelled`, `password_named`, `covered` and `many_items` with its `ITEMS`, read by `stand_snapshot_parts` and `stand_act_spans`»
  replace-with: «the pages more than one check reads — `password_labelled`, `password_named`, `covered` and `many_items` with its `ITEMS` (read by `stand_snapshot_parts` and `stand_act_spans`) —»

- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «(`password_labelled`, `password_named`, `covered`) and booted»
  source: .andromeda/test-plan.md:320 «and `many_items` (`ITEMS`, 400, buttons under `#items` with the ids `item-NNN`, a screen whose text passes `SNAPSHOT_TEXT_BUDGET`; tests/blitz-tests/tests/fixtures/mod.rs:64-86)»
  replace-with: «(`password_labelled`, `password_named`, `covered`, `many_items` — `ITEMS`, 400 buttons whose screen passes `SNAPSHOT_TEXT_BUDGET`) and booted»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «and since 2026-10-20-load-wait it reads 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read again with the stand checks' named list, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %»
  replace-with: «since 2026-10-20-load-wait 4695/4794 = 97.93 %, and read again with the stand checks' named list it reads 4918/5019 = 97.99 %, the 97 % floor holding, 3 of 3 crates still at or above their floors)»

- leaf: .claude/rules/testing.md:48   class: omission
  leaf-text: «and the stand checks hold a zero-failure flakiness budget (`ci-leg.sh stand-repeat`, three rounds, no retry)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «run the 48 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry»
  replace-with: «and the stand checks hold a zero-failure flakiness budget (`ci-leg.sh stand-repeat`: the `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks, three rounds, no retry)»
