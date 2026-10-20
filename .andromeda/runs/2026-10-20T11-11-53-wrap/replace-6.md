- line: .andromeda/a11y-plan.md:57
old: «that model's text form (`Snapshot::to_text`), its diff»
new: «that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its diff»
- line: .andromeda/a11y-plan.md:57
old: «all four of which its crate doc names»
new: «all five of which its crate doc names»
- line: .andromeda/a11y-plan.md:290
old: «change-tracking-and-diff/report.md; since 2026-10-10-mcp-surface an MCP tool result is a further reader»
new: «change-tracking-and-diff/report.md; the text's parts (`Snapshot::text_parts`, `TextPart`) are one more reader of that model through the text form and of nothing else — consecutive pre-order lines of `to_text`, never a split line, the parts concatenating to the whole text with every node id on exactly one line — and the driver's `snapshot` reads a screen over `SNAPSHOT_TEXT_BUDGET` in parts: on an over-budget fixture, in both layout modes, each part's lines equal their lines in the whole text, a password's line reads `MASKED_VALUE` in every part, and exactly one line reads `focused` after a click (packages/dioxus-native-dom/src/snapshot_text.rs:59-81; tests/blitz-tests/tests/stand_snapshot_parts.rs:82-204) — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md; since 2026-10-10-mcp-surface an MCP tool result is a further reader»
- line: .andromeda/a11y-plan.md:290
old: «an accepted `snapshot` result is `to_text`'s output itself and an acting call's result»
new: «an accepted `snapshot` result is `to_text`'s output itself for a whole screen and, for a screen read in parts, its JSON line `{"text","part","parts"}` holding that part's text — through `serve_mcp` a walk's text blocks equal the JSON lines `Session::run` writes, in both layout modes (tests/blitz-tests/tests/stand_snapshot_parts.rs:243-304) — and an acting call's result»
