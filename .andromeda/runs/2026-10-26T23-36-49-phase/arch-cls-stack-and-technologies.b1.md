# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the refactored findings, its helpers and the two test bodies are Rust that must build at edition 2024 and the rust-version 1.91.0 MSRV
stack-and-technologies#1.2 · no · the chunk changes no manifest, version pin or inherited workspace metadata
stack-and-technologies#1.3 · yes · the contrast defect and stand_lint's zero-contrast rows read the computed colour, font size and weight that Stylo's Stylist cascade produces, while text_contrast is called unchanged
stack-and-technologies#1.4 · no · the lint's defects read geometry, text extent and computed colour rather than the parsed tree, and the chunk calls no parser API
stack-and-technologies#1.5 · yes · overlap, clipped, off-screen and zero-size read the bounds that taffy's block, flex, grid and cached layout produces, which stand_lint re-asserts in both layout modes
stack-and-technologies#1.6 · yes · the fonts-unavailable check through can_paint_text must stay before any read, and text_extent and truncated read parley's inline layout of the stand's bundled font
stack-and-technologies#1.7 · no · the lint paints nothing and takes no shot, so anyrender and its renderer backends are not reached
stack-and-technologies#1.8 · no · no geometry, colour, GPU or SVG crate is called or changed, and the contrast arithmetic is the harness's own
stack-and-technologies#1.9 · no · the driver and lint run headless, with no window, clipboard, file dialog or key type reached
stack-and-technologies#1.10 · no · the lint walks the snapshot's nodes and reads no AccessKit name, role or state, and no accesskit crate or adapter is called
stack-and-technologies#1.11 · no · the lint and the listing test fetch nothing, so blitz-net and reqwest are not reached
stack-and-technologies#1.12 · no · no script runs under the lint, the listing test or the stand's pages, so blitz-vibey-script is not reached
stack-and-technologies#1.13 · no · the lint walks a snapshot that dioxus-native-dom builds and reads no virtual DOM or renderer state, and the chunk calls no Dioxus API
stack-and-technologies#1.14 · yes · escher-driver's serde_json is its reader alone, so the lint's hand-written json answer must stay byte-identical and unwritten by serde_json
stack-and-technologies#1.15 · no · no wasm32 build is named by the scope and the chunk's code is native, so the browser-target builds are not reached
stack-and-technologies#1.16 · no · the chunk adds no span, log site or parallel path, and the command span stays in Session::run, unchanged
stack-and-technologies#1.17 · no · rusqlite is the browser app's history store, which the chunk does not reach
stack-and-technologies#1.18 · no · the browser's UI, nucleo, mimalloc and Android entries are not reached
stack-and-technologies#1.19 · no · the rdme markdown viewer is not reached
stack-and-technologies#1.20 · yes · the targets stand_lint and stand_requirement_sweep and the unit file mcp_tests.rs are the chunk's tests, built on the dev-dependencies seven_guis and escher-driver
stack-and-technologies#1.21 · no · the Nix flake, the dev host toolchain and cross-compilation are not reached, and the MSRV is carried by #1.1
stack-and-technologies#1.22 · yes · the fast, doc and stand-repeat legs the scope names, and test_blitz_tests_targets.py, which needs Python 3.11 or later for tomllib, are the gates and re-reads the chunk owes
stack-and-technologies#1.23 · yes · the fast and doc gates the scope names, fmt, clippy --locked -D warnings and rustdoc -D warnings over every workspace crate, must pass
