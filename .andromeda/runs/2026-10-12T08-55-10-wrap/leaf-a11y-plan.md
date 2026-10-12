group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «its crossing to an MCP client ratified by the founder (2026-10-10).»
  source: .andromeda/a11y-plan.md:275 «since 2026-10-12-self-description a password input typed through an MCP `type` reads `MASKED_VALUE` in the tool result, role `PasswordInput` and a non-empty name, the typed sentinel 0 times in both results»
  replace-with: «its crossing to an MCP client ratified by the founder (2026-10-10). Since 2026-10-12-self-description a password input typed through an MCP `type` reads `MASKED_VALUE` in the tool result, role `PasswordInput` and a non-empty name, the typed text in neither result, in both layout modes — a test-only element of `stand_mcp_masked.rs`'s fixture, no stand task or app gaining one; and two further readers of the same model state closed words only: the node keys' help that `help` serves (each key's reading, the boolean model's two limits among them) and `DiffNode::differs`, a changed node naming the readings that differ from before the step in node-key order — the Flight Booker's Book names eight nodes that only moved, each `differs` = `[bounds]` (`stand_diff.rs`).»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «The snapshot takes the three from the tree node, never from the element.»
  source: .andromeda/a11y-plan.md:275 «the node keys' help, which `help` serves — each key's reading stated by the tool, the boolean model's two limits among them (a mixed toggle reads `pressed` false, a grammar or spelling error reads `invalid` true)»
  replace-with: «The snapshot takes the three from the tree node, never from the element; the driver's served node-key help (`help`) states the boolean model's two limits (a mixed toggle reads `pressed` false, a grammar or spelling error reads `invalid` true), so a change to how the three are read changes that text too.»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «(`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s)»
  source: .andromeda/a11y-plan.md:275 «`DiffNode::differs`, a changed node naming the readings that differ from before the step in node-key order, the Flight Booker's Book naming eight nodes that only moved, each `differs` = `[bounds]`»
  replace-with: «(`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s; a changed node's `DiffNode::differs` names the readings that differ from before the step, in node-key order)»
