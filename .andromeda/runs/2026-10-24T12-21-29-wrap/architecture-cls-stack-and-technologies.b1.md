# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the take changes no edition or rust-version and root Cargo.toml stays byte-identical
stack-and-technologies#1.2 · no · root Cargo.toml stays byte-identical, so the workspace version, the in-repo pins and the crate versions stand
stack-and-technologies#1.3 · no · Stylo stays 0.22.0 with the 0.23-pre left out, and the cited stylo.rs lines 14-20 sit above the shift at line 29
stack-and-technologies#1.4 · no · the html5ever, markup5ever and xml5ever pins stay and none of the cited parser lines move
stack-and-technologies#1.5 · yes · stale · the take shifts resolve.rs after line 29, moving the taffy re-export citation at resolve.rs:456-460
stack-and-technologies#1.6 · no · the parley, skrifa, ICU and wuff pins are unchanged and none of the cited text, font or net lines move
stack-and-technologies#1.7 · no · the anyrender pins are unchanged and the cited renderer selection and example lines do not move
stack-and-technologies#1.8 · no · the graphics pins are unchanged and the cited resolve.rs:7 sits above the shift at line 29
stack-and-technologies#1.9 · no · the winit, arboard, rfd and cursor-icon pins are unchanged and no cited blitz-shell line moves
stack-and-technologies#1.10 · no · the accesskit pins are unchanged and the take reaches no accessibility builder hunk
stack-and-technologies#1.11 · yes · add-to · tokio now resolves to 1.54.1 in Cargo.lock alone, past RUSTSEC-2026-0386, while the manifest requirement 1.42 stands
stack-and-technologies#1.12 · no · the Boa pins are unchanged and no cited blitz-vibey-script line moves
stack-and-technologies#1.13 · no · Dioxus stays 0.7.3, and the Parallel launch setting in dioxus_native::launch is a launch-path fact rather than a stack pin
stack-and-technologies#1.14 · no · the media, serde and serde_json pins are unchanged and none of the cited net.rs or escher-driver lines move
stack-and-technologies#1.15 · no · the WASM pins and the cited example entry files are untouched
stack-and-technologies#1.16 · yes · add-to · every Parallel style traversal now runs under the new process-wide static Mutex PARALLEL_TRAVERSAL_LOCK in stylo.rs, and the take shifts stylo.rs after line 29, moving the citation at 149-153
stack-and-technologies#1.17 · no · the browser persistence crate and its rusqlite pins are untouched
stack-and-technologies#1.18 · yes · add-to · the browser's default feature gains mimalloc, so the optional allocator (resolved 0.1.52) is now on by default and is the browser's global allocator
stack-and-technologies#1.19 · no · the rdme manifest lines and pins are not in the take
stack-and-technologies#1.20 · yes · add-to · wpt/runner/Cargo.toml gains mimalloc as a direct dependency and the WPT runner's global allocator in main.rs, which this dependency list does not carry
stack-and-technologies#1.21 · yes · contradict · flake.nix line 31 moves the rust-bin stable pin from 1.90.0 to 1.91.0, so the 1.90.0 pin and its drift below rust-version 1.91.0 are no longer true
stack-and-technologies#1.22 · no · the .github workflows and scripts are unchanged, so the CI legs, cache rules and publish pins stand
stack-and-technologies#1.23 · no · the fmt, clippy, doc, audit and coverage legs and deny.toml are unchanged and the take adds no ignore
