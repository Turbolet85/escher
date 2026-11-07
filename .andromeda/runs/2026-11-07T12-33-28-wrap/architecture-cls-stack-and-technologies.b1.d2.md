@answer n=1 entity=stack-and-technologies#1.1 v=no
why: the session moves no Rust edition, toolchain or workspace manifest line
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the workspace version and in-repo crate pins sit in Cargo.toml, which the session leaves unmoved
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is untouched by a bridge change that only reaches dioxus-native-dom's render path
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: html5ever and xml5ever parsing are untouched by the session
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy layout is untouched, since the mutation buffer records DOM mutations and not layout
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: no text, font or ICU dependency moved, so the text row holds
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: no renderer or backend code changed, so the rendering row holds
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: no geometry, colour or GPU crate moved, so the graphics row holds
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: no winit or keyboard-types line moved and the session adds no keyboard-types use
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the accessibility tree builder is unchanged and the buffer sits outside every accessibility gate
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: no networking crate or fetch path changed
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: no boa crate or script path moved
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the bridge still renders the Dioxus virtual DOM into the Blitz document with Dioxus 0.7.3 and both manifests unchanged
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: no media crate or decode path moved
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: no WASM crate or Trunk setup moved
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: no parallelism or misc crate moved and the session adds none
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: no rusqlite or persistence crate moved
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: no browser app manifest or extra crate moved
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: no markdown crate or pulldown-cmark lock line moved
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the blitz-tests dev-dependencies and the test files this row cites are unchanged, and the row names none of the probe's tests
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the flake, rust-bin pin and dev-host measurement are untouched
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: no CI job, step, leg or action changed
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: the dioxus-native-dom coverage figure is dated to the 2026-10-14 chunk and the 83 % floor is unchanged
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity tool and its dated host reading are untouched
