# Complexity — before (step 1's two known-verdict controls)

Instrument: `rust-code-analysis-cli 0.0.25`, `-m -O json`, every space of kind `function`, `metrics.cognitive.sum`
(the audit's rule, `.andromeda/runs/2026-10-15T10-44-59-code-audit/record.json` `commands.complexity`), read by
`evidence/complexity.py`. Both readings were taken 2026-10-16 before any source edit, on the untouched tree at
`d236e9186e4ff6ca2610ddfbc9f54f590e8dd01b`.

## Must fail — the complexity gate entry (entry 4) on the untouched tree

Fired as itself: `gate.py run --plan {chunk_dir}/plan.md --run-dir .andromeda/runs/2026-10-16T14-27-34-implement --marker 2026-10-16-handler-splits-and-readme --entry 4`.

Entry line:

```
  4 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.05s · 520 B → 4.log · d=$(mktemp -d) && rust-code-analysis-cli -m -O json -o "$d… (284 chars)
```

Its log (`4.log`), whole:

```
packages/blitz-dom/src/events/keyboard.rs:20 handle_key_or_input_event 56
packages/blitz-dom/src/events/pointer.rs:167 touch_action_pan_axes 17
packages/blitz-dom/src/events/pointer.rs:206 handle_pointermove 77
packages/blitz-dom/src/events/pointer.rs:383 handle_pointerdown 40
packages/blitz-dom/src/events/pointer.rs:640 handle_click 74
packages/blitz-dom/src/node/range.rs:234 parse_float 19
over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove parse_float touch_action_pan_axes
```

Red as required: exit 1, `parse_float` in the `over-ceiling:` line, the three parents at 74 · 40 · 56 — the audit's
figures on the same instrument.

## Must pass — the merge-base copies

`git show 434e06615d5f47a3df92c0f94e916df088fdcf4d:packages/blitz-dom/src/events/{pointer,keyboard}.rs` written to a
scratch directory (`node/range.rs` does not exist at the merge base: `git cat-file -e` reads `exists on disk, but not
in '434e0661'`), then, from that directory, `rust-code-analysis-cli -m -O json -o out -p src/events/pointer.rs -p
src/events/keyboard.rs` and `python3 evidence/complexity.py out`:

```
src/events/keyboard.rs:16 handle_key_or_input_event 50
src/events/pointer.rs:167 touch_action_pan_axes 17
src/events/pointer.rs:206 handle_pointermove 77
src/events/pointer.rs:383 handle_pointerdown 35
src/events/pointer.rs:614 handle_click 69
over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove touch_action_pan_axes
exit 0
```

Green as required: exit 0, the last line the entry's exact `last line` atom, the parents at 69 · 35 · 50.
