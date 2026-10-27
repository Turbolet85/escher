# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · add-to · F1 and F3: a cognitive-complexity ceiling of 15 per function now stands beside the gate set as a dev-host reading (no CI leg), and nested closures count in their parent.
conventions#2.1 · no · the session edits no manifest, Cargo.lock or deny.toml, and none of the reached files is a manifest the entity cites.
conventions#3.1 · no · the reached files keep their opening //! docs (first edits at lint.rs :273, mcp_tests.rs :480 and the sweep test :1017), and the factored helpers are private, so no new public item needs a ///.
conventions#4.1 · no · the session adds no TODO, FIXME or ordering-sensitive marker, and none of the files the entity cites is reached.
conventions#5.1 · no · the factored items are private, with no new pub(crate), pub(super) or naming pattern, and none of the files the entity cites is reached.
conventions#6.1 · no · escher-driver still takes tracing ungated and builds its spans by hand (info_span! 2, no #[instrument]), and the three reached files add no span or event (their silence census reads 0).
conventions#7.1 · no · none of the files the entity cites is reached, and the reached changes are private-helper factorings with no unsafe block or borrowing workaround named in the report.
conventions#8.1 · yes · add-to · F2: a table-asserting test keeps its row walk in the test function and moves each row's checks into private #[track_caller] helpers, as the MCP listing test and the sweep test now do.
conventions#9.1 · no · mcp::tests stays a #[path] child in mcp_tests.rs with its test paths and names unchanged (only private helpers added), and the files this entity cites, cli.rs and recording.rs, are unreached.
conventions#9.2 · no · the fixtures module and its pages are unreached, the session adds no shared module, and the sweep test's new helpers stay private in their own file.
conventions#10.1 · no · the apps and examples this entity cites are unreached, and the session changes no app or example file.
conventions#11.1 · no · the HTML fixtures under examples/assets are unreached, and the session touches none.
