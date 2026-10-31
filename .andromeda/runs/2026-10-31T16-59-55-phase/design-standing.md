# design · standing conventions

- S1 · A new test that measures or asserts on text runs in the blitz-tests crate with `blitz-dom`'s `system-fonts` feature, or it passes vacuously on text of 0x0 · Typography section of the body
  (.andromeda/design-system-detail/typography.md:35) typography
- S2 · A new pixel check through the driver's `screenshot` owes a font context that resolves `sans-serif`, or it is refused `fonts-unavailable` with nothing painted · Typography section of the body
  (.andromeda/design-system-detail/typography.md:33) typography
- S3 · A new test's time runs through the harness's `VirtualClock`: settle never advances it, `advance` moves it by its `ms`, and `click`, `type` and `press` leave it · Motion section of the body
  (.andromeda/design-system-detail/motion.md:28) expression-motion
- S4 · New driver output on `escher-session` or its `mcp` role stays JSON on stdout, one line per command or JSON-RPC line, with no colour or ESC byte and no terminal test · Surface: cli, Component Patterns, in the body
  (.andromeda/design-system-detail/surface.md:126) cli-output
