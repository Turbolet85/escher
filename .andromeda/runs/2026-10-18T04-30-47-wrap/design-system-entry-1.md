## 2026-10-18-engine-debts — the hit stops at a `contain: paint` box
**Section:** §Depth Strategy (Engine depth behavior, the hit-testing bullet)
**Change:** the hit stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to — the root element excepted; was «a node that clips by `overflow`» alone.
**Why:** F5 A — PROVISIONAL, delegate operator; hit and paint agree. No design-system line states a client rect, so F8 moves nothing here.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
