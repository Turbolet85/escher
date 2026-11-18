# design · standing conventions

- S1 · each new stand check boots the app headlessly through the stand and runs in both layout modes, built against the test engine with system fonts · typography
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · each new served task the escher-session host serves is listed and reached through both the command line and the mcp role, its answers in the agent-read forms · surface
