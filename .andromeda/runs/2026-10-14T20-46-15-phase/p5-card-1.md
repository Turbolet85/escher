# P5 review — 2026-10-14-stand-requirement-sweep · "Stand requirement sweep"

Plan: `escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/plan.md` (546 lines, about 410 of prose outside the gate
fence; the template's band is ~150-400, so this is noted, not a halt: 16 folded CARRYs) · research.md · scope.md
(3 premises corrected at P3).

## Goal
Every 0.1.0 requirement the stand proves is proven by an agent, through `Session::run`, on each of the four lean tasks,
with a fixture beside the stand where the tasks lack the case. One new check, `stand_requirement_sweep`, carries it,
with a `WITNESSES` table naming each requirement's task-specific witnesses, each checked to exist. There is **no
engine change**: every founder fork was answered defer or measure (inputs#I3).

## Top acceptance criteria
1. On each of the four lean tasks, in both modes, through the driver alone:
   - one distinct id per node, equal to the tree's `author_id`;
   - the screen text within the budget;
   - an acting call settled, its diff equal to the check's own, and a no-op call's diff empty;
   - Tab focus read back;
   - two equal 800 × 600 shots;
   - `not-found` with its remedy, nothing changed.
2. `WITNESSES`: 16 rows (v010-01…14, 16, 17), every named test found in its file.
3. `unkeyed_actionable()` reads 0 on Temperature Converter, Circle Drawer and Cells (pin 2 · 3 · 676 → 0 · 0 · 0).
   The task files change by `id:` lines alone (a probe counts the other changed lines: 0).
4. Measured and pinned:
   - C3: no-role click targets per task (predicted Circle Drawer 1, Cells 676, others 0);
   - C12: the dialog range width (predicted 240);
   - C11: the password caret for narrow vs wide text;
   - C5: a one-axis-clip target refused `covered` (predicted), and `covered`'s fixed text states it;
   - C4: hits at an image's, a sub-document's and a text input's own box;
   - C6: a transformed-and-scrolled box; a vertical-writing-mode scrolled box under both builds;
   - C13: range attribute removal and a type change.
5. C9: the 90/91-byte bound runs on the macOS matrix leg of the pushed sha's CI run, which reads `verdict: green`.
6. C16: `evidence/log-counts.md` re-reads the host and `mcp` line, byte and span counts under both builds. The
   needles stay at 0.
7. Preservation guard: no change under `packages` but escher-driver's `refusal.rs` / `json.rs` / `lib.rs` (the
   `covered` text), and no manifest, workflow or lean-task change.

## Touchpoints
- **New:**
  - `tests/blitz-tests/tests/stand_requirement_sweep.rs`
  - `tests/blitz-tests/tests/stand_non_lean_tasks.rs`
  - `tests/blitz-tests/tests/password_caret_click.rs`
- **Modified:**
  - the three non-lean task files (ids)
  - `stand_actionable_keys.rs`
  - `stand_act_obstructed.rs`
  - `hit_clipped_at_scrolling_box.rs`
  - `scrolled_box_client_rect.rs`
  - `range_input_interaction.rs`
  - `examples/seven_guis/tests/cli_commands.rs`
  - escher-driver `refusal.rs`, `json.rs`, `lib.rs`

## Test commands (17 entries; baselines read on the untouched tree)
- Entries 1-3: the new and extended blitz-tests targets, per-package and workspace builds. Each reads red today
  because its new target is absent (exit 101), the plan's own red.
- Entry 4: escher-driver unit tests (green, 72).
- Entries 5-6: seven_guis `cli_commands` · `mcp_server` · `host_log` · `mcp_log`, both builds (green: 11 · 5 · 1 · 1).
- Entries 7-9: silence census · preservation guard (base 19756f2d) · attribute-only probe. Green on empty input; each
  control read both sides over a scratch clone (`controls-p5.md`).
- Entries 10-11: `agent-run.sh boot`, then `run stand` (green 162 · 0 · 7).
- Entries 12-14: `ci-leg.sh a11y` (green, six files) · `fast` · `doc`.
- Entries 15-17 (operator): hygiene, the pre-push gate with the clean-tree push, then `ci.py conclusion --sha HEAD
  --wait 1800`.
- No mutation runs (inputs#I1).

## Concretization preview
None. No capability is claimed: the pool holds only v010-15, which "Cold-agent test" owns. Three ledger notes are owed
at the wrap, as Expected amendments: `matrix#v010-01` (one distinct id per node through `Session::run` on all four
tasks), `matrix#v010-05` (focus read back through the driver's text on all four) and `matrix#v010-16` (the
actionable-key rule at 0 on the three non-lean tasks). Status, ref and acceptance are untouched.

## Decisions taken at P4
- **The founder's eight forks** (relayed verbatim by the overseer; inputs#I3, superseding inputs#I2, which reads
  drifted because the relay file grew):
  - C4 «Отложить (Рекомендую)»;
  - C5 «Ничего, замерить и записать (Рекомендую)»;
  - C14 «Отложить на следующую версию (Рекомендую)»;
  - C10 «Отложить на следующую версию (Рекомендую)»;
  - C7 «Отложить (Рекомендую)»;
  - C15 «Оставить по наличию, отложить (Рекомендую)»;
  - C11 «Замерить сейчас, исправить потом (Рекомендую)»;
  - C3 «Замерить сейчас, поле позже (Рекомендую)»;
  - and his words «но надо все отложеное где то записывать».
- **The operator** (by question dialog at P4): the sweep's form, "One driver sweep check + table"; and C2 / C8, "Key
  all three; C8 deferred".

## Beyond the plan
Each row is a `.andromeda/residuals.md` line, `(target: next)`, written at this chunk's wrap and naming the label it
rests on (the founder's words, inputs#I3):
1. C4: widening the hit's stop to a `contain: paint` box. The founder's «Отложить (Рекомендую)». The pin in
   `hit_clipped_at_scrolling_box` and `covered`'s text stand.
2. C14: fixing `Node::absolute_position` for a self-scrolled box, with its seven readers. The founder's «Отложить на
   следующую версию (Рекомендую)». `scrolled_box_absolute_position` stands as its pin.
3. C10: moving the overlay scrollbar fade onto the harness clock. The founder's «Отложить на следующую версию
   (Рекомендую)». The `screenshot` help's limit stands.
4. C7: reading `aria-checked` (as `checked` or `pressed`). The founder's «Отложить (Рекомендую)».
   `accessibility_states`' fourth test stands.
5. C15: narrowing `disabled` focusability to `button` · `input` · `select` · `textarea`. The founder's «Оставить по
   наличию, отложить (Рекомендую)». `stand_keyboard`'s pin stands.
6. C11 (fix): placing a password's click caret by the bullet layout. The founder's «Замерить сейчас, исправить потом
   (Рекомендую)». This chunk's measurement is the owner's starting reading.
7. C3 (field): a snapshot reading for "reacts to a click". The founder's «Замерить сейчас, поле позже (Рекомендую)».
   This chunk's per-task count is the owner's starting reading.
8. C8: Cells stating its selected cell (`aria-selected` under a grid role). The operator's "Key all three; C8
   deferred".

Apply? (yes / review / cancel)
