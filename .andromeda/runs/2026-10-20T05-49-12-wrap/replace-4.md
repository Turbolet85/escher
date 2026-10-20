- line: .andromeda/test-plan.md:11
old: «`a_range_input_submits_its_current_value` (packages/blitz-dom/src/form.rs:482-542)»
new: «`a_range_input_submits_its_current_value` (packages/blitz-dom/src/form.rs:482-542); a named `select`'s entries — every chosen option that is not disabled, by its value — have no test of their own (packages/blitz-dom/src/form.rs:276-290)»
- line: .andromeda/test-plan.md:12
old: «a transformed box read at its client rect) are asserted by no check»
new: «a transformed box read at its client rect) are asserted by no check; the single-choice `select` model — its rules by the four `node::select` unit tests and, over a Dioxus page in both layout modes, by blitz-tests' `select_interaction` (the box holding the chosen option, the markup seed, the arrows, `Home` and `End`, the typeahead over one second of the document's clock, one `input` and one `change` per changed choice, the disabled gate on key and click, a `selected` write re-seeding the choice) — its stated limits (a `multiple` select and an `optgroup`'s options not modelled, no popup painted, an option with no author key reading in `unkeyed_actionable`) asserted by no check; a `<label>` naming every labelable element is pinned by `accessibility_label`, while a click on a select's label re-dispatched to the select is asserted by no check»
- line: .andromeda/test-plan.md:15
old: «a re-seed and a re-sanitize following the attributes (packages/blitz-dom/src/node/range.rs:303-529)»
new: «a re-seed and a re-sanitize following the attributes (packages/blitz-dom/src/node/range.rs:303-529); `node/select.rs` holds 4 inline table tests of the select's rules — the selectedness setting algorithm keeping one chosen option, a step skipping disabled options and stopping at the ends, typed text matching the first enabled label it starts, text stripped and collapsed (packages/blitz-dom/src/node/select.rs:195-301)»
- line: .andromeda/test-plan.md:156
old: «a re-seed and a re-sanitize on attribute writes (packages/blitz-dom/src/node/range.rs:303-529)»
new: «a re-seed and a re-sanitize on attribute writes (packages/blitz-dom/src/node/range.rs:303-529); four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), a step skipping disabled options and stopping at the ends, typed text choosing the first enabled option whose label starts with it and nothing for an empty text, and the strip and collapse of an option's text (packages/blitz-dom/src/node/select.rs:195-301)»
- line: .andromeda/test-plan.md:22
old: «covered by the fourteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth)»
new: «covered by the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)»
- line: .andromeda/test-plan.md:25
old: «73 unit tests in nine files — since»
new: «75 unit tests in nine files — a select reading its chosen option's value over the four-row `SELECT_VALUES` table (`a_select_reads_its_chosen_options_value`) and a select's options read as its children named by their labels, an option's `label` attribute naming it (`a_selects_options_are_its_children_named_by_their_labels`) (packages/dioxus-native-dom/src/snapshot.rs:932-1014); since»
- line: .andromeda/test-plan.md:25
old: «eighteen on the snapshot builder —»
new: «twenty on the snapshot builder —»
- line: .andromeda/test-plan.md:25
old: «an `aria-invalid` value reading invalid, each over a stated table of seven rows — compiled only»
new: «an `aria-invalid` value reading invalid, each over a stated table of seven rows, and a select's value and its options as its children (above) — compiled only»
- line: .andromeda/test-plan.md:27
old: «`accessibility_label` 10 (`aria-label` as the label;»
new: «`accessibility_label` 13 (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, and a `<label>` naming no hidden `input`, HTML's labelable elements (tests/blitz-tests/tests/accessibility_label.rs:309-432); `aria-label` as the label;»
- line: .andromeda/test-plan.md:27
old: «run by the workspace `test` leg and `run all` and not by the `a11y` leg's six or `run stand`, the behaviour they pin PROVISIONAL;»
new: «run by the workspace `test` leg and `run all` and not by the `a11y` leg's six or `run stand`;»
- line: .andromeda/test-plan.md:27
old: «on the in-process `Session::lint` (PROVISIONAL: in process only):»
new: «on the in-process `Session::lint` (in process only):»
- line: .andromeda/test-plan.md:27
old: «picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»
new: «picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md; `select_interaction` 7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix (run by the workspace `test` leg, by name and by `run all`, not by `run stand`), in both layout modes on a controlled single-choice select page whose `oninput` writes `selected` from the event's value — the select laid out as a box holding its chosen option; the choice seeded from the markup; `ArrowDown` · `ArrowUp` · `Home` · `End` choosing an enabled option over a 14-row table; typed text choosing the first enabled option its label starts, over a 7-row table; typed text kept for one second of the document's clock; a disabled select taking no key and no click; an app writing `selected` on another option re-seeding the choice — each row asserting `has_changes` and one `input` and one `change` per changed choice, none for no change (tests/blitz-tests/tests/select_interaction.rs:113-343); and `stand_act_select` 5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically: at boot `fx-pick` reads `ComboBox`, `Fruit`, value `b`, its three options under it named Apple · Banana · Cherry, Banana selected; `type --id fx-pick --text Cherry` accepted and settled, its diff naming `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`, the next snapshot value `c`; a click then a key choosing an option; the Tab walk `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the disabled `fx-off` skipped, equal before and after the choices; a disabled select and an unchosen option refused (`click --id fx-apple` refused `covered`) (tests/blitz-tests/tests/stand_act_select.rs:79-256); picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
- line: .andromeda/test-plan.md:68
old: «141 `.rs` files, 140 test targets and the unbuilt `all.rs` (140 and 139 until 2026-10-20-load-wait»
new: «143 `.rs` files, 142 test targets and the unbuilt `all.rs` (141 and 140 until 2026-10-20-select-by-id added `select_interaction.rs` and `stand_act_select.rs`, each its own auto-discovered target with no manifest line; 140 and 139 until 2026-10-20-load-wait»
- line: .andromeda/test-plan.md:68
old: «read through `mod session_common;` by 27 checks (26 before `stand_load_wait.rs`»
new: «read through `mod session_common;` by 28 checks (27 before `stand_act_select.rs` at 2026-10-20-select-by-id, 26 before `stand_load_wait.rs`»
- line: .andromeda/test-plan.md:68
old: «`stand_settle.rs`, which declares it and not `mod common;`, the fourteen `stand_act_*.rs`»
new: «`stand_settle.rs`, which declares it and not `mod common;`, the fifteen `stand_act_*.rs`»
- line: .andromeda/test-plan.md:68
old: «`stand_act_obstructed`, `stand_act_filled`) declare both»
new: «`stand_act_obstructed`, `stand_act_filled`) declare both (`stand_act_select` declares no `mod common;`)»
- line: .andromeda/test-plan.md:68
old: «read through `mod fixtures;` by 21 checks — 20 `stand_*.rs` (`stand_load_wait` since 2026-10-20-load-wait,»
new: «read through `mod fixtures;` by 23 checks — 21 `stand_*.rs` (`stand_act_select` since 2026-10-20-select-by-id, `stand_load_wait` since 2026-10-20-load-wait,»
- line: .andromeda/test-plan.md:68
old: «`stand_snapshot_state`) and `font_face_sniffed_source.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-107)»
new: «`stand_snapshot_state`), `font_face_sniffed_source.rs` and `select_interaction.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-107)»
- line: .andromeda/test-plan.md:102
old: «six of the fourteen driver-action checks»
new: «six of the fifteen driver-action checks»
- line: .andromeda/test-plan.md:103
old: «so do the fourteen driver-action checks `stand_act_*`»
new: «so do the fifteen driver-action checks `stand_act_*`»
- line: .andromeda/test-plan.md:103
old: «(27 readers since 2026-10-20-load-wait added `stand_load_wait`»
new: «(28 readers since 2026-10-20-select-by-id added `stand_act_select`; 27 since 2026-10-20-load-wait added `stand_load_wait`»
- line: .andromeda/test-plan.md:104
old: «21 checks declare `mod fixtures;` — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
new: «23 checks declare `mod fixtures;` — 21 `stand_*` files (`stand_act_select` since 2026-10-20-select-by-id; `stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source` and `select_interaction`»
- line: .andromeda/test-plan.md:119
old: «`run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»
new: «`run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md; re-counted at 2026-10-20-select-by-id, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 190 · failed 0 · ignored 7 over 47 files — +5 passed (`stand_act_select` 5, a new file picked up by its `stand_` prefix with no script change); `select_interaction`, no `stand_` prefix, is read by name and by `run all`; `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
- line: .andromeda/test-plan.md:132
old: «`run stand` runs the 20 `stand_*` readers of the shared module, `run {file}` (`run font_face_sniffed_source`, the one non-stand reader, which holds a load) and `run all` the others»
new: «`run stand` runs the 21 `stand_*` readers of the shared module, `run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`) and `run all` the others»
- line: .andromeda/test-plan.md:132
old: «`run stand` 185 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»
new: «`run stand` 190 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
- line: .andromeda/test-plan.md:146
old: «while the fourteen `stand_act_*` files drive it»
new: «while the fifteen `stand_act_*` files drive it»
- line: .andromeda/test-plan.md:185
old: «tests/blitz-tests/tests/stand_act_transformed.rs:76-116) (the fourteen `stand_act_*` files, §1)»
new: «tests/blitz-tests/tests/stand_act_transformed.rs:76-116); a `type` on a single-choice select chooses the first option whose label starts with the text — accepted and settled, its diff naming the select with `value` in `differs` and the two options whose `selected` moved, the select's options read as its children named by their labels, a disabled select skipped by the Tab walk and refused, in both layout modes (`stand_act_select`; tests/blitz-tests/tests/stand_act_select.rs:79-256) (the fifteen `stand_act_*` files, §1)»
- line: .andromeda/test-plan.md:324
old: «and `stand_load_wait` since 2026-10-20-load-wait,»
new: «`stand_load_wait` since 2026-10-20-load-wait and `stand_act_select` since 2026-10-20-select-by-id,»
- line: .andromeda/test-plan.md:324
old: «— 46 files, `stand_session_recording`»
new: «— 47 files, `stand_session_recording`»
- line: .andromeda/test-plan.md:334
old: «+3 · +6 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md»
new: «+3 · +6 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md; re-read at 2026-10-20-select-by-id on CI#39371468367, the pre-CI commit `14818317c` (green 17/17): linux 204 · 1083 · 0 · 12; macos and coverage 185 · 1080 · 0 · 12; windows 185 · 1038 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 165 · 660 · 0 · 21 — +2 result lines and +21 passed on the four test jobs over CI#39360124515 (`node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3), +3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/ci.md»
- line: .andromeda/test-plan.md:340
old: «(`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-load-wait/evidence/counts.md)»
new: «(`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-load-wait/evidence/counts.md); re-counted at 2026-10-20-select-by-id: 204 result lines, 1083 passed · 0 failed · 12 ignored, from 202 · 1062 — +2 result lines (`select_interaction`, `stand_act_select`) and +21 passed (`node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md)»
- line: .andromeda/test-plan.md:353
old: «run the 46 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 45 until 2026-10-20-load-wait»
new: «run the 47 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 46 until 2026-10-20-select-by-id added `stand_act_select`, whose CI `Stand checks repeated` on CI#39371468367 read 165 · 660 · 0 · 21, +3 · +15; 45 until 2026-10-20-load-wait»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «the fourteen driver-action files, `Session::run` in each»
new: «the fifteen driver-action files, `Session::run` in each»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «`stand_act_transformed` 2 (50; 48 before `stand_act_transformed`»
new: «`stand_act_transformed` 2 · `stand_act_select` 5 (55; 50 before `stand_act_select`, 48 before `stand_act_transformed`»
