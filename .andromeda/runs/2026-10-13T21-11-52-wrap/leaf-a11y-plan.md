group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «`press` / `press_with` (`stand_snapshot_state.rs`);»
  source: .andromeda/a11y-plan.md:212 «A range-input key and click check runs in both layout modes, since 2026-10-13-range-input-interaction: `range_input_interaction` drives a 7-row keys table and a 5-row clicks table through the harnes»
  replace-with: «`press` / `press_with` (`stand_snapshot_state.rs`); since 2026-10-13-range-input-interaction a range-input key and click check runs in both layout modes (`range_input_interaction.rs`: a 7-row keys table and a 5-row clicks table, one `input` per change and none on a no-change Home, the input focused after every click, a disabled range taking neither keys nor a click), and on the stand `stand_act_range.rs` moves the Timer slider through the driver by click and key, an arrow step after two Tabs leaving it focused with role `Slider`, its name and the forward focus sequence unchanged;»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «`:focus-visible` / `:focus-within` never match.»
  source: .andromeda/a11y-plan.md:165 «A focused `input type="range"` with no `disabled` attribute (read by presence) steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and its maximum on End»
  replace-with: «`:focus-visible` / `:focus-within` never match. Range input (since 2026-10-13-range-input-interaction, in every document, ratified by the founder): focused and with no `disabled` attribute (read by presence), it steps up on ArrowRight/ArrowUp, down on ArrowLeft/ArrowDown, goes to its minimum on Home and its maximum on End, every other key doing nothing — each change marks the changed set and fires one `input`, a move that changes nothing none; a click focuses it without moving its value, and a main-button pointerdown sets the value from the hit's fraction of the content box, clamped to [0, 1].»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «`checked` and `value` from the element (a password»
  source: .andromeda/a11y-plan.md:279 «else — for a range input, since 2026-10-13-range-input-interaction — its current value (`ElementData::range_input_value`, the formatted value a key or a pointerdown moved, while its `value` attribute»
  replace-with: «`checked` and `value` from the element (a range input's `value` its current value since 2026-10-13-range-input-interaction — `ElementData::range_input_value`, the formatted value a key or a pointerdown moved, while its `value` attribute keeps the boot value; a password»

- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «removing the focused node resets focus to body and runs blur side effects.»
  source: .andromeda/a11y-plan.md:165 «A focused `input type="range"` with no `disabled` attribute (read by presence) steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and its maximum on End»
  replace-with: «removing the focused node resets focus to body and runs blur side effects. Since 2026-10-13-range-input-interaction (in every document, ratified by the founder) a focused `input type="range"` with no `disabled` attribute (read by presence) steps up on ArrowRight/ArrowUp, down on ArrowLeft/ArrowDown, to its minimum on Home and its maximum on End, every other key doing nothing; its current value is held apart from the `value` attribute (`RangeInputData`), sanitized to `[min, max]` on the step grid, each change marking the changed set and firing one `input` (none when nothing changes); a click focuses it without moving the value, and a main-button pointerdown sets it from the hit's fraction of the content box.»

- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «`stand_accessibility_ids` pins the lean tasks' `focus_next_node` sequence);»
  source: .andromeda/a11y-plan.md:212 «A range-input key and click check runs in both layout modes, since 2026-10-13-range-input-interaction: `range_input_interaction` drives a 7-row keys table and a 5-row clicks table through the harnes»
  replace-with: «`stand_accessibility_ids` pins the lean tasks' `focus_next_node` sequence); a range-input key and click check runs in both layout modes (`range_input_interaction`, a disabled range taking neither keys nor a click; on the stand `stand_act_range` moves the Timer slider through the driver by click and key, and an arrow step after two Tabs leaves it focused with role `Slider`, its name and the forward focus sequence unchanged);»
