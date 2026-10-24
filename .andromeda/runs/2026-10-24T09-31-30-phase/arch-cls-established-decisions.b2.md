# Classified — arch · established-decisions.b2

## Answers
established-decisions#19.1 · yes · `DioxusDocument`'s `base_url` default sits in `dioxus_document.rs`, which the 0.8 re-sync replaces, and its UA-sheet path runs through `config.rs`, which the serialised Parallel traversal edits
established-decisions#20.1 · no · the browser app's history store, search and runtime, which the task excludes; mimalloc is its only reach, an allocator and not app behaviour
established-decisions#21.1 · no · the WPT runner's render target, blocked tests, suites and timers, which the task excludes as upstream-only; mimalloc is its only reach, an allocator that bears on none of them
established-decisions#22.1 · yes · its feature-reach and lock claims (no platform adapter in escher-driver's graph, `Cargo.lock` unchanged) and its `NamedKey` re-exports bind through the take's lock and the Dioxus 0.8 keyboard-types move, while its socket, idle expiry, MCP, carrier and span rules are not reached
established-decisions#23.1 · no · the recording, replay and owner-only rules of a kept check and the committed checks, which the task excludes as not needed, none of them touched by the take
