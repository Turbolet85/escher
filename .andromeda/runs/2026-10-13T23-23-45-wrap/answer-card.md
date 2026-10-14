# Answer card — 2026-10-13-stand-keyboard-harness wrap (P2 Escalate)

## The founder's answers on record
The P4 forks are recorded in escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/plan.md:8-11. The answers were given
by the founder by question dialog in the overseer session on 2026-10-13 and relayed verbatim by the overseer as his
chosen option labels (inputs#I3, ../additional/escher-overseer/relays/keyboard-harness-founder-answers.md, verify:
unchanged):

- F1 — Enter/Space on a focused control run its click default action: «Клавиши нажимают (Рекомендую)»
- F2 — a pointer click on a plain button focuses it: «Клик даёт фокус (Рекомендую)»
- F3 — any `disabled` attribute removes focusability: «По наличию (Рекомендую)»

F1 is the boundary widening (playbook :36, never routine): Enter and Space now reach the click default actions — form
submission, link navigation and `summary` toggling — in every document, the browser app's remote pages included. They are
gated on the `disabled` attribute's presence, with no print, log site, env read or panic path, and with no carrier, verb,
key name, field, log field or span added. F2 and F3 are engine changes for every document (playbook :64).

## The escalated proposals read as lying wholly inside these answers
Each is numbered as in `fanout-results.md`.

- architecture 1 · §Established Decisions → [Scrolling and input] · records the three direct engine changes for every
  document (F1 keyboard activation, F2 a click focuses a button, F3 `disabled` by presence for focusability), each
  ratified by the founder by his label and upstreamable against 434e0661. The merge surface (`events/keyboard.rs`,
  `events/mod.rs`, `events/pointer.rs`, `node/element.rs`) is owned by a CARRY on "Quality gates" (card 2 row 4).
- architecture 7 · §Standard Contracts → Events · KeyUp gains its first default action (`handle_key_up`, Space → Click).
  `handle_key_or_input_event` dispatches a Click for Enter after the text-input and range arms, through
  `dispatch_activation_click`, so the app's handlers run before `handle_click`. The button arm of `handle_click` focuses
  before submitting. The signatures are unchanged.
- security-plan 1 · §Input Validation · a new row: keyboard activation and a button click's focus, in every document,
  remote pages included. Enter → Click on a focused `button`, `summary`, `input` of type submit/reset/button, or `a` with
  `href`. Space keyup → Click on a `button`, `summary`, or `input` of type submit/reset/button/checkbox/radio. Both are
  gated on `disabled` presence by a closed element-name and type match. A click on a button now focuses it. There is no
  print, log site, env read or panic, and no carrier, verb, key name, field or span is added. Recorded as an engine change
  for every document and a boundary widening, ratified by the founder (F1, F2).
- security-plan 2 · §Input Validation, the `disabled` row · focusability reads presence (F3). The boolean parse and "both
  stay focusable" are retired. The `<div disabled tabindex=0>` known limit is named, its narrowing owned by "Stand
  requirement sweep" (card 2 row 5). Presence also gates the Enter and Space arms.
- security-plan 3 · §Input Validation, the `attr_parsed` row · "as for `tabindex` and `disabled`" becomes "as for
  `tabindex`" (dependent of security-plan 2).
- a11y-plan 6 · §5 Keyboard handling · Enter (keydown) and Space (keyup) activate a focused button-like control in every
  document. SC 2.1.1 is met on the stand's buttons by measurement (20 rows; the driver's `press enter`/`press space`).

These six are all the proposals this card covers. Every other proposal is routine (playbook :28, :52, :56, :64) and
applies after the card, in the same pass.

## The question
Do the six proposals above lie inside the founder's answers F1–F3 (inputs#I3), so they are applied as his ratification?
- Confirm (Recommended) — the six apply. Each sidecar entry names his labels as the ratification, relayed verbatim by
  the overseer.
- Name the ones outside — the named ones escalate on their own question; the rest stand confirmed.

## The answer (inputs#I7)
The founder answered by question dialog in the overseer session; the overseer's clock read 2026-10-14T05:56:44Z right
after. Relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's:

«Подтвердить (Рекомендую)» = Confirm: all six apply as his ratification. He added no words of his own.

Resolution: architecture 1 · 7, security-plan 1 · 2 · 3 and a11y-plan 6 apply. Each owning sidecar entry names his
labels F1–F3 (inputs#I3) and this confirm (inputs#I7) as the ratification, relayed verbatim by the overseer.
