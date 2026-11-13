# design · standing conventions

- S1 · each new headless frame test runs in both layout modes and reads its frame through the screenshot verb without skipping text · surface
- S2 · each new test is built with system fonts, not bundled fonts · typography
- S3 · each new test that drives time moves only the one virtual clock and never waits on animation · motion · motion-calibrated-to-expression-level-unrecorded
