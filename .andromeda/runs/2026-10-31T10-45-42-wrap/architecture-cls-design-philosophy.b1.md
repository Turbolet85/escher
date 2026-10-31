# Classified — architecture · design-philosophy.b1

## Answers
design-philosophy#1.1 · no · the claim still holds on the changed files, as no crate or manifest moves, packages/dioxus-native-dom/src/lib.rs:3 is not re-pointed, and .github/workflows/ci.yml:28-30 sits above the change at :311-316 and CI is green at 40ac6ddac
design-philosophy#2.1 · yes · stale · the citations at packages/blitz-dom/src/document.rs:476-487 and packages/dioxus-native-dom/src/dioxus_document.rs:57 and :83-84 are re-pointed by the session's edits to those files, and the provider claim still holds
design-philosophy#3.1 · yes · stale · the citations at packages/blitz-dom/src/document.rs:1017-1036, packages/blitz-dom/src/events/pointer.rs:161-169 and packages/blitz-dom/src/events/keyboard.rs:468 are re-pointed by the session's edits to those files, and the interaction-state teardown it cites now has select_cursor to read against
design-philosophy#4.1 · yes · stale · the citation at packages/blitz-dom/src/document.rs:2253-2270 is re-pointed by the session's edits to that file, and the pipeline claim still holds
design-philosophy#5.1 · no · the claim still holds on the changed tests directory, since the new integration tests join tests/blitz-tests/tests/ where the claim already places them, and its other cited harness, WPT, paint-bench and browser files are unreached
