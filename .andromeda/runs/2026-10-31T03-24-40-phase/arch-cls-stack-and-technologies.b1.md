# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the chunk's Rust in blitz-dom, escher-driver and blitz-tests must compile at edition 2024 and rust-version 1.91.0
stack-and-technologies#1.2 · yes · the in-repo `=0.3.0-beta.4` pins and the Dioxus 0.7.0 pins are dependency pins the chunk holds unmoved with no bump, the version and license lines not reached
stack-and-technologies#1.3 · yes · Stylo's selector matching settles `:open` and `:focus-within`, its SnapshotMap is taken before an inert attribute mutation, and its pin stands unmoved
stack-and-technologies#1.4 · yes · html5ever's tree builder produces the DOM of the chunk's new HTML fixtures and of the details and dialog `open` markup its `:open` check reads
stack-and-technologies#1.5 · yes · Taffy's layout and cache produce the client rects, hoisted positions and option bounds the fixes and the incremental oracle read, and its pin stands unmoved
stack-and-technologies#1.6 · yes · Parley lays out the text of the pinned stand and its one bundled font, and its pin stands unmoved
stack-and-technologies#1.7 · no · the rows and fixes read client rects, hits, the tree and redraw requests, and the popup stays unpainted as a stated limit, so the paint path is not reached
stack-and-technologies#1.8 · no · the geometry, colour, GPU and SVG crates state no behaviour, call or value the fix or rows must meet, and none is among the Stylo, Taffy, Parley and Dioxus pins the Needs hold
stack-and-technologies#1.9 · yes · winit's ApplicationHandler carries the windowed shell's redraw-route check, and keyboard-types 0.8 beside Dioxus's 0.7 binds the Enter and released-Space activation, where a module glob-importing dioxus::prelude must name `::keyboard_types`
stack-and-technologies#1.10 · yes · accesskit 0.25's `toggled` property is the one closed value the accessibility.rs write sets for a native checkbox or radio and the platform adapter receives, the accesskit_xplat crates being out
stack-and-technologies#1.11 · no · the load page's fetch is answered by the in-process LoadNet stand-in, so blitz-net, its cache, reqwest, tokio and the outbound hosts are not reached
stack-and-technologies#1.12 · no · the JavaScript runtime's bootstrap, module loader, timer thread and document_may_load are not needed, so Boa is not reached
stack-and-technologies#1.13 · yes · Dioxus drives the stand's served pages, its bridge's skipping of event kinds with zero handlers bears on the change event, and its 0.7.3 pin stands unmoved
stack-and-technologies#1.14 · yes · serde_json is escher-driver's reader of kept checks and of the live answer line stand_replay compares, which the chunk's lint and schema edits can move
stack-and-technologies#1.15 · no · the wasm entry points and browser-target builds are not needed
stack-and-technologies#1.16 · no · rayon's style traversal, parallel-construct and the other misc crates sit in no reached file or named Need, and test-that and tracing reach the chunk through the testing entity
stack-and-technologies#1.17 · no · the browser's rusqlite history store and its migration defaults are not needed
stack-and-technologies#1.18 · no · the browser app's extras (nucleo, mimalloc, the rfd portal, the Android entry) are not needed
stack-and-technologies#1.19 · no · the rdme markdown app is not needed
stack-and-technologies#1.20 · yes · the blitz-tests dev-dependencies the new and re-asserted checks build on are blitz-test-harness, escher-driver, the seven_guis headless stand, anyrender_vello_cpu, dioxus and test-that
stack-and-technologies#1.21 · no · the Nix flake and the dev host's build tooling are not reached by the chunk's code or checks
stack-and-technologies#1.22 · no · standing S8 · the per-crate coverage floors that coverage_floor.py holds for the coverage leg, a convention the task lists
stack-and-technologies#1.23 · no · standing S7 · the fmt, clippy and rustdoc `-D warnings` gates run through ci-leg.sh, a convention the task lists; the stand-repeat budget and coverage floors it also states are S8's
stack-and-technologies#2.1 · no · the dev-host cognitive-complexity reading, never in CI, is a per-function ceiling every code change owes equally, which the chunk does not name
