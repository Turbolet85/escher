# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the Rust 2024 edition and rust-version 1.91.0 bind every Rust line the chunk writes in seven_guis and blitz-tests
stack-and-technologies#1.2 · no · the shared version, licence and in-repo `=` pins are manifest metadata, and the chunk changes no manifest or dependency (Not needed: dependency pinning, release versioning)
stack-and-technologies#1.3 · yes · Stylo's selector matching and computed styles bind the stand's const CSS, and the contrast rows read their colours, opacity and backgrounds from those computed styles
stack-and-technologies#1.4 · yes · the HTML pages fixtures/mod.rs holds (reached whole) are parsed by html5ever on boot
stack-and-technologies#1.5 · yes · taffy's block, flexbox, grid and cached layout place every stand screen the chunk re-pins, cells and the flight booker's select included, in both incremental and non-incremental modes
stack-and-technologies#1.6 · yes · the stand's headless boot registers its one bundled font through the fontique and FontContext text stack, and that boot is one of the chunk's edits
stack-and-technologies#1.7 · no · the chunk paints nothing: no screenshot check or live-run goal reaches a renderer, and the windowed renderer is out of scope
stack-and-technologies#1.8 · no · the chunk parses no SVG and uses no GPU or geometry type, and the colours its contrast rows read are the computed styles Stylo gives (#1.3)
stack-and-technologies#1.9 · yes · the Enter and Space key tables and the select's arrow keys name keyboard-types, and with two keyboard-types versions in the graph the chunk's key code must name the workspace's crate, not the Dioxus prelude's
stack-and-technologies#1.10 · yes · the roles, names and states the snapshot, element-id, diff and act checks read come from the accessibility tree blitz-dom builds on accesskit 0.25
stack-and-technologies#1.11 · no · HTTP, reqwest, tokio, the HTTP cache and blitz-net's fetching are not reached: the stand boots offline, names no blitz_net, and its held load is an in-process NetProvider stand-in
stack-and-technologies#1.12 · no · the stand is a Dioxus app with no script, so Boa and its ScriptFetcher and timers are not reached
stack-and-technologies#1.13 · yes · the cells and flight booker tasks are Dioxus 0.7.3 components with `#[derive(Store)]` state, so their rsx and state code must honour that API
stack-and-technologies#1.14 · yes · serde_json reads the kept checks and the MCP requests a replayed agent check is sent as, so the recording and replay goal binds it; the image, WOFF and escaping crates are not reached
stack-and-technologies#1.15 · no · the chunk runs natively, and the wasm32 and Trunk builds of the examples are not reached
stack-and-technologies#1.16 · yes · tracing and tracing-subscriber write the tool_call span lines the cold-agent pipe counts wrong calls from
stack-and-technologies#1.17 · no · rusqlite and its migrations serve the browser's history database, which the chunk does not reach
stack-and-technologies#1.18 · no · the browser app's extras (nucleo suggestions, mimalloc, xdg-portal dialogs, the Android entry) are not reached
stack-and-technologies#1.19 · no · the rdme markdown viewer's comrak, pulldown-cmark and notify are not reached
stack-and-technologies#1.20 · yes · blitz-tests' dev-dependencies (the headless stand, escher-driver, test-that) and the unix-gated cli_flow target that runs flight.sh are the test surfaces the chunk edits
stack-and-technologies#1.21 · no · the Nix flake is not used on the dev host, and the chunk changes no toolchain or system package; its code's MSRV binds through #1.1
stack-and-technologies#1.22 · yes · ci-leg.sh's stand-repeat glob and its a11y leg names (stand_keyboard, stand_contrast, stand_act_select), and the cold-agent pipe's stdlib-only python judge and claude CLI, bind the reached scripts and test_cold_agent.py
stack-and-technologies#1.23 · yes · cargo fmt, clippy -D warnings, rustdoc -D warnings and the stand-repeat three-round zero-failure budget gate the chunk's code and every re-pinned stand check
