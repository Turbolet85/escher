# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · rust-version 1.91.0 (Cargo.toml:41) is an inherited pin the take may move or keep, in the root manifest that is a merge surface
stack-and-technologies#1.2 · yes · the workspace version, licence, in-repo =0.3.0-beta.4 pins and dioxus-native(-dom) 0.7.0 pins sit in the root manifest, a merge surface a whole-merge take can move
stack-and-technologies#1.3 · yes · Stylo 0.22.0 with selectors 0.41.0 and cssparser 0.38.0 are the inherited Stylo pins the take keeps or swaps for upstream's git Stylo, in the root manifest (merge surface)
stack-and-technologies#1.4 · yes · the markup5ever, html5ever and xml5ever pins are the parser couplings to Stylo's web_atoms that move only as one set, in the root manifest (merge surface)
stack-and-technologies#1.5 · yes · the taffy 0.15 registry pin the take keeps or swaps for upstream's git Taffy sits in the root manifest (merge surface), and the reached scrolling code reads Taffy's ScrollableOverflow
stack-and-technologies#1.6 · yes · the parley 0.12 and skrifa 0.44 pins are coupled to vello and move as one set, with the ICU4X and wuff pins, in the root manifest (merge surface)
stack-and-technologies#1.7 · yes · the anyrender and vello pins at Cargo.toml:120-128 are a merge surface coupled to skrifa, and the stand's headless screenshot pixels painted through anyrender_vello_cpu are re-measured under them
stack-and-technologies#1.8 · yes · the svgtypes 0.16.1 and usvg 0.48.1 pins are a coupled pair the take keeps together, with color, peniko, kurbo and wgpu, in the root manifest (merge surface)
stack-and-technologies#1.9 · yes · the winit =0.31.0-beta.3 pin the take may move to beta.4, arboard's removal and keyboard-types 0.8 sit in the root manifest (merge surface) and bind the shell's ApplicationHandler and winit's IME copy
stack-and-technologies#1.10 · yes · the accesskit 0.25 pin sits in the root manifest (merge surface), and the accesskit_unix 0.24.0 to zbus chain that the whole-graph audit reads binds the a11y tree builder under the take's IME, :open and hit-test merges
stack-and-technologies#1.11 · yes · the url, http, reqwest, tokio and http-cache pins sit in the root manifest (merge surface), and tokio's resolved 1.54.1, past RUSTSEC-2026-0386, is one the whole-graph audit reads under a lock move
stack-and-technologies#1.12 · yes · the Boa 0.22 pins sit in the root manifest (merge surface), and the boa_engine annex-b feature binds the runtime.rs conflict and the ScriptFetcher and report_js_error contract
stack-and-technologies#1.13 · yes · the Dioxus 0.7.3 crate pins sit in the root manifest (merge surface) and are what a take moves to 0.8.0-alpha.3, binding dioxus_document.rs and the stable-id bridge
stack-and-technologies#1.14 · yes · the image, png, serde and serde_json pins sit in the root manifest (merge surface), and png 0.18 is the encoder for escher-driver's shots and the decoder for blitz-tests' screenshot checks
stack-and-technologies#1.15 · yes · the wasm-bindgen, web-sys, web-time and console_error_panic_hook pins sit in the root manifest (merge surface), which a merge can move, though no wasm build is among the chunk's proofs
stack-and-technologies#1.16 · yes · the rustc-hash, slotmap, tracing 0.1.40, rayon, thread_local and test-that pins sit in the root manifest (merge surface), and rayon and thread_local back parallel-construct in resolve.rs and document.rs
stack-and-technologies#1.17 · no · the browser app's history database (rusqlite, migrations, history_entries) is named Not needed in the task, and apps/browser/persistence is not a reached file
stack-and-technologies#1.18 · no · apps/browser's manifest and sources are not a reached file, and its explicit clipboard feature keeps the app's clipboard when the move leaves dioxus-native's default list
stack-and-technologies#1.19 · no · apps/readme's manifest and sources are not a reached file and its pins sit in no predicted merge surface
stack-and-technologies#1.20 · yes · the root dev-dependencies at Cargo.toml:298-303 (env_logger, vello, vello_cpu, vello_gpu, vello_common) sit in the root manifest (merge surface), and the blitz-tests dev-deps that the joined upstream test files and re-measured pixels use bind
stack-and-technologies#1.21 · yes · the flake's rust 1.91.0 pin must follow Cargo.toml's rust-version if the take moves it, by the flake's own keep-in-sync comment; the rest of the Nix packaging is named Not needed
stack-and-technologies#1.22 · yes · the ci-leg.sh legs and fork CI the chunk re-runs, cargo-deny@0.20.2 under the audit leg, and test_blitz_tests_targets.py's Python 3.11+ tomllib floor that the new targets join bind; the runner's S7 line is standing
stack-and-technologies#1.23 · yes · the whole-graph cargo-deny audit, the dioxus-native-dom 83% line floor that the take's dioxus_document.rs merge reaches, the blitz-dom second clippy line and the stand-repeat budget bind; the fmt, clippy and rustdoc lines are standing S3 and S4
stack-and-technologies#2.1 · no · a dev-host-only per-function complexity reading that no CI leg runs and the scope does not name, a kind-wide rule every code change owes alike
