group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «and a click on any other element (a CRUD row, the list) still clears focus;»
  source: .andromeda/a11y-plan.md:162 «- Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point»
  replace-with: «and a click on any other element (a CRUD row, the list) still clears focus; two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point with no `advance` between them are a double click, selecting a word in a text input — while each still focuses its target as one click does (PROVISIONAL);»

- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «and a click on any other element — a CRUD row or the list — still clears focus.»
  source: .andromeda/a11y-plan.md:162 «- Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point»
  replace-with: «and a click on any other element — a CRUD row or the list — still clears focus; two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point with no `advance` between them are a double click, selecting a word in a text input — while each still focuses its target as one click does (PROVISIONAL).»
