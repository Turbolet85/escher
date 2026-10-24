# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · edition 2024 and rust-version 1.91.0 are named inherited values a take could move, and the MSRV job builds on 1.91
stack-and-technologies#1.2 · yes · the one workspace version and the in-repo `=0.3.0-beta.4` pins are what a Dioxus package sync and the manifest merge touch
stack-and-technologies#1.3 · yes · Stylo 0.22.0 is a named inherited value, and the Stylo 0.23 pre-release upstream names touches accessibility.rs
stack-and-technologies#1.4 · yes · html5ever 0.40.1 is one side of the coupled html5ever ↔ stylo web_atoms pin the take must keep
stack-and-technologies#1.5 · yes · taffy 0.15 is a named inherited value that stays a registry version with no git dependency in the root manifest
stack-and-technologies#1.6 · yes · parley 0.12 is a named inherited value, and skrifa 0.44 is its coupled pin (skrifa ↔ parley/vello)
stack-and-technologies#1.7 · no · the renderer backends are Not needed and upstream's delta changes none of the anyrender crates
stack-and-technologies#1.8 · yes · svgtypes 0.16.1 ↔ usvg 0.48.1 is a named coupled pin the take must keep, and usvg is a blitz-tests dev-dependency
stack-and-technologies#1.9 · yes · the exact winit beta is a named coupled pin, and the Dioxus-carried second keyboard-types version is a named need a Dioxus 0.8 move can change
stack-and-technologies#1.10 · yes · accesskit_unix 0.24.0 under accesskit_xplat is the path that brings tokio into the audited graph, so the shell-and-windowing decision binds through it
stack-and-technologies#1.11 · yes · tokio 1.42 is the crate the semver-compatible RUSTSEC-2026-0386 update moves
stack-and-technologies#1.12 · no · the JavaScript runtime is Not needed, upstream's delta changes no script behaviour, and no Boa version is a take value
stack-and-technologies#1.13 · yes · Dioxus 0.7.3 is a named inherited value, and upstream's move of every Dioxus package to 0.8.0-alpha.3 is a shape the take may take
stack-and-technologies#1.14 · no · no take need names image, png, wuff, serde_json or the other media crates, and the driver's encoder and the stand's decoder are only re-run
stack-and-technologies#1.15 · no · no take need names wasm-bindgen, web-sys or the Trunk examples, and browser-target builds fall under the Not-needed cross-compilation
stack-and-technologies#1.16 · yes · rayon and thread_local back the parallel style traversal and parallel construct that upstream's serialised traversal edits, and test-that 0.6.0 is the blitz-tests framework
stack-and-technologies#1.17 · no · the browser's history store is Not needed, and no take need names rusqlite or the persistence crate
stack-and-technologies#1.18 · yes · mimalloc 0.1.48 in apps/browser is upstream's new dependency landing through the manifest conventions as process-wide state, and the browser's tokio features are reached by the tokio update
stack-and-technologies#1.19 · no · the markdown viewer's crates are no take value and no take need names apps/readme or its manifest
stack-and-technologies#1.20 · yes · the blitz-tests dev-dependencies (test-that, usvg, the stand and harness crates) are the tests the take re-runs, and upstream's two new targets would join them
stack-and-technologies#1.21 · yes · the flake's rust 1.90.0 pin sits below rust-version 1.91.0, upstream's flake.nix commit moves it, and the Nix packaging decision is a named need
stack-and-technologies#1.22 · yes · the workflows and their ci-leg.sh legs including the MSRV build, the pinned actions, the CI-installed cargo-deny and the target-pin script are named CI contracts
stack-and-technologies#1.23 · yes · the fmt, clippy, doc, audit, coverage and stand-repeat legs and the cargo-deny gate over the whole workspace are named CI contracts the take keeps green
