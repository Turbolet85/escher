# Answer card — 2026-10-13-range-input-interaction

A boundary widening the founder answered himself: the range interaction model in the engine, for every document with an `input type="range"`, the browser app's remote pages included. One halt, one question. The confirm checks that each proposal listed below lies wholly inside his answers. The ratification stays his answers, named as his in the owning sidecar.

## His answers, each cited by its record
- **The ruling, 2026-10-07:** «Пусть будет а». Given by question dialog in the overseer session for the 2026-10-07-act-by-id wrap, relayed verbatim by the overseer, choosing option (a) — a minimal range interaction model in the engine. The option's wording is the overseer's. Record: working-route.md:90, the entry's first CARRY.
- **The three P4 forks, 2026-10-13:** answered by question dialog in the overseer session. They are relayed verbatim by the overseer as his chosen option labels; the English wording is the overseer's. Records: inputs#I2 and inputs#I3, cited in the plan's Provenance (plan.md:451).
  - Q1 — where a moved range value is stored, engine-wide: «Отдельное текущее значение» = a separate current value. This was not the plan's recommendation. Its stated costs were the public type, the construction edit, the snapshot reader, the larger merge surface, and the re-seed on an attribute write.
  - Q2 — which keys step a focused range input: «Стрелки + Home/End (Рекомендую)» = the four arrows plus Home and End.
  - Q3 — whether a pointer click focuses a range input: «Клик даёт фокус (Рекомендую)» = the click focuses.
- **The operator at the P5 review (inputs#I4)** confirmed three leans as consequences of these answers, not new forks: submission sends the current value, a value write resets it, and the pointerdown moves the value while the click focuses. This is the operator's word on consequences. It carries no ratification of its own.

## The escalated proposals lying wholly inside them
Numbered as `fanout-results.md` numbers them.

- **security-plan 1** · §Input Validation · a new row, Boundary `Markup attributes`, `input type="range"`:
  - `min`, `max`, `step` and `value` are read through the HTML float grammar, with the defaults, bounds and sanitization keeping the value finite, bounded and on the grid;
  - the six keys, the pointerdown and the click are default actions in every document, remote pages included, `disabled` by presence;
  - each change fires one `input` event;
  - a `value` write re-seeds the value and a `min`/`max`/`step` write re-sanitizes it;
  - there is no log site, panic or env read;
  - the value leaves only as the form entry, the `input` event and the snapshot's `value`, inside the ratified carriers;
  - the row cites his ruling and his three answers verbatim.
  - The answers: the ruling for the model, Q1 for the value, Q2 for the keys, Q3 for the click.
  - Grouped occurrence (check 6): **security-plan 2** · §Input Validation · the `disabled` row adds the range input's key, pointerdown and click arms to what `disabled` presence gates.
- **architecture 1** · §Established Decisions → [Scrolling and input] · the fifth direct engine change for every document. It holds:
  - the current value apart from the attribute;
  - the keys, the pointerdown by position and the click focus;
  - one `input` per change;
  - the re-seed and re-sanitize, the cost Q1 took on;
  - form submission of the current value;
  - its ratification by his ruling and his three answers;
  - upstreamable against `434e0661`, its merge surface the seven upstream files plus `node/range.rs`.
  - Playbook :64 would make this routine, but its own clause sends a change that widens a boundary to escalation.
  - Grouped occurrences (check 6, `dependent-of` that proposal):
    - **architecture 2** · §Established Decisions → [Layout]: a range input gets `RangeInputData` at construction, seeded once and cleared off `type=range` (Q1).
    - **architecture 3** · §Existing Scopes → blitz-tests: `stand_act_range` now asserts that the driver moves the slider by click and key, and `range_input_interaction` is registered. This is test inventory. It rides the group by its mark, and settles the claim the report disproves at architecture.md:263.
- **a11y-plan 1** · §5 Keyboard handling: a focused, enabled range input steps by the arrows and goes to its minimum and maximum by Home and End, in every document, one `input` per change (Q2, under his label).
- **a11y-plan 2** · §5 Focus restoration: a click focuses a range input and changes no value; a main-button pointerdown sets the value by position (Q3, under his label).

Nothing else rides this card. Every other proposal of the pass is routine (`fanout-results.md`), and no proposal lies only partly inside his answers.

## The question
"The wrap records the range interaction model — the six keys, the click focus, the pointerdown by position and a value held apart from the attribute, for every document, remote pages included — as ratified by your ruling «Пусть будет а» (2026-10-07) and your three answers «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)», «Клик даёт фокус (Рекомендую)» (2026-10-13). Do the seven proposals in answer-card.md lie inside those answers?"

1. **Confirm (Recommended).** All seven are recorded as ratified by his answers, named as his in each owning sidecar.
2. **Name the ones outside.** The rest stand confirmed; each named one escalates on its own, in the full form.

## The answer
The founder's answer to the card, relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's: «Подтвердить (Рекомендую)» = 1, Confirm. He was told it also covers the three P5 consequences. Record: ~/dev/projects/additional/escher-overseer/relays/range-input-founder-answers.md, section 'The wraps answer card'.

- Authority: the founder, 2026-10-13, by question dialog in the overseer session (the overseer's clock 21:27:32Z right after), relayed verbatim by the overseer — his own word (gate-contract §Scope, *Whose word*). Not provisional.
- Snapped: the relay as inputs#I6 (`relay-2.md`); the overseer's record as inputs#I7 (its section "The wrap's answer card").
- Outcome: all seven proposals — security-plan 1–2, architecture 1–3, a11y-plan 1–2 — stand confirmed, recorded as ratified by his answers, each owning sidecar naming them as his. His confirmation also covers the three consequences the operator confirmed at the P5 review (inputs#I4): submission sends the current value; a `value` write resets it; the pointerdown moves the value and the click focuses.
