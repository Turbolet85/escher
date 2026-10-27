group: architecture-2
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:39   class: omission
  leaf-text: «neither tool lockfile-resolved (the recipe in test-plan §10).»
  source: .andromeda/architecture-detail/stack-and-technologies.md:86 «- **Code quality** beside the host-run mutation tier: a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile»
  replace-with: «neither tool lockfile-resolved (the recipe in test-plan §10); beside it, since 2026-10-26-codebase-health, a cognitive-complexity reading on the dev host alone, never in CI: `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15.»

- leaf: .claude/docs/stack.md:23   class: omission
  leaf-text: «tokio 1.42, reqwest 0.13»
  source: .andromeda/architecture-detail/stack-and-technologies.md:39 «tokio "1.42" (resolved 1.54.1, past RUSTSEC-2026-0386), reqwest "0.13"»
  replace-with: «tokio 1.42 (resolved 1.54.1, past RUSTSEC-2026-0386), reqwest 0.13»

- leaf: .claude/docs/workflow.md:8   class: omission
  leaf-text: «- **Main branch:** `main` (upstream Blitz CI runs on `main` and `v0.*`; escher's `ci.yml` also runs on every `build/**` push).»
  source: .andromeda/architecture-detail/project-intent.md:27 «The front page GitHub renders is the root `README.md` of the repository's default branch, `build/escher-0.3.0`»
  replace-with: «- **Main branch:** `main` (upstream Blitz CI runs on `main` and `v0.*`; escher's `ci.yml` also runs on every `build/**` push); the fork's GitHub default branch is the version's build branch, `build/escher-0.3.0`, whose root `README.md` is the front page GitHub renders.»
