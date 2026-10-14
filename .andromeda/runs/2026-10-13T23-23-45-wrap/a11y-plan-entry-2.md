
## 2026-10-13-stand-keyboard-harness — `disabled` by presence, the CRUD list's keys and the keyboard check
**Section:** §5 Focus order · Per-surface keyboard shortcuts · Script and framework exposure · Test harness pattern · §3 Keyboard test harness · §7 Accessibility tree output · §8 Error recovery · §9 CI Integration
**Change:**
- §5 Focus order: an element is focusable only with no `disabled` attribute, any value, on any element. Disabled-ness is keyed one way, by presence. Was: focusability parsed the value as a bool and `disabled="false"` or a bare `disabled` stayed focusable. Known limit: a `<div disabled tabindex=0>` is not focusable, unlike in browsers, its narrowing owned by a CARRY on "Stand requirement sweep".
- §5 Per-surface: the CRUD list takes `tabindex="0"` (the third Tab stop), and Arrow/Home/End move the selection. The rows stay unfocusable options; the tree's focus is the list; no `aria-activedescendant`, owned by a CARRY on "Stand a11y assertions". Was: no keyboard reaches an option.
- §5 Script exposure: `inert` the 28th name the check reads; the engine honours no `inert`, owned by a CARRY on "Stand a11y assertions". Was: "it no longer makes content inert".
- §5 Test harness pattern and §3: + `stand_keyboard`, and the driver's Enter and Space in `stand_act_keys`. Its focus reading drops the engine's root fallback.
- §7: the snapshot's `enabled`, a present `disabled` not focusable either (was "both stay focusable"); the CRUD list focusable and a Tab stop in the actionable-key rule; the open gap restated as tree focus on the list; the 2026-10-06 Tab order bound to its chunk.
- §8: the forward sequence read against `common::tab_order`, CRUD six stops.
- §9: the a11y leg runs 6 + 6 + 4 tests (was 6 + 6 + 3).
**Why:** `disabled` by presence is the founder's own answer «По наличию (Рекомендую)» by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the `<div>` limit is named on the operator's condition. The list's keys are the operator's F5 decision. The `inert` reading corrects a claim about unedited code to its measured limit.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
