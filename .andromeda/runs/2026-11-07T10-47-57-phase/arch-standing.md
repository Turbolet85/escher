# arch · standing conventions

- S1 · Unit tests sit in a #[cfg(test)] mod tests at the end of the source file, or in a #[path] child file of their own. · conventions
  (architecture/conventions fct-iemqu3j3) conv-tests
  (architecture/existing-scopes fct-vfh4fl66) scopes
- S2 · Each integration test file covers one behavior and opens with a //! doc naming it. Scenarios run in both layout modes. Tests first assert that the fixture produces the condition. blitz-tests keeps one test target per file. · conventions
  (architecture/established-decisions fct-3oxpzjue) dec-layout
  (architecture/existing-scopes fct-5emkjn4o) scopes
  (architecture/existing-scopes fct-6d5l2imz) scopes
- S3 · Modules and files open with //! docs, public items carry ///, and doc comments state the rationale for constants. This applies to the new mutation_buffer.rs. · conventions
  (architecture/conventions fct-aqtiieoj) conv-documentation
- S4 · Formatting and lints: cargo fmt --all --check, cargo clippy --workspace --locked -- -D warnings, and rustdoc warnings as errors, each run as a .github/scripts/ci-leg.sh leg. clippy::expect_used and clippy::unwrap_used each sit next to a justification. · conventions
  (architecture/conventions fct-kjxsappk) conv-formatting-and-lints
  (architecture/inherited-defaults fct-77ljaxug) defaults
  (architecture/stack-and-technologies fct-5nyfapub) stack
- S5 · Crate internals are pub(crate) or pub(super). The new buffer type stays crate-private unless the bridge's public API needs it. · conventions
  (architecture/conventions fct-h6rtvaqy) conv-visibility-and-naming
- S6 · unsafe blocks carry allow unsafe_code or a // SAFETY: comment. Borrow conflicts are worked around by taking a value out and restoring it. · conventions
  (architecture/conventions fct-knw5nmxh) conv-unsafe-and-borrowing
