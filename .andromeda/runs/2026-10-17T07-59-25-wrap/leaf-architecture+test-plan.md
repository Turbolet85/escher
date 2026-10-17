group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:30   class: omission
  leaf-text: «ignored benchmarks (`PAINT_TREE_BENCH_HTML=<file>` for an external page)»
  source: .andromeda/architecture.md:67 «a mutation tier run on the dev host alone, never in CI: cargo-mutants 27.1.0, the tier's tool, with `--test-tool nextest` and `--baseline=skip`»
  replace-with: «ignored benchmarks (`PAINT_TREE_BENCH_HTML=<file>` for an external page); the mutation tier (since 2026-10-16-fast-failing-mutants, on the dev host alone, never in CI): cargo-mutants 27.1.0 with `--test-tool nextest` and `--baseline=skip` under `NEXTEST_PROFILE=mutants`, the `mutants` profile of the root `.config/nextest.toml` (fail-fast at the first failure, four test threads; the default profile untouched), each unit's mutant test command first read green on the unmutated tree with `NEXTEST_PROFILE=mutants cargo nextest run --locked` (a `-p` per package on the unit's list line, then `--lib` and its `--test=` targets) — cargo-mutants 27.1.0 and cargo-nextest 0.9.146 host tools no lockfile resolves (the recipe in test-plan §10)»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «the stand's time step (milliseconds onto Timer ticks)»
  source: .andromeda/test-plan.md:10 «`cargo test -p seven_guis --locked --lib` reads 2 passed, as measured at escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/report.md»
  replace-with: «the stand's time step (milliseconds onto Timer ticks; since 2026-10-16-fast-failing-mutants the library's second unit test, `tasks::timer::tests`, the Timer's tick stream, runs under `cargo test -p seven_guis --locked --lib`, 2 passed)»
