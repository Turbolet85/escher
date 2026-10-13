
## 2026-10-13-range-input-interaction — the range input becomes keyboard-operable and clicks focus it
**Section:** §5 Keyboard Navigation (Keyboard handling · Focus restoration · Test harness pattern) · §7 Screen Reader Support (Accessibility tree output)
**Change:**
- §5 Keyboard handling: in every document, a focused range input with no `disabled` attribute (presence) steps up on ArrowRight/ArrowUp and down on ArrowLeft/ArrowDown, and goes to its minimum on Home and its maximum on End. Other keys do nothing. Each change marks the changed set and fires one `input`.
- §5 Focus restoration: a click focuses a range input and moves no value, so a label's click focuses it too; a main-button pointerdown sets the value by position. Was: no click focused or moved a range input.
- §5 Test harness pattern: `range_input_interaction` (a 7-row keys table and a 5-row clicks table, both modes), and `stand_act_range` driving the Timer slider. After two Tabs an arrow step leaves it focused, with role `Slider`, its name and the forward focus sequence unchanged.
- §7: the snapshot's `value` for a range input is its current value (`range_input_value`), was its `value` attribute. The file-input clause stands.
**Why:** SC 2.1.1 keyboard reach on the stand's slider. The keys and the click focus were ratified by the founder (2026-10-13, his fork answers «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)», confirmed on this wrap's answer card «Подтвердить (Рекомендую)», each by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label).
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
