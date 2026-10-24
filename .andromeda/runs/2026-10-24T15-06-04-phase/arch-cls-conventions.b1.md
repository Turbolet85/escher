# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the new checks pass the fmt, clippy -D warnings and rustdoc -D warnings legs the task's Needs name, and ci-leg.sh, the one leg runner, is a reached conflict surface
conventions#2.1 · yes · the reached tests/blitz-tests/Cargo.toml is a merge surface, and any new [[test]] target there takes workspace dependencies as { workspace = true } with publish = false
conventions#3.1 · yes · each new check file opens with a //! doc, and any public item the reached lint.rs or text_extent.rs gains takes /// under rustdoc -D warnings
conventions#4.1 · no · names TODO/FIXME markers in files the chunk does not reach; a re-check every change owes, a gap the chunk leaves open in the reached text_extent.rs or lint.rs carries its TODO/FIXME
conventions#5.1 · no · names crate internals in files the chunk does not reach; a re-check every change owes, an item added to the reached lint.rs or text_extent.rs stays pub(crate) or pub(super)
conventions#6.1 · yes · the host_log and mcp_log trace measurement reads escher-driver's command and tool_call spans and escher-telemetry's events, all emitted ungated, so it reads them under both builds
conventions#7.1 · no · names unsafe blocks and borrow workarounds in files the chunk does not reach; a re-check every change owes, the chunk adds no unsafe block and the reached text_extent reader is read-only
conventions#8.1 · yes · the test conventions bind each new check: one behaviour per file opened by a //! doc, both layout modes, the fixture asserted first, shared helpers in session_common/mod.rs, and the target table that test_blitz_tests_targets.py pins
conventions#9.1 · no · names the cli, recording and off-unix escher-driver test modules, none of them reached; lint.rs keeps its inline unit tests under conventions#8.1
conventions#9.2 · yes · fixtures/mod.rs is reached, and the new fixture checks take its fixture boot and hold, shared pages and HeldNet, with a page that has one reader staying in its check's file
conventions#10.1 · no · names the binary, Android, Dioxus example and browser-app shapes, none reached; the reached examples/seven_guis files are test targets, outside those rules
conventions#11.1 · no · names the examples/assets HTML fixtures, excluded by the task's Not needed; the sweep's fixtures are Rust check pages in blitz-tests
