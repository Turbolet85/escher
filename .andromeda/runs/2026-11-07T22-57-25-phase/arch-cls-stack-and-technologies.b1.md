@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the Rust edition 2024 and rust-version 1.91.0 bound the new code of dioxus-native-dom, seven_guis and escher-driver
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the in-repo exact pins and the dioxus-native-dom version line govern any in-repo dependency line the chunk adds
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is not touched, since the graph reads already-built nodes and changes no style
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: html5ever and xml5ever parsing is not reached, the read works on the Dioxus virtual DOM and parses no markup
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy layout is not reached, the graph changes no layout and names no taffy type
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the text and font stack is not reached, the graph carries no text measure or font value
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the rendering backends are not reached, the chunk paints nothing
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the graphics crates are not reached, the graph reads no colour, geometry or GPU value
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: windowing and input are not reached, the chunk names no keyboard_types type and a click reading is not a keyboard event
@answer n=10 entity=stack-and-technologies#1.10 v=yes
why: accesskit 0.25 supplies the agent-visible node the graph is keyed on, under the accessibility feature
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: networking is not reached, the graph makes no fetch
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the Boa script engine is not reached, the stand is a Dioxus app with no script
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the Dioxus 0.7.3 crates are the vdom walk, Readable::subscribers and ReactiveContext that links 1 to 4 read
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: serde_json is escher-driver's reader of MCP requests and kept checks, so the graph answer is written by hand and only reading uses it
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the WASM browser builds of examples are not reached, the chunk adds no build target and no wasm-only code
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the parallelism and misc crates are not reached, the chunk spawns no thread and names no rayon or thread-local use
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the browser app database is not reached, it is browser history persistence only
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the browser app extras are not reached, they are browser UI, fuzzy suggestions and the Android entry
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme Markdown app is not reached, the graph is read on the headless stand and the driver
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: blitz-tests hosts stand_replay over the two kept checks and depends on seven_guis and escher-driver, so its dev-dependency set binds the chunk's tests
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Nix flake and the dev host pin no toolchain the chunk changes, and the chunk's rust-version bound is already carried by entity 1.1
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the CI script test_blitz_tests_targets.py reads the blitz-tests manifest, so a test target the chunk adds there must be registered in it
@answer n=23 entity=stack-and-technologies#1.23 v=no standing=S3
why: the fmt, clippy and rustdoc legs of ci-leg.sh, with the coverage floor and stand-repeat gates, are a convention every change owes
@answer n=24 entity=stack-and-technologies#2.1 v=yes
why: the cognitive-complexity ceiling of 15 on the dev host is a bound each new function of the graph reader and the registry must stay inside
