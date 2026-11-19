# layouts · standing conventions

- S1 · every stand test, moved or new, reaches controls by their stable keys, boots its stand fresh at the pinned headless viewport, and runs in both layout modes · surface-desktop-native
- S2 · every dev-host script the plan adds or changes (a sweep among them) owns a usage line, exits 2 on bad usage, states its stdout, and has a Windows pass-through script · surface-cli
  contract/cold-agent-cli/exit-2 · cold-agent.sh run pipe · exit 2 — A usage error prints to stderr with empty stdout and exits 2, checked before any precondition.
  contract/cold-agent-cli/stdout · cold-agent.sh run pipe · stdout — Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
  contract/cold-agent-cli/windows-pass-through · cold-agent.sh run pipe · cold-agent.ps1 — The Windows pass-through to cold-agent.sh.
