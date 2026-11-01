# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · manifests, lock and deny.toml are byte-identical and no cited file is reached
stack-and-technologies#1.2 · no · Cargo.toml is byte-identical, so no workspace version or crate pin moved
stack-and-technologies#1.3 · yes · stale · document.rs:63-71 and :479 and stylo.rs:14-20 are cited, both reached merge surfaces whose digits move
stack-and-technologies#1.4 · yes · stale · lib.rs:101-104 is cited in a reached file, and the retired comment at lib.rs:96-97 shifts its lines
stack-and-technologies#1.5 · yes · contradict · the comment at lib.rs:96-97 that the entry names as heading no pub use is retired, so that citation no longer holds
stack-and-technologies#1.6 · yes · stale · net.rs:342-371 is cited in a merge surface, and lib.rs:110 and :140-166 sit in a reached file whose digits move
stack-and-technologies#1.7 · no · cites examples, dioxus_renderer.rs and blitz/src/lib.rs, none reached; anyrender and vello pins unchanged
stack-and-technologies#1.8 · yes · stale · resolved_style.rs:63 is cited and is a reached merge surface, so its digits move
stack-and-technologies#1.9 · no · cites blitz-shell files, which are not reached; winit and keyboard-types pins unchanged
stack-and-technologies#1.10 · no · cites blitz-dom accessibility.rs, which is not reached; accesskit pins unchanged
stack-and-technologies#1.11 · yes · stale · blitz-net/src/lib.rs:1-3 is cited and is a reached merge surface whose error arm and waker moved
stack-and-technologies#1.12 · no · cites blitz-vibey-script's Cargo.toml and src/lib.rs, neither reached; Boa pins unchanged and the new bootstrap script runs on the same engine
stack-and-technologies#1.13 · no · cites the dioxus-native and dioxus-native-dom manifests, which are not reached; Dioxus 0.7.3 pins unchanged
stack-and-technologies#1.14 · yes · stale · net.rs:576-604 is cited and is a reached merge surface, so its digits move
stack-and-technologies#1.15 · no · cites wasm example and manifest files, none reached
stack-and-technologies#1.16 · yes · stale · stylo.rs:28 and :156-165 and document.rs:77-78 are cited, all reached merge surfaces whose digits move
stack-and-technologies#1.17 · no · cites the browser persistence manifest, and nothing under apps/ is reached
stack-and-technologies#1.18 · no · cites apps/ manifests and sources, and nothing under apps/ is reached
stack-and-technologies#1.19 · no · cites apps/readme files and Cargo.lock:6505, none reached and the lock is byte-identical
stack-and-technologies#1.20 · no · cites blitz-tests manifest and test files, none reached; the new target is not a dev-dependency and no pin moved
stack-and-technologies#1.21 · no · cites flake.nix and the root manifest, none reached; toolchain pin unchanged
stack-and-technologies#1.22 · yes · stale · ci.yml, ci-leg.sh and test_ci_workflows.py are reached and the entry cites their lines (ci.yml:311-316 and others), so the digits move
stack-and-technologies#1.23 · yes · stale · the fast leg now ends with the embolden step after ci-scripts, and the cited ci-leg.sh lines move
stack-and-technologies#2.1 · no · cites only the dev host's rust-code-analysis reading, and no reached file is cited
