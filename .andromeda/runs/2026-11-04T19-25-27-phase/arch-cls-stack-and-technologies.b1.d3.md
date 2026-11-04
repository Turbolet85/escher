@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the probe's Rust code and tests are written in edition 2024 under rust-version 1.91.0, so they must compile under both.
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the probe moves no version or pin, and in-repo crate versions belong to release versioning, which the chunk does not reach.
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the UI-graph links read the bridge and the listener map, not Stylo's computed style or its selector matching.
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: the Dioxus-built DOM is never parsed from HTML or XML, so html5ever and xml5ever are not reached.
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: taffy produces the layouts that item 8 compares across both layout modes, so the both-mode verdict runs reach it.
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: fonts are a CSS and layout decision the chunk does not make, and the probe reads no text geometry.
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: painting and renderer backends are outside the probe, which runs no paint path.
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the colour, geometry and SVG crates serve painting and styling, which the probe does not read.
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: probe code that handles a key handler or imports dioxus::prelude must honour the second keyboard-types version the graph carries, which the entry names as ::keyboard_types.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the probe reads the dioxus-native-dom snapshot read-only and never the accesskit tree or its platform adapters.
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the networking and async crates are outside the probe, which boots the stand offline and fetches nothing.
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: Boa runs page scripts, and the probe reads a Dioxus app rather than page scripts.
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the four links are read through the Dioxus 0.7.3 crates this entry pins, and the probe must not move that pin.
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the media and JSON crates are not what the probe reads, and the wire and snapshot stay as they are under the nothing-crosses rule.
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: WASM and Trunk builds are browser-target work, and the probe runs natively on the headless stand.
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: parallel style traversal is unset on the headless stand, and the miscellaneous crates are not read by the probe.
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: rusqlite and history persistence belong to the browser app, which the chunk does not reach.
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the browser app extras are outside the probe, which reads a Dioxus app on the stand.
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the markdown app rdme is outside the probe.
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: the probe's test target lives in blitz-tests, whose dev-dependencies name the seven_guis stand, escher-driver and dioxus that the probe runs on.
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Nix flake and the dev-host toolchain build the workspace but add no rule the probe's code must honour beyond the rust-version already held by entry 1.1.
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the test, stand-repeat and coverage-floor legs of the CI workflows are the gates item 18 names for any probe test.
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: item 18 names the fmt, clippy and rustdoc gates, the per-crate coverage floors and the stand-repeat budget, so the scope names them and the standing rule does not drop this entry.
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity ceiling is a host-only function reading that every change owes equally, and the scope does not name it.
