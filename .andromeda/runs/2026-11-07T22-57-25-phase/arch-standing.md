# arch · standing conventions

- S1 · Each new test sits in a `#[cfg(test)] mod tests` at the end of its source file or in a `#[path]` child file. Each integration test file covers one behaviour and opens with a `//!` doc naming it. Scenarios run in both layout modes, tests first assert that the fixture produces the condition, and stand checks share helpers through non-target modules under `tests/`. · conventions
  (architecture/conventions fct-iemqu3j3) conv-tests
  (architecture/conventions fct-cnla3gly) conv-tests
  (architecture/design-philosophy fct-dpohxdsn) philosophy
  (architecture/existing-scopes fct-vfh4fl66) scopes
  (architecture/standard-contracts fct-et27vuvt) contract-test-harness
- S2 · Each new module or file opens with a `//!` doc. Public items carry `///`, and constants state their rationale. · conventions
  (architecture/conventions fct-aqtiieoj) conv-documentation
- S3 · Each change passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings` through `.github/scripts/ci-leg.sh` legs. `expect` / `unwrap` carry a justification. · conventions
  (architecture/conventions fct-kjxsappk) conv-formatting-and-lints
  (architecture/inherited-defaults fct-77ljaxug) defaults
  (architecture/stack-and-technologies fct-5nyfapub) stack
  (architecture/stack-and-technologies fct-ovms5x7l) stack
- S4 · Tracing in the engine crates is gated per call site with a `cfg(not(feature = "tracing"))` fallback. escher's own crates take `tracing` ungated, with spans built by hand and no `#[instrument]`. · conventions
  (architecture/inherited-defaults fct-fgkqqy76) defaults
- S5 · Error types are enums with `Display` and a `From` per wrapped error. Mutex locks recover from poisoning with `into_inner`. · conventions
  (architecture/occupied-resources fct-knrtlksy) res-process-wide-state-and-threads
- S6 · The per-crate coverage floor and the `stand-repeat` leg's zero-failure budget hold for each new crate code and stand check. · cross-cutting-patterns
  (architecture/occupied-resources fct-io2lzr2y) res-ci-infrastructure
