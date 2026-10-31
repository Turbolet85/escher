# Apply — test-plan · part 1 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan.md:17#1
- fanout-test-plan.md 1 · folded into fanout-test-plan--cascade-g1.md 2
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan.md:19#1, .andromeda/test-plan.md:19#2
- fanout-test-plan.md 2 · folded into fanout-test-plan--cascade-g1.md 4
- fanout-test-plan.md 3 · folded into fanout-test-plan--cascade-g1.md 4
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan.md:19#3
- fanout-test-plan.md 4 · folded into fanout-test-plan--cascade-g1.md 12
- fanout-test-plan--cascade-g3.md 11 · applied · .andromeda/test-plan.md:260#1
- fanout-test-plan--cascade-g4.md 15 · applied · .andromeda/test-plan.md:377#1
- fanout-test-plan--cascade-g4.md 16 · applied · .andromeda/test-plan.md:413#1
- fanout-test-plan.md 7 · applied · .andromeda/test-plan.md:151#1
- fanout-test-plan.md 8 · applied · .andromeda/test-plan.md:190#1
- fanout-test-plan.md 9 · applied · .andromeda/test-plan.md:190#2
- fanout-test-plan.md 14 · applied · .andromeda/test-plan.md:377#2
- fanout-test-plan.md 15 · refused · the report's F6 bears the fact out, but .andromeda/test-plan.md holds no Session lifecycle section, no session-start text and no line on `type` replacing after its focusing click: §3 stands there only as the pointer at line 192 to the contracts registry, so the claim's place is that registry's contract file (.andromeda/registries/contracts/test-plan/session-lifecycle.md, the report's `session-lifecycle.md`), outside this part's one target

## Edits
- line: .andromeda/test-plan.md:17
  old: «a named `select`'s entries have no test of their own»
  new: «a named `select`'s entries are read by form.rs's tests, a `multiple` select's set less its disabled rows and a single select's choice through a group»
- line: .andromeda/test-plan.md:19
  old: «the single-choice `select` model»
  new: «the `select` model, single and `multiple`, with `optgroup`,»
- line: .andromeda/test-plan.md:19
  old: «`text_extent`, while the select's are asserted by no check, nor is the process-wide `Parallel` traversal lock»
  new: «`text_extent` and the select's by `select_interaction` and `stand_act_select`; the process-wide `Parallel` traversal lock is asserted by no check»
- line: .andromeda/test-plan.md:19
  old: «`:open` matching a `details` or `dialog` carrying `open` has no test of its own»
  new: «`:open` matching a `details` or `dialog` carrying `open` is read by blitz-tests' `open_state`»
- line: .andromeda/test-plan.md:260
  old: «Cells walked in parts through the driver (`stand_cells_served`»
  new: «Cells walked through the driver, in parts and by keys to a cell whose committed value reads in the first part (`stand_cells_served`»
- line: .andromeda/test-plan.md:377
  old: «ten named blitz-tests targets outside the `stand_` prefix»
  new: «fifteen named blitz-tests targets outside the `stand_` prefix»
- line: .andromeda/test-plan.md:413
  old: «the ten named blitz-tests targets»
  new: «the fifteen named blitz-tests targets»
- line: .andromeda/test-plan.md:151
  old: «24 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source` and `select_interaction`;»
  new: «28 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md;»
- line: .andromeda/test-plan.md:190
  old: «`run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`)»
  new: «`run {file}` the six non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and `run text_range_no_change_event`)»
- line: .andromeda/test-plan.md:190
  old: «`run stand` 205 · 0 · 7 over 50 files, as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»
  new: «`run stand` 222 · 0 · 7 over 50 files, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/test-plan.md:377
  old: «every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`)»
  new: «every cargo leg `--locked` but `fmt` (.github/scripts/ci-leg.sh:20) and `examples/wasm_hello` (no `Cargo.lock`)»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = "entries have no test of their own"

[[pattern]]
id = "tp1-02"
fixed = "single-choice `select` model"

[[pattern]]
id = "tp1-03"
fixed = "the select's are asserted by no check"

[[pattern]]
id = "tp1-04"
fixed = "carrying `open` has no test of its own"

[[pattern]]
id = "tp1-05"
fixed = "ten named blitz-tests targets outside"

[[pattern]]
id = "tp1-06"
fixed = "the ten named blitz-tests targets"

[[pattern]]
id = "tp1-07"
fixed = "24 checks declare `mod fixtures;`"

[[pattern]]
id = "tp1-08"
fixed = "the two non-stand readers"

[[pattern]]
id = "tp1-09"
fixed = "`run stand` 205 · 0 · 7"

[[pattern]]
id = "tp1-10"
fixed = "every cargo leg `--locked` but `examples/wasm_hello`"
