# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · rust-version "1.91.0" (Cargo.toml:41) is an inherited pin the take may move or keep, in the merge-surface manifest
stack-and-technologies#1.2 · yes · the in-repo crates' =0.3.0-beta.4 pins and the dioxus-native, debug_timer and accesskit_xplat lines sit in the merge-surface Cargo.toml [workspace.dependencies] (lines 44-60); the release version alone is Not needed
stack-and-technologies#1.3 · yes · Stylo 0.22.0 with selectors and cssparser (Cargo.toml:64-71) is an inherited pin the take may move to git Stylo or keep
stack-and-technologies#1.4 · yes · the html5ever family pins (Cargo.toml:73-76) are the parser coupling the one-set pin decision carries
stack-and-technologies#1.5 · yes · taffy "0.15" from the registry (Cargo.toml:104-114) is an inherited pin the take may move to git Taffy or keep
stack-and-technologies#1.6 · yes · the registry parley "0.12" and skrifa "0.44" pins (Cargo.toml:115-118) are the parley and skrifa couplings the pin decision names
stack-and-technologies#1.7 · yes · the anyrender pins (Cargo.toml:120-128) are a merge-surface rule, and anyrender_vello_cpu paints the screenshot pixels the chunk re-measures
stack-and-technologies#1.8 · yes · svgtypes "0.16.1" and usvg "0.48.1" (Cargo.toml:130-137) are the svgtypes-usvg coupling the pin decision names
stack-and-technologies#1.9 · yes · winit "=0.31.0-beta.3" is the exact beta the take moves to beta.4 with its clipboard move, and arboard leaves the manifest (Cargo.toml:139-146)
stack-and-technologies#1.10 · yes · accesskit "0.25" (Cargo.toml:139-146) is a merge-surface pin, and accesskit_unix 0.24.0 is the path of the zbus advisories deny.toml ignores and the audit re-reads
stack-and-technologies#1.11 · yes · the networking pins (Cargo.toml:148-156) are a merge-surface rule, and tokio past RUSTSEC-2026-0386 is an input the audit re-reads; blitz-net's behaviour stays Not needed
stack-and-technologies#1.12 · yes · boa_engine, boa_runtime and boa_gc "0.22" (Cargo.toml:179-182) are the Boa API the runtime.rs conflict and the CSSOM accessors depend on
stack-and-technologies#1.13 · yes · the Dioxus 0.7.3 crates (Cargo.toml:85-102) are the pin the take moves to 0.8.0-alpha.3 or keeps, and they drive the dioxus-native-dom bridge
stack-and-technologies#1.14 · yes · the image, png and serde pins (Cargo.toml:158-165) are a merge-surface rule, and png 0.18 is the decoder the blitz-tests screenshot checks use
stack-and-technologies#1.15 · yes · the wasm-bindgen, web-sys and console_error_panic_hook pins (Cargo.toml:171-177) are a merge-surface rule a take moves under the lock
stack-and-technologies#1.16 · yes · the misc pins (Cargo.toml:184-199) are a merge-surface rule, and parallel-construct's rayon and thread_local uses (document.rs:74-75, a merge surface) bind it too
stack-and-technologies#1.17 · no · the browser app's history database and its rusqlite persistence are named Not needed, and apps/browser/persistence/Cargo.toml is not reached
stack-and-technologies#1.18 · yes · apps/browser enables dioxus-native's clipboard feature by name (apps/browser/Cargo.toml:47-54), the value the clipboard move's default-list change must leave on
stack-and-technologies#1.19 · no · rdme's manifest (apps/readme/Cargo.toml) is not reached and names no value the scope moves, so its blitz-shell and reqwest use is a workspace build every change owes
stack-and-technologies#1.20 · yes · the root dev-dependencies (Cargo.toml:298-303) are a merge-surface rule, and blitz-tests' test-that, png, anyrender_vello_cpu and cursor-icon (for pointer_leave's ShellProvider, tests/blitz-tests/tests/pointer_leave.rs:60-64) are what the re-run checks build against
stack-and-technologies#1.21 · yes · the flake's rust-bin "1.91.0" must follow rust-version if the take moves it (flake.nix:31), a fork the scope leaves open; the packaging itself is Not needed
stack-and-technologies#1.22 · yes · the ci-leg.sh legs the chunk re-runs, test_blitz_tests_targets.py reading the target pin, cargo-deny@0.20.2 for the audit, and publish-browser.yml's Rust and dioxus-cli pins under its github.repository guard
stack-and-technologies#1.23 · yes · the fast, doc, audit and coverage legs the scope names, the clippy leg's blitz-dom feature line included; the fmt and rustdoc lines are standing S3 and S4 and not the reason
stack-and-technologies#2.1 · no · a dev-host cognitive-complexity reading (ceiling 15) that no leg, Need or re-run in the scope names, so a re-check every change owes
