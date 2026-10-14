group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «the first twelve 40 tests and two ignored children»
  source: .andromeda/architecture.md:263 «since 2026-10-10-snapshot-states, `stand_act_states`, 41 tests and two ignored children»
  replace-with: «the first twelve 41 tests and two ignored children»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «`stand_act_keys` (focus moves; the deleting-key check)»
  source: .andromeda/architecture.md:263 «focus moves and a deleting key — and, since 2026-10-13-stand-keyboard-harness, `press enter` and `press space` on a Tab-focused button each counting as its click»
  replace-with: «`stand_act_keys` (focus moves; the deleting-key check; since 2026-10-13-stand-keyboard-harness `press enter` and `press space` on a Tab-focused button each counting as its click)»

- leaf: .claude/docs/services/escher-driver.md:53   class: omission
  leaf-text: «a control lying where a scrolling box's scrolled-out row extends is clicked as itself (`stand_act_obstructed`).»
  source: .andromeda/architecture.md:133 «its handler runs and no row's, and since 2026-10-13-stand-keyboard-harness the clicked button reads `focused` after the accepted click, a click focusing a button in every document»
  replace-with: «a control lying where a scrolling box's scrolled-out row extends is clicked as itself, and since 2026-10-13-stand-keyboard-harness the clicked button reads `focused` after the accepted click, a click focusing a button in every document (`stand_act_obstructed`).»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «`press` presses its key on what holds focus (shift when asked)»
  source: .andromeda/architecture.md:87 «keyboard activation — a pressed `Enter` whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it carries no `disabled` attribute (presence)»
  replace-with: «`press` presses its key on what holds focus (shift when asked) — since 2026-10-13-stand-keyboard-harness a pressed `enter`, or a `space`, on a focused button with no `disabled` attribute dispatches its click (the engine's keyboard activation, [Scrolling and input])»
