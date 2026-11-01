# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · standing S4 · new code passes cargo fmt --all --check, cargo clippy --workspace --locked -- -D warnings and rustdoc -D warnings, with clippy::expect_used and clippy::unwrap_used allowed only next to a justification
conventions#2.1 · no · standing S8 · a dependency line a resolution writes is declared once in [workspace.dependencies] and consumed as { workspace = true }, in-repo crates with default-features = false
conventions#3.1 · no · standing S5 · new engine code opens with //! docs, gives public items /// and cites the spec URL next to a web-semantics implementation
conventions#4.1 · yes · the TODO/FIXME known-gap markers at construct.rs:417 and 602, scrolling.rs:212 and 570, and stylo.rs:1232 sit in predicted conflict surfaces, so a resolution could drop or move them
conventions#5.1 · yes · the __blitz_ natives in bootstrap.rs:93-121 and the pub(crate) item at scrolling.rs:33 sit in conflict surfaces, and the document_may_load re-export is the script-engine gate the scope keeps
conventions#6.1 · yes · the absent menu, muda and hot-reload features in the root Cargo.toml, a conflict surface the take edits, bind as the scope's no-new-crate obligation; the per-call-site tracing gate is standing S6 and is not carried here
conventions#7.1 · yes · the SAFETY comments on the unsafe blocks at stylo.rs:751-758, 1342 and 1414-1419, the borrow take-and-restore at construct.rs:264-273 and the let-chain at node.rs:1509-1510 sit in conflict surfaces a resolution must keep
conventions#8.1 · yes · the blitz-tests target rule binds: upstream's tests/all.rs is declared and held at test = false, and each joining target is recorded in the list test_blitz_tests_targets.py pins; the per-file rules are standing S1 and S2 and are not carried here
conventions#9.1 · no · the chunk does not reach escher-driver, where these three moved test modules sit, and no reached file or scope item names them
conventions#9.2 · no · standing S3 · shared fixture pages, boot and hold live once in tests/blitz-tests/tests/fixtures/mod.rs, and a page with one reader stays in its check's file
conventions#9.3 · no · the row-walk test pattern lives in escher-driver's mcp_tests.rs and in stand_requirement_sweep.rs, neither reached nor named by the scope
conventions#10.1 · no · its rules sit in apps and example entry points, CSS consts and Store uses in files the take does not edit; the reached cells.rs carries none of them
conventions#11.1 · no · the HTML fixture forms under examples/assets are not reached, and the scope names no change to the google or graphite pages
