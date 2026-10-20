
## 2026-10-20-large-screen-in-parts — `snapshot --part` on the cli surface
**Section:** §Surface: cli → Primary screens
**Change:** `escher-session`'s instance flags gain `--part`, one value. `snapshot` takes an optional `--part` (a whole number, 1 to 1000) and answers one line: `{"text":…}` for a whole screen, `{"text":…,"part":n,"parts":m}` for a screen read in parts (from part 1 without `--part`), a part past the last the empty text with the same `parts`, a `--part` outside its bound refused `malformed`, exit 1. The usage line opens `snapshot [--part <part>]` (was `snapshot`). Over the `mcp` role a paged `snapshot` result is its JSON line as one text block.
**Why:** the founder's F1 «Аргумент part, части по умолчанию (Рекомендую)» and F2 «Пустая часть (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm «Подтвердить все (Рекомендую)» (inputs#I5).
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
