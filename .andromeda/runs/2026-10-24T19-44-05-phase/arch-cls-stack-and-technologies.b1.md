# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the reached test files are edition 2024 Rust, so the new rows must honour the 2024 rules; the MSRV pin is Not needed
stack-and-technologies#1.2 · no · the chunk moves no manifest and no in-repo pin, so the =0.3.0-beta.4 and 0.7.0 pins stay as they are; dependency pinning is Not needed
stack-and-technologies#1.3 · yes · stand_contrast reads colour, background and the hover and active states as Stylo computes them (Stylist, SnapshotMap, selector matching)
stack-and-technologies#1.4 · no · the stand rows build their DOM through Dioxus, not html5ever, and HTML parsing is Not needed
stack-and-technologies#1.5 · no · layout is Not needed; the rows read focus order, key effects and computed colours, not box geometry, and the both-modes invariant is the pipeline's, not taffy's
stack-and-technologies#1.6 · no · fonts are Not needed; no reached row reads glyphs, text metrics or editing, and the bundled font is the stand's fixed boot that the chunk does not change
stack-and-technologies#1.7 · no · painting and the headless screenshot are Not needed; contrast is read from computed styles, not pixels, and no reached row paints
stack-and-technologies#1.8 · no · graphics, colour and inline SVG are Not needed; the harness composites colour itself and no reached test names a graphics crate
stack-and-technologies#1.9 · yes · the Enter, Space, arrow, Home and End rows synthesise keyboard-types 0.8 keys, and a module that glob-imports the Dioxus prelude must name them ::keyboard_types; winit windowing is Not needed
stack-and-technologies#1.10 · yes · the re-read rows and the widened a11y checks read the tree blitz-dom builds on accesskit 0.25 nodes (labels, toggled, selected, invalid); the platform adapter is out of scope
stack-and-technologies#1.11 · no · networking and resource loading are Not needed; the stand boots offline and the fixtures' held stand-in replaces the net provider
stack-and-technologies#1.12 · no · JavaScript on Boa is Not needed; the stand and the reached rows run no script
stack-and-technologies#1.13 · yes · the stand is Dioxus 0.7.3 components, and its rows name controls by the dioxus-native-dom element ids and read the snapshot's state readings
stack-and-technologies#1.14 · no · image decoding, WOFF, URL encoding and the driver's JSON reader are not reached; resource loading and the driver session are Not needed
stack-and-technologies#1.15 · no · the a11y leg builds and runs native targets only; no wasm build or Trunk page is reached
stack-and-technologies#1.16 · no · parallel style traversal, rayon construct, tracing and the misc crates are not reached by the rows; test-that binds through entity 1.20
stack-and-technologies#1.17 · no · the browser's rusqlite history is outside the chunk, and the browser app is not reached
stack-and-technologies#1.18 · no · the browser app's extras (nucleo, mimalloc, the rfd portal, the Android entry) are outside the chunk
stack-and-technologies#1.19 · no · the rdme markdown app is outside the chunk; no reached file or leg runs it
stack-and-technologies#1.20 · yes · the reached stand files build on blitz-tests' dev-dependencies (the harness, seven_guis' stand, dioxus-native-dom, dioxus, test-that), and one target per file binds the widened checks
stack-and-technologies#1.21 · no · the Nix flake and its 1.91.0 shell are used by no leg; the chunk runs on rustup and the CI runner, and the flake pin is not reached
stack-and-technologies#1.22 · yes · the a11y job, the leg's --test list in ci-leg.sh (a merge surface), the pin in test_ci_workflows.py, apt-install.sh, the SHA-pinned actions and the Python 3.11 floor of the ci-scripts leg bind the widening
stack-and-technologies#1.23 · yes · fmt, clippy -D warnings, the doc leg's RUSTDOCFLAGS (in ci-leg.sh, a merge surface) and the stand-repeat zero-failure budget bind the reached test files and the re-recorded rows
