@compare side=phase doc=architecture marker=2026-11-04-dioxus-runtime-probe e_items=71 held=33 lost_classify=3 lost_strong=6 beside=- cascade_only=15 listed=5 body=11 unplaced=0 held_ids=conventions#8.1,cross-cutting-patterns#1.1,cross-cutting-patterns#3.1,cross-cutting-patterns#5.1,cross-cutting-patterns#6.1,design-philosophy#1.1,design-philosophy#2.1,design-philosophy#4.1,design-philosophy#5.1,established-decisions#15.1,established-decisions#25.1,existing-scopes#1.23,existing-scopes#1.31,existing-scopes#1.32,existing-scopes#2.1,existing-scopes#3.2,occupied-resources#1.1,occupied-resources#11.1,occupied-resources#5.1,occupied-resources#7.2,occupied-resources#8.1,project-intent#1.0,stack-and-technologies#1.13,stack-and-technologies#1.20,stack-and-technologies#1.23,stack-and-technologies#1.9,standard-contracts#1.1,standard-contracts#15.1,standard-contracts#17.1,standard-contracts#28.1,standard-contracts#28.2,standard-contracts#33.1,standard-contracts#4.1 only_ids=cross-cutting-patterns#2.1,established-decisions#11.3,established-decisions#29.1,established-decisions#5.1,existing-scopes#1.26,occupied-resources#12.1,occupied-resources#4.2,occupied-resources#9.1,standard-contracts#16.1,standard-contracts#19.1,standard-contracts#19.2,standard-contracts#20.1,standard-contracts#21.1,standard-contracts#29.3,standard-contracts#30.1
@compare_item n=1 part=lost how=classify entity=conventions#2.1 label=architecture/conventions fact=fct-mimrhyxw
@carried n=7 label=architecture/conventions fact=fct-mimrhyxw
why: Any dependency the probe adds, such as a dev-dependency on a Dioxus runtime crate it reads, must be declared once at the workspace and consumed as `{ workspace = true }`, and the seven_guis entry must keep its renderer feature.
quote: dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`
@carried n=7 run=2 label=architecture/conventions fact=fct-mimrhyxw
quote: except the `seven_guis` path entry, which keeps its default `hybrid` renderer feature because `dioxus-native` raises `compile_error!` with no renderer
@compare_item n=2 part=lost how=classify entity=conventions#5.1 label=architecture/conventions fact=fct-h6rtvaqy
@carried n=9 label=architecture/conventions fact=fct-h6rtvaqy
why: An «exposed with a cost» verdict that names an escher-side change inside dioxus-native-dom must state its boundary as crate-internal visibility, and nothing the probe keeps may widen it.
quote: crate internals are `pub(crate)` / `pub(super)`
@compare_item n=3 part=lost how=strong entity=conventions#6.1 label=architecture/conventions fact=fct-bhxvo7z2
@carried n=10 label=architecture/conventions fact=fct-bhxvo7z2
why: The probe must not route anything it reads (a component name, a handler, a signal's value) through the bridge's crate-local trace macro, because nothing it reads may be logged.
quote: `dioxus-native-dom` defines a crate-local `trace!` macro expanding to `tracing::debug!` under `tracing`
@compare_item n=4 part=lost how=strong entity=cross-cutting-patterns#7.1 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
@carried n=6 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
why: If link (4), state to the nodes that read it, is measured through the changed set, its verdict must treat that set as engine-touched and a superset of what a snapshot reads, never as the set of nodes that read a state.
quote: the set is engine-touched, a superset of what a snapshot reads as changed (an attribute re-written to itself marks and changes no reading; bounds-only movement changes a reading and marks nothing)
@carried n=6 run=2 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
quote: so a `resolve` with nothing written marks nothing in either layout mode
@compare_item n=5 part=lost how=strong entity=existing-scopes#1.22 label=architecture/existing-scopes fact=fct-dac2ih77
@carried n=2 label=architecture/existing-scopes fact=fct-dac2ih77
why: The windowed path of the MountedData hazard is read, not run, and that reading must go through dioxus-native's dependency on the bridge, the one crate where the windowed shell meets `dioxus_document.rs`'s `poll`.
quote: depends on dioxus-native-dom (packages/dioxus-native/Cargo.toml:92)
@compare_item n=6 part=lost how=strong entity=inherited-defaults#3.1 label=architecture/inherited-defaults fact=fct-fgkqqy76
@carried n=3 label=architecture/inherited-defaults fact=fct-fgkqqy76
why: An escher-side change in `dioxus-native-dom`, if the probe needs one, must keep tracing gated per call site in the engine crates and add no span or log field, so that nothing the probe reads gets logged.
quote: Optional capabilities: Cargo features, tracing gated per call site in the engine crates — escher-telemetry and escher-driver (its `command` span and, since 2026-10-10-mcp-surface, its `tool_call` span) are ungated
@compare_item n=7 part=lost how=classify entity=stack-and-technologies#2.1 label=architecture/stack-and-technologies fact=fct-tttkltfs
@carried n=6 label=architecture/stack-and-technologies fact=fct-tttkltfs
why: Each function the probe keeps must read at or under the cognitive-complexity ceiling of 15 on the dev host.
quote: a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15
@compare_item n=8 part=lost how=strong entity=standard-contracts#29.6 label=architecture/standard-contracts fact=fct-llnppo2j
@carried n=6 label=architecture/standard-contracts fact=fct-llnppo2j
why: The MountedData fix options touch the mounted-event flush and its guard, and the Dioxus 0.7 divergence from upstream's test, so each option must say what it does to them.
quote: `flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
@compare_item n=9 part=lost how=strong entity=standard-contracts#5.1 label=architecture/standard-contracts fact=fct-qmowki7i
@carried n=8 label=architecture/standard-contracts fact=fct-qmowki7i
why: `MountedData::scroll_to` is a named caller of the engine's `scroll_into_view`, whose signature is held for every caller, so no fix option may move that signature.
quote: its signature unchanged and for every caller: script `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation and the test harness's input helper
@compare_item n=1 part=listed how=strong entity=conventions#1.1 label=architecture/conventions fact=fct-kjxsappk
@carried n=1 label=architecture/conventions fact=fct-kjxsappk
why: Every probe check or example the chunk adds must pass the workspace fmt and clippy gates, and upstream's blitz-dom clippy line, as the ci-leg.sh legs run them.
quote: `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — each run as its `.github/scripts/ci-leg.sh` leg in CI and on the dev host
@carried n=1 run=2 label=architecture/conventions fact=fct-kjxsappk
quote: `clippy::expect_used` / `clippy::unwrap_used` allowed locally next to a justification
@compare_item n=2 part=listed how=classify entity=conventions#9.2 label=architecture/conventions fact=fct-cnla3gly
@carried n=5 label=architecture/conventions fact=fct-cnla3gly
why: A fixture page or boot the probe shares between checks must live in the fixtures module, and a page with only one reader must stay in its own check's file.
quote: A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file
@compare_item n=3 part=listed how=classify entity=conventions#3.1 label=architecture/conventions fact=fct-aqtiieoj
@carried n=6 label=architecture/conventions fact=fct-aqtiieoj
why: Every probe module, test file or example must open with a `//!` doc, and an example must say what it demonstrates.
quote: modules and files open with `//!` docs
@carried n=6 run=2 label=architecture/conventions fact=fct-aqtiieoj
quote: examples open with a `//!` comment describing what they demonstrate
@compare_item n=4 part=listed how=classify entity=conventions#10.1 label=architecture/conventions fact=fct-2wv4ux4q
@carried n=8 label=architecture/conventions fact=fct-2wv4ux4q
why: A Dioxus example the probe writes, with state that a handler writes and another node reads, must follow the example shape and the signal and store hooks.
quote: Dioxus examples declare `fn main() { dioxus_native::launch(app); }` and `fn app() -> Element`
@carried n=8 run=2 label=architecture/conventions fact=fct-2wv4ux4q
quote: state via `use_signal`, `use_store`, `use_memo`, `use_future`
@compare_item n=5 part=listed how=strong entity=inherited-defaults#2.1 label=architecture/inherited-defaults fact=fct-77ljaxug
@carried n=2 label=architecture/inherited-defaults fact=fct-77ljaxug
why: Any probe code kept in `tests/blitz-tests/`, `examples/seven_guis/` or `packages/dioxus-native-dom/` must pass the inherited quality gates (fmt, clippy, the blitz-dom clippy line, rustdoc, cargo-deny, the coverage floors) and keep the `stand-repeat` leg's zero-failure budget.
quote: `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and, since 2026-10-14-quality-gates, upstream's blitz-dom clippy line under `--no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets` (all run through `.github/scripts/ci-leg.sh`), rustdoc `-D warnings` set workflow-wide and enforced over every workspace library crate — the docs job runs `cargo doc --workspace --no-deps --locked`; a cargo-deny audit of the advisories and licences of the whole workspace and a cargo-llvm-cov coverage report gated by per-crate floors, with the `stand-repeat` leg's zero-failure budget beside them, all legs of the same script
