# Complexity — after (step 7)

The same instrument and reader as `complexity-before.md`, over the working tree after steps 2–5.

## The complexity gate entry (entry 4), in the full block's first firing

Fired in `gate.py run --plan {chunk_dir}/plan.md --run-dir .andromeda/runs/2026-10-16T14-27-34-implement --marker 2026-10-16-handler-splits-and-readme`:

```
  4 probe       green · exit 0 · 0.05s · 452 B → 4.2.log · d=$(mktemp -d) && rust-code-analysis-cli -m -O json -o "$d… (284 chars)
```

Its log (`4.2.log`), whole:

```
packages/blitz-dom/src/events/keyboard.rs:20 handle_key_or_input_event 53
packages/blitz-dom/src/events/pointer.rs:167 touch_action_pan_axes 17
packages/blitz-dom/src/events/pointer.rs:206 handle_pointermove 77
packages/blitz-dom/src/events/pointer.rs:383 handle_pointerdown 35
packages/blitz-dom/src/events/pointer.rs:655 handle_click 71
over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove touch_action_pan_axes
```

## The parents and the new functions, read from the same instrument's output

| function | before | after | upstream floor (434e0661) |
|---|---|---|---|
| `handle_click` (`events/pointer.rs`) | 74 | 71 | 69 |
| `handle_pointerdown` (`events/pointer.rs`) | 40 | 35 | 35 |
| `handle_key_or_input_event` (`events/keyboard.rs`) | 56 | 53 | 50 |
| `parse_float` (`node/range.rs`) | 19 | 10 | — |
| `range_track_fraction` (`events/pointer.rs:569`) | — | 2 | — |
| `press_range_input` (`events/pointer.rs:580`) | — | 1 | — |
| `click_button` (`events/pointer.rs:891`) | — | 1 | — |
| `press_range_input_key` (`events/keyboard.rs:144`) | — | 1 | — |
| `take_sign` (`node/range.rs:273`) | — | 0 | — |
| `take_digits` (`node/range.rs:278`) | — | 1 | — |
| `take_exponent` (`node/range.rs:288`) | — | 2 | — |

The over-ceiling set falls from six names to five: `parse_float` leaves it, and the set is the entry's exact `last
line`. Every extracted function and every `parse_float` step reads 2 or less. `handle_pointerdown` reaches its
upstream figure (35). `handle_click` (71) and `handle_key_or_input_event` (53) stay 2 and 3 above theirs: the
range-focus block in `handle_click` keeps its `if` in the walk (plan step 3), and the range-key and Enter arms'
conditions stay in the `else if` chain (plan step 4), as the plan directs.
