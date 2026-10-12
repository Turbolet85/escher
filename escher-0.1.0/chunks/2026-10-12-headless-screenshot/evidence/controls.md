# Controls — 2026-10-12-headless-screenshot

## Step 1 — the password on a frame, before any fix (2026-10-12, before the step-3 edit)

`cargo test -p blitz-tests --locked --test password_painted_masked --no-fail-fast` on the untouched engine
(blitz-paint at 938cd1b8, no mask), exit 101:

```
test two_password_values_of_one_length_paint_the_same_frame ... FAILED
test a_focused_password_input_paints_its_caret ... FAILED
test a_password_input_paints_its_bullets ... ok
test a_text_input_paints_its_characters ... ok
assertion `left == right` failed: mode incremental=false: two password values paint different frames
assertion `left == right` failed: mode incremental=false: two focused password values paint different frames
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

- The masking test reads red: two equal-length password values with no character in common paint different
  frames, so the engine paints a password's characters (scope, CARRY 1 — the hypothesis holds on a frame).
- The control reads green: the same two values in a `text` input paint different frames, so the check sees
  characters.
- The premise of fork 2 stands; the plan proceeds.

## Step 19 — mutation controls (2026-10-12, after the gates' first full green)

Each control saves the file's bytes, applies one exact replacement, runs
`cargo test -p blitz-tests --locked --no-fail-fast` on the named targets with stderr merged into stdout, writes the
saved bytes back and compares sha256 before the next one. Every restore read equal; the final full gate firing ran
after all three.

- **mask off** — `render.rs`: the password branch guarded `if false && …`. Step 4's two masking tests and step
  14's password case read red; every other case stays green.
- **crop rounded inward** — `execute.rs`: `floor`/`ceil` swapped in `crop`. Only the 369 × 420 case reads red.
- **font check removed** — `screenshot.rs`: the `can_paint_text` refusal guarded `if false && …`. Only the
  `fonts-unavailable` case reads red.

```
## control: mask off (packages/blitz-paint/src/render.rs)
exit 101 · restore sha256 74971c1c003aecfd -> 74971c1c003aecfd · restored
Running tests/password_painted_masked.rs (target/debug/deps/password_painted_masked-d9daaff3c5ad7f3c)
test two_password_values_of_one_length_paint_the_same_frame ... FAILED
test a_focused_password_input_paints_its_caret ... FAILED
test a_password_input_paints_its_bullets ... ok
test a_text_input_paints_its_characters ... ok
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s
Running tests/stand_act_screenshot.rs (target/debug/deps/stand_act_screenshot-084ea74239696c5f)
test an_app_with_no_usable_font_is_refused_and_nothing_changes ... ok
test a_typed_password_never_leaves_as_pixels ... FAILED
test an_element_shot_is_refused_as_every_id_addressed_call_is ... ok
test an_element_shot_is_its_bounds_rounded_outward ... ok
test a_shot_changes_nothing ... ok
test the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints ... ok
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.11s

## control: crop rounded inward (packages/escher-driver/src/execute.rs)
exit 101 · restore sha256 2ec742888f88b7ca -> 2ec742888f88b7ca · restored
Running tests/stand_act_screenshot.rs (target/debug/deps/stand_act_screenshot-084ea74239696c5f)
test an_app_with_no_usable_font_is_refused_and_nothing_changes ... ok
test an_element_shot_is_its_bounds_rounded_outward ... FAILED
test an_element_shot_is_refused_as_every_id_addressed_call_is ... ok
test a_typed_password_never_leaves_as_pixels ... ok
test a_shot_changes_nothing ... ok
test the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints ... ok
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.22s

## control: font check removed (packages/blitz-test-harness/src/screenshot.rs)
exit 101 · restore sha256 83e1927fc001a930 -> 83e1927fc001a930 · restored
Running tests/stand_act_screenshot.rs (target/debug/deps/stand_act_screenshot-084ea74239696c5f)
test an_app_with_no_usable_font_is_refused_and_nothing_changes ... FAILED
test an_element_shot_is_refused_as_every_id_addressed_call_is ... ok
test an_element_shot_is_its_bounds_rounded_outward ... ok
test a_typed_password_never_leaves_as_pixels ... ok
test a_shot_changes_nothing ... ok
test the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints ... ok
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.71s
```
