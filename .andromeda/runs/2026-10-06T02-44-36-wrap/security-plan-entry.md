
## 2026-10-06-headless-stand — `disabled` row: parsed for focus, presence for state and clicks
**Section:** §Input Validation → Markup attributes (`disabled`)
**Change:** was "Parsed as a boolean value; elements with it ignore pointer selection and click default actions"; now parsed as a bool for focusability only, while its presence alone — `disabled="false"` included — sets the DISABLED element state and makes the element ignore pointer selection and click default actions.
**Why:** the headless stand chunk measured `disabled="false"` matching `:disabled`; the row conflated the two readers.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
