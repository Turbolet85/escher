# Apply — test-plan · part 5 of 5

## Verdicts
- fanout-test-plan--cascade-g5.md 13 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16#1
- fanout-test-plan--cascade-g5.md 14 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22#1
- fanout-test-plan--cascade-g5.md 15 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22#2
- fanout-test-plan--cascade-g5.md 16 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22#3
- fanout-test-plan--cascade-g5.md 17 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34#1
- fanout-test-plan--cascade-new-test-data-fixtures.md 1 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:24+#1
- cascade-compare-test-plan.md 8 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:22+#1
- cascade-compare-test-plan.md 9 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:23+#1
- fanout-test-plan.md 1 · applied · .andromeda/registries/contracts/test-plan/session-lifecycle.md:5#1

## Edits
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  old: «escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at the `Coverage report` job of CI#41192433140, read in escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at CI#41235503227, read in escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
  old: «run the 50 `stand_*.rs` files»
  new: «run the 51 `stand_*.rs` files»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
  old: «the fifteen named blitz-tests targets»
  new: «the seventeen named blitz-tests targets»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
  old: «with the fifteen CI#41192433140 read 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «with the seventeen and `stand_act_popup` CI#41235503227 read 228 · 1155 · 0 · 21, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34
  old: «`clear_interaction_state_for_removed_node` (17.0)»
  new: «`clear_interaction_state_for_removed_node` (17.0; this reading and the count of 85 as read before that function took the clearing of the select list's field, packages/blitz-dom/src/document.rs:1408-1410, no later reading claimed)»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:22
  anchor: «| Inline HTML | inline HTML string constants or `format!`-built pages | (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29) |»
  add: «| Hostile `select` markup | `hostile_html`, a page generated in code with `format!`: 2,000 options, a 5,500-character label and a hostile `size`, read by `hostile_markup_keeps_every_box_finite_and_panics_nowhere` in both layout modes | (tests/blitz-tests/tests/select_popup.rs:913-930; tests/blitz-tests/tests/select_popup.rs:938-975) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:23
  anchor: «| Shared fixture page (`rsx!`) | `many_items`: `ITEMS` (400) buttons under `#items`, ids `item-NNN`, a screen over the snapshot text budget, read by `stand_snapshot_parts` and `stand_act_spans` | (tests/blitz-tests/tests/fixtures/mod.rs:64-86) |»
  add: «| Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long` | (tests/blitz-tests/tests/select_popup.rs:32-107; tests/blitz-tests/tests/select_popup.rs:616-626; tests/blitz-tests/tests/select_popup.rs:653-675; tests/blitz-tests/tests/select_popup.rs:857-868; tests/blitz-tests/tests/select_popup.rs:977-993; tests/blitz-tests/tests/select_popup.rs:1092-1103; tests/blitz-tests/tests/select_popup.rs:1124-1134; tests/blitz-tests/tests/select_popup_painted.rs:22-34; tests/blitz-tests/tests/select_popup_painted.rs:189-209; tests/blitz-tests/tests/select_popup_painted.rs:255-267; tests/blitz-tests/tests/select_popup_painted.rs:299-310; tests/blitz-tests/tests/stand_act_popup.rs:168-196) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:24
  anchor: «| Stand screen over the budget | the stand's own Cells screen, booted through `stand::boot_cells`: 47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md, walked in parts by `stand_cells_served` and read by part through the host by `host_log` | (examples/seven_guis/src/stand.rs:131-135; tests/blitz-tests/tests/stand_cells_served.rs:94-136) |»
  add: «| Select option needles | `option_needles()` in seven_guis' shared test module: the needle list of a select's options, read from a snapshot | (examples/seven_guis/tests/common/mod.rs:425-453) |»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
  old: «(a `multiple` select focused with none, packages/escher-driver/src/execute.rs:557-563)»
  new: «(a select, single or `multiple`, focused with none and no DOM focus event fired, so a text opening on a space types nothing on a single select, packages/escher-driver/src/execute.rs:557-563)»

## Patterns
[[pattern]]
id = "tp5-01"
fixed = 'escher-driver 4950/5049'

[[pattern]]
id = "tp5-02"
fixed = 'run the 50 `stand_*.rs` files'

[[pattern]]
id = "tp5-03"
fixed = 'the fifteen named blitz-tests targets'

[[pattern]]
id = "tp5-04"
fixed = 'with the fifteen CI#41192433140 read'

[[pattern]]
id = "tp5-05"
fixed = 'a `multiple` select focused with none'
