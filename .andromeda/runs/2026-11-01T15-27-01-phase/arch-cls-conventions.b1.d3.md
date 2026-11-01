# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · standing S4 · cargo fmt --all --check, cargo clippy --workspace --locked -D warnings, rustdoc -D warnings, and expect_used/unwrap_used allowed only next to a justification
conventions#2.1 · no · standing S8 · a dependency line is declared once in [workspace.dependencies] and consumed as { workspace = true }, in-repo crates with default-features = false
conventions#3.1 · no · standing S5 · engine code opens with //! docs, public items carry ///, and web-semantics implementations cite the spec URL
conventions#4.1 · yes · the TODO/FIXME markers at construct.rs:417 and :602, scrolling.rs:212 and :570, and stylo.rs:1232 sit in conflict surfaces the resolutions may move, and a known gap the panic fork leaves open takes the same marker
conventions#5.1 · yes · the pub(crate) item at scrolling.rs:33 and the __blitz_ native prefix at bootstrap.rs:93-121 sit in conflict surfaces the resolutions may move
conventions#6.1 · no · standing S6 · a log line in the engine crates is gated per call site with a cfg(not(feature = "tracing")) fallback
conventions#7.1 · yes · unsafe blocks at stylo.rs:751-758, :1342 and :1414-1419 need an allow or SAFETY comment, and the borrow workaround at construct.rs:264-273 and the accessor macro and let-chain at node.rs:138-180 and :1509-1510 sit in conflict surfaces
conventions#8.1 · yes · the take's upstream test files join targets that test_blitz_tests_targets.py must pin, with tests/all.rs declared and held at test = false (tests/blitz-tests/Cargo.toml:46-59), a rule the scope names under the blitz-tests rule
conventions#9.1 · no · the three escher-driver test-module homes (cli, recording, off-unix) are in a crate the take does not reach and no reached file cites them
conventions#9.2 · no · standing S3 · the shared fixture boot, hold and pages live once in tests/blitz-tests/tests/fixtures/mod.rs, and a page with one reader stays in its check's file
conventions#9.3 · no · the row-walk assertion helpers of mcp_tests.rs and stand_requirement_sweep.rs are in files outside the reached set, and no reached check walks rows through them
conventions#10.1 · no · the app and example rules cite apps and seven_guis main.rs and counter.rs files the take does not reach, and no line of cells.rs carries one
conventions#11.1 · no · the HTML fixture forms under examples/assets are not reached, and the scope names no change to the google or graphite fixtures
