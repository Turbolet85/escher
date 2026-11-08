@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the new Rust in dioxus-native-dom, seven_guis and escher-driver must build at edition 2024 and within the workspace rust-version 1.91.0
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the chunk's manifest edits add in-repo and workspace-declared dependencies, and the in-repo pins and the workspace version this entity records are the form those edits must keep
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is read-only to the graph, which changes no style, and the stylesheet and selector matching are not reached
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: html5ever and xml5ever parse no markup in the graph or the headless Dioxus stand
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: the graph reads already-built nodes and changes no layout (not_needed item 6), and the taffy re-export rule lives in blitz-dom, which the chunk does not change
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: text layout, fonts and shaping are untouched, and the stand's values come from state, not from shaped text
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the graph paints nothing and no renderer or backend is changed
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the chunk's scope names no geometry, colour, GPU or SVG code
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the chunk names no winit, cursor or keyboard-types path, and the glob-prelude naming hazard is reached only by a keyboard_types path the new code does not write
@answer n=10 entity=stack-and-technologies#1.10 v=yes
why: the graph is keyed on the agent-visible AccessKit node that build_accessibility_tree builds into its TreeUpdate, so the accesskit 0.25 node types the new code uses bind
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the graph makes no fetch, so networking, the User-Agent and the HTTP cache are not reached (not_needed item 3)
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the stand is a Dioxus app with no script, so Boa is not reached (not_needed item 5)
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the registry reads link 4 through Readable::subscribers and ReactiveContext on Dioxus 0.7.3 with dioxus-core 0.7.10, which the scope names
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: escher-driver reads the kept checks and the live answer line a replay compares through serde_json, so the recorded graph step and the two kept checks reach it, while the answer itself stays hand-written in the json module
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: no wasm-bindgen, web-sys or Trunk page is touched, and the stand is headless
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the Stylo thread pool and parallel construction are not reached (not_needed item 9), and the tracing and test-that versions name no behaviour the new code must honour
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the browser app's persistence database is out of scope (not_needed item 1)
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the browser app's extras are out of scope (not_needed item 1)
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme markdown stack is out of scope (not_needed item 1)
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: the scope's stand checks, stand_replay and the two kept checks are blitz-tests integration tests, and this entity names blitz-tests' dev-dependencies on seven_guis and escher-driver along with the test framework the new tests use
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the chunk changes no flake, dev-shell or toolchain file, and the rust-version bound it would carry reaches the code through entity 1.1
@answer n=22 entity=stack-and-technologies#1.22 v=no standing=S6
why: the per-crate coverage floor it states is the convention S6 names, and the rest is CI the chunk changes no part of
@answer n=23 entity=stack-and-technologies#1.23 v=no standing=S3
why: the fmt, clippy and rustdoc -D warnings legs it states are the convention S3 names, and the audit, coverage and host tier are not reached by the chunk's own changes
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity reading runs on the dev host only with no CI leg and no scope name, so it is a re-check every change owes and not a rule over the chunk's units
