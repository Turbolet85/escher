# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · rust-version 1.91.0 in Cargo.toml, a conflict surface; the MSRV stays 1.91 in both manifests (Needs, dependency and workspace surface)
stack-and-technologies#1.2 · yes · workspace version and the in-repo `=0.3.0-beta.4` pins live in Cargo.toml, a conflict surface, so a resolution can move them
stack-and-technologies#1.3 · yes · stylo 0.22.0, selectors 0.41.0 and cssparser 0.38.0 are the coupled pins the take carries as one set (Needs); Cargo.toml:64-71 is a conflict surface and stylo.rs is reached
stack-and-technologies#1.4 · yes · markup5ever, html5ever and xml5ever are the parser crates the Needs match to Stylo's web_atoms; Cargo.toml:73-76 is a conflict surface
stack-and-technologies#1.5 · yes · taffy 0.15 is a coupled pin (Needs), the root-manifest no-git-dependency rule applies, and the lib.rs:96-97 taffy re-export comment this entity heads is the orphan the chunk settles
stack-and-technologies#1.6 · yes · parley, skrifa and the ICU4X lines at Cargo.toml:115-118 and 202-206 sit on a conflict surface that a merge of upstream's pin set can move; text.rs is reached
stack-and-technologies#1.7 · yes · anyrender and wgpu_context pins at Cargo.toml:120-128 sit on a conflict surface a merge can move
stack-and-technologies#1.8 · yes · color, peniko, kurbo, wgpu, usvg, svgtypes, euclid and app_units pins at Cargo.toml:130-137 and 78-83 sit on a conflict surface a merge can move
stack-and-technologies#1.9 · yes · winit `=0.31.0-beta.3` is a coupled pin the Needs name, and the take's beta.4 option moves this line; Cargo.toml:139-146 is a conflict surface
stack-and-technologies#1.10 · yes · accesskit 0.25 pins at Cargo.toml:139-146 sit on a conflict surface a merge can move; the a11y target rests on them
stack-and-technologies#1.11 · yes · url, http, tokio, reqwest and http-cache pins at Cargo.toml:148-156 sit on a conflict surface a merge can move
stack-and-technologies#1.12 · yes · boa_engine, boa_runtime and boa_gc pins at Cargo.toml:179-182 sit on a conflict surface; the script-engine surface in bootstrap.rs and events.rs is reached
stack-and-technologies#1.13 · yes · Dioxus 0.7.3 crates are a coupled pin the Needs name (Cargo.toml:85-102 is a conflict surface); the Dioxus bridge in dioxus_document.rs is reached
stack-and-technologies#1.14 · yes · image, png, serde and serde_json lines at Cargo.toml:158-165 sit on a conflict surface; png 0.18.1 is consumed by blitz-tests and escher-driver
stack-and-technologies#1.15 · yes · wasm-bindgen, wasm-bindgen-futures, web-sys and related pins at Cargo.toml:171-177 sit on a conflict surface a merge can move
stack-and-technologies#1.16 · yes · rustc-hash, tracing, test-that and related pins at Cargo.toml:184-199 and 78-83 sit on a conflict surface; the test-that 0.6.0 pin is a bump the take may carry
stack-and-technologies#1.17 · no · rusqlite and rusqlite_migration are declared in apps/browser/persistence/Cargo.toml, which the scope does not reach (Not needed: the browser app)
stack-and-technologies#1.18 · no · browser-app extras in apps/browser/Cargo.toml and the Android activities are outside the scope, which names no change in apps/ (Not needed)
stack-and-technologies#1.19 · no · rdme's markdown and watch dependencies in apps/readme/Cargo.toml are outside the scope (Not needed: the rdme app)
stack-and-technologies#1.20 · yes · the blitz-tests dev-dependency set and the root dev-dependencies at Cargo.toml:298-303 are what the take's upstream test files and the stand checks draw on; the wpt runner part does not bind
stack-and-technologies#1.21 · no · the Nix flake and the host toolchain are read by no scope part (Not needed: the flake's packaging); rust-version stays 1.91 through stack-and-technologies#1.1
stack-and-technologies#1.22 · yes · the matrix_test free-disk `if:` edit in ci.yml, ci-leg.sh, the tomllib Python 3.11 requirement of test_blitz_tests_targets.py and test_ci_workflows.py over the edited file bind; its uses-SHA line is standing S7 and is not carried here
stack-and-technologies#1.23 · yes · the fast, doc, audit and stand-repeat legs of ci-leg.sh are the proof gates and the free-disk watch the chunk runs and settles
stack-and-technologies#2.1 · no · rust-code-analysis-cli is a dev-host tool that no CI leg runs and no scope part reads
