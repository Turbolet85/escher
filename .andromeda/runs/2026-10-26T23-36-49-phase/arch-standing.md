# arch · standing conventions

- S1 · Unit tests sit in `#[cfg(test)] mod tests` at the end of the source file or in a `#[path]` child file of their own, a moved module keeping its test paths · conventions.md (conv-tests)
  (.andromeda/architecture-detail/conventions.md:60) conv-tests
  (.andromeda/architecture-detail/existing-scopes.md:141) scopes
- S2 · Each integration test file covers one behaviour and opens with a `//!` doc; scenarios run in both layout modes; stand checks share helpers through modules under `tests/` that are no test target · conventions.md (conv-tests)
  (.andromeda/architecture-detail/conventions.md:61) conv-tests
  (.andromeda/architecture-detail/design-philosophy.md:59) philosophy
  (.andromeda/architecture-detail/existing-scopes.md:142) scopes
- S3 · `cargo fmt --all --check` and clippy `--workspace --locked -D warnings` clean; `clippy::expect_used` / `clippy::unwrap_used` allowed only next to a justification · conventions.md (conv-formatting-and-lints)
- S4 · Modules and files open with `//!` docs, public items carry `///`, doc comments state the rationale for constants · conventions.md (conv-documentation)
  (.andromeda/architecture-detail/conventions.md:21) conv-documentation
- S5 · Crate internals are `pub(crate)` / `pub(super)` or private; a new helper is no public item · conventions.md (conv-visibility-and-naming)
  (.andromeda/architecture-detail/conventions.md:33) conv-visibility-and-naming
- S6 · Known gaps marked `TODO` / `FIXME`, sections delimited with `// === … ===` · conventions.md (conv-known-gaps)
  (.andromeda/architecture-detail/conventions.md:27) conv-known-gaps
- S7 · Let-chains in `if` conditions; borrow conflicts worked around by taking a list out and restoring it · conventions.md (conv-unsafe-and-borrowing)
  (.andromeda/architecture-detail/conventions.md:46) conv-unsafe-and-borrowing
