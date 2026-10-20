
## 2026-10-20-select-by-id — the user-agent select box
**Section:** §Surface: desktop-native → Tokens (platform-specific)
**Change:** a new bullet — in the user-agent sheet a `select` takes `input, textarea`'s border (`1px solid #999`), padding (`2px`) and background (`white`) and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token.
**Why:** this chunk's stylesheet change (the plan's expected amendment); the engine showing the chosen option's label is F2 «Engine shows its label», ratified by the founder (2026-10-20, «Утвердить все четыре (Рекомендую)», relayed verbatim by the overseer, chunk inputs#I3).
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/
