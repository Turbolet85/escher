
## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — `:focus-within` matches
**Section:** §5 Keyboard Navigation (Focus order per layout (engine))
**Change:** was «`:focus-visible` and `:focus-within` never match»; now `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state, while `:focus-visible` still never matches (PROVISIONAL). Focus order is unmoved: no stand style uses `:focus-within`, `stand_keyboard` and `inert_subtree` are green, and the a11y leg reads 6 · 43.
**Why:** upstream #1226's behaviour for every document, taken on F1 answered by the operator as delegate (2026-10-19, by question dialog in the phase session). It is PROVISIONAL until the Epoch 4 boundary batch.
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/
