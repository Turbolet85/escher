# arch · standing conventions

- S1 · Each new integration test file covers one behaviour, opens with a `//!` doc naming it, and shares helpers only through modules under `tests/` that are no test target · .andromeda/architecture-detail/conventions.md
- S2 · A new test's scenarios run in both layout modes and first assert that the fixture produces the condition · .andromeda/architecture-detail/conventions.md
- S3 · Shared fixture pages, boot and hold live once in `tests/blitz-tests/tests/fixtures/mod.rs`, and a page with one reader stays in its check's file · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:61) conv-tests
  (.andromeda/architecture-detail/existing-scopes.md:144) scopes
- S4 · New code passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings`, `clippy::expect_used` and `clippy::unwrap_used` allowed only next to a justification · .andromeda/architecture-detail/conventions.md
- S5 · New engine code opens with `//!` docs, gives public items `///` and cites the spec URL next to a web-semantics implementation · §Conventions and §Design Philosophy of .andromeda/architecture.md
  (.andromeda/architecture-detail/conventions.md:21) conv-documentation
- S6 · A new log line in the engine crates is gated per call site with a `cfg(not(feature = "tracing"))` fallback · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/inherited-defaults.md:20) defaults
- S7 · A new or changed CI leg runs as `bash .github/scripts/ci-leg.sh {leg}` with `--locked`, the same command CI and the dev host run, its `uses:` pinned to a 40-hex SHA, no job-level permission, and its log uploaded on failure · §Infrastructure Patterns of .andromeda/architecture.md and the CI contracts group of .andromeda/architecture-detail/standard-contracts.md
  (.andromeda/architecture-detail/occupied-resources.md:36) res-filesystem
- S8 · A dependency line a resolution writes is declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, in-repo crates with `default-features = false` · .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/conventions.md:15) conv-manifests
