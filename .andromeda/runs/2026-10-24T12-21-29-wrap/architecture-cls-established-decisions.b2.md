# Classified — architecture · established-decisions.b2

## Answers
established-decisions#19.1 · yes · add-to · dioxus_native::launch now sets Parallel style threading in lib.rs, a document-config fact this decision must carry, and the cited lib.rs:166-175 and config.rs:37-41, :76-95 lines moved too
established-decisions#20.1 · yes · add-to · the take makes mimalloc the default-feature global allocator of apps/browser and sets Parallel style threading in document_loader.rs and toolbar.rs, browser facts this decision must carry; no cited file is on the shifted list
established-decisions#21.1 · yes · stale · wpt/runner/src/main.rs gains three lines above every cited line and attr_test.rs shifts after line 3, moving each cited main.rs and attr_test.rs line; the runner also gains mimalloc as its global allocator
established-decisions#22.1 · no · escher-driver, the root Cargo.toml and dioxus-native-dom are byte-unchanged, so the driver facts hold, and the take's Cargo.lock edits add no driver dependency (Cargo.lock:3099 not re-measured)
established-decisions#23.1 · no · recording and replay sit in escher-driver, byte-unchanged, and the two committed checks are untouched, so no fact here moves
