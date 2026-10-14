group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:35   class: omission
  leaf-text: «`covered`'s meaning, the `contain: paint` reading»
  source: .andromeda/architecture.md:133 «a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows" — its second limit added at 2026-10-14-stand-requirement-sweep»
  replace-with: «`covered`'s meaning, two readings — the `contain: paint` one and, since 2026-10-14-stand-requirement-sweep, a box clipped on one axis stopping a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows»

- leaf: .claude/docs/services/escher-driver.md:53   class: omission
  leaf-text: «— `covered`'s meaning says so.»
  source: .andromeda/architecture.md:133 «two readings are still off — content clipped by `contain: paint` beside a `visible` `overflow`, which a hit still reaches, and, since 2026-10-14-stand-requirement-sweep, a target lying beside a box clipped on one axis»
  replace-with: «— `covered`'s meaning says so. Measured limit, unfixed, since 2026-10-14-stand-requirement-sweep: a box clipped on one axis stops a hit at its whole padding box while `visible_region` narrows it on its clipped axis only, so a target lying beside it along the axis it does not clip reads in view and is refused `covered` with the instance unchanged, in both layout modes (`stand_act_obstructed`) — `covered`'s meaning says that too.»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «the first twelve 41 tests and two ignored children»
  source: .andromeda/architecture.md:263 «`stand_act_states`, 42 tests (41 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s sixth) and two ignored children»
  replace-with: «the first twelve 42 tests (41 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s sixth) and two ignored children»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «a button where scrolled-out rows extend clicked as itself)»
  source: .andromeda/architecture.md:263 «a covered and an out-of-view target refused on in-file fixtures — since 2026-10-14-stand-requirement-sweep also a target beside a box clipped on one axis, refused `covered` with the instance unchanged»
  replace-with: «a button where scrolled-out rows extend clicked as itself; since 2026-10-14-stand-requirement-sweep a target beside a box clipped on one axis refused `covered` with the instance unchanged)»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «12 shots in both layout modes).»
  source: .andromeda/architecture.md:263 «`stand_requirement_sweep`, 2 tests in both layout modes: every requirement held through `Session::run` alone on each lean task (the screen inside `SNAPSHOT_TEXT_BUDGET`, one distinct id per node line»
  replace-with: «12 shots in both layout modes), and since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` (2 tests in both layout modes: every requirement held through `Session::run` alone on each lean task — the screen inside `SNAPSHOT_TEXT_BUDGET`, one distinct id per node line, each an accessibility node's `author_id`, one id-addressed acting call settled, an empty diff for a click on `task-title`, `press tab` focusing the first Tab stop, two byte-equal 800 × 600 shots, an unread id refused `not-found` — and `WITNESSES`, 16 rows naming each requirement's witness tests).»
