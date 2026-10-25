group: architecture-2
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:43   class: stale
  leaf-text: «read at 2.1.288 and, at 2026-10-15-cold-agent-test, at 2.1.296 on the dev host»
  source: .andromeda/architecture-detail/stack-and-technologies.md:73 «read at 2.1.308 on the dev host, the live run's `client_version` the same (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md)»
  replace-with: «read at 2.1.308 on the dev host, the live run's `client_version` the same (at 2026-10-25-cold-agent-bar)»

- leaf: .claude/docs/stack.md:23   class: omission
  leaf-text: «tokio 1.42, reqwest 0.13»
  source: .andromeda/architecture-detail/stack-and-technologies.md:39 «tokio "1.42" (resolved 1.54.1, past RUSTSEC-2026-0386), reqwest "0.13"»
  replace-with: «tokio 1.42 (resolved 1.54.1, past RUSTSEC-2026-0386), reqwest 0.13»
