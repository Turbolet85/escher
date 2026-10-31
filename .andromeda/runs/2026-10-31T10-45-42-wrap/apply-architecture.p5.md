# Apply — architecture · part 5 of 5

## Verdicts
- fanout-architecture--cascade-g7.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:94+#1
- fanout-architecture--cascade-g7.md 2 · applied · .andromeda/architecture-detail/occupied-resources.md:94+#2
- fanout-architecture--cascade-g7.md 3 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:40#1
- fanout-architecture--cascade-g8.md 1 · applied · .andromeda/architecture-detail/existing-scopes.md:64#1
- fanout-architecture--cascade-g8.md 2 · applied · .andromeda/architecture-detail/existing-scopes.md:76#1
- fanout-architecture--cascade-g8.md 3 · applied · .andromeda/architecture-detail/existing-scopes.md:102#1
- fanout-architecture--cascade-g8.md 4 · applied · .andromeda/architecture-detail/existing-scopes.md:102#2
- fanout-architecture--cascade-g8.md 5 · applied · .andromeda/architecture-detail/existing-scopes.md:104#1
- fanout-architecture--cascade-g8.md 6 · applied · .andromeda/architecture-detail/existing-scopes.md:106#1
- fanout-architecture--cascade-g8.md 7 · applied · .andromeda/architecture-detail/existing-scopes.md:106#2
- fanout-architecture--cascade-g8.md 8 · applied · .andromeda/architecture-detail/existing-scopes.md:112#1
- fanout-architecture--cascade-g8.md 9 · applied · .andromeda/architecture-detail/existing-scopes.md:112#2
- fanout-architecture--cascade-g8.md 10 · applied · .andromeda/architecture-detail/existing-scopes.md:112#3
- fanout-architecture--cascade-g8.md 11 · applied · .andromeda/architecture-detail/existing-scopes.md:113#1
- fanout-architecture--cascade-g8.md 12 · applied · .andromeda/architecture-detail/existing-scopes.md:116#1
- fanout-architecture--cascade-g8.md 13 · applied · .andromeda/architecture-detail/existing-scopes.md:135#1, .andromeda/architecture-detail/existing-scopes.md:135#2, .andromeda/architecture-detail/existing-scopes.md:135#3, .andromeda/architecture-detail/existing-scopes.md:135#4
- fanout-architecture--cascade-g8.md 14 · applied · .andromeda/architecture-detail/existing-scopes.md:142#1, .andromeda/architecture-detail/existing-scopes.md:142#2
- fanout-architecture--cascade-g8.md 15 · applied · .andromeda/architecture-detail/existing-scopes.md:145#1
- fanout-architecture--cascade-g8.md 16 · applied · .andromeda/architecture-detail/existing-scopes.md:147+#1
- cascade-compare-architecture.md 9 · applied · .andromeda/architecture-detail/existing-scopes.md:49#1

## Edits
- after: .andromeda/architecture-detail/occupied-resources.md:94
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-ci-infrastructure -->»
  add: «»
  add: «- The `Stand checks repeated` job runs `jlumbroso/free-disk-space` at the pinned sha `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`; on CI#41192433140 it reads `completed · success` and takes `/dev/root` from 86 G free to 109 G free. It is not shown to be the cure of that job's out-of-disk failure (`Free space left: 5 MB`, CI#41190771843): the green runner had 86 GB free before the step, the failed run has no `df` line, and the red job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread (.github/workflows/ci.yml:311-316; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:94
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-ci-infrastructure -->»
  add: «»
  add: «- A `jlumbroso/free-disk-space` step also stands in the `matrix_test` job, skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest`, read `completed · skipped` on `Test (android)` in CI#41190771843, so the `Stand checks repeated` step is this workflow's one measured run of the action (.github/workflows/ci.yml:392-394; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  old: «which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option»
  new: «which marks a select where a key or typeahead or, on a `multiple` select, a click on a row (packages/blitz-dom/src/events/pointer.rs:1013-1054) changed its chosen option or set and nothing for a choice of the chosen option»
- line: .andromeda/architecture-detail/existing-scopes.md:49
  old: «packages/blitz-dom/src/node/mod.rs:29-30), scrollbars, serialization»
  new: «packages/blitz-dom/src/node/mod.rs:29-30), select (the crate-private rules of the select model, holding `SelectCursor`, `RowGesture` and `Gestured`; packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89), scrollbars, serialization»
- line: .andromeda/architecture-detail/existing-scopes.md:64
  old: «Modules application, convert_events, event, net, window, accessibility»
  new: «Modules application, convert_events, event (holding the crate's one `#[cfg(test)]` module, one unit test, a redraw request reaching the receiver and waking the loop once: packages/blitz-shell/src/event.rs:128-172), net, window, accessibility»
- line: .andromeda/architecture-detail/existing-scopes.md:76
  old: «its three types, `is_masked_input` and the `MASKED_VALUE` constant re-exported»
  new: «its three types, `is_masked_input`, the `MASKED_VALUE` constant, `closed_select_of` and `is_multiple_select` re-exported»
- line: .andromeda/architecture-detail/existing-scopes.md:102
  old: «and neither attribute on a native checkbox or radio, a 5,500-character `aria-checked` nowhere in the tree, 5 tests in both layout modes»
  new: «and neither attribute on a native checkbox or radio, whose `toggled` is written from its checked state under a checkable role and not at all under any other role, a click and a Space moving it, a 5,500-character `aria-checked` nowhere in the tree, 6 tests in both layout modes»
- line: .andromeda/architecture-detail/existing-scopes.md:102
  old: «and restored when `inert` is removed, the focus-fixup limit pinned, 4 tests»
  new: «and restored when `inert` is removed, a node that holds focus when it comes under `inert` giving focus up with no DOM `blur` or `focusout` and taking no key, the tree's focus reading the window when the focused node was not built, 6 tests»
- line: .andromeda/architecture-detail/existing-scopes.md:104
  old: «a target inside a translated scrolling box reading outside its visible region»
  new: «a target inside a translated scrolling box reading inside its visible region»
- line: .andromeda/architecture-detail/existing-scopes.md:106
  old: «`INPUT_NAMES` and `rendered` —»
  new: «`INPUT_NAMES`, `rendered`, `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (the load page's and Cells' Tab sequences; tests/blitz-tests/tests/common/mod.rs:93-98) —»
- line: .andromeda/architecture-detail/existing-scopes.md:106
  old: «`stand_keyboard`, 15 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg; the fifteenth the flight type select moving its choice by its arrows)»
  new: «`stand_keyboard`, 21 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg; the fifteenth the flight type select moving its choice by its arrows; the sixteenth to the twenty-first the load page's stated walk with Enter and Space on its button, Cells' stated walk, Cells' arrows and Enter opening the moved-to cell and its value committed by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the arrows clamping at the grid's ends and bringing a cell into view: tests/blitz-tests/tests/stand_keyboard.rs:157-335)»
- line: .andromeda/architecture-detail/existing-scopes.md:112
  old: «a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes»
  new: «a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows and a control in a scrolling box under a zoomed world reached where it shows, 3 tests in both layout modes»
- line: .andromeda/architecture-detail/existing-scopes.md:112
  old: «a disabled select and an unchosen option refused — 5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:84-261)»
  new: «a disabled select and an unchosen option refused, a `multiple` select's name, role and rows each read with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` — 10 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:84-457)»
- line: .andromeda/architecture-detail/existing-scopes.md:112
  old: «a disabled select, a re-seed — 7 tests in both layout modes (tests/blitz-tests/tests/select_interaction.rs:123-353)»
  new: «a disabled select, a re-seed, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, its keys and typed text, its clicks choosing, flipping and spanning, a disabled one taking no click and no key, `multiple` removed settling the set to one option, the chosen row's and a plain row's ratios, a seeded set stepped and spanned from its last chosen row, and `optgroup` under a single and a `multiple` select — 18 tests in both layout modes (tests/blitz-tests/tests/select_interaction.rs:123-894)»
- line: .andromeda/architecture-detail/existing-scopes.md:113
  old: «`stand_contrast`, 5 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions)»
  new: «`stand_contrast`, 7 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions; two of them the load page's ratios and Cells' ratios: tests/blitz-tests/tests/stand_contrast.rs:647-704)»
- line: .andromeda/architecture-detail/existing-scopes.md:116
  old: «the per-task count of elements that react to a click and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676, measured and pinned)»
  new: «the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid` with its `onkeydown`, measured and pinned)»
- line: .andromeda/architecture-detail/existing-scopes.md:135
  old: «`stand_cells_served`, 4 tests in both layout modes»
  new: «`stand_cells_served`, 5 tests in both layout modes»
- line: .andromeda/architecture-detail/existing-scopes.md:135
  old: «landing on the shown overlay scrollbar, and the lint at boot reading its measured findings»
  new: «landing on the shown overlay scrollbar, the lint at boot reading its measured findings, and the keyboard walking to a cell and committing a value read in the first part»
- line: .andromeda/architecture-detail/existing-scopes.md:135
  old: «`stand_load_page`, 2 tests in both layout modes»
  new: «`stand_load_page`, 3 tests in both layout modes»
- line: .andromeda/architecture-detail/existing-scopes.md:135
  old: «— as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  new: «— as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, the 5 and the 3 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/architecture-detail/existing-scopes.md:142
  old: «by 24 (22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)»
  new: «by 28 (22 `stand_*` files, `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`)»
- line: .andromeda/architecture-detail/existing-scopes.md:142
  old: «the 31 at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md.»
  new: «the 31 at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, the 28 at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md.»
- line: .andromeda/architecture-detail/existing-scopes.md:145
  old: «the file does not run both layout modes»
  new: «each of the three runs both layout modes»
- after: .andromeda/architecture-detail/existing-scopes.md:147
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scopes -->»
  add: «»
  add: «- blitz-tests' engine-fact targets, each one file and one target, in both layout modes: `visible_region_transformed`, 6 tests — a world under an untransformed viewport showing a node and panning one out, a target in a node body under a transformed world reading where it shows, a page with no transform reading its padding box in the client rect's frame, a translate moving the region, a box mapped to a point leaving nothing visible inside it, and a rotation reading the rotated box's bounding box (tests/blitz-tests/tests/visible_region_transformed.rs:132-276); `hit_non_invertible_transform`, 2 tests — a box under a non-invertible transform taking no hit and its invertible twin hit where it shows (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71); `open_state`, 4 tests — `open` read from the attribute at construction, a write and a removal through the mutator moving it, and a summary's click and Enter flipping its details (tests/blitz-tests/tests/open_state.rs:54-143); `checkable_change_event`, 3 tests — a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's (tests/blitz-tests/tests/checkable_change_event.rs:68-136); `text_range_no_change_event`, 2 tests — typing in a text input, and a range's key and pointerdown, firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-97).»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch5-01"
fixed = 'a key or typeahead changed its chosen option'

[[pattern]]
id = "arch5-02"
fixed = 'nowhere in the tree, 5 tests'

[[pattern]]
id = "arch5-03"
fixed = 'the focus-fixup limit pinned, 4 tests'

[[pattern]]
id = "arch5-04"
fixed = 'scrolling box reading outside its visible region'

[[pattern]]
id = "arch5-05"
fixed = '`stand_keyboard`, 15 tests'

[[pattern]]
id = "arch5-06"
fixed = 'read where it shows, 2 tests in both layout modes'

[[pattern]]
id = "arch5-07"
fixed = 'an unchosen option refused — 5 tests'

[[pattern]]
id = "arch5-08"
fixed = 'a re-seed — 7 tests'

[[pattern]]
id = "arch5-09"
fixed = '`stand_contrast`, 5 tests'

[[pattern]]
id = "arch5-10"
fixed = 'react to a click and state no role'

[[pattern]]
id = "arch5-11"
fixed = 'Cells 676, measured and pinned'

[[pattern]]
id = "arch5-12"
fixed = '`stand_cells_served`, 4 tests'

[[pattern]]
id = "arch5-13"
fixed = '`stand_load_page`, 2 tests'

[[pattern]]
id = "arch5-14"
fixed = 'by 24 (22 `stand_*` files'

[[pattern]]
id = "arch5-15"
fixed = 'the file does not run both layout modes'
