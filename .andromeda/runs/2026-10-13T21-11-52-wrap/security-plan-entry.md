
## 2026-10-13-range-input-interaction — the range input row, a boundary widening ratified
**Section:** §Input Validation (a new `Markup attributes` row for `input type="range"`; the `disabled` row)
**Change:**
- New row. In every document, the browser app's remote pages included, a range input holds a current value apart from its `value` attribute.
  - `min`, `max`, `step` and `value` are read through `parse_float`, the HTML floating-point rules: a parse error, NaN or an infinity reads absent; `-0` reads `0`.
  - Defaults: `min` 0, `max` 100, `step` 1. A `max` below `min` reads `min`; a step ≤ 0 or unparsable reads 1; `any` sets no grid. The base is `min`, else `value`, else 0; the default is the overflow-free midpoint.
  - Every seed, write and move is sanitized: clamped, put on the grid with an overflow-free index, rounded to the step's and base's decimals and written shortest round-trip. The value stays finite, bounded and on the grid on degenerate markup (8 unit rows, 7 integration rows).
  - The keys (arrows, `Home`, `End`), a main-button pointerdown by position and a focusing click act only with no `disabled` attribute.
  - One `input` per change. A `value` write re-seeds and a `min`/`max`/`step` write re-sanitizes.
  - No print, log site, `unwrap`, `expect`, panic or env read.
  - The value leaves only as the form entry, the `input` event's value and the snapshot's `value`, a control value inside the ratified carriers. No carrier, verb, field, log field, span or accessibility value or action is added.
- `disabled` row: presence now also gates the range input's key, pointerdown and click arms.
**Why:** a boundary widening, a validated surface admitting a new input class from any document's markup. It was ratified by the founder, never routine, by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label:
- his ruling of 2026-10-07 «Пусть будет а»;
- his three fork answers of 2026-10-13 «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)»;
- his confirm of this wrap's answer card «Подтвердить (Рекомендую)», which also covers the three consequences the operator confirmed at the review: submission sends the current value, a `value` write resets it, and the pointerdown moves the value while the click focuses.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
