# design · standing conventions

- S1 · each new engine or stand check is built with system fonts for its text, and a shot with text never paints with text skipped · typography
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · each new check drives the sample through the driver verbs on the one virtual clock, the clock moving only by advance · motion-calibrated-to-expression-level-unrecorded · motion
  contract/driver-verbs-and-virtual-clock/press · driver verbs and the virtual clock · press — The driver's press leaves the VirtualClock where it was.
  contract/driver-verbs-and-virtual-clock/type · driver verbs and the virtual clock · type — The driver's type leaves the VirtualClock where it was.
  decision/load-wait-moves-no-clock · Load wait moves no clock — While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
- S3 · driver and session output read by an agent, with no colour and no terminal test · surface · surface-cli
  decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
  decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
  decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
