# design · standing conventions

- S1 · tests and the test engine are built with system fonts, and a screenshot never paints with text skipped · typography
- S2 · driver output stays agent-read, with no colour and no terminal test, exit statuses read from the schema's table · surface
  contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
  contract/escher-session-commands/layout-answer · escher-session commands · layout answer — layout --id answers one uncoloured line of JSON {id, bounds, laid_out, placed, shifts, width, height} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; bounds and laid_out read x, y, width, height, an optional key left out when absent.
  contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused.
  contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error.
  contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error; an mcp role ending in Err also exits 3.
  contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted.
  contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused.
  contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage.
  contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
  contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
  contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
  contract/escher-session-stdout/mcp-line · escher-session stdout · mcp role output — Newline-delimited JSON-RPC 2.0 lines.
  decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
  decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
  decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
  decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
