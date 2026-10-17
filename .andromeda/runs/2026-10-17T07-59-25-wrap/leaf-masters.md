group: masters
read: CLAUDE.md · 148 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:98   class: omission
  leaf-text: «log in `target/ci-logs/{leg}.log`»
  source: .andromeda/test-plan.md:345 «a mutation run is cargo-mutants with `--test-tool nextest` under the `mutants` profile of the root `.config/nextest.toml`»
  replace-with: «log in `target/ci-logs/{leg}.log`; the mutation tier is no leg — run on the dev host alone, never in CI: cargo-mutants `--test-tool nextest --baseline=skip` under `NEXTEST_PROFILE=mutants` (the root `.config/nextest.toml`'s `mutants` profile), each unit's mutant test command first read green on the unmutated tree (`.andromeda/test-plan.md` §10)»
