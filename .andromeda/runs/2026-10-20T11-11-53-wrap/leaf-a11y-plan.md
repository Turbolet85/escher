group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «a `snapshot` result is `to_text`'s output itself, equal byte for byte to `Session::run`'s screen text in both layout modes (`stand_mcp_tools.rs`)»
  source: .andromeda/a11y-plan.md:290 «an accepted `snapshot` result is `to_text`'s output itself for a whole screen and, for a screen read in parts, its JSON line `{"text","part","parts"}` holding that part's text»
  replace-with: «a `snapshot` result is `to_text`'s output itself for a whole screen, equal byte for byte to `Session::run`'s screen text in both layout modes (`stand_mcp_tools.rs`), and, for a screen read in parts, its JSON line `{"text","part","parts"}` holding that part's text — through `serve_mcp` a walk's text blocks equal the JSON lines `Session::run` writes, in both layout modes (`stand_snapshot_parts.rs`)»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «one line per node, its role and name the node's, with no second mapping (`stand_snapshot_text.rs`).»
  source: .andromeda/a11y-plan.md:290 «the text's parts (`Snapshot::text_parts`, `TextPart`) are one more reader of that model through the text form and of nothing else — consecutive pre-order lines of `to_text`, never a split line»
  replace-with: «one line per node, its role and name the node's, with no second mapping (`stand_snapshot_text.rs`). The text's parts (`Snapshot::text_parts`, `TextPart`) are one more reader of that model through the text form and of nothing else — consecutive pre-order lines of `to_text`, never a split line, the parts concatenating to the whole text with every node id on exactly one line — and the driver's `snapshot` reads a screen over `SNAPSHOT_TEXT_BUDGET` in parts: on an over-budget fixture, in both layout modes, each part's lines equal their lines in the whole text, a password's line reads `MASKED_VALUE` in every part, and exactly one line reads `focused` after a click (`stand_snapshot_parts.rs`).»

- leaf: .claude/rules/a11y.md:27   class: omission
  leaf-text: «its text form `Snapshot::to_text` and its diff `Snapshot::diff` read that model and map nothing again»
  source: .andromeda/a11y-plan.md:57 «that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all five of which»
  replace-with: «its text form `Snapshot::to_text`, that text's parts `Snapshot::text_parts` and its diff `Snapshot::diff` read that model and map nothing again»
