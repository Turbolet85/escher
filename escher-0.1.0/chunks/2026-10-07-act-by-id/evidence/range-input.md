# Range input — the Timer's `timer-duration` slider driven through the driver

Step 11's measurement, taken by hand on the dev host (Linux) on 2026-10-07 and pinned by
`tests/blitz-tests/tests/stand_act_range.rs`. Every action below is a `Session::run` call; nothing
reaches the held harness by selector, coordinate or direct dispatch. Readings are the snapshot's
(`DioxusDocument::snapshot`), taken after each call returned.

## Fixture

- The stand's Timer task, booted through `seven_guis::stand::boot_timer(stand::options(incremental))`
  and held in a `Session` carrying the stand's time step (`session_common::hold(LeanTask::Timer, …)`).
- Both layout modes: `incremental = false` and `incremental = true`. Every reading below is the
  same in both.
- The control: id `timer-duration` · role `Slider` · name `Duration:` · enabled · value `15` at
  boot (the `value` attribute the app writes from its 15 s duration) · not focused at boot.
- The display beside it: id `timer-duration-value`, name `15.0s` at boot.
- The markup: `input type="range" min="0" max="30" step="0.5"` with an `oninput` handler that
  sets the duration (`examples/seven_guis/src/tasks/timer.rs`).

## Sequence A — the plan's: a click on the id, then four keys

| # | call | settled | diff | `timer-duration` value | focused | `timer-duration-value` |
|---|---|---|---|---|---|---|
| 0 | (boot) | — | — | `15` | no | `15.0s` |
| 1 | `click` id=`timer-duration` | yes | empty | `15` | no | `15.0s` |
| 2 | `press` key=`arrow-right` | yes | empty | `15` | no | `15.0s` |
| 3 | `press` key=`arrow-left` | yes | empty | `15` | no | `15.0s` |
| 4 | `press` key=`home` | yes | empty | `15` | no | `15.0s` |
| 5 | `press` key=`end` | yes | empty | `15` | no | `15.0s` |

The click is accepted (the id resolves; no refusal) and changes nothing: it does not move the
value and it does not focus the slider, so the four presses that follow land on no focused control.

## Sequence B — focus reached by the keyboard, then the same four keys

Added to the plan's sequence because sequence A's click left the slider unfocused: an agent's
other way to reach it is Tab. On a fresh instance:

| # | call | settled | diff (`changed`) | `timer-duration` value | focused | `timer-duration-value` |
|---|---|---|---|---|---|---|
| 0 | (boot) | — | — | `15` | no | `15.0s` |
| 1 | `press` key=`tab` | yes | `back-btn` | `15` | no | `15.0s` |
| 2 | `press` key=`tab` | yes | `back-btn`, `timer-duration` | `15` | yes | `15.0s` |
| 3 | `press` key=`arrow-right` | yes | empty | `15` | yes | `15.0s` |
| 4 | `press` key=`arrow-left` | yes | empty | `15` | yes | `15.0s` |
| 5 | `press` key=`home` | yes | empty | `15` | yes | `15.0s` |
| 6 | `press` key=`end` | yes | empty | `15` | yes | `15.0s` |

With the slider holding focus the four keys still change nothing.

## What the run measures

- Predicted from the code read (research.md §Patterns detected, "No range interaction model"):
  the value reads the same after every call. **Measured: it does** — `15` and `15.0s` after each
  of the eleven calls above, in both layout modes.
- Not predicted, measured: a pointer click on the slider does not focus it; Tab does.
- So through the driver's five verbs the Timer's duration cannot be changed: neither a click nor
  an arrow, Home or End key moves a range input, focused or not.

## Not measured

- A pointer drag along the slider's track: the verb table has no drag, so an agent cannot make one.
- `arrow-up` and `arrow-down` are in the key list and were read once, after sequence A (the
  slider unfocused), with the same result — value `15`, empty diff. They were not read with the
  slider focused, are outside the plan's four keys and are not pinned.
- Any platform other than the dev host's.

No engine code was written for a range input, and no owner is named here: the owner is decided at
the wrap (the operator, 2026-10-07, at the P4 forks).
