# arch · standing conventions

- S1 · Unit tests in a `#[cfg(test)] mod tests` at the end of the source file or a `#[path]` child file; an integration test file covers one behaviour and opens with a `//!` doc naming it; blitz-tests keeps one test target per file · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:52) conv-tests
  (.andromeda/architecture-detail/conventions.md:60) conv-tests
  (.andromeda/architecture-detail/existing-scopes.md:141) scopes
- S2 · Scenarios run in both layout modes, and a test first asserts its fixture produces the condition · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/established-decisions.md:102) dec-layout
- S3 · Stand checks share helpers through modules under `tests/` that are no test target; `tests/blitz-tests/tests/fixtures/mod.rs` states the boot, hold and shared pages once, and a page with one reader stays in its check's file · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:61) conv-tests
  (.andromeda/architecture-detail/existing-scopes.md:142) scopes
- S4 · Modules and files open with `//!`, public items carry `///`, spec URLs are cited beside implementations, and a constant's doc states its rationale · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:21) conv-documentation
- S5 · Every change passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc warnings as errors, through `.github/scripts/ci-leg.sh`; `expect_used` and `unwrap_used` allowed only beside a justification · Conventions, Formatting and lints
  (.andromeda/architecture-detail/conventions.md:8) conv-formatting-and-lints
  (.andromeda/architecture-detail/inherited-defaults.md:14) defaults
  (.andromeda/architecture-detail/stack-and-technologies.md:75) stack
- S6 · Crate internals `pub(crate)` or `pub(super)`, paint rendering split one module per concern, known gaps marked `TODO` or `FIXME` · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:27) conv-known-gaps
  (.andromeda/architecture-detail/conventions.md:33) conv-visibility-and-naming
- S7 · In engine crates tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback, and optional functionality sits behind cargo features, `accessibility` gating the snapshot modules · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:39) conv-feature-gating
  (.andromeda/architecture-detail/inherited-defaults.md:20) defaults
- S8 · A new log line carries no snapshot, snapshot text or diff, and escher-driver logs no id, name or value; engine targets print only safe fields · .andromeda/architecture-detail/cross-cutting-patterns.md
  (.andromeda/architecture-detail/cross-cutting-patterns.md:14) crosscut-logging-and-timing
  (.andromeda/architecture-detail/standard-contracts.md:156) contract-telemetry-bootstrap
- S9 · New code and tests stand under the per-crate coverage floor and the `stand-repeat` leg's zero-failure budget · Inherited Defaults, Code quality
  (.andromeda/architecture-detail/inherited-defaults.md:14) defaults
