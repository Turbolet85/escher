# design · standing conventions

- S1 · every new check added to cover an arm before it moves builds the engine with system fonts, the same as every other test · typography
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · every new check that drives clicks, presses or typing goes through the driver verbs on the virtual clock, never through wall-clock waits · motion-calibrated-to-expression-level-unrecorded · motion
