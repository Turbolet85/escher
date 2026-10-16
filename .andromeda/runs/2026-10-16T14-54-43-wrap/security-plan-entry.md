
## 2026-10-16-handler-splits-and-readme — the range and keyboard-activation rows cite the named functions
**Section:** §Input Validation (the `input type="range"` row · the keyboard-activation row)
**Change:**
- The range row: `parse_float` reads the four attributes through its private steps `take_sign`, `take_digits` and `take_exponent`, the readings unchanged; the keys cite the range arm's gate beside the key mapping (`press_range_input_key`); the pointerdown cites `press_range_input`.
- The keyboard-activation row: the button arm's focus and submit sit in the private `click_button`.
- No gate, input class, reading, carrier or log site moved; the added lines carry no `unwrap`, `expect`, panic, print, `tracing` site or env read, so §Error Handling's panic-path list gains nothing.
**Why:** the code audit's complexity split moved those bodies into private functions with no behaviour changed; the rows' claims stand and cite where the code now is.
**Ref:** .andromeda/runs/2026-10-16T14-54-43-wrap/
