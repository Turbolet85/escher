# design · standing conventions

- S1 · every new command line output of the session tooling is read by an agent: plain text or JSON lines, no colour and no terminal test. · surface
  decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
- S2 · every new exit status reported over the session binary follows the four-value exit grammar 0, 1, 2, 3 read from the schema's table. · surface-cli
