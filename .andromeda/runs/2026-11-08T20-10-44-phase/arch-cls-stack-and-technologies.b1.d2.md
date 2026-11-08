@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: edition 2024 and rust-version 1.91.0 are a bound the new Rust code of the chunk must stay inside
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: workspace version and in-repo crate pins are release metadata the chunk moves none of (not_needed dependency pinning and release versioning)
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is not reached; the chunk changes no style or selector path
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: HTML parsing with html5ever and xml5ever is not reached; the chunk changes no parse path
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: Taffy layout is not reached; the new code uses no Taffy type or layout call
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: text layout, fonts and font metrics are not reached; the stand's pinned font is not stated in this entity
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: rendering abstraction and backends are not reached; no painting is built
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: geometry, colour, GPU and SVG parsing are not reached by the chunk
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the winit window shell is not reached and the chunk names no keyboard type; events are recorded by name only
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the accessibility adapter and platform-tree refresh are not reached
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: networking, blitz-net and outbound hosts are not reached; the command makes no request
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the Boa JavaScript path in blitz-vibey-script is not reached
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the scope's Dioxus DOM bridge event path into the vdom runs on the Dioxus 0.7.3 crates this entity pins, which the new code calls
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: serde_json is escher-driver's sole reader of kept checks and of the answer line a replay compares, which Check::from_bytes and Check::replay use
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the WASM and Trunk builds of the examples are not reached
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the rayon, style-traversal and test-that version pins state no rule over the chunk's code
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the browser app's rusqlite history persistence is not reached
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: browser app extras (nucleo, mimalloc, rfd, Android entry) are not reached
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme markdown app and its file watching are not reached
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: blitz-tests depends on the seven_guis headless stand and on escher-driver, the session the owed replay test runs through, and the cli_flow shell tests run the escher-session binary the chunk extends
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Nix flake and dev-host toolchain are not reached; the toolchain bound is carried by #1.1
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the chunk edits .github/scripts/test_capability_statement.py, which the ci-scripts leg runs through ci-leg.sh on Python 3.11 or later with stdlib only
@answer n=23 entity=stack-and-technologies#1.23 v=no standing=S4
why: the fmt, clippy and rustdoc gates run through ci-leg.sh, a convention every change owes; the per-crate coverage floor is S6
@answer n=24 entity=stack-and-technologies#2.1 v=yes
why: the cognitive-complexity ceiling of 15 is a bound the chunk's new functions must stay inside on the dev-host reading
