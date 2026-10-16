
## 2026-10-16-handler-splits-and-readme — the range and button arms as private named functions
**Section:** §Established Decisions ([Scrolling and input]) · §Standard Contracts (Events)
**Change:**
- Events: `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` keep their signatures, and the bodies of their range and button arms are private named functions: `range_track_fraction` and `press_range_input` for the range pointerdown, `click_button` for the button arm's focus and submit, `press_range_input_key` for the range keys. The range ones still write only through `apply_range_move`. The range input's private `parse_float` reads its sign, digits and exponent through the private steps `take_sign`, `take_digits` and `take_exponent`. No behaviour changed for any document. The button arm's citation gains `click_button`.
- [Scrolling and input]: the range keys cite the range arm's gate beside the key mapping, the range pointerdown cites `press_range_input`, and the button click cites `click_button`; every claim stands as it read.
**Why:** the code audit's complexity split, the parents left above the ceiling at their upstream floor so the next upstream sync's diff stays out of arms escher does not own.
**Kept:** the gates (`disabled` by presence, the closed element and type sets), event order and counts, and the range-focus block inside `handle_click`'s walk, each unchanged and in place.
**Ref:** .andromeda/runs/2026-10-16T14-54-43-wrap/
