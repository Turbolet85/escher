# layouts · standing conventions

- S1 · each new contract test of the cold-agent pipe keeps the pipe building nothing and running no live agent, and asserts its stdout, exit codes and Windows pass-through as the pipe's contracts state · surface-cli
  contract/cold-agent-cli/exit-2 · cold-agent.sh run pipe · exit 2 — A usage error prints to stderr with empty stdout and exits 2, checked before any precondition.
  contract/cold-agent-cli/exit-3 · cold-agent.sh run pipe · exit 3 — run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run.
  contract/cold-agent-cli/stdout · cold-agent.sh run pipe · stdout — Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
  contract/cold-agent-cli/windows-pass-through · cold-agent.sh run pipe · cold-agent.ps1 — The Windows pass-through to cold-agent.sh.
- S2 · each new test that reads a stand document boots it under the headless stand's pinned viewport and font and reads ids, names, roles and states by attributes alone · surface-desktop-native
