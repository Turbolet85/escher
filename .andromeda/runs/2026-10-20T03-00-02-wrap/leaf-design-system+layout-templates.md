group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «so after a driver `scroll` a settle reads `animating` true until 700 ms of that clock pass.»
  source: .andromeda/design-system.md:292 «The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still»
  replace-with: «so after a driver `scroll` a settle reads `animating` true until 700 ms of that clock pass; since 2026-10-20-load-wait the driver's load wait moves no clock — while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the `VirtualClock` stands still, so a timer or an animation does not run during the wait.»
