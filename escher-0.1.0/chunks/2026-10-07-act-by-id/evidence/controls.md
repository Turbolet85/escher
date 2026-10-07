# Controls — four mutations of the executor, each shown red and reverted

Step 12's record, made by hand on the dev host (Linux) on 2026-10-07. Each mutation was applied
alone to `packages/escher-driver/src/execute.rs`, the six driver-action checks were run, and the
file was restored before the next one.

- Command, the same for every mutation:
  `cargo test -p blitz-tests --locked --no-fail-fast --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range`
- Before the first mutation the same six files read 16 passed, 0 failed (6 · 2 · 2 · 1 · 2 · 3
  in the order diff, ids, keys, range, refused, timer).
- The pristine file's sha256 is `0216a8b1ced17c75c754aa27283183b179b15a4337da81f4bf41e238285ab897`
  before the first mutation and after the last revert.
- Every failure below is the first one a test met, in the `incremental=false` pass, which runs
  first; no test reached its `incremental=true` pass once it had failed. A message is quoted as
  the check printed it: a layout mode, a task and an index, no id and nothing a screen reads.

## 1 — the settle removed from a settled step

`settled_step`: `let busy = harness.settle().err().map(|not_settled| not_settled.busy);` replaced
by `let busy: Option<Busy> = None;` — the snapshot after the step is taken with no pass in between.

Named check: `stand_act_timer`. **Red** — cargo exit 101, 3 of 16 tests failed:

| file | test | first failure |
|---|---|---|
| `stand_act_timer` | `a_reset_then_an_advance_returns_settled_with_the_delayed_update_present` | `incremental=false: Timer: step 0: the elapsed time is in the returned diff as it reads after the advance` |
| `stand_act_diff` | `an_advance_on_the_timer_returns_the_elapsed_time` | `incremental=false: Timer: step 0 names exactly the nodes stated for it` |
| `stand_act_ids` | `an_id_addressed_click_has_the_effect_of_a_selector_click_on_a_twin` | `incremental=false: Timer: the driver's diff for Reset is the selector click's` |

The other 13 stayed green. What that shows: without the settle only `advance` reads wrong — its
ticks are delivered and no pass applies them before the second snapshot. `click`, `type` and
`press` stay green because each harness input helper makes one pass of its own, and no stand step
driven here needs a second. The settle's second-pass case (a write made while an element mounts)
is `stand_settle`'s, not a driver check's.

## 2 — the diff replaced by an empty one

`settled_step`: `(busy, before.diff(&after))` replaced by `(busy, before.diff(&before))`.

Named check: `stand_act_diff`. **Red** — cargo exit 101, 12 of 16 tests failed, in all six files:

| file | failed / tests | first failure of its first test |
|---|---|---|
| `stand_act_diff` | 5 / 6 | `incremental=false: Counter: step 0 returns the diff of the snapshots before and after it` |
| `stand_act_ids` | 1 / 2 | `incremental=false: Counter: control 1: the driver's diff is the selector click's` |
| `stand_act_keys` | 2 / 2 | `incremental=false: step 0 names exactly the controls stated for it` |
| `stand_act_range` | 1 / 1 | `incremental=false: sequence B: step 0 names exactly the controls stated for it` |
| `stand_act_refused` | 1 / 2 | `incremental=false: row 3: the steps before the call ran` |
| `stand_act_timer` | 2 / 3 | `incremental=false: Timer: step 0: the elapsed time is in the returned diff as it reads after the advance` |

The four that stayed green assert nothing a non-empty diff carries: the empty-diff check on a
click that changes nothing, the id acceptance check, the bound check on an id's length and the
refused `advance`. In `stand_act_diff` the password check went red on its positive control —
`incremental=false: fixture: what the call returned, written out whole, holds the mask` — which
is what keeps its "holds none of the typed text" assertion from passing on an empty result.

## 3 — an unresolved id answered with the first element

`centre_of`: the lookup `.find(|(_, listed)| listed == id).ok_or(not_found)?` given a fallback to
the first pair `element_ids()` lists, so an id that names nothing clicks the `html` element.

Named check: `stand_act_refused`. **Red** — cargo exit 101, 3 of 16 tests failed:

| file | test | first failure |
|---|---|---|
| `stand_act_refused` | `a_refused_call_names_its_cause_and_leaves_the_instance_unchanged` | `incremental=false: row 2: the call is refused` |
| `stand_act_refused` | `the_longest_admitted_id_is_looked_up_and_one_byte_more_is_malformed` | `incremental=false: an id of the longest admitted length reaches the lookup` |
| `stand_act_ids` | `the_snapshot_the_accessibility_tree_and_the_driver_read_one_id` | `incremental=false: Counter: an id no element carries is refused not-found` |

Rows 0 and 1 of the refused table (an unknown verb, a missing argument) passed before row 2
failed: they are refused by `validate`, which the mutation does not touch.

## 4 — `advanced_ms` returned as the milliseconds asked

`Session::run`, the `advance` arm: `advanced_ms = step(harness, ms).min(ms);` replaced by
`step(harness, ms); advanced_ms = ms;`.

Named check: `stand_act_timer`. **Red** — cargo exit 101, 1 of 16 tests failed:

| file | test | first failure |
|---|---|---|
| `stand_act_timer` | `a_reset_then_an_advance_returns_settled_with_the_delayed_update_present` | `incremental=false: Timer: step 2: the advance reports the whole ticks it moved` |

Step 2 is the advance of 50 ms, which moves no whole tick; steps 0 and 1 ask for whole ticks
(1000 and 300) and pass under the mutation, as they must.

## 5 — the clamp removed (a fifth control, beside the plan's four)

Mutation 4 removes the clamp together with the step's own report, so it does not isolate the
clamp, and the stand's step never claims more than it was asked. A seventeenth test was therefore
added after mutations 1 to 4 were run — `stand_act_timer`'s
`an_advance_reports_no_more_than_it_was_asked`, on a fixture session whose time step claims a
fixed amount — and the six files read 17 passed, 0 failed (6 · 2 · 2 · 1 · 2 · 4) on the pristine
file. Then:

`Session::run`, the `advance` arm: `advanced_ms = step(harness, ms).min(ms);` replaced by
`advanced_ms = step(harness, ms);`.

**Red** — cargo exit 101, 1 of 17 tests failed:

| file | test | first failure |
|---|---|---|
| `stand_act_timer` | `an_advance_reports_no_more_than_it_was_asked` | `incremental=false: fixture: row 3: the advance reports the lesser of what was asked and what the step claims` |

Row 3 asks for 100 ms of a step that claims 600; rows 0 to 2 claim no more than was asked and
pass under the mutation. The file's sha256 after the revert is the pristine one above.

## Not controlled

- The macOS arm of `press` (`deleteBackward:` dispatched after a `backspace`) is compiled out
  on the dev host; no mutation of it was run.

## A reading beside the controls — the delete binding on the dev host

Not a mutation. Read once through a temporary probe, deleted afterwards, in both layout modes:
on a held CRUD session with `crud-name` reading `Ad`, `Harness::apple_keybinding("deleteBackward:")`
left it reading `A`. So the binding the macOS arm dispatches deletes one character backward on
Linux, where the engine compiles the same handler. Whether a macOS build then deletes exactly one
character per `press backspace` — the key arm compiled out there, the binding alone acting — is
not measurable on this host; its one witness is the fork's macOS CI leg running `stand_act_keys`.
