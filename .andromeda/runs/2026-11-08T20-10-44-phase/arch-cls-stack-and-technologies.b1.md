@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the workspace edition 2024 and rust-version 1.91.0 bound the new Rust code the chunk adds to dioxus-native-dom and escher-driver
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: workspace version and crate pins; the chunk moves no version or dependency
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling; the chunk reads the graph and adds no style path
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: HTML parsing; no parse path or HTML fixture changes
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy layout; the chunk changes no layout and keys nothing on taffy or dom node ids
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: font stack and metrics; this entity does not state the stand's pinned font and the chunk changes no text path
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: rendering backends; no painting is built and the headless replay takes no shot
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: graphics crates; the graph, replay and driver do not reach them
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: winit and keyboard-types; the new record keys on event names and names no keyboard type
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: accesskit tree; the accessibility adapter is untouched and the actionable rule is the graph's, not the tree's
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: networking; the command makes no request
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: Boa JavaScript; no script path is touched
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: states Dioxus's virtual DOM driving Blitz through dioxus-native-dom, the bridge whose DOM event path the chunk records (element, event) handlers on
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: serde_json is escher-driver's reader of kept checks and of the replay answer line that the new Check::from_bytes read and the key-by-key actions comparison use
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: WASM browser builds; the chunk does not reach them
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: parallel style traversal and tracing versions; the chunk adds no span, subscriber or parallel path
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: browser history database; the chunk does not touch the browser app
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: browser UI extras; not reached by the chunk
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: rdme markdown app; not reached by the chunk
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: blitz-tests dev-dependencies escher-driver (the driver session) and seven_guis (the headless stand) are where the owed replay test runs
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: Nix flake and dev-host toolchain; no toolchain pin moves
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: CI workflows and stdlib-only Python scripts; the chunk adds no CI job, script or workflow, and the existing capability-statement test is not a new instance of that convention
@answer n=23 entity=stack-and-technologies#1.23 v=no standing=S4
why: the cargo fmt, clippy -D warnings and rustdoc -D warnings gates run through ci-leg.sh, the convention S4 names
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: cognitive-complexity reading on the dev host alone, never in CI; a convention every change owes equally that no scope names
