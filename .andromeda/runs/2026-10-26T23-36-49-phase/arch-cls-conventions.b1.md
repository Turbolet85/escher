# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the scope's own Gates obligation (fast: fmt and clippy `--locked -D warnings`; doc: rustdoc `-D warnings`) is what this rules over, so the refactored code and tests must pass them
conventions#2.1 · no · the chunk adds no crate, dependency or manifest entry, and no manifest is among its reached files
conventions#3.1 · no · standing S4 · modules and files open with `//!` docs, public items carry `///`, constants state their rationale
conventions#4.1 · no · standing S6 · known gaps marked `TODO` or `FIXME`, sections delimited with `// === … ===`
conventions#5.1 · no · standing S5 · crate internals are `pub(crate)`, `pub(super)` or private, and a new helper is no public item
conventions#6.1 · no · the chunk adds no feature, tracing call site or span, and execute.rs, which builds the command span, is not reached
conventions#7.1 · no · standing S7 · let-chains in `if` conditions, and borrow conflicts worked around by taking a list out and restoring it
conventions#8.1 · yes · the blitz-tests target pin the scope re-reads holds stand_requirement_sweep's one target and name, mcp_tests.rs keeps the listing test's path, and stand_lint runs in both layout modes
conventions#9.1 · no · standing S1 · a unit-test file of its own keeps its test paths when a module moves, and the chunk moves no module
conventions#9.2 · no · standing S2 · shared test helpers live in a module under tests/ that is no test target, the fixtures module among them
conventions#10.1 · no · the chunk edits no app or example crate, and examples/seven_guis is named only as the witness the sweep reads
conventions#11.1 · no · the chunk adds and edits no HTML fixture, and examples/assets is not among its reached files
