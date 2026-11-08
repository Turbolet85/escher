# design · standing conventions

- S1 · every answer, refusal or session error a driver command emits on the command line or its mcp role is JSON only on stdout, one line per command or newline-delimited JSON-RPC 2.0, never colour or an ESC byte, the same piped or not, with the exit status carrying the class (0 accepted, 1 refused, 2 usage, 3 session error; an mcp Err ending exits 3) · surface (Surface: cli, Component Patterns, cli-output)
  (design-system/surface fct-6fb4pz6c) cli-output
