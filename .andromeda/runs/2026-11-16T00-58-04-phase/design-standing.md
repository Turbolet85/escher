# design · standing conventions

- S1 · every new escher-session verb has its command, arguments and answer listed in help and the self-description, its exit statuses taken from the schema table · surface
- S2 · every new MCP tool result follows the MCP role's result and error-ending form, one JSON result on stdout · surface
- S3 · escher-session output carries no colour and gets no terminal test; it is read by agents · surface · surface-cli
  decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
  decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
- S4 · every new test and fixture is built with system fonts so text lays out the same on every runner · typography
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
