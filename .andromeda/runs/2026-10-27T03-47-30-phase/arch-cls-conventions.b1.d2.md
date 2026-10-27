# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · the chunk writes and changes no Rust file (its two Rust reads stay unchanged), so the fmt, clippy and rustdoc legs see no change
conventions#2.1 · no · the chunk changes no manifest or lockfile and no Cargo version moves (Not needed: dependency pinning, default features, release versioning)
conventions#3.1 · no · the chunk adds no Rust module, file or public item, so no `//!` or `///` doc comment is written; the README is prose
conventions#4.1 · no · the chunk adds no TODO or FIXME marker and touches no listed code line; the residual lines it absorbs are records, not code markers
conventions#5.1 · no · the chunk adds no Rust item and changes no visibility; the README's capitalised name is prose, which this Rust naming rule does not cover
conventions#6.1 · no · the chunk adds no tracing call, span, field or cargo feature (Not needed: the telemetry bootstrap and logging pattern) and changes no manifest
conventions#7.1 · no · the chunk writes no unsafe block and no borrow-conflict code, so there is no Rust code for this rule to govern
conventions#8.1 · no · the chunk writes no test, test file or fixture; its re-read test files change nothing, and the CI Python unittest shape governs a pin it does not edit
conventions#9.1 · no · the chunk moves no unit-test module out of a source file and writes no test (Not needed: unit-test placement)
conventions#9.2 · no · the chunk writes no test or fixture, so the shared `fixtures/mod.rs` stays untouched (Not needed: tests and fixtures)
conventions#9.3 · no · the chunk edits neither the escher-driver MCP listing test nor the 0.2.0 requirement sweep, so their helper and row-walk rule governs no file it changes
conventions#10.1 · no · the chunk adds no app, example or binary and touches no example source (Not needed: apps and examples)
conventions#11.1 · no · the chunk writes no HTML fixture, so the reduced-page, numbered-case and sibling-asset rules have nothing to govern (Not needed: HTML fixtures)
