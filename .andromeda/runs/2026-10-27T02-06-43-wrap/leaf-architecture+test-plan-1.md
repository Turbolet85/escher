group: architecture+test-plan-1
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:30   class: omission
  leaf-text: «cargo-mutants 27.1.0 and cargo-nextest 0.9.146 host tools no lockfile resolves (the recipe in test-plan §10)»
  source: .andromeda/architecture-detail/stack-and-technologies.md:86 «- **Code quality** beside the host-run mutation tier: a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile»
  replace-with: «cargo-mutants 27.1.0 and cargo-nextest 0.9.146 host tools no lockfile resolves (the recipe in test-plan §10); beside it, also on the dev host alone and never in CI, the cognitive-complexity reading (since 2026-10-26-codebase-health): `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, a host tool no lockfile resolves and no CI leg runs, reading each function, test functions counted, against the ceiling of 15 — no ceiling on every function, the engine's functions over it left as they are (test-plan §10, `gate-complexity`)»
