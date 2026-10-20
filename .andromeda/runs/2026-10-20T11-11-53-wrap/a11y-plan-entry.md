
## 2026-10-20-large-screen-in-parts — the snapshot's parts, one more reader of the model
**Section:** §2 A11y Strategy (Feature exposure) · §7 Screen Reader Support
**Change:**
- §7: the text's parts (`Snapshot::text_parts`, `TextPart`) are one more reader of the snapshot model through the text form and of nothing else, and the driver's `snapshot` reads a screen over `SNAPSHOT_TEXT_BUDGET` in parts. On an over-budget fixture, in both layout modes, each part's lines equal their lines in the whole text, a password's line reads `MASKED_VALUE` in every part, and exactly one line reads `focused` after a click. An MCP `snapshot` result is `to_text`'s output for a whole screen and its JSON line for a paged one (was `to_text`'s output alone); through `serve_mcp` a walk's text blocks equal `Session::run`'s JSON lines.
- §2: the `accessibility` feature also gates `Snapshot::text_parts`; five readers named by the crate doc (was four).
**Why:** this chunk's a11y readings (report Outcome), on the paged answer the founder ratified, F1 «Аргумент part, части по умолчанию (Рекомендую)», relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm «Подтвердить все (Рекомендую)» (inputs#I5).
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
