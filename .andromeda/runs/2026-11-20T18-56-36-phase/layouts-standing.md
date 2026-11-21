# layouts · standing conventions

- S1 · every stand check reaches the stand through the session roles' shared stand boot, a held instance reading as a fresh boot, at the pinned headless viewport · surface-desktop-native
  decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
  decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
  decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
- S2 · every new or widened verb argument appears alike in the escher-session help answer, the command form and the MCP tools listing · surface-cli
