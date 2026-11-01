# Classified — architecture · stack-and-technologies.b1

## Answers

stack-and-technologies#1.1 · no · the manifests are byte-identical and no file the entry cites (Cargo.toml, apps, examples) is reached
stack-and-technologies#1.2 · no · the workspace version and in-repo crate pins sit in Cargo.toml, which holds no diff
stack-and-technologies#1.3 · yes · stale · packages/blitz-dom/src/document.rs:63-71 and :479 and stylo.rs:14-20 are cited in reached merge surfaces, so their line digits can move
stack-and-technologies#1.4 · yes · stale · the packages/blitz-dom/src/lib.rs:101-104 citation sits in a reached file whose lines move above it (the orphan comment at lines 96-97 is gone)
stack-and-technologies#1.5 · yes · stale · lib.rs:96-127 and the lib.rs:96-97 orphan comment are cited in reached lib.rs, the comment is retired, and the taffy pin itself is unchanged
stack-and-technologies#1.6 · yes · stale · net.rs:342-371 (wuff) is cited in a reached merge surface, and lib.rs:110 and :140-166 are in a reached file whose digits move
stack-and-technologies#1.7 · no · the anyrender pins and the cited examples, dioxus_renderer.rs and blitz/src/lib.rs paths are not reached
stack-and-technologies#1.8 · yes · stale · resolved_style.rs:63 is cited for the euclid geometry use and is a reached merge surface
stack-and-technologies#1.9 · no · blitz-shell and accesskit_xplat are not reached and the winit and keyboard-types pins are unchanged
stack-and-technologies#1.10 · no · accessibility.rs and the accesskit pins are not reached and no manifest moved
stack-and-technologies#1.11 · yes · stale · packages/blitz-net/src/lib.rs:1-3 is cited in a reached merge surface, so its lines can move
stack-and-technologies#1.12 · no · the Boa pins are unchanged and blitz-vibey-script lib.rs:1-6 is not reached; the new URLSearchParams global is a bootstrap addition under an engine that still runs `<script>`
stack-and-technologies#1.13 · no · the Dioxus pins are unchanged and the cited dioxus-native and dioxus-native-dom Cargo.toml lines are not reached
stack-and-technologies#1.14 · yes · stale · packages/blitz-dom/src/net.rs:576-604 is cited in a reached merge surface
stack-and-technologies#1.15 · no · the cited wasm example files are not reached and examples/seven_guis holds no diff
stack-and-technologies#1.16 · yes · stale · stylo.rs:28 and :156-165 and document.rs:77-78 are cited in reached merge surfaces, so those digits can move
stack-and-technologies#1.17 · no · the browser persistence crate and its manifest are not reached (nothing under apps/ is reached)
stack-and-technologies#1.18 · no · nothing under apps/ is reached and the browser extras' pins are unchanged
stack-and-technologies#1.19 · no · apps/readme is not reached and its pins are unchanged
stack-and-technologies#1.20 · no · the blitz-tests and wpt manifests are unchanged and the cited test files are not reached; the new blitz-tests target adds no dependency
stack-and-technologies#1.21 · no · the flake and toolchain pins are unchanged and the cited build files are not reached
stack-and-technologies#1.22 · yes · stale · ci.yml, ci-leg.sh and test_ci_workflows.py are reached and their cited line digits (ci.yml:311-316 and :392, ci-leg.sh:1-6, test_ci_workflows.py:15) move
stack-and-technologies#1.23 · yes · contradict · the `fast` leg's listed order (fmt → clippy → test → ci-scripts) is no longer complete, since `embolden` now runs last; the ci-leg.sh citations move too
stack-and-technologies#2.1 · no · the cognitive-complexity reading cites only an escher-0.3.0 chunk report and no file the change reaches
