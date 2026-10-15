group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:26   class: stale
  leaf-text: «The seven_guis stand binary enables no `accessibility` feature, so the windowed stand builds no platform adapter;»
  source: .andromeda/a11y-plan.md:14 «the feature reaches seven_guis through that edge although seven_guis' own manifest still names none, and both its binaries»
  replace-with: «The seven_guis stand names no `accessibility` feature in its own manifest — dioxus-native-dom's reaches it through escher-driver, so both its binaries compile the accessibility modules — and builds no platform adapter (`accesskit_xplat` and `accesskit_winit` in neither graph), so the stand binary gives no assistive technology a tree;»

- leaf: .claude/docs/a11y-summary.md:33   class: stale
  leaf-text: «2. Headless keyboard dispatch with focused-node read-back → "Stand keyboard harness"»
  source: .andromeda/a11y-plan.md:75 «since 2026-10-13-stand-keyboard-harness `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks through the same `press` / `press_with`, reading the focu»
  replace-with: «2. Headless keyboard dispatch with focused-node read-back → discharged: `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks, reading the focused node back per step (2026-10-13-stand-keyboard-harness)»

- leaf: .claude/docs/a11y-summary.md:34   class: stale
  leaf-text: «the driver leg waits on Epoch 4»
  source: .andromeda/a11y-plan.md:281 «since 2026-10-10-mcp-surface an MCP tool result is a further reader of that model and of nothing else — an accepted `snapshot` result is `to_text`'s output itself»
  replace-with: «driver leg built: `Session::run` refuses `disabled` on the snapshot's `enabled` reading, and an MCP tool result reads that model and nothing else, with no second role or name mapping»

- leaf: .claude/docs/a11y-summary.md:35   class: stale
  leaf-text: «→ "Stand a11y assertions"»
  source: .andromeda/a11y-plan.md:340 «since that chunk it runs the stand's keyboard, contrast and accessibility-id assertions, and the pushed sha's CI#38527662148 read green, 16/16, with "Accessibility (a11y) tests" among the checks»
  replace-with: «→ discharged: since 2026-10-14-stand-a11y-assertions the `a11y` leg runs six files, the stand's keyboard, contrast and accessibility-id assertions among them, and since 2026-10-14-quality-gates those three also run three times in the `stand-repeat` leg under the zero-failure budget»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «what an assistive technology reads of them is unmeasured (windowed witness owed).»
  source: .andromeda/a11y-plan.md:46 «the windowed witness, at an active assistive technology, is owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14), the headless half»
  replace-with: «what an assistive technology reads of them is unmeasured — the windowed witness, at an active assistive technology, owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 (2026-10-14), the headless half — the tree the shell hands the adapter — proven on the stand since 2026-10-14-stand-a11y-assertions.»

- leaf: .claude/rules/a11y.md:27   class: omission
  leaf-text: «its windowed witness still owed;»
  source: .andromeda/a11y-plan.md:36 «since 2026-10-14-stand-a11y-assertions both halves are proven headlessly on the stand, in both layout modes: on Counter, after a drained changed set, each Tab of the whole `tab_order` leaves `has_changes()`»
  replace-with: «its windowed witness, at the platform adapter, still owed and owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 (2026-10-14), both halves proven headlessly on the stand since 2026-10-14-stand-a11y-assertions;»
