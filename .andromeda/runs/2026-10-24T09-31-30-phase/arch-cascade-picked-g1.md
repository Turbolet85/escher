# arch · picked accounting · group 1 of 7

## Picked
design-philosophy#1.1 · carried · the MSRV effort binds a take that carries Dioxus 0.8.0-alpha.3 or Stylo 0.23-pre, which the MSRV job reads at 1.91
design-philosophy#2.1 · carried · `fb297a38c` touches `config.rs` and `286874cc1` replaces upstream's side of `dioxus_document.rs`, so the provider config and the `DioxusDocument` entry shape must hold through the resolution
design-philosophy#3.1 · not in scope · the chunk writes no web semantics of its own; it takes upstream's spec-cited behaviour as upstream wrote it and adopts no new upstream API
design-philosophy#4.1 · carried · the oracle, the deferred inline phase in the conflicted `construct.rs`, the one scroll primitive in the conflicted `scrolling.rs`, versioned ids under `f102663b8` and Node's tree-only shape under `46d7cf993` each bind the take
design-philosophy#5.1 · carried · upstream's new test files land as blitz-tests integration targets under the headless harness; the WPT and browser lines are upstream-only surfaces
stack-and-technologies#1.1 · carried · the workspace `rust-version` "1.91.0" holds through the take (scope: MSRV stays 1.91)
stack-and-technologies#1.2 · carried · a take of `286874cc1` moves the in-repo dioxus-native and dioxus-native-dom pins with every Dioxus package, as one set
stack-and-technologies#1.3 · carried · Stylo 0.22.0 is the heart of the pin fork: upstream's v0.23-pre arrives only as a git dependency
stack-and-technologies#1.4 · carried · html5ever and markup5ever are coupled to Stylo's web_atoms and move only with Stylo, never alone
stack-and-technologies#1.5 · carried · taffy "0.15" from the registry is CARRY 1's ground: upstream names git Taffy `802e497b…`
stack-and-technologies#1.6 · not in scope · upstream's measured delta moves no Parley, skrifa or ICU4X pin
stack-and-technologies#1.8 · not in scope · upstream's measured delta moves no graphics pin (color, peniko, kurbo, wgpu, usvg, svgtypes)
stack-and-technologies#1.9 · carried · the two keyboard-types versions hang on Dioxus 0.7.3; a Dioxus 0.8 take, and with it `keyboard_compat.rs`'s retirement (CARRY 2), moves this line
stack-and-technologies#1.10 · carried · accesskit_unix 0.24.0 is the path RUSTSEC-2026-0386's tokio reaches, and the audit fix leaves the accesskit pins alone
stack-and-technologies#1.11 · carried · tokio "1.42" is the requirement the audit fix's semver-compatible lock update to ≥ 1.53.3 must stay inside
stack-and-technologies#1.13 · carried · Dioxus "0.7.3" against upstream's `0.8.0-alpha.3` is the pin fork's third side
stack-and-technologies#1.16 · carried · `fb297a38c` serialises parallel style traversals, which moves the role of Stylo's global `STYLE_THREAD_POOL`; tracing's pins are the sink re-check's ground
stack-and-technologies#1.18 · carried · `9eeebb302` makes `mimalloc` 0.1.48 the browser's default allocator and adds it to the WPT runner, a crate reviewed by hand and run through the licence audit
stack-and-technologies#1.20 · carried · the blitz-tests dev-dependency set takes upstream's new test files, and any crate one of them needs comes from the workspace
stack-and-technologies#1.21 · carried · `f9cbb2756` moves the flake's rust-bin pin to 1.91.0, which closes the recorded flake/`rust-version` drift if taken
stack-and-technologies#1.22 · carried · `test_blitz_tests_targets.py` pins the target list the new upstream tests join, and `cargo-deny@0.20.2` is the audit the chunk turns green
stack-and-technologies#1.23 · carried · the fast, doc and audit legs (advisories and licences over the whole workspace's graph) are the chunk's done-gates
