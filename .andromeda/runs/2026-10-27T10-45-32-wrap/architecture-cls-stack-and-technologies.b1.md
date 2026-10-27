# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the editions and rust-version sit in manifests byte-identical to the base, and no crate was added or retired
stack-and-technologies#1.2 · no · the workspace version and the in-repo crate pins sit in Cargo.toml and the crate manifests, all byte-identical to the base
stack-and-technologies#1.3 · yes · stale · stylo.rs is a merge surface and its selector-matching lines cited at 14-20 can have moved, while the Stylo 0.22.0 pins stay
stack-and-technologies#1.4 · no · the html5ever, markup5ever and xml5ever pins sit in an unchanged manifest, and the parser and namespace files it cites are not reached
stack-and-technologies#1.5 · no · taffy 0.15 stays in an unchanged manifest, and layout/mod.rs, the file its layout-mode role cites, is not reached
stack-and-technologies#1.6 · no · the parley, skrifa, ICU4X and wuff pins sit in unchanged manifests, and the text, font and net files it cites are not reached
stack-and-technologies#1.7 · no · the anyrender pins sit in the unchanged root manifest, and the example and renderer files it cites are not reached
stack-and-technologies#1.8 · no · the graphics pins sit in the unchanged root manifest, and the resolve, resolved_style and svg files it cites are not reached
stack-and-technologies#1.9 · yes · stale · blitz-shell lib.rs is a merge surface cited at 54-67 for the window shell, and the windowed redraw now routes through the event loop while winit stays =0.31.0-beta.3
stack-and-technologies#1.10 · no · the accesskit pins sit in unchanged manifests, and accessibility.rs, the file its role cites, is not reached
stack-and-technologies#1.11 · no · the networking pins sit in unchanged manifests, and blitz-net lib.rs, the file its role cites, is not reached
stack-and-technologies#1.12 · yes · stale · blitz-vibey-script lib.rs is a merge surface cited at 1-6 for the script-execution role, while Boa itself stays in unchanged manifests
stack-and-technologies#1.13 · no · the Dioxus 0.7.3 pins and the two manifests this entity cites are byte-identical, and the conflict file dioxus_document.rs is cited by no line here
stack-and-technologies#1.14 · no · the media pins sit in unchanged manifests, and net.rs, where its decoding role lives, is not reached
stack-and-technologies#1.15 · no · the wasm pins sit in the unchanged root manifest, and the example index and lib files it cites are not reached
stack-and-technologies#1.16 · yes · stale · stylo.rs is a merge surface cited at 28 and 156-165 for the parallel style traversal, and its lines can have moved
stack-and-technologies#1.17 · no · rusqlite and rusqlite_migration sit in the unchanged persistence manifest, and the persistence crate is not reached
stack-and-technologies#1.18 · yes · stale · apps/browser/src/main.rs is a merge surface cited at 7-9 for the mimalloc allocator and its lines can have moved, while the browser manifests stay byte-identical
stack-and-technologies#1.19 · no · the comrak, pulldown-cmark, notify and reqwest pins sit in the unchanged rdme manifest, and the session reaches no markdown app file
stack-and-technologies#1.20 · no · the blitz-tests and wpt dev-dependencies sit in unchanged manifests, the cited test and runner files are not reached, and the new target adds no dependency
stack-and-technologies#1.21 · no · the rust-version 1.91.0 pin is unchanged, and the session touches no build environment file
stack-and-technologies#1.22 · yes · stale · publish-browser.yml is a merge surface that gained lines at 126 and 153-166, inside the cited ranges 122-126 and 152-166
stack-and-technologies#1.23 · no · the fmt, clippy, rustdoc and audit legs cite ci.yml, wpt.yml, ci-leg.sh and the browser manifest, which are unreached or unchanged, and no ci-leg.sh change lands
stack-and-technologies#2.1 · no · the host-only complexity reading names no reached file, and the escher-driver and escher-telemetry diffs are empty with no CI script changed
