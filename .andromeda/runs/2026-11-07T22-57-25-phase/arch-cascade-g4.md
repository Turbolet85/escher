@carried n=1 label=architecture/conventions fact=fct-mimrhyxw
why: any dependency the signal registry or graph reader takes (dioxus-core for `Readable::subscribers()` and `ReactiveContext::origin_scope()`) must be declared once at the workspace root and consumed by reference, and any in-repo crate a new dependency names must take `default-features = false`.
quote: dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (Cargo.toml:43; Cargo.toml:270-306; packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:24-36; packages/dioxus-native-dom/Cargo.toml:29-47; packages/dioxus-native/Cargo.toml:73-119; packages/stylo_taffy/Cargo.toml:15-18), except the WPT runner's direct `mimalloc = "0.1.48"` (wpt/runner/Cargo.toml:15); in-repo crates declared with `default-features = false` (Cargo.toml:47-58)
@carried n=1 run=2 label=architecture fact=fct-jhzr5ovi
quote: dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`; in-repo crates declared with `default-features = false`, except the `seven_guis` path entry, which keeps its renderer feature
@carried n=2 label=architecture/conventions fact=fct-h6rtvaqy
why: the signal registry, the graph reader and the driver's new verb and outcome must keep their internals `pub(crate)` / `pub(super)`, exposing only what the stand and the driver call, and any consuming builder they add must be named `with_…` / `without_…`.
quote: crate internals are `pub(crate)` / `pub(super)`
@carried n=2 run=2 label=architecture fact=fct-bbsxpr3o
quote: crate internals are `pub(crate)` / `pub(super)`; no-op implementations are prefixed `Dummy` and internal JS natives `__blitz_`; consuming builder methods `with_…` / `without_…`
@carried n=3 label=architecture/conventions fact=fct-bhxvo7z2
why: the graph reader and the «reacts to a click» field must sit behind the cargo feature that gates the snapshot (`accessibility`), and no new log site in dioxus-native-dom (its crate-local `trace!`) nor any field on escher-driver's hand-built `command` span for the new verb may carry a node, a component name, a handler, a state or a value.
quote: optional functionality gated by cargo features
@carried n=3 run=2 label=architecture/conventions fact=fct-bhxvo7z2
quote: `dioxus-native-dom` defines a crate-local `trace!` macro expanding to `tracing::debug!` under `tracing` (packages/dioxus-native-dom/src/lib.rs:57-80), whose 4-argument arm passes only the first two items (packages/dioxus-native-dom/src/lib.rs:71-74)
@carried n=3 run=3 label=architecture fact=fct-6yr3qnpo
quote: optional functionality gated by cargo features; in the engine and upstream crates tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback, while escher's own two crates take `tracing` ungated — escher-telemetry emits its startup and panic events ungated and escher-driver builds its spans by hand, with no `#[instrument]`
@carried n=4 label=architecture/conventions fact=fct-knw5nmxh
why: the graph read's vdom walk and the registry's write-time record of link 3 must not hold the document's `RefCell` borrowed across vdom work, keeping the bridge's render-into-buffer-then-borrow-once order.
quote: on the Dioxus bridge, by rendering the vdom into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then borrowing the document once and replaying the buffer through one `MutationWriter` (packages/dioxus-native-dom/src/dioxus_document.rs:166-171; packages/dioxus-native-dom/src/dioxus_document.rs:302-307)
@carried n=4 run=2 label=architecture fact=fct-ps4gqbg4
quote: borrow conflicts worked around by taking a child list or layout out and restoring it
@carried n=5 label=architecture/conventions fact=fct-zd5gb466
why: a stand check that holds a `Session` to drive the graph verb must share its helpers through `session_common` and must be recorded in this list, and a new driver-action check moves the count of the `stand_act_*` checks.
quote: the checks that hold a `Session` — the session checks, the settle check `stand_settle`, the sixteen driver-action checks `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools`, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat` and, since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`, and the served-page checks `stand_cells_served` and `stand_load_page` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;` (tests/blitz-tests/tests/session_common/mod.rs:1-7)
@carried n=5 run=2 label=architecture fact=fct-wqndq65v
quote: stand checks share helpers through modules under `tests/` that are no test target
@carried n=6 label=architecture/conventions fact=fct-qr7xkzw2
why: the MCP listing test the new graph tool extends must keep its outer assertions and row walk in the test function, its per-row helpers naming the row and index only.
quote: escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers whose messages name the row and index only
@carried n=7 label=architecture/conventions fact=fct-2wv4ux4q
why: the stand's hook-local signals moved onto the registry must stay signal and store hooks in `#[component]` components, the wrapper keeping their read, write and subscribe behaviour.
quote: components with `#[component]` (examples/custom_widget.rs:60; examples/custom_widget.rs:73; examples/transforms.rs:249), state via `use_signal`, `use_store`, `use_memo`, `use_future`
@carried n=7 run=2 label=architecture fact=fct-jn753oge
quote: reactive state uses `#[derive(Store)]`; assets come through `asset!`; Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks
@binding n=1
**Apps and examples ↔ headless stand contract**: the stand's signals, wrapped by the registry, must keep the signal and store hooks' behaviour (architecture/conventions fct-2wv4ux4q) while the stand still boots its four lean tasks, Cells and the load page in the pinned 800x600 Light viewport, installing no telemetry and reading no env var (architecture fct-tewgiusr).
@binding n=2
**Feature gating ↔ never-logged and the snapshot carriers**: the graph reader and the snapshot field follow the `accessibility` gating of the snapshot modules (architecture/conventions fct-bhxvo7z2) and the features forwarded down the crates (architecture fct-nmnb5xux), and no `trace!` site or `command` / `tool_call` span field carries a node, a component name, a handler, a state or a value, beside a diff's three counts alone (architecture fct-tev7ieys).
@binding n=3
**Tests ↔ driver session schema**: a new verb joins the `'static` schema of twelve verbs (architecture fct-ksgwc2yv) and its MCP tool joins the listing test's rows (architecture/conventions fct-qr7xkzw2); a stand check that holds a `Session` for it joins the `session_common` list and the `stand_act_*` count (architecture/conventions fct-zd5gb466).
@binding n=4
**Visibility and manifests ↔ Dioxus integration**: the registry reads link 4 through public dioxus-core API only, a dependency declared once in `[workspace.dependencies]` (architecture/conventions fct-mimrhyxw), keeping its own internals `pub(crate)` (architecture/conventions fct-h6rtvaqy), since a scope's hooks are `pub(crate)` in dioxus-core and no fork is taken (architecture fct-53h7rw4e).
@binding n=5
**Borrowing ↔ Dioxus DOM bridge**: the graph read keeps the bridge's buffered render order (architecture/conventions fct-knw5nmxh) while reading link 1 by the same vdom walk the stable ids run, computed on demand and written to neither DOM nor vdom (architecture fct-srb7slak).
