group: design-system
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «and the driver's `scroll` is instant in every box and in the viewport (ratified by the founder, 2026-10-07).»
  source: .andromeda/design-system.md:291 «the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (PROVISIONAL)»
  replace-with: «and the driver's `scroll` is instant in every box and in the viewport (ratified by the founder, 2026-10-07); since 2026-10-18-one-virtual-clock the harness's documents read one `VirtualClock` — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — and the driver's `advance` moves it by exactly its `ms` together with the app's own time (PROVISIONAL), while `click`, `type` and `press` leave it where it was, so after a driver `scroll` a settle reads `animating` true until 700 ms of that clock pass.»
