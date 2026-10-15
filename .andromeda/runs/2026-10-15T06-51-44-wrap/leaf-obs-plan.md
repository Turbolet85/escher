group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:43   class: stale
  leaf-text: «on the windowed stand it is unmeasured»
  source: .andromeda/obs-plan.md:296 «the windowed `seven_guis_native` built with the workspace, on the host's display (`wayland-1`) — 28 runs by build and level, 0 needles in every run»
  replace-with: «since 2026-10-14-quality-gates a client's own stderr, the host on flows beyond those three commands and the windowed stand built with the workspace, on the host's display, are measured too — 28 runs by build and level, 0 needles in every run»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «; the windowed binary's stderr by level as that build makes it is not measured.»
  source: .andromeda/obs-plan.md:42 «Measured since 2026-10-14-quality-gates: the windowed `seven_guis_native`'s stderr by level as a workspace build makes it, on the host's display, with 0 needles»
  replace-with: «; the windowed binary's stderr by level as that build makes it is measured since 2026-10-14-quality-gates, on the host's display, with 0 needles.»

- leaf: .claude/rules/observability.md:32   class: stale
  leaf-text: «and unmeasured on the windowed stand, where no command runs»
  source: .andromeda/obs-plan.md:296 «the windowed `seven_guis_native` built with the workspace, on the host's display (`wayland-1`) — 28 runs by build and level, 0 needles in every run»
  replace-with: «and, since 2026-10-14-quality-gates, on the windowed stand, where no command runs, built with the workspace and on the host's display — 0 needles at every level»

- leaf: .claude/rules/observability.md:36   class: stale
  leaf-text: «The windowed `seven_guis_native`'s stderr by level as a workspace-wide build makes it, and typed text in its log — no command reaches the windowed binary. `cargo test --workspace` compiles blitz-dom's `tracing` call sites in and a `-p` build does not; its by-level readings are of a `-p seven_guis` build.»
  source: .andromeda/obs-plan.md:42 «Measured since 2026-10-14-quality-gates: the windowed `seven_guis_native`'s stderr by level as a workspace build makes it, on the host's display, with 0 needles»
  replace-with: «Nothing is left here of the windowed `seven_guis_native` — no command reaches it, and its stderr by level as a workspace-wide build makes it is measured since 2026-10-14-quality-gates, on the host's display, with 0 needles (`cargo test --workspace` compiles blitz-dom's `tracing` call sites in and a `-p` build does not).»
