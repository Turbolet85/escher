# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · Cargo.toml is a conflict surface; its rust-version 1.91.0 and edition 2024 are the MSRV the take keeps in both manifests
stack-and-technologies#1.2 · yes · root Cargo.toml is a conflict surface; the workspace version, in-repo crate pins and dioxus-native 0.7.0 lines are what a resolution writes
stack-and-technologies#1.3 · yes · the stylo 0.22.0, selectors and cssparser pins are the coupled set the take carries from upstream or not at all
stack-and-technologies#1.4 · yes · the markup5ever, html5ever and xml5ever pins are Cargo.toml conflict-surface lines in the matched parser set
stack-and-technologies#1.5 · yes · the taffy 0.15 pin and its registry-versus-git state are the coupled pin set, and the lib.rs:96-97 orphan comment the chunk edits sits under it
stack-and-technologies#1.6 · yes · the parley, skrifa and ICU4X pins are Cargo.toml conflict-surface lines the take's resolution may move
stack-and-technologies#1.7 · yes · the anyrender and wgpu_context pins are Cargo.toml conflict-surface lines the take's resolution may move
stack-and-technologies#1.8 · yes · the color, peniko, kurbo, wgpu, usvg and svgtypes pins are Cargo.toml conflict-surface lines
stack-and-technologies#1.9 · yes · the winit =0.31.0-beta.3 pin and the keyboard-types 0.8 versus Dioxus's 0.7 are the coupled set and Cargo.toml conflict-surface lines
stack-and-technologies#1.10 · yes · the accesskit 0.25 and accesskit_xplat 0.2 pins are Cargo.toml conflict-surface lines the a11y target rests on
stack-and-technologies#1.11 · yes · the url, http, tokio, reqwest and http-cache pins are Cargo.toml conflict-surface lines, the tokio line tied to the audit's advisory
stack-and-technologies#1.12 · yes · the boa_engine and boa_gc pins are Cargo.toml conflict-surface lines under the script engine security surface
stack-and-technologies#1.13 · yes · the Dioxus 0.7.3 crates are the coupled pin set the take carries as one set, and the bridge in dioxus_document.rs is reached
stack-and-technologies#1.14 · yes · the image, png, serde_json and wuff pins are Cargo.toml conflict-surface lines, and no new crate may enter the lock
stack-and-technologies#1.15 · yes · the wasm-bindgen, web-sys and console_error_panic_hook pins are Cargo.toml conflict-surface lines
stack-and-technologies#1.16 · yes · the rustc-hash through thin-vec pins, test-that 0.6.0 among them, are Cargo.toml conflict-surface lines
stack-and-technologies#1.17 · no · the rusqlite pin lives in apps/browser/persistence/Cargo.toml, which the chunk does not reach and the scope leaves out
stack-and-technologies#1.18 · no · the browser app's manifest, mimalloc, nucleo and Android entries sit in apps/browser, which the scope names no change in
stack-and-technologies#1.19 · yes · the pulldown-cmark resolution past RUSTSEC-2026-0414 is a Cargo.lock conflict-surface line the take's lock must keep for the audit leg
stack-and-technologies#1.20 · yes · the blitz-tests dev-dependencies, png and cursor-icon for upstream's pointer_leave check included, are what the taken test files and stand checks run on; the root dev-dependencies at Cargo.toml:298-303 are a conflict surface
stack-and-technologies#1.21 · no · the Nix flake's packaging and its 1.91.0 pin are named out of scope, and the dev host toolchain is read by no leg the chunk runs
stack-and-technologies#1.22 · yes · ci.yml's matrix_test free-disk if:, the stand-checks free-disk watch, test_ci_workflows.py, test_blitz_tests_targets.py and the ci-leg.sh legs are the CI surfaces the chunk edits or re-reads
stack-and-technologies#1.23 · yes · the fmt, clippy, doc, audit and coverage-floor legs and the stand-repeat budget are the proof gates the chunk must pass green
stack-and-technologies#2.1 · no · the dev-host cognitive-complexity reading is run by no leg the chunk runs and named by no scope line
