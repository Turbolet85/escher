# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the Rust the chunk writes in the reached stand, task and check files must pass `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and the `-D warnings` rustdoc leg
conventions#2.1 · no · the chunk edits no manifest and adds no dependency (its Not-needed list rules out any manifest change), so the `[workspace.dependencies]`, `default-features` and `publish = false` rules rule over nothing it touches
conventions#3.1 · yes · the reached seven_guis files open with `//!` docs, and the public items and constants the chunk adds to them (`stand::options` among the units it edits) need `///` docs with rationale
conventions#4.1 · no · the TODO/FIXME sites, the DO NOT REORDER block and the `// ===` section markers sit in blitz-dom, blitz-paint, blitz-vibey-script, dioxus-native-dom and dioxus-native, none of them reached
conventions#5.1 · yes · the headless boot in stand.rs is where embedder services take their no-op defaults, so a no-op provider the chunk adds is named `Dummy*`, and the internals it writes (the cells grid, the select, any load-holding provider) stay `pub(crate)` or `pub(super)`
conventions#6.1 · no · the chunk adds no `tracing` call, span, log field or cargo feature and edits no engine crate, so the per-call-site gating and escher's ungated spans rule over nothing it touches
conventions#7.1 · no · the chunk writes no `unsafe` and touches no engine traversal, layout or borrow-workaround code, those sites sitting in blitz-dom and stylo_taffy, none reached
conventions#8.1 · yes · the reached stand checks keep their `//!` behaviour doc, both layout modes and fixture-first assert, the tables `tab_order` and `ENABLED_TAB_ORDER` stay once in `common/mod.rs`, and `test_cold_agent.py` keeps unittest's one TestCase per function
conventions#9.1 · no · the three escher-driver unit-test modules (`cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs`) sit in a crate whose files the chunk does not edit, and the chunk adds no unit test there
conventions#9.2 · yes · `tests/blitz-tests/tests/fixtures/mod.rs` is reached whole, so the fixture boot, hold, shared pages and `HeldNet` the chunk reuses or adds for a stand-held load belong in that one module, a page with one reader staying in its check's file
conventions#10.1 · yes · the cells grid and the flight booker's one-way / return select the chunk writes are Dioxus tasks that take `#[component]` components, `#[derive(Store)]` state and CSS in `const` raw strings
conventions#11.1 · no · the entity covers HTML fixtures under `examples/assets`, none reached, and the Not-needed list puts the example HTML fixture shapes out of scope
