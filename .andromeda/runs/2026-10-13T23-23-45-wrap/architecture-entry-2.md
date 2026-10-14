
## 2026-10-13-stand-keyboard-harness — `disabled` by presence, no `inert`, and the keyboard check
**Section:** §Established Decisions → [DOM semantics] · [Unsupported features] · §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → blitz-tests
**Change:**
- [DOM semantics]: `disabled` is read one way, by presence. `ElementData::flush_is_focussable` reads `has_attr(disabled)`, so a bare, a `"false"` and a `"true"` `disabled` on any element remove focusability, as the element state, the click target and the activation keys already key on it. The Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` move with it; the recompute on a `disabled` write is unchanged. Was: focusability parsed a boolean through `attr_parsed`, and `disabled="false"` and a bare `disabled` stayed focusable. Known limit, pinned by `stand_keyboard`: a `<div disabled tabindex=0>` is not focusable, unlike in browsers; its narrowing to `button`, `input`, `select`, `textarea` is owned by a CARRY on "Stand requirement sweep".
- [Unsupported features]: `inert` is not implemented. A truthy `inert` takes no focusability away; whether an inert subtree takes a hit is not measured. A fix is an engine change for every document, owned by a CARRY on "Stand a11y assertions".
- Dioxus DOM bridge: the snapshot `enabled` note — a `disabled="false"` and a bare `disabled` read `Some(false)` and are not focusable either. Was: "while the engine's focusability reader still takes both as focusable". The `inert: false` clause: cleared, and the engine honours no `inert` either way. Was: "it no longer makes content inert".
- §Existing Scopes:
  - `dioxus_falsy_boolean_attrs` reads 28 names, `inert` the 28th. Was 27, the falsy `inert` not measured.
  - The shared module states `tab_order` (CRUD six stops) and `focused` as well, and holds seven helpers. It is read by 19 stand checks, `stand_keyboard` the nineteenth (was 18).
  - The new target `stand_keyboard` is listed: 11 tests in both layout modes.
  - The twelve driver-action files hold 41 tests (was 40), `stand_act_keys` adding Enter and Space through the driver.
**Why:** `disabled` by presence is an engine change for every document. The founder answered the fork himself, «По наличию (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the `<div>` limit is named on the operator's condition. The `inert` reading corrects a claim about code the chunk did not edit to its measured limit, the fix carried. The counts are this chunk's own.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
