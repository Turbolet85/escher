# Apply — security-plan · part 1 of 3

## Verdicts
- fanout-security-plan--cascade-g3.md 29 · applied · .andromeda/security-plan.md:133#1
- fanout-security-plan--cascade-g2.md 5 · folded into fanout-security-plan--cascade-g3.md 29
- fanout-security-plan--cascade-g3.md 30 · applied · .andromeda/security-plan.md:135#1
- fanout-security-plan--cascade-g2.md 9 · folded into fanout-security-plan--cascade-g3.md 30
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:10+#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:27+#1
- fanout-security-plan--cascade-g1.md 3 · applied · .andromeda/security-plan-detail/threat-model-summary.md:27+#2
- fanout-security-plan--cascade-g1.md 4 · applied · .andromeda/security-plan-detail/threat-model-summary.md:37+#1

## Edits
- line: .andromeda/security-plan.md:133
  old: «`rowspan` clamped to 1..=65534, `<ol start>` parsed as `usize`.»
  new: «`rowspan` clamped to 1..=65534, `size` on a `multiple` select capped at 65534 with `0` or a non-numeric value no hint, `<ol start>` parsed as `usize`; the `size` read is an engine change for every document and a boundary widening, ratified by the founder (2026-10-31).»
- line: .andromeda/security-plan.md:135
  old: «Enter activating buttons and links and Space buttons as a click, names and descriptions as page text; they leave escher only by the ratified carriers, the page's own form submission and events aside.»
  new: «Enter activating buttons and links and Space buttons as a click, Enter and Space opening a single select's engine-painted list, Space only with no typed text standing, names and descriptions as page text, an option's and a group's label also painted in an open list; they leave escher only by the ratified carriers, the page's own form submission and events aside; the list's keys and painted labels are an engine change for every document and a boundary widening, ratified by the founder (2026-10-31).»
- after: .andromeda/security-plan-detail/threat-model-summary.md:10
  anchor: «  - **Trust boundary:** Iframe nesting depth is limited to guard against infinitely recursive self-embedding pages (packages/blitz-dom/src/iframe.rs:15-18). `@import` nesting depth is limited to prevent unbounded recursion, e.g. an import URL that grows geometrically per level (packages/blitz-dom/src/net.rs:178-182). Stale iframe responses (iframe removed or re-navigated since the request) are discarded by request id (packages/blitz-dom/src/iframe.rs:155-184). Snapshotting a never-styled node is skipped because Stylo invalidation would otherwise panic (packages/blitz-dom/src/document.rs:1980-1987). Per-input checks are listed under Input Validation.»
  add: « A page's markup, a remote page's included, also reaches the engine-painted select list on every document: its option labels as the list's rows, its group labels as their headers, and `size` on a `multiple` select as a capped whole number, each held under Input Validation. A remote page can also reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden`, listed under Error Handling's panic paths (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- after: .andromeda/security-plan-detail/threat-model-summary.md:27
  anchor: « an answer also takes a `lint`'s findings out — stable ids, closed defect words and numbers, no page text, colour or value — the visual lint's crossing onto every ratified carrier (the Input Validation verb-table row)»
  add: « An answer's `screenshot` of an open list, on the socket and MCP carriers alike, also takes a page's option labels and group `label`s out as pixels, the shot's fields unmoved and `cli_commands` pinning a hosted shot equal to an in-process one: a boundary widening, ratified by the founder (2026-10-31) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- after: .andromeda/security-plan-detail/threat-model-summary.md:27
  anchor: « an answer also takes a `lint`'s findings out — stable ids, closed defect words and numbers, no page text, colour or value — the visual lint's crossing onto every ratified carrier (the Input Validation verb-table row)»
  add: « The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing) and as pixels in a shot; `NodeState` holds no field of it, `NODE_FIELDS` is thirteen and no `expanded` node key stands (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- after: .andromeda/security-plan-detail/threat-model-summary.md:37
  anchor: « a `lint` result carries its findings to the client — stable ids, closed defect words and numbers — the visual lint's crossing onto every ratified carrier (the Input Validation verb-table row); the pipe's second live run is a third carriage — through the `claude` client (2.1.308) and the operator's own login it carried six apps' screens to the model provider: element ids, accessible names and control values in snapshot texts, paged parts and diffs, Cells' contents, the flight booker's `flight-type` select with its options and value, one lint's findings and one screenshot, over 66 tool calls, 0 wrong, fired once (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  add: « A result's `image` block of an open list also holds a page's option labels and group `label`s as pixels, a boundary widening ratified by the founder (2026-10-31), and a snapshot or diff a row's own `bounds`, the crossing the boundary-session-socket group states; what a client does with them, a model provider included, is the client's (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»

## Patterns
(none)
