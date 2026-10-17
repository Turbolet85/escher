group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:39   class: omission
  leaf-text: «and the `stand-repeat` leg (not a fast leg) holds the stand checks to a zero-failure budget over three rounds.»
  source: .andromeda/architecture.md:67 «and, since 2026-10-16-fast-failing-mutants, a mutation tier run on the dev host alone, never in CI: cargo-mutants 27.1.0, the tier's tool, with `--test-tool nextest` and `--baseline=skip`»
  replace-with: «and the `stand-repeat` leg (not a fast leg) holds the stand checks to a zero-failure budget over three rounds; since 2026-10-16-fast-failing-mutants a mutation tier runs on the dev host alone, never in CI: cargo-mutants 27.1.0 with `--test-tool nextest` and `--baseline=skip`, its test tool cargo-nextest 0.9.146 under the `mutants` profile of the root `.config/nextest.toml` (fail-fast at the first failure, four test threads, one priority override naming the direct tests; the default profile untouched), selected by `NEXTEST_PROFILE=mutants`, each unit's mutant test command read green on the unmutated tree first — neither tool lockfile-resolved (the recipe in test-plan §10).»
