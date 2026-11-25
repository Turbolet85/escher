# layouts · standing conventions

- S1 · every new stand check runs on the headless stand pinned viewport against the stand's keyed controls and boots clean of lint · surface-desktop-native
- S2 · every new process check of the binary reads escher-session answers through its command form, exit codes and MCP tools as contracted, echoing no caller input · surface-cli
  contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the sixteen verbs of the driver's verb table, --session exactly once on every command but help.
  decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
