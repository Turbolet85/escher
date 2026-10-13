
## 2026-10-13-range-input-interaction — the range interaction model, the fifth direct engine change
**Section:** §Established Decisions → [Scrolling and input] · [Layout] · §Standard Contracts → Events · §Cross-cutting Patterns → Invalidation and state integrity
**Change:**
- [Scrolling and input]: a fifth direct engine change for every document, beside the three the decision counts and the password paint under [Painting].
  - An `input` whose `type` reads `range` (ASCII case-insensitive) holds its current value apart from its `value` attribute.
  - The rules follow the WHATWG range state in `node/range.rs`: `min` 0, `max` 100 and `step` 1 by default; a `max` below `min` reads `min`; a step ≤ 0 or unparsable reads 1; `any` sets no grid; the step base is `min`, else the `value` attribute, else 0; the default is the midpoint; every value is sanitized to `[min, max]` and the nearest grid value, the greater on a tie.
  - Focused and not `disabled` (presence): the arrows step it, `Home`/`End` take it to its minimum/maximum, other keys do nothing.
  - A main-button pointerdown sets the value by the hit's fraction of the content box; a click focuses and moves nothing.
  - One `input` per change, none for a no-change move.
  - A `value` write re-seeds the value and a `min`/`max`/`step` write or removal re-sanitizes it, a departure from the spec's dirty-value flag.
  - Form submission sends the current value.
  - Flagged upstreamable against 434e0661, no pull request open. Merge surface: seven upstream files plus `node/range.rs`.
  - Was: no key or click moved it, and a click did not focus it.
- [Layout]: a range input gets `RangeInputData` at construction. It is seeded only when none is held and cleared off `type="range"`, with no early return.
- Events: the range `input` dispatch goes through `BaseDocument::apply_range_move`, and the click focuses through `generate_focus_events`. No handler signature changes.
- Cross-cutting: the changed set is written at four kinds of site, was three. The fourth is `apply_range_move`, which marks a range input whose value changed and nothing for a no-change move.
**Why:** a boundary widening and an engine change for every document. It was ratified by the founder by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label:
- his ruling of 2026-10-07 «Пусть будет а»;
- his three fork answers of 2026-10-13 «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)»;
- his confirm of this wrap's answer card «Подтвердить (Рекомендую)», which also covers the three consequences the operator confirmed at the review.

The re-seed is the first answer's stated cost, so a Dioxus re-render keeps control. The merge surface is owned by a CARRY on "Quality gates".
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
