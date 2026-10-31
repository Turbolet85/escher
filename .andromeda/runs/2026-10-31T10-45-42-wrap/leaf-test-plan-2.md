group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »
- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 24 checks — 22 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source` and `select_interaction` — share from»
  source: .andromeda/test-plan.md:151 «28 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source`,»
  replace-with: «what 28 checks — 22 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event` — share from»
- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «`RecordingShell`, `NoopEventHandler`, probe widgets»
  source: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:12 «a test-local `CountedWakes` implements `EventLoopProxyProvider`, read by `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` for a loop woken once»
  replace-with: «`RecordingShell`, blitz-shell's test-local `CountedWakes`, `NoopEventHandler`, probe widgets»
- leaf: .claude/rules/testing.md:27   class: retired
  leaf-text: «(upstream's `pseudo_element_flow_classification` is an exception, asserting neither mode pair, `incremental_oracle` its layout-identity witness)»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:8 «runs `for incremental in [false, true]` as the pipeline tests do, and `incremental_oracle`, green beside it, is the layout-identity witness»
  replace-with: «(upstream's `pseudo_element_flow_classification` among them, `incremental_oracle` the layout-identity witness)»
- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «it reads 4930/5031 = 97.99 % (dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %»
  replace-with: «it reads 4950/5049 = 98.04 % (dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %)»
- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «the ten named blitz-tests targets of its `--test` list»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «the fifteen named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry»
  replace-with: «the fifteen named blitz-tests targets of its `--test` list»
- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run font_face_sniffed_source` and `run select_interaction` its two non-stand readers»
  source: .andromeda/test-plan.md:190 «`run {file}` the six non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and»
  replace-with: «`run font_face_sniffed_source`, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and `run text_range_no_change_event` its six non-stand readers»
