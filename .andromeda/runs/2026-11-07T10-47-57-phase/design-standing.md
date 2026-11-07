# design · standing conventions

- S1 · a test in the blitz-tests crate builds blitz-dom with the system-fonts feature, so text measures and font-dependent geometry assertions do not pass vacuously · Typography section of the body (Loading)
  (design-system/typography fct-zmi56wtl) typography
- S2 · a harness-driven test reads animation time only through the VirtualClock its documents read; settle never advances it, only advance moves it · Motion section of the body (Animation runtime)
  (design-system/motion fct-e2e2e6vg) expression-motion
