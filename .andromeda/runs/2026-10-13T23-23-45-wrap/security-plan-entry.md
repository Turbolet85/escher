
## 2026-10-13-stand-keyboard-harness — keyboard activation, a boundary widening ratified, and `disabled` by presence
**Section:** §Input Validation → Markup attributes: keyboard activation (new row) · `disabled` · `attr_parsed`
**Change:**
- New row, keyboard activation (Enter keydown, Space keyup) and a button click's focus, in every document, the browser app's remote pages included:
  - A pressed `Enter` on the focused node, after the text-input and range arms, dispatches a Click on a `button`, a `summary`, an `input` of type `submit`/`reset`/`button` or an `a` with `href`.
  - A released Space dispatches the same Click through the new `handle_key_up` (KeyUp's first default action) on a `button`, a `summary` or an `input` of type `submit`/`reset`/`button`/`checkbox`/`radio`. A link takes no Space; a text or range input takes neither key there.
  - The Click runs the app's handlers, then `handle_click`, so Enter and Space reach form submission, link navigation and `summary` toggling.
  - A click on a button focuses it before a submit button submits its form owner.
  - The gates are a closed element-name and type match and the `disabled` presence check. No print, log site, env read, `unwrap`, `expect` or panic in the added engine lines. No carrier, verb, key name, field, log field or span added.
- `disabled` row: focusability reads presence. Any `disabled`, bare, `"false"` or `"true"`, on any element removes it, for the Tab walk, autofocus and `unkeyed_actionable`. Presence also gates the Enter and Space arms. The snapshot's `enabled` and focusability read the same presence. Was: "Parsed as a boolean value for focusability", and a `disabled="false"` and a bare `disabled` "both stay focusable". Known limit: a `<div disabled tabindex=0>` is not focusable, unlike in browsers, its narrowing owned by a CARRY on "Stand requirement sweep".
- `attr_parsed` row: "as for `tabindex` and `disabled`" is now "as for `tabindex`"; `disabled` no longer goes through it.
**Why:** a boundary widening, never routine: a key now reaches the click default actions of any document. The founder answered the forks himself by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label: «Клавиши нажимают (Рекомендую)» (keyboard activation), «Клик даёт фокус (Рекомендую)» (a click focuses a button), «По наличию (Рекомендую)» (`disabled` by presence), 2026-10-13. He confirmed on this wrap's answer card, 2026-10-14, «Подтвердить (Рекомендую)», that the three rows lie inside those answers; the `disabled` row rode the card because presence gates the widening's arms.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
