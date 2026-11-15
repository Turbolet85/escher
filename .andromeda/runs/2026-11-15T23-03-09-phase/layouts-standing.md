# layouts · standing conventions

- S1 · each new or changed harness command owes its contract rows — usage, stdout, exit codes and log file — and a contract test in the CI scripts' unittest leg that pins them without live builds · surface-cli
- S2 · each POSIX harness script under scripts/ carries a Windows pass-through that forwards its arguments and exits unchanged · surface-cli
  contract/cold-agent-cli/windows-pass-through · cold-agent.sh run pipe · cold-agent.ps1 — The Windows pass-through to cold-agent.sh.
