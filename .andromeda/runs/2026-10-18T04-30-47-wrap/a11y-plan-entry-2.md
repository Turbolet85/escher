## 2026-10-18-engine-debts — §5, §6: `inert` and `disabled` in focus order, keys and the hit
**Section:** §5 Keyboard Navigation (Focus order per layout · Focus restoration · Keyboard handling · Per-surface keyboard shortcuts · Script and framework exposure · Test harness pattern) · §6 Visual Design Verification (Visibility and hit-testing)
**Change:**
- Focus order: an element is focusable when it is not inert (presence, on itself or an ancestor — `Node::is_inert`) and no disabled form control; `disabled` read by presence on a form control alone (`ElementData::is_disabled`), so a `<div disabled tabindex=0>` takes focus — was «on any element … the C15 limit»; the focus-fixup limit stated, owned by a `.andromeda/residuals.md` line; disabled-ness keyed by `is_disabled` for the pointer, click, activation and focus gates, the `DISABLED` state still by presence.
- Focus restoration: a disabled form control stops the click walk, a disabled ancestor `div` not.
- Keyboard handling: a `disabled` attribute on a `summary` or an `a` gates neither Enter nor Space.
- CRUD list: the unread `aria-activedescendant` pinned by `accessibility_active_descendant_unread` (was owned by a residuals line).
- Script and framework exposure: was «the engine honours no `inert` … F3 … whether an inert subtree takes a hit unmeasured»; now a truthy `inert` takes focusability, hits and the tree away, a falsy one is removed.
- Test harness: `stand_keyboard`'s `disabled` and `inert` tests restated.
- Visibility and hit-testing: an element carrying `inert` and its subtree leave the tree and the snapshot, a hit passes through it, removing `inert` restores both.
**Why:** F1 A, F3 B and F7 A — PROVISIONAL, delegate operator, as the first entry states.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
