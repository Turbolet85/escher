# design · standing conventions

- S1 · A new test that reads text size or position builds `blitz-dom` with `system-fonts`, since without it text measures 0x0 and the assertion passes vacuously · `## Typography`
  (.andromeda/design-system-detail/typography.md:35) typography
- S2 · A proof by driver `screenshot` takes `fonts-unavailable` as a refusal with nothing painted, never a shot with text skipped · `## Typography`
  (.andromeda/design-system-detail/typography.md:33) typography
- S3 · A new test that waits or animates takes time from the harness's `VirtualClock`: settle never advances it, the load wait leaves it standing, and only `advance` moves it · `## Motion (calibrated to expression level `unrecorded`)`
  (.andromeda/design-system-detail/motion.md:21) expression-motion
  (.andromeda/design-system-detail/motion.md:33) expression-motion
- S4 · A new answer or refusal line on `escher-session` or its `mcp` role holds only JSON on stdout, with no colour or ESC byte, and keeps the exit-status classes · `## Surface: cli`
  (.andromeda/design-system-detail/surface.md:122) cli-output
- S5 · A new fixture cites no project-wide colour, type, spacing or radius token, since no such set is measured · `## Color Palette`, `## Typography`, `## Spacing`, `## Border Radius`
  (.andromeda/design-system-detail/typography.md:69) typography
  (.andromeda/design-system-detail/typography.md:70) typography
  (.andromeda/design-system-detail/typography.md:72) typography
