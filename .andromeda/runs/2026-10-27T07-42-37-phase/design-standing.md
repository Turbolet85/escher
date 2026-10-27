# design · standing conventions

- S1 · every blitz-tests target that reads text builds blitz-dom with the system-fonts feature, or text measures 0x0 and its assertions pass vacuously · Typography (body, Loading)
  (.andromeda/design-system-detail/typography.md:37) typography
- S2 · a test that reads animation or scroll time drives it from the harness's VirtualClock, and a settle answers the animating set the same way, never waiting on or advancing it · Motion (body, Animation runtime)
