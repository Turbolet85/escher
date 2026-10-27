# arch · standing conventions

- S1 · each integration test file covers one behaviour, opens with a //! doc naming it, and is one blitz-tests target, upstream's all.rs declared and held at test = false · .andromeda/architecture-detail/conventions.md
- S2 · scenarios run in both layout modes · .andromeda/architecture-detail/conventions.md
- S3 · cargo fmt --all --check and clippy --workspace --locked -D warnings through ci-leg.sh · .andromeda/architecture-detail/conventions.md
- S4 · rustdoc warnings are errors; modules open with //! docs and public items carry /// · .andromeda/architecture-detail/conventions.md
- S5 · engine-crate tracing gated per call site with a cfg(not(feature = "tracing")) fallback · .andromeda/architecture-detail/conventions.md
- S6 · dependencies declared once in [workspace.dependencies] and consumed as workspace = true, in-repo crates with default-features = false · .andromeda/architecture-detail/conventions.md
- S7 · every CI leg runs as bash .github/scripts/ci-leg.sh {leg}, --locked on every cargo leg · Infrastructure Patterns (CI/CD) in the body
