# Classified — architecture · standard-contracts.b4

## Answers
standard-contracts#20.1 · no · the recorder's rule lives in escher-driver recording.rs and session.rs, an empty diff, and the masked-control predicate it calls is dioxus-native-dom's, where the report names only a test at dioxus_document.rs:725-759
standard-contracts#20.2 · no · the replay rule lives in escher-driver replay.rs, an empty diff, and names no reached file
standard-contracts#20.3 · no · the six readings live in escher-driver lint.rs, an empty diff, and the report re-reads visible_region as unchanged
standard-contracts#20.4 · no · the load wait lives in escher-driver session.rs, an empty diff, and reads only Harness::settle, which no reached file holds
standard-contracts#21.1 · no · the overlap reading and its pin live in escher-driver lint.rs and stand_lint.rs, neither a reached file
standard-contracts#21.2 · no · the load-fetch witness and its LoadNet stand-in live in stand_load_page.rs and the stand, neither a reached file
standard-contracts#22.1 · yes · add-to · F5 adds document_may_load and a FetchError refusal to the fetch error list and F6 the CSSStyleDeclaration accessors to the CSSOM list, and the runtime.rs bootstrap and event citations moved with the module split
standard-contracts#23.1 · no · the launch functions, configs and hooks live in the blitz and dioxus-native crates, none reached, and the report moves no launch API
