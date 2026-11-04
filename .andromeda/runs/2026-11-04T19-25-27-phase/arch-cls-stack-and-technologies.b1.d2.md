@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: probe test and example code is Rust that must compile under edition 2024 and rust-version 1.91.0.
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the probe adds no crate and moves no pin, so inherited package metadata and in-repo pins stay untouched; the Dioxus pin it must hold is in #1.13.
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is used unchanged by the stand, no scope item names its behaviour, and the probe reads no style.
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: html5ever and xml5ever parse HTML and XML documents, which the Dioxus-driven stand does not load and no scope item names.
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy layout is used unchanged and the probe reads no geometry (not-needed item 14), so taffy's API and the blitz-dom public-API note do not reach the chunk.
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the text stack is used unchanged; the stand's one bundled font is set by the stand and not stated here, and the probe reads no text.
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: painting and renderer backends are not-needed item 5, and the probe reads no paint output.
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: geometry, colour, GPU and SVG crates are used unchanged, no link reads geometry, and painting is out of scope.
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: the key-handler task the hazard probe drives needs key types, and a module glob-importing dioxus::prelude::* names keyboard_types as the workspace's 0.8 crate, the two-version clash this entity states; the winit shell read of need 10 is also here.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the accesskit tree and platform adapters are not-needed item 12, and the accessibility feature named by the scope gates the snapshot modules, not the accesskit stack stated here.
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: networking crates are not-needed item 6, and the offline boot the scope names is a stand property this entity does not state.
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: Boa and page scripts are not-needed item 2; the probe reads a Dioxus app, not page scripts.
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the Dioxus 0.7.3 crates are what the probe reads through the bridge and the runtime (needs 1, 3 and 4), with dioxus-native and dioxus-native-dom driving Blitz.
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the media crates are used unchanged by the stand, and the serde_json and png roles stated here belong to escher-driver's own work, which no scope item names.
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: WASM and Trunk builds are not part of the headless native stand the probe runs.
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: parallel style traversal and STYLE_THREAD_POOL are not-needed item 11 (no style_threading), and the version pins state nothing the probe must act on.
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the browser history database is not-needed item 4.
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: browser app extras are not-needed item 4.
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme markdown app is not-needed item 4.
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: the blitz-tests dev-dependencies (harness, dioxus-native-dom, the seven_guis stand, escher-driver) are the test target the probe is built into (needs 5 to 7).
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Nix flake and host toolchain are the build environment; the probe adds no tool or pin, and no scope item names them.
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: scope item 18 names the test leg, the stand-repeat zero-failure budget and the coverage floors that probe tests must pass, and these CI legs are defined here.
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: scope item 18 names the clippy, fmt and rustdoc gates and the stand-repeat budget and coverage floors that probe code must pass, so the standing S3 drop does not apply.
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity reading is a dev-host-only check that no scope item names and no CI leg runs, so it is a code convention every change owes equally.
