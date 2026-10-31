# arch · standing conventions

- S1 · Each new integration test file covers one behavior, opens with a `//!` doc naming it, is one test target, and first asserts its fixture produces the condition · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S2 · Each new scenario runs in both layout modes · Conventions (Tests); .andromeda/architecture-detail/conventions.md
- S3 · A shared fixture page, boot or hold lives once in `tests/blitz-tests/tests/fixtures/mod.rs`, stand helpers in modules that are no test target, and a page with one reader stays in its check's file · Conventions (Tests); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:61) conv-tests
- S4 · Unit tests sit in `#[cfg(test)] mod tests` at the end of the source file or in a `#[path]` child file of their own · Conventions (Tests); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:60) conv-tests
  (.andromeda/architecture-detail/existing-scopes.md:141) scopes
- S5 · A new HTML fixture follows the fixture layout: numbered cases under headings with a CSS comment per case and `./`-relative sibling assets · Conventions (HTML fixtures); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:75) conv-html-fixtures
- S6 · New code carries `//!` and `///` docs and cites the spec URL above or next to the implementation · Conventions (Documentation); Design Philosophy
  (.andromeda/architecture-detail/conventions.md:21) conv-documentation
- S7 · New code passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings` through `.github/scripts/ci-leg.sh`, with `clippy::expect_used` and `clippy::unwrap_used` next to a justification · Conventions (Formatting and lints); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:8) conv-formatting-and-lints
  (.andromeda/architecture-detail/inherited-defaults.md:14) defaults
  (.andromeda/architecture-detail/stack-and-technologies.md:75) stack
- S8 · A new check holds the `stand-repeat` leg's zero-failure budget and the per-crate coverage floors · Inherited Defaults; .andromeda/architecture-detail/inherited-defaults.md
- S9 · New engine code reads time only through `DocumentConfig.clock`, with no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs` · Standard Contracts (Document core); .andromeda/architecture-detail/standard-contracts.md
  (.andromeda/architecture-detail/established-decisions.md:76) dec-scrolling-and-input
  (.andromeda/architecture-detail/standard-contracts.md:16) contract-document-core
- S10 · New engine code keeps persistent interaction state on DOM nodes only, takes `NodeId` in every public API, resolves ids through `get_node`, and mutates through `doc.mutate()` · Design Philosophy; Standard Contracts (Document core, Mutation)
- S11 · A new engine log line gates tracing per call site with a `cfg(not(feature = "tracing"))` fallback · Conventions (Feature gating); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:39) conv-feature-gating
  (.andromeda/architecture-detail/inherited-defaults.md:20) defaults
- S12 · New crate internals stay `pub(crate)` or `pub(super)`, and a known gap left in code is marked `TODO` or `FIXME` · Conventions (Visibility and naming, Known gaps); .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:27) conv-known-gaps
  (.andromeda/architecture-detail/conventions.md:33) conv-visibility-and-naming
