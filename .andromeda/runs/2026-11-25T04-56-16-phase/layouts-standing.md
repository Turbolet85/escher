# layouts · standing conventions

- S1 · each new driver check that reads the snapshot or runs a verb is a session recording that replays as a kept check · surface-cli
  contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
  contract/escher-session-record/recording-failed · escher-session recording · recording-failed — A recording that ends answers this session error, exit 3; over MCP isError.
- S2 · each new headless check reads its screen at the pinned headless viewport, the held instance reading as a fresh boot · surface-desktop-native
  decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
  decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
- S3 · a new refusal reading or verb answer keeps the session's exit codes and refusal words closed, writing no caller input back · surface-cli
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
  contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
  contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
  contract/escher-session-mcp/end · escher-session MCP server · end — The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err.
  contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot, lint or canvas on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
  contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.
  contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1.
