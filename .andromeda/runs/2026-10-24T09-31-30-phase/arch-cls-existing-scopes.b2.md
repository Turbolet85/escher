# Classified — arch · existing-scopes.b2

## Answers
existing-scopes#1.26 · yes · the Dioxus 0.8 re-sync and the manifest merge reach its dependencies on dioxus-native-dom and blitz-test-harness and its `accessibility` feature, and its driver checks are re-run as proofs
existing-scopes#1.27 · yes · mimalloc, the take's only reach into this app, lands as a global allocator in its main.rs and manifest, beside these module declarations
existing-scopes#1.28 · no · the take's only reach into apps/browser is mimalloc, and the chunk names the history store as not needed
existing-scopes#1.29 · no · the take names no change to the markdown viewer, and its entry names no dependency the take moves
existing-scopes#1.30 · yes · mimalloc lands as a global allocator in this runner's main.rs and manifest, beside these module declarations
existing-scopes#1.31 · yes · the take adds upstream's hit_test_dropped_nodes and stale_node_mapping as targets and moves the target pin, and every check it re-runs holds in both layout modes
existing-scopes#1.32 · yes · upstream's edit to examples/seven_guis/src/tasks/cells.rs lands here, and the stand checks and the escher-session binary are re-run against the Taffy and Stylo moves
existing-scopes#1.33 · no · the take names no change to these five separate example crates, and their entry names no dependency the take moves
existing-scopes#1.34 · yes · upstream's new root example examples/multi_doc.rs lands in this directory, so its println! and env reads are checked against the telemetry sink and the env registry
existing-scopes#1.35 · no · the chunk states that no fixture page is changed and lists the fixture forms and HTML fixture conventions as not needed
existing-scopes#2.1 · no · the chunk changes no driver code and names no layout convention for escher-driver's unit tests
existing-scopes#2.2 · yes · the target pin and the one-target-per-file convention must keep these three shared modules non-targets as the two new targets land, and the checks reading them are re-run
