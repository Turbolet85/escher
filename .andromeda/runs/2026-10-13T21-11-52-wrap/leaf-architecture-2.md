group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:55   class: retired
  leaf-text: «A click on a range input is accepted and does not focus it, and no key moves its value: through the instance verbs a range input cannot be changed (measured on the stand's Timer; a route entry owns the engine's range interaction model).»
  source: .andromeda/architecture.md:87 «a main-button pointerdown sets the value from the hit's fraction of the content box, clamped to [0, 1], a box of no width reading 0, and a click focuses it and changes no value»
  replace-with: «Since 2026-10-13-range-input-interaction the instance verbs move a range input (the engine's range interaction model, [Scrolling and input]): a `click` sets its value from the hit's fraction of its content box and focuses it, and once focused `press` `arrow-right` / `arrow-up` steps it up, `arrow-left` / `arrow-down` down, `home` / `end` to its minimum and maximum — the diff names the change on the slider (`value`) and on its display (`name`); until then a click did not focus it and no key moved its value (measured at 2026-10-07-act-by-id on the stand's Timer).»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «`stand_act_range` (the range input's measured reading)»
  source: .andromeda/architecture.md:263 «the range input moved by the driver's `click` and keys — since 2026-10-13-range-input-interaction, its value changes named by the diff on the slider (`value`) and on its display (`name`)»
  replace-with: «`stand_act_range` (the range input moved by `click` and keys, since 2026-10-13-range-input-interaction its value changes named by the diff on the slider (`value`) and on its display (`name`), where until then it pinned a measured no-op)»
