# design · standing conventions

- S1 · A blitz-tests target, upstream's or ours, builds `blitz-dom` with `system-fonts`, so font-dependent assertions do not pass on text measuring 0x0 · Typography, Loading, in the body
  (.andromeda/design-system-detail/typography.md:36) typography
- S2 · A test that takes a driver screenshot needs a font context resolving `sans-serif`, since the shot is refused otherwise · Typography, Loading, in the body
  (.andromeda/design-system-detail/typography.md:34) typography
- S3 · A test reading animation or settle takes time from the harness's `VirtualClock` · Motion, Animation runtime, in the body
- S4 · A new print or log line keeps the agent-facing command output JSON only, with no colour or ESC byte and no terminal test · Surface: cli, Component Patterns, in the body
  (.andromeda/design-system-detail/surface.md:136) cli-output
