group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «73 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 17 in two `#[path]` test files (72 until 2026-10-14-quality-gates,»
  source: .andromeda/test-plan.md:21 «- **escher-driver:** 74 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 18 in two `#[path]` test files (73 until 2026-10-16-fast-failing-mutants, 72 until»
  replace-with: «74 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 18 in two `#[path]` test files (73 until 2026-10-16-fast-failing-mutants, 72 until 2026-10-14-quality-gates,»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_tests.rs` 11 (`mcp::tests`, in `mcp.rs` until 2026-10-16-file-splits; 10 until 2026-10-12-headless-screenshot; each line read by its rules — the byte bound, JSON, the envelope; a line read whole across reads;»
  source: .andromeda/test-plan.md:21 «mcp_tests.rs 12 (`mcp::tests`, in mcp.rs until 2026-10-16-file-splits; 11 until 2026-10-16-fast-failing-mutants, 10 until 2026-10-12-headless-screenshot;»
  replace-with: «`mcp_tests.rs` 12 (`mcp::tests`, in `mcp.rs` until 2026-10-16-file-splits; 11 until 2026-10-16-fast-failing-mutants, 10 until 2026-10-12-headless-screenshot; each line read by its rules — the byte bound, JSON, the envelope; a line read whole across reads and an unended last line read as one; since 2026-10-16-fast-failing-mutants `framing::read_request` taking each line with its terminator over a slice and over a one-byte `BufReader`, each reader making at most its line count plus one reads, the count asserted;»

- leaf: .claude/docs/tests-summary.md:44   class: omission
  leaf-text: «| Performance budget | NOT YET MEASURED | — |»
  source: .andromeda/test-plan.md:345 «- **Mutation tier:** since 2026-10-16-fast-failing-mutants a mutation run is cargo-mutants with `--test-tool nextest` under the `mutants` profile of the root `.config/nextest.toml`»
  replace-with: «| Mutation tier · Performance budget | mutation: no threshold — since 2026-10-16-fast-failing-mutants a mutation run is cargo-mutants `--test-tool nextest --baseline=skip` under the root `.config/nextest.toml`'s `mutants` profile (selected by `NEXTEST_PROFILE=mutants`: `fail-fast = { max-fail = 1, terminate = "immediate" }`, `test-threads = 4`, the direct tests at `priority = 100`, the default profile untouched), each unit's test command (`NEXTEST_PROFILE=mutants cargo nextest run --locked`, a `-p` per package, then `--lib` and its `--test=` targets) first read green on the unmutated tree — plain `cargo test` still reads the 0.1.0 audit's six timeout-only mutants `timeout`; dev host only, never in CI; scoped runs over `timer.rs`, `mcp/framing.rs` and `mutation_writer.rs` read 137 mutants — 126 caught, 11 unviable, 0 missed, 0 timeout; performance: NOT YET MEASURED | cargo-mutants 27.1.0 + cargo-nextest 0.9.146 (host tools no lockfile resolves) · — |»

- leaf: .claude/rules/testing.md:47   class: omission
  leaf-text: «and the stand checks hold a zero-failure flakiness budget (`ci-leg.sh stand-repeat`, three rounds, no retry).»
  source: .andromeda/test-plan.md:345 «- **Mutation tier:** since 2026-10-16-fast-failing-mutants a mutation run is cargo-mutants with `--test-tool nextest` under the `mutants` profile of the root `.config/nextest.toml`»
  replace-with: «and the stand checks hold a zero-failure flakiness budget (`ci-leg.sh stand-repeat`, three rounds, no retry). Since 2026-10-16-fast-failing-mutants a mutation run (dev host only, never in CI — cargo-mutants 27.1.0 and cargo-nextest 0.9.146 are host tools) is cargo-mutants `--test-tool nextest --baseline=skip` under the root `.config/nextest.toml`'s `mutants` profile (`NEXTEST_PROFILE=mutants`: fail-fast at one failure, terminated immediately, the default profile untouched), each unit's test command (`NEXTEST_PROFILE=mutants cargo nextest run --locked`, a `-p` per package, then `--lib` and its `--test=` targets) first read green on the unmutated tree — never under plain `cargo test`, which runs the hung blitz-tests integration binaries before the library tests and reads their mutants `timeout`.»
