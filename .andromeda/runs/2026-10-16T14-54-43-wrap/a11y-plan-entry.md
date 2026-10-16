
## 2026-10-16-handler-splits-and-readme — §5's click and range rows cite the named functions
**Section:** §5 Keyboard Navigation (the button click focus row · the range click row · the range keys row)
**Change:** the button click focus row cites the private `click_button`; the range click row cites `press_range_input` for the pointerdown; the range keys row cites the range arm's gate beside the key mapping. Every claim stands as it read — focus through `generate_focus_events` / `set_focus_to`, the keys, the order and the `disabled` presence reads unchanged.
**Why:** the code audit's complexity split moved those bodies into private functions with no behaviour changed for any document; the a11y leg passed over its six files unchanged.
**Ref:** .andromeda/runs/2026-10-16T14-54-43-wrap/
