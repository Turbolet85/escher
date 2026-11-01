# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the root manifest's rust-version "1.91.0" and edition 2024 are the MSRV rule the take keeps in both manifests (root Cargo.toml, reached whole)
stack-and-technologies#1.2 · yes · the in-repo workspace version, its `=0.3.0-beta.4` pins and the dioxus-native "0.7.0" line sit in the reached root manifest and move with the coupled Dioxus 0.7.3 set
stack-and-technologies#1.3 · yes · stylo 0.22.0, selectors 0.41.0 and cssparser 0.38.0 are named coupled pins the take carries as one set (root Cargo.toml, reached whole)
stack-and-technologies#1.4 · yes · markup5ever, html5ever and xml5ever are the parser crates the Needs match to Stylo's web_atoms (root Cargo.toml, reached whole)
stack-and-technologies#1.5 · yes · taffy 0.15 is a named coupled pin, and the lib.rs:96-97 comment this entity heads none is the edited orphan comment
stack-and-technologies#1.6 · yes · parley, skrifa and ICU4X pins sit in the reached root manifest, which the take's one-set pin rule can move
stack-and-technologies#1.7 · yes · anyrender, anyrender_vello and wgpu_context pins sit in the reached root manifest, which the take's one-set pin rule can move, and they drive the window renderer choice
stack-and-technologies#1.8 · yes · color, peniko, kurbo, wgpu, usvg and euclid pins sit in the reached root manifest, which the take's one-set pin rule can move
stack-and-technologies#1.9 · yes · winit `=0.31.0-beta.3` is a named coupled pin, and the keyboard-types 0.8 versus Dioxus's 0.7 glob-import naming is what a resolution in dioxus_document.rs must meet
stack-and-technologies#1.10 · yes · the accesskit 0.25 pin sits in the reached root manifest, and the a11y target is proven through the accessibility tree and the a11y leg the scope names
stack-and-technologies#1.11 · yes · url, http, tokio, reqwest and http-cache pins sit in the reached root manifest, and tokio's RUSTSEC-2026-0386 resolution is what the audit leg's ignored IDs record
stack-and-technologies#1.12 · yes · boa_engine and boa_gc pins sit in the reached root manifest, and the script engine's bootstrap.rs and events.rs are conflict surfaces the security rule covers
stack-and-technologies#1.13 · yes · Dioxus 0.7.3 is a named coupled pin, and dioxus-native-dom's dioxus_document.rs bridge is a conflict surface
stack-and-technologies#1.14 · yes · image, png, serde_json and wuff pins sit in the reached root manifest, and the no-new-package rule over the png and serde_json lock entries is what P3 re-reads
stack-and-technologies#1.15 · yes · wasm-bindgen, web-sys and the other WASM pins sit in the reached root manifest, which the take's one-set pin rule can move; the Trunk links are not reached and do not bind
stack-and-technologies#1.16 · yes · rayon, test-that, tracing and the other misc pins sit in the reached root manifest, and the STYLE_THREAD_POOL use is in stylo.rs, a conflict surface
stack-and-technologies#1.17 · no · the browser history's rusqlite and rusqlite_migration pins live in apps/browser/persistence/Cargo.toml, which is not reached, and the browser app is in Not needed
stack-and-technologies#1.18 · no · the browser app's extras (nucleo, mimalloc, rfd, android-activity) live in apps/browser/Cargo.toml, which is not reached, and the browser app is in Not needed
stack-and-technologies#1.19 · no · the rdme markdown app's pins live in apps/readme/Cargo.toml, which is not reached, and rdme is in Not needed
stack-and-technologies#1.20 · yes · blitz-tests' dev-dependencies (blitz-test-harness, escher-driver, png, cursor-icon) are what the take's upstream test files run under, and the pointer_leave check's ShellProvider rides cursor-icon
stack-and-technologies#1.21 · no · the Nix flake's inputs and rust-bin pin are flake packaging, which Not needed drops, and the chunk reads no flake file
stack-and-technologies#1.22 · yes · the ci.yml free-disk step of the matrix_test job, the Stand checks repeated free-disk watch, and test_ci_workflows.py's read over the edited file are named by the scope
stack-and-technologies#1.23 · yes · the scope names the fast, doc, a11y and audit legs, whose fmt, clippy and RUSTDOCFLAGS gates are the entity's named ones; the host-only mutation tier does not bind
stack-and-technologies#2.1 · no · the cognitive-complexity reading runs on the dev host alone, never in CI, and no leg the scope re-runs or unit it names calls rust-code-analysis
