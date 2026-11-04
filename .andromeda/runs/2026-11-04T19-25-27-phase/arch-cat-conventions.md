@carried n=1 label=architecture/conventions fact=fct-kjxsappk
why: Every probe check or example the chunk adds must pass the workspace fmt and clippy gates, and upstream's blitz-dom clippy line, as the ci-leg.sh legs run them.
quote: `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — each run as its `.github/scripts/ci-leg.sh` leg in CI and on the dev host
@carried n=1 run=2 label=architecture/conventions fact=fct-kjxsappk
quote: `clippy::expect_used` / `clippy::unwrap_used` allowed locally next to a justification
@carried n=2 label=architecture/conventions fact=fct-zd5gb466
why: A probe check of the bridge's element-id and listener reads must sit in the bridge tests child module, reaching private fields with no new `pub`, constructor or `cfg(test)` shim, so the probe changes no bridge surface.
quote: dioxus-native-dom's bridge tests, declared at the end of `dioxus_document.rs` as `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;`, a child of the module under test so they reach its private fields with no constructor, `pub` or `cfg(test)` shim added
@carried n=3 label=architecture/conventions fact=fct-zd5gb466
why: Each probe integration check in tests/blitz-tests must be one test-target file covering one behavior with a `//!` doc, run in both layout modes, and must first assert that its fixture produces the condition it reads.
quote: each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug
@carried n=3 run=2 label=architecture/conventions fact=fct-zd5gb466
quote: scenarios run in both layout modes `for incremental in [false, true]`
@carried n=3 run=3 label=architecture/conventions fact=fct-zd5gb466
quote: tests first assert the fixture produces the condition under test
@carried n=3 run=4 label=architecture/conventions fact=fct-zd5gb466
quote: blitz-tests keeps one test target per file directly under `tests/` — its `[package]` sets no `autotests` —
@carried n=4 label=architecture/conventions fact=fct-zd5gb466
why: A probe check on the seven_guis stand (Counter or CRUD, a served task) must take the shared stand tables and helpers from `common` and, when it holds a `Session`, from `session_common`, instead of restating them.
quote: the seven_guis stand checks state the tables and helpers they share once, in the module `tests/blitz-tests/tests/common/mod.rs`, which a check reads by declaring `mod common;`
@carried n=4 run=2 label=architecture/conventions fact=fct-zd5gb466
quote: keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;`
@carried n=5 label=architecture/conventions fact=fct-cnla3gly
why: A fixture page or boot the probe shares between checks must live in the fixtures module, and a page with only one reader must stay in its own check's file.
quote: A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file
@carried n=6 label=architecture/conventions fact=fct-aqtiieoj
why: Every probe module, test file or example must open with a `//!` doc, and an example must say what it demonstrates.
quote: modules and files open with `//!` docs
@carried n=6 run=2 label=architecture/conventions fact=fct-aqtiieoj
quote: examples open with a `//!` comment describing what they demonstrate
@carried n=7 label=architecture/conventions fact=fct-mimrhyxw
why: Any dependency the probe adds, such as a dev-dependency on a Dioxus runtime crate it reads, must be declared once at the workspace and consumed as `{ workspace = true }`, and the seven_guis entry must keep its renderer feature.
quote: dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`
@carried n=7 run=2 label=architecture/conventions fact=fct-mimrhyxw
quote: except the `seven_guis` path entry, which keeps its default `hybrid` renderer feature because `dioxus-native` raises `compile_error!` with no renderer
@carried n=8 label=architecture/conventions fact=fct-2wv4ux4q
why: A Dioxus example the probe writes, with state that a handler writes and another node reads, must follow the example shape and the signal and store hooks.
quote: Dioxus examples declare `fn main() { dioxus_native::launch(app); }` and `fn app() -> Element`
@carried n=8 run=2 label=architecture/conventions fact=fct-2wv4ux4q
quote: state via `use_signal`, `use_store`, `use_memo`, `use_future`
@carried n=9 label=architecture/conventions fact=fct-h6rtvaqy
why: An «exposed with a cost» verdict that names an escher-side change inside dioxus-native-dom must state its boundary as crate-internal visibility, and nothing the probe keeps may widen it.
quote: crate internals are `pub(crate)` / `pub(super)`
@carried n=10 label=architecture/conventions fact=fct-bhxvo7z2
why: The probe must not route anything it reads (a component name, a handler, a signal's value) through the bridge's crate-local trace macro, because nothing it reads may be logged.
quote: `dioxus-native-dom` defines a crate-local `trace!` macro expanding to `tracing::debug!` under `tracing`
@binding n=1
**Tests: bridge child module ↔ the body's Dioxus DOM bridge contract**: a probe check reads `DioxusDocument`'s private element-id and listener state as a bridge_tests child, so the public bridge surface (`element_id`, `element_ids`, the snapshot) gains no item (architecture/conventions fct-zd5gb466) (architecture fct-srb7slak)
@binding n=2
**Tests: both layout modes ↔ the body's one incremental pipeline**: a verdict that layout touches must hold in both modes, `for incremental in [false, true]`, because both modes must produce identical layouts (architecture/conventions fct-zd5gb466) (architecture fct-57klyykw)
@binding n=3
**Feature gating: the bridge trace! macro ↔ the body's bridge no-log rule**: no probe value may reach a log or event, because no log or event may carry the snapshot, its text or a diff (architecture/conventions fct-bhxvo7z2) (architecture fct-tev7ieys)
@binding n=4
**Manifests ↔ the body's Inherited Defaults Dioxus pin**: a dependency the probe adds goes through `[workspace.dependencies]`, so Dioxus stays at "0.7.3" with no pin moved (architecture/conventions fct-mimrhyxw) (architecture fct-656exhoj)
@binding n=5
**Formatting and lints ↔ the body's Code quality line**: probe code must pass the same ci-leg.sh fmt, clippy and rustdoc gates (architecture/conventions fct-kjxsappk) (architecture fct-c7w2iblv)
