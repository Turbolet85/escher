
## 2026-10-20-large-screen-in-parts — the paged `snapshot` answer line on the cli surface
**Section:** §Surface: cli §Component Patterns
**Change:** `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`. Over the `mcp` role a paged `snapshot` result is that JSON line (was the screen text alone).
**Why:** the paged answer the founder ratified, F1 «Аргумент part, части по умолчанию (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm «Подтвердить все (Рекомендую)» (inputs#I5).
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
