group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-31: State what a CI job did from that job's own lo»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «the fifteen driver-action checks `stand_act_*`»
  source: .andromeda/test-plan.md:149 «so do the sixteen driver-action checks `stand_act_*`»
  replace-with: «the sixteen driver-action checks `stand_act_*`»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «(31 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first;»
  source: .andromeda/test-plan-detail/test-strategy.md:58 «read through `mod session_common;` by 32 checks, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  replace-with: «(32 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first, `stand_act_popup` the thirty-second;»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 28 checks — 22 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source`, `select_interaction`, `inert_subtree`,»
  source: .andromeda/test-plan.md:151 «31 checks declare `mod fixtures;` — 23 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_act_popup`)»
  replace-with: «what 31 checks — 23 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint, `stand_act_popup`), `font_face_sniffed_source`, `select_interaction`, `select_popup`, `select_popup_painted`, `inert_subtree`,»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «— called by `host_log` and `mcp_log` alone;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:14 «and `option_needles()`, the needle list of a select's options read from a snapshot, called by the same two (examples/seven_guis/tests/common/mod.rs:425-453)»
  replace-with: «— called by `host_log` and `mcp_log` alone, and `option_needles()` (the needle list of a select's options read from a snapshot), called by the same two;»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «`RecordingShell`, blitz-shell's test-local `CountedWakes`»
  source: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:12 «`select_popup`'s own hand-written `CountingShell`, a `ShellProvider` counting redraw requests,»
  replace-with: «`RecordingShell`, `select_popup`'s own `CountingShell`, blitz-shell's test-local `CountedWakes`»

- leaf: .claude/rules/testing.md:44   class: omission
  leaf-text: «**Before a push:** `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → CI scripts)»
  source: .andromeda/test-plan-detail/test-strategy.md:40 «a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`»
  replace-with: «**Before a push:** `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → CI scripts); no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «4950/5049 = 98.04 % (dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %»
  replace-with: «4959/5058 = 98.04 % (dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %)»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «the fifteen named blitz-tests targets of its `--test` list»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «the seventeen named blitz-tests targets of its `--test` list and seven_guis' eight process checks»
  replace-with: «the seventeen named blitz-tests targets of its `--test` list»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run stand` runs the 22 `stand_*` readers of the shared module, `run font_face_sniffed_source`, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and `run text_range_no_change_event` its six non-stand readers»
  source: .andromeda/test-plan.md:191 «`run stand` runs the 23 `stand_*` readers of the shared module, `run {file}` the eight non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`,»
  replace-with: «`run stand` runs the 23 `stand_*` readers of the shared module, `run font_face_sniffed_source`, `run select_interaction`, `run select_popup`, `run select_popup_painted`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and `run text_range_no_change_event` its eight non-stand readers»
