# Range input — the readings after the minimal range interaction model

Step 12's record, taken by /implement on the dev host (Linux) on 2026-10-13, at the working tree
over `97ce760d`. Beside it, the starting point measured at 2026-10-07-act-by-id
(`escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/range-input.md`): no driver verb moved the
slider, and a click did not focus it.

## The Timer slider's box at the pinned 800 × 600 viewport

Read from `final_layout()` of `#timer-duration` after `seven_guis::stand::boot(LeanTask::Timer,
stand::options(incremental))`, in a throwaway probe test deleted after the reading. Both layout
modes read the same:

| reading | value |
|---|---|
| border box (page) | x 360 · y 331 · 109 × 2 |
| border | 1 px each side |
| padding | 0 px each side |
| content box | 107 × 0 |

The plan's hypothesis was that a centre click maps to the midpoint, 15. **Measured: it does.** But
its stated basis does not hold. The default sheet's 2 px padding does not apply, because the stand's
styles leave the slider with 0 padding. What makes the centre of the box the centre of its content
box is the symmetric 1 px border. After Home, the display beside the slider narrows (`0.0s`) and the
slider widens to 111 (content 109). The click at step A9 lands at the centre of that box, which is
fraction 0.5 of the wider content box, so it reads 15 there too.

## Sequence A — through the driver, from boot

Taken through the `escher-session` command line (`start timer`, the nine calls, `stop`; state
directory `target/tmp/…`, removed by `stop`). Every call exited 0, returned settled and added or
removed nothing. `tests/blitz-tests/tests/stand_act_range.rs` asserts the same sequence through
`Session::run` in both layout modes. Each diff equals the file's own `before.diff(&after)`, and the
diffs are equal across the two modes.

| # | call | `timer-duration` value | focused | changed (`differs`) | `timer-duration-value` |
|---|---|---|---|---|---|
| 0 | (boot) | `15` | no | — | `15.0s` |
| 1 | `click` id=`timer-duration` | `15` | yes | `timer-duration` (`focused`) | `15.0s` |
| 2 | `press` `arrow-right` | `15.5` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `15.5s` |
| 3 | `press` `arrow-up` | `16` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `16.0s` |
| 4 | `press` `arrow-left` | `15.5` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `15.5s` |
| 5 | `press` `arrow-down` | `15` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `15.0s` |
| 6 | `press` `end` | `30` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `30.0s` |
| 7 | `press` `home` | `0` | yes | `timer-duration` (`value`, `bounds`) · `timer-duration-value` (`name`, `bounds`) | `0.0s` |
| 8 | `press` `arrow-right` | `0.5` | yes | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `0.5s` |
| 9 | `click` id=`timer-duration` | `15` | yes | `timer-duration` (`value`, `bounds`) · `timer-duration-value` (`name`, `bounds`) | `15.0s` |

The display's text is narrower at `0.0s` and `0.5s` (40 px against 42 px). The slider has
`flex: 1`, so steps 7 and 9 also read `bounds` on both nodes. The stand check asserts `value` and
`name` among what differs, not the whole list.

Against 2026-10-07: then, all eleven calls read `15` · `15.0s` with an empty diff, and the click
left the slider unfocused. Now the click focuses it, every key that moves the value names the
slider and the display, and a click sets the value by position.

## Sequence B — focus by the keyboard, from a fresh instance

Asserted by `stand_act_range` in both layout modes:

| # | call | changed | value | display |
|---|---|---|---|---|
| 1 | `press` `tab` | `back-btn` | `15` | `15.0s` |
| 2 | `press` `tab` | `back-btn` · `timer-duration` | `15` | `15.0s` |
| 3 | `press` `arrow-right` | `timer-duration` (`value`) · `timer-duration-value` (`name`) | `15.5` | `15.5s` |

After step 3 the slider is still focused in the snapshot and is still the accessibility tree's
focus. It reads role `Slider` and the same name before and after.

## The engine check — `range_input_interaction`

Inline HTML through `blitz_test_harness::Harness`, both layout modes, with every row asserted. The
fixture input has a 200 px width, 4 px × 20 px padding and a 2 px border.

- **Keys**, on `min=0 max=10 step=2 value=4`, focused (7 rows): right `6`, up `8`, left `6`,
  down `4`, End `10`, Home `0`, Home again `0`. One `input` event per change; none for the last
  row. The input stays focused.
- **Clicks** (5 rows): content-box fractions 0 · 0.25 · 0.5 · 1 read `0` · `2` · `6` · `10`, and a
  point in the left padding reads `0`. Each fires one `input` event, and the input is focused after
  each.
- **Degenerate attributes** (7 rows), each driven by a right arrow and a centre click: `step="0"`,
  `step="-1"`, `step="any"`, `min="10" max="5"`, `value="abc"`, `value="1e308"` and `min="NaN"`.
  Each completes with no panic and reads a finite value inside `[min, max]`, on its grid. With no
  `min`, `value="1e308"` is the step base, by the spec's step-base rule.
- **Decimals:** `step="0.1"` stepped three times from 0 reads `0.3`.
- **Disabled:** neither a right arrow nor a click moves `4`, and no `input` event fires.
- **Attribute writes:** after a right arrow (`6`), a `value="7"` write reads `8`, and then a
  `max="5"` write reads `4`.

## Totals the gates read

| gate | measured | before (stand-contrast-harness) |
|---|---|---|
| `bash scripts/agent-run.sh run stand` | passed 141 · failed 0 · ignored 7 over 37 files; `stand_act_range` 1 test | 141 · 0 · 7 over 37 files |
| workspace tests (`target/ci-logs/test.log`, `ci-leg.sh fast`) | 181 result lines · 915 passed · 0 failed · 12 ignored | 180 · 900 · 0 · 12 |
| `tests/blitz-tests/tests/*.rs` | 120 files (119 targets) | 119 (118) |

The workspace adds 15 passing tests:
- `range_input_interaction`: 6;
- `range.rs`'s unit tables: 7;
- the form entry test: 1;
- the snapshot's added test: 1.

The renamed snapshot test is counted once, under its new name. The `stand_act_range` rewrite keeps
one test.
