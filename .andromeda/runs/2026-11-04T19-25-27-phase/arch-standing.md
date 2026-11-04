# arch · standing conventions

- S1 · each new integration test file covers one behavior, opens with a `//!` doc naming it, runs its scenario in both layout modes, first asserts the fixture produces the condition, and is one test target per file in blitz-tests · conventions
  (architecture/established-decisions fct-3oxpzjue) dec-layout
- S2 · stand checks share helpers through modules under `tests/` that are no test target; `tests/blitz-tests/tests/fixtures/mod.rs` states the fixture boot and hold once, and a page with one reader stays in its check's file · conventions
  (architecture/conventions fct-cnla3gly) conv-tests
  (architecture/occupied-resources fct-jzxi3spj) res-process-wide-state-and-threads
- S3 · new code passes `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, with `clippy::expect_used` / `clippy::unwrap_used` only next to a justification · conventions
  (architecture/conventions fct-kjxsappk) conv-formatting-and-lints
  (architecture/inherited-defaults fct-77ljaxug) defaults
- S4 · new modules and files open with `//!` docs, public items carry `///`, and an example opens with a `//!` comment describing what it demonstrates · conventions
  (architecture/conventions fct-aqtiieoj) conv-documentation
- S5 · a Dioxus example declares `#[component]` components with signal and store hooks, reactive state through `#[derive(Store)]`, its CSS in `const` raw strings, and fails with `.unwrap()`, `.expect(..)` or `panic!` · conventions
  (architecture/conventions fct-2wv4ux4q) conv-apps-and-examples
- S6 · `unsafe` blocks carry `// SAFETY:` comments, and borrow conflicts are worked around by taking a child list or layout out and restoring it · conventions
  (architecture/conventions fct-knw5nmxh) conv-unsafe-and-borrowing
