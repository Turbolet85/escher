# a11y-plan · picked accounting · group 4 of 5

## Picked
visual-design-verification#1.2 · not touched · `render.rs` gains only the `paint_select_list` call and its module line, and the list's own thumb lives in `render/select_list.rs`, so the default scrollbar's stroke holds
visual-design-verification#1.4 · proposed · `STATE_ROWS` moved 18 → 20 with tally (19, 0, 1, 0), per the report's `state-rows`
visual-design-verification#1.5 · not touched · the a11y leg's +2 are `stand_keyboard`'s and `state-rows` is `stand_contrast`'s one move, so its two further tests hold
visual-design-verification#1.6 · proposed · the `a11y` leg now reads 114 tests over 14 targets, while the lint's contrast rule, `at_boot` and the planted pair hold
visual-design-verification#1.7 · proposed · the retired `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group, which also takes F22 beside it
visual-design-verification#1.8 · proposed · the single select's unmeasured pair must split into a closed select's unread pair and an open list's measured one, which also takes F21 beside it
visual-design-verification#1.9 · proposed · the `multiple` row's pair stands per `select-colours`, and F20 and F26 sit beside it
visual-design-verification#3.8 · not touched · `default.css` gains only the `select[multiple][size]` rules at 183-193, so the focus outline and its suppression hold
visual-design-verification#3.10 · not touched · every other `stroke_text` caller passes `RunPaint::default()` and `render.rs` adds only the list's call, so caret and bullet painting hold
visual-design-verification#5.1 · proposed · the absence holds with its gate entry green, and F24 sits beside it
visual-design-verification#6.2 · not touched · the report states `:open` and `flush_open_state` unmoved, so `details` and `dialog` read as written
visual-design-verification#6.3 · proposed · the WPT line holds, and F23's not-colour-alone header sits beside it
visual-design-verification#7.7 · not touched · `default.css` gains only the `select[multiple][size]` rules, so the `inverted-colors` rule holds
visual-design-verification#7.8 · not touched · the report's pointer edits are the list's press, click, hover and wheel arms and none reads `touch-action`, so touch panning holds
visual-design-verification#8.1 · not touched · `hit_inner` is not edited, a select made inert closes its list and inert pass-through stands, so the tree and hit rule hold
visual-design-verification#8.6 · proposed · the report's `hit-first` says the thumb hazard stands unchanged, and F25 sits beside it
