# design · standing conventions

- S1 · every escher-session command's output is one line on stdout read by an agent, with no colour and no terminal test · surface · surface-cli
  decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
  decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
  decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
- S2 · every exit status and exit code a command ends with is read from the schema's table · surface · surface-cli
  decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
