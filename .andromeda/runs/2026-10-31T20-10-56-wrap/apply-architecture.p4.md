# Apply — architecture · part 4 of 4

## Verdicts
- fanout-architecture--cascade-g7.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:32#1
- fanout-architecture--cascade-g7.md 2 · applied · .andromeda/architecture-detail/occupied-resources.md:57#1
- fanout-architecture--cascade-g7.md 3 · applied · .andromeda/architecture-detail/occupied-resources.md:71#1
- cascade-compare-architecture.md 14 · applied · .andromeda/architecture-detail/occupied-resources.md:13+#1
- fanout-architecture--cascade-g7.md 4 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:21#1
- fanout-architecture--cascade-g7.md 5 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:40#1, .andromeda/architecture-detail/cross-cutting-patterns.md:40#2
- fanout-architecture--cascade-g7.md 6 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:40#3
- fanout-architecture--cascade-g7.md 7 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:40+#1
- fanout-architecture--cascade-g8.md 1 · applied · .andromeda/architecture-detail/existing-scopes.md:28#1
- fanout-architecture--cascade-g8.md 2 · applied · .andromeda/architecture-detail/existing-scopes.md:49#1
- fanout-architecture--cascade-g8.md 3 · applied · .andromeda/architecture-detail/existing-scopes.md:58#1
- fanout-architecture--cascade-g8.md 4 · applied · .andromeda/architecture-detail/existing-scopes.md:76#1
- fanout-architecture--cascade-g8.md 5 · applied · .andromeda/architecture-detail/existing-scopes.md:106#1
- fanout-architecture--cascade-g8.md 6 · applied · .andromeda/architecture-detail/existing-scopes.md:112#1
- fanout-architecture--cascade-g8.md 7 · applied · .andromeda/architecture-detail/existing-scopes.md:112+#1
- fanout-architecture--cascade-g8.md 8 · applied · .andromeda/architecture-detail/existing-scopes.md:113#1
- fanout-architecture--cascade-g8.md 9 · applied · .andromeda/architecture-detail/existing-scopes.md:116#1
- fanout-architecture--cascade-g8.md 10 · applied · .andromeda/architecture-detail/existing-scopes.md:142#1, .andromeda/architecture-detail/existing-scopes.md:142#2, .andromeda/architecture-detail/existing-scopes.md:142#3

## Edits
- line: .andromeda/architecture-detail/occupied-resources.md:32
  old: «`hl-cells`, `cc-*`, `cf-*` and `ht-timer` (seven_guis: `host_binary`, `host_log`, `cli_commands`, `cli_flow`, `host_timer`)»
  new: «`hl-cells`, `hl-popup`, `cc-*`, `cf-*` and `ht-timer` (seven_guis: `host_binary`, `host_log`, `cli_commands`, `cli_flow`, `host_timer`; the select list's two `cc-popup` and `hl-popup`, `mcp_log`'s select list test recording nothing and no `ml-popup` existing: examples/seven_guis/tests/cli_commands.rs:442; examples/seven_guis/tests/host_log.rs:296)»
- line: .andromeda/architecture-detail/occupied-resources.md:57
  old: «and `host_log` on `crud` and on `cells` (its paged part read: a sentinel committed into a part-2 cell, then `snapshot --part 2`; examples/seven_guis/tests/host_log.rs:137-280) with `RUST_LOG=trace`,»
  new: «and `host_log` on `crud`, on `cells` (its paged part read: a sentinel committed into a part-2 cell, then `snapshot --part 2`; examples/seven_guis/tests/host_log.rs:137-280) and for a select's list opened and chosen through the host (examples/seven_guis/tests/host_log.rs:282-393) with `RUST_LOG=trace`,»
- line: .andromeda/architecture-detail/occupied-resources.md:71
  old: «`occurrences`, the unix-only `recorded_needles`»
  new: «`occurrences`, `option_needles` (the needles of a select's options), the unix-only `recorded_needles`»
- after: .andromeda/architecture-detail/occupied-resources.md:13
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-network-ports-and-listeners -->»
  add: «»
  add: «- A shot of an open select list also carries that list's option labels and group `label`s, a page's text as painted pixels, on both carriers, and a row's own `bounds` cross in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing); no key, node key, field, carrier, listener or port is added; the pixels are a boundary widening ratified by the founder (2026-10-31) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:21
  old: «device changes and a focus set or cleared, the Tab key asking for none of its own»
  new: «device changes, a focus set or cleared and a select list opening, moving, scrolling or closing (tests/blitz-tests/tests/select_popup.rs:1045-1090), the Tab key asking for none of its own»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  old: «is written at four kinds of site»
  new: «is written at five kinds of site»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  old: «— and is not written by node creation, hover,»
  new: «— and, with no select list standing, is not written by node creation, hover,»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  old: «or, on a `multiple` select, a click on a row»
  new: «or, on a `multiple` select or a single select's open list, a click on a row»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:40
  anchor: « tests/blitz-tests/tests/fragment_navigation.rs:31-43); a color-scheme change recascades everything (tests/blitz-tests/tests/device_coalescing.rs:94-110); reading the stylist device flushes pending changes (tests/blitz-tests/tests/device_coalescing.rs:80-92); the changed set (§Standard Contracts → Document core) is written at four kinds of site — the 15 mutation-flag sites of `DocumentMutator`, each marking the node whose in-document flag its own condition reads and only when that condition holds (the node itself in `set_node_text`, `append_text_to_node`, `set_attribute`, `clear_attribute`, `set_style_property`, `remove_style_property`, `set_sub_document`, `remove_sub_document`, `set_custom_widget`, `remove_custom_widget`, `remove_node` and `remove_and_drop_node_with`, the parent in `remove_and_drop_all_children` and `add_children_to_parent`, the moved child in `add_children_to_parent`); `snapshot_node_and` when its `ElementState` argument intersects `FOCUS` or `CHECKED`, so a focus move, a checkbox toggle and `toggle_radio` (which marks every radio of the set, changed or not); the `Input` arm of `apply_generated_text_input_event` (a typed character, an IME commit), the last two wherever the node is, in the document or not; and, since 2026-10-13-range-input-interaction, `BaseDocument::apply_range_move`, which marks a range input where a key or a pointerdown changed its value and nothing for a move that changes nothing (packages/blitz-dom/src/events/keyboard.rs:532-553), and `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead or, on a `multiple` select, a click on a row (packages/blitz-dom/src/events/pointer.rs:1121-1162) changed its chosen option or set and nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:470-496) — and is not written by node creation, hover, active, scroll, a viewport resize, a resource load, an animation or layout, so a `resolve` with nothing written marks nothing in either layout mode;»
  add: « the fifth kind of site is a select's list: its opening or closing marks the select, a page scroll or a move of the select's box that closes the list marks it through `reconcile_select_list` at the next resolve, a wheel elsewhere marks it through `close_select_list`, and a highlight or a wheel move inside the list marks nothing (packages/blitz-dom/src/document.rs:811-844; packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:891-907);»
- line: .andromeda/architecture-detail/existing-scopes.md:28
  old: «(packages/blitz-dom/src/document.rs:2098-2159); hover, focus, active»
  new: «(packages/blitz-dom/src/document.rs:2098-2159); a single select's open list, its readers and writer and its hit, answered first in `hit_with_scrollbar` (packages/blitz-dom/src/document.rs:761-1089; packages/blitz-dom/src/document.rs:2404-2406); hover, focus, active»
- line: .andromeda/architecture-detail/existing-scopes.md:49
  old: «packages/blitz-dom/src/node/select.rs:80-89), scrollbars, serialization»
  new: «packages/blitz-dom/src/node/select.rs:80-89), select_list (the crate-private state and geometry of a single select's open list, with 6 unit tests, its two public read types `SelectListView` and `SelectListRow` re-exported from the crate root; blitz-paint's `render::select_list` paints it, and `select_popup`, `select_popup_painted` and `stand_act_popup` are its checks; packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/node/select_list.rs:78-108; packages/blitz-dom/src/lib.rs:108-111), scrollbars, serialization»
- line: .andromeda/architecture-detail/existing-scopes.md:58
  old: «layers, render, sizing, text»
  new: «layers, render (with the child module `select_list`, `paint_select_list` painting a single select's open list after the root's render and before the devtools overlays: packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/render.rs:246-247), sizing, text»
- line: .andromeda/architecture-detail/existing-scopes.md:76
  old: «`closed_select_of` and `is_multiple_select` re-exported»
  new: «`closed_select_of`, `is_multiple_select` and `is_select` re-exported»
- line: .andromeda/architecture-detail/existing-scopes.md:106
  old: «`stand_keyboard`, 21 tests in both layout modes»
  new: «`stand_keyboard`, 23 tests in both layout modes»
- line: .andromeda/architecture-detail/existing-scopes.md:112
  old: «Enter and Space opening nothing on a single select»
  new: «Enter and Space opening its list on a single select»
- after: .andromeda/architecture-detail/existing-scopes.md:112
  anchor: « since 2026-10-12-headless-screenshot a thirteenth driver-action file, `stand_act_screenshot` — the whole screen a PNG of the harness's raster with text painted, an element shot its bounds rounded outward, a shot refused as every id-addressed call is, an app with no usable font refused `fonts-unavailable` with nothing changed, a typed password never leaving as pixels and a shot changing nothing, in both layout modes (tests/blitz-tests/tests/stand_act_screenshot.rs:80-302) — and a fourteenth, `stand_act_transformed`, a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows and a control in a scrolling box under a zoomed world reached where it shows, 3 tests in both layout modes (tests/blitz-tests/tests/stand_act_transformed.rs:81-193), a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children, `type` with a label choosing that option, a click then a key, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused, a `multiple` select's name, role and rows each read with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` — 10 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:85-466), run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix), the engine check `select_interaction`, no `stand_*` file — the select's box, its seed, keys and typeahead over one second of the document's clock, a disabled select, a re-seed, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, its keys and typed text, its clicks choosing, flipping and spanning, a disabled one taking no click and no key, `multiple` removed settling the set to one option, the chosen row's and a plain row's ratios, a seeded set stepped and spanned from its last chosen row, and `optgroup` under a single and a `multiple` select — 18 tests in both layout modes (tests/blitz-tests/tests/select_interaction.rs:125-911), in the `a11y` CI leg and in the `stand-repeat` leg (a named `--test` of its list), `stand_screenshot_repeat` — a shot of one state byte-identical in two child runs and in the parent on one host, 12 shots in both layout modes — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step (as measured at CI#39225375577) (tests/blitz-tests/tests/stand_screenshot_repeat.rs:75-98)»
  add: « — a sixteenth driver-action file, `stand_act_popup`, 12 tests in both layout modes — a click opening the list and a click on a row choosing it, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option refused `covered`, a row past the scrollport off screen and brought in by `scroll`, a button under the list refused `covered`, `type` leaving no list open and choosing by its text, a shot of a row cropped to it with the list left open, the lint reading no `overlap` on a row and the planted pair reading, the open screen under its budget with every actionable element keyed, two clicks on one point opening and closing and each focusing, a screen read after a step holding the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423), in the `stand-repeat` leg by its `stand_` prefix;»
  add: « the engine checks `select_popup`, 25 tests — the list's opening, keys, choice, closing, tree, clips, hostile markup and a `multiple` select's `size` (tests/blitz-tests/tests/select_popup.rs:202-1186) — and `select_popup_painted`, 6 tests — the highlight and a disabled row against their constants, the plain pair, opacity and clips, a closed select's frame byte-equal, a group's header darker than a row by weight and a thumb that takes no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351) —, both in both layout modes and named `--test`s of the `stand-repeat` list (.github/scripts/ci-leg.sh:59-60), outside the `a11y` leg's fourteen files; and `stand_keyboard`'s twenty-second and twenty-third, the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select and the walk going on from it (tests/blitz-tests/tests/stand_keyboard.rs:1082-1134)»
- line: .andromeda/architecture-detail/existing-scopes.md:113
  old: «`STATE_ROWS`, 18: 17 pass, 1 exempt»
  new: «`STATE_ROWS`, 20: 19 pass, 1 exempt»
- line: .andromeda/architecture-detail/existing-scopes.md:116
  old: «by the fifteen `stand_act_*`»
  new: «by the sixteen `stand_act_*`»
- line: .andromeda/architecture-detail/existing-scopes.md:142
  old: «`tests/session_common/mod.rs`, by 31, and»
  new: «`tests/session_common/mod.rs`, by 32, and»
- line: .andromeda/architecture-detail/existing-scopes.md:142
  old: «`HeldNet`, by 28 (22 `stand_*` files, `font_face_sniffed_source`, `select_interaction`, `inert_subtree`»
  new: «`HeldNet`, by 31 (23 `stand_*` files, `stand_act_popup` among them, `font_face_sniffed_source`, `select_interaction`, `select_popup`, `select_popup_painted`, `inert_subtree`»
- line: .andromeda/architecture-detail/existing-scopes.md:142
  old: «the 31 at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, the 28 at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md.»
  new: «the 32 and the 31 at escher-0.3.0/chunks/2026-10-31-select-popup/report.md.»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = 'written at four kinds of site'

[[pattern]]
id = "arch4-02"
fixed = '— and is not written by node creation'

[[pattern]]
id = "arch4-03"
fixed = 'on a `multiple` select, a click on a row'

[[pattern]]
id = "arch4-04"
fixed = '`stand_keyboard`, 21 tests'

[[pattern]]
id = "arch4-05"
fixed = 'Enter and Space opening nothing'

[[pattern]]
id = "arch4-06"
fixed = '`STATE_ROWS`, 18: 17 pass'

[[pattern]]
id = "arch4-07"
fixed = 'by the fifteen `stand_act_*`'

[[pattern]]
id = "arch4-08"
fixed = '`tests/session_common/mod.rs`, by 31'

[[pattern]]
id = "arch4-09"
fixed = 'by 28 (22 `stand_*` files'
