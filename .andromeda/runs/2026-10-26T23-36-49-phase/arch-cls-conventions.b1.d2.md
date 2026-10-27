# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the refactored code and tests must pass `cargo fmt --all --check`, `cargo clippy --workspace --locked -D warnings` and the rustdoc `-D warnings` of the `doc` leg, the gates the Needs name
conventions#2.1 · no · the chunk adds no dependency, package, feature or target and reaches no Cargo.toml
conventions#3.1 · no · standing S4 · modules and files open with `//!` docs, public items carry `///`, constants carry rationale doc comments
conventions#4.1 · no · standing S6 · known gaps marked `TODO` or `FIXME`, sections delimited with `// === … ===`
conventions#5.1 · no · standing S5 · crate internals are `pub(crate)`, `pub(super)` or private, a new helper is no public item
conventions#6.1 · yes · escher-driver builds its spans by hand with `tracing::info_span!`, ungated and with no `#[instrument]`, and the Needs name the `lint` command span and bar any added span, so a new helper takes neither `#[instrument]` nor a feature gate
conventions#7.1 · no · standing S7 · let-chains in `if` conditions, borrow conflicts worked around by taking a list out and restoring it
conventions#8.1 · yes · the Needs name three of its instances: `stand_requirement_sweep` stays one blitz-tests target of its own file under its pinned name and target, `stand_lint` is re-read in both layout modes, and `mcp_tests.rs` stays the `#[path]` child whose listing test keeps its path and name
conventions#9.1 · no · standing S1 · unit tests in `#[path]` child files, each moved whole with its test paths unchanged
conventions#9.2 · no · standing S2 · stand checks share helpers through modules under `tests/` that are no test target, as the fixtures module does
conventions#10.1 · no · the chunk reaches no app or example source, its Reached files being escher-driver, blitz-tests and CI files
conventions#11.1 · no · the chunk reaches no HTML page, its Reached files being Rust sources, blitz-tests files and CI scripts
