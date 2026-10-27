# Task — architecture · 2026-10-27-upstream-sync-engine-debts

## The session
An upstream sync took eleven upstream DioxusLabs/blitz commits by no-commit cherry-pick on the founder's word. The merge base `23b9b26a9` did not move. Upstream main was read at `ebacd68bd` (it was `f9cbb2756`), and sixteen commits were left out, each with a measured reason.
The take brings six behaviour changes that reach every document: `:open` matches, `::before`/`::after` vote in flow classification, IME is enabled for an input already focused when its editor is built, a `file:` script load is refused for a non-`file:` document, CSS properties become `CSSStyleDeclaration` accessors, and the windowed shell's redraw requests go through its event loop.
blitz-vibey-script's runtime and element bindings were split into module trees, with nine JS bootstrap scripts. The code moved between files and nothing was removed.
blitz-tests gains the target `pseudo_element_flow_classification`. Dependencies, `Cargo.lock`, `deny.toml`, config, schema, escher-driver and escher-telemetry are unchanged, and no crate was added or retired.
Three conflict hunks were resolved. Residual lines 35 and 36 move to Epoch 3's «Upstream sync ahead of the UI graph» as their owner. The PROVISIONAL marks on the founder's F2 stand-row move come off now that he has ratified it.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/css.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/dom_exception.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/node_constants.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/style.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/stylesheets.js · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/window.js · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface

## Touches
- Project Intent's Upstream sync line (architecture.md:287, project-intent.md:9): upstream main is now read at `ebacd68bd` (was `f9cbb2756`). This sync took eleven commits by no-commit cherry-pick and left sixteen out, with the merge base `23b9b26a9` unmoved. The founder's word is the take's authority. Stylo "0.22.0", Taffy "0.15" and Dioxus "0.7.3" stay.
- Established Decisions' upstream "take read at" readings (established-decisions.md:67, :69) move to this sync's reading.
- Residual line 35 (the left-out upstream delta, grown by `a09f46792` · `527394724` · `d3ed4e2c7` · `ccfe15332` · `0980cce5f` · `ebacd68bd`) and residual line 36 (the merge surfaces of the engine-debts fixes, the clock and the select model) now have Epoch 3's «Upstream sync ahead of the UI graph» as their owner. Every architecture site that names an owner for them is affected: architecture.md:82 and established-decisions.md:67, :68, :75, :76, :78, :137.
- [DOM semantics] (dec-dom-semantics): `:open` now matches a `details` or `dialog` that carries `open`, through blitz-dom's element state.
- [Unsupported features] (architecture.md:54, dec-unsupported-features): the list of pseudo-classes that always match false must not take `:open`. The pseudo-element wording should be checked against the new `::before`/`::after` flow vote.
- [JavaScript] (architecture.md:84, dec-javascript): the runtime is now split into modules, a `file:` URL loads only for a `file:` document (a narrowing), and CSS properties are `CSSStyleDeclaration` accessors.
- [Shell and windowing] (architecture.md:80, dec-shell-and-windowing) and Service wiring (cross-cutting-patterns.md:21): redraw requests in the windowed shell now travel through its event loop. The shell provider now also carries an IME enable when a focused input's editor is constructed.
- Paint and shell contract (standard-contracts.md:96, contract-paint-and-shell): blitz-shell's redraw route changed, and the shell provider's `set_ime_enabled` is called at editor construction.
- JavaScript contract (architecture.md:173, standard-contracts.md:210, contract-javascript): the ScriptFetcher contract gains `document_may_load` and a `FetchError` variant. The "JS bootstrap" is now nine scripts under `runtime/bootstrap/`, and `CSSStyleDeclaration` gains property accessors.
- Mutation, query and CSSOM (contract-mutation-query-and-cssom): `DocumentMutator::set_attribute` and `remove_attribute` now flush open state on an `open` write. In `set_attribute`, that arm stands after escher's range re-sanitize arm as a second independent `if`.
- Nodes and widgets (contract-nodes-and-widgets): `ElementData` gains the public `flush_open_state`, called from `ElementData::new`.
- Environment variables (architecture.md:216 reads "in one unit test only"; occupied-resources.md:78): there are now two test-only `TMPDIR` reads. The second is `std::env::temp_dir()` in blitz-vibey-script `tests/dom.rs` `temp_dir_with_files`, which is test code only and not secret.
- Error handling (architecture.md:269, cross-cutting-patterns) and process-wide state (occupied-resources.md:52): the uncaught-JS-error `tracing::error!` and `js_console` sites moved from `runtime.rs` to `runtime/events.rs:21` (`report_js_error`). It stays feature-gated with engine target `blitz_vibey_script`, so every citation into `runtime.rs` for it must be re-pointed.
- Citations into `packages/blitz-vibey-script/src/runtime.rs` and `dom/element.rs` whose code moved into the new modules: architecture.md:205 (`runtime.rs:1352-1358` navigator.userAgent, `:2009-2017` location), :254 (`runtime.rs:1183-1246`, the module loader, now `runtime/modules.rs`), :298 (`runtime.rs:38-41`, history). `runtime.rs` keeps 151 lines.
- Citations into blitz-vibey-script `document.rs` (architecture.md:254 `:340-400`, `:205-206`, `:238-239`, `:290-291`, `:418-420`; :298 `:242-246`), whose file gained lines at 16, 163-169, 210-211 and 222.
- Citations into blitz-dom `layout/construct.rs` (architecture.md:68 `:442-473`; :277 `:191-268`), `mutator.rs` (:119 `:47-80`, `:279-282`) and `stylo.rs` (:44 `:29-35`, `:156-165`, `:157-163`; :64 `:94-108`, `:1387-1407`; :277 `:874-1204`). construct.rs gained lines at 313-317, 860 and 867-872, mutator.rs at 368-371 and 520-523, and stylo.rs at 510.
- Citations into blitz-shell `lib.rs` (architecture.md:252 `lib.rs:1`) and `window.rs` (:214 `:652-686`; :252 `:165-179`). lib.rs gained lines at 90, 93-98 and 104-106, and window.rs at 193-194.
- Citations into `packages/dioxus-native-dom/src/dioxus_document.rs` (architecture.md:205 `:93`, `:122-129`), which gained a test at 725-759, and into `apps/browser/src/main.rs` (:205 `:79-81`), which gained lines 192-193.
- Citations into `.github/workflows/publish-browser.yml` (architecture.md:246 `:122-126`, `:152-157`, `:187-193`; :249 `:100-104`; :311 `:152-157`), which gained lines at 126 and 153-166. The upstream-only and repository-guard claims are unchanged.
- Tests convention (conv-tests) and existing scopes' blitz-tests: one more auto-discovered target, `pseudo_element_flow_classification`, so the directory has 148 `.rs` files and 147 test targets. `all.rs:54` gains its `mod` line, and `all.rs` stays declared at `test = false`.
- Existing Scopes (existing-scopes): blitz-vibey-script now has `runtime/` and `dom/element/` module trees. `existing-scopes.md:61` names `tests/dom.rs`, which now holds 30 `#[test]` (was 27).
- Headless stand contract (standard-contracts.md:128, contract-headless-stand): the OPEN PROVISIONAL gap marker owned by F2 drops, now that the founder has ratified it («Подтвердить (Рекомендую)»).
- Through the ratification: the six every-document behaviour changes are upstream's, taken on the founder's word under the playbook pattern for an engine behaviour change for every document. A rule over engine changes for every document (established-decisions.md:58, :60) must be checked so it does not count them as escher's own direct changes.
- Browser app ([Browser app], contract-browser-app): `apps/browser/src/tab.rs` changed, and tab lenses now survive a webview props replacement (upstream-only app).

## New facts
- F1 · established-decisions · `:open` matches a `details` or `dialog` carrying `open`. The public `ElementData::flush_open_state` sets `ElementState::OPEN` at construction and on an `open` set or removal through `DocumentMutator`, and `NonTSPseudoClass::Open` reads that state. Before this, it never matched.
- F2 · established-decisions · Every box-generating `::before`/`::after` votes in flow classification (`classify_flow_children`), now that the `display: contents` filter is dropped.
- F3 · established-decisions · A text input that is already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor`.
- F4 · established-decisions · The windowed shell's redraw requests travel through its event loop.
- F5 · standard-contracts · blitz-vibey-script's public `document_may_load(document_url, url)` lets a `file:` URL load only for a `file:` document. A refusal is a `FetchError` variant, re-exported from the crate root and checked by `fetch()`, module `import` and `<script src>`. This narrows the default fetcher, which still takes only `file:` and `data:`, and same-origin and CORS checks are still absent.
- F6 · standard-contracts · CSS properties are accessors on `CSSStyleDeclaration`, named from Stylo's property list (`runtime/css.rs` `css_property_names`, `bootstrap/style.js` `installStyleProperties`). This is a realization on the registered script-to-DOM crossing, not a widening.
- F7 · existing-scopes · blitz-vibey-script's runtime is the module tree `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs`, with nine JS bootstrap scripts under `runtime/bootstrap/`. Its element bindings are `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`. `ScriptRuntime`'s public methods keep their names.
- F8 · established-decisions · The six every-document behaviour changes of this take (`:open`, the pseudo vote, IME-on-construct, the `file:` refusal, the CSS accessors, the redraw route) are upstream's. The founder ratified them for this take under the playbook pattern for an engine behaviour change for every document.
- F9 · established-decisions · In the browser app, tab lenses survive a webview props replacement (upstream-only app).

## Untouched
- Dependency pinning and Inherited Defaults' pins (architecture.md:29, :308): `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the base, with no git dependency.
- The driver session decisions and contract (dec-driver-session, contract-driver-session) and the telemetry bootstrap (contract-telemetry-bootstrap): escher-driver and escher-telemetry have an empty diff.
- Network ports and listeners and outbound hosts: no new port, socket, listener, print or log site.
- The scroll and hit readers (dec-scrolling-and-input, contract-scrolling-selection-tree): `scroll_into_view`, `visible_region`, `hit_inner`, `absolute_position` and `physical_unrounded_geometry` were re-read and are unchanged. The `visible_region` limit for a transformed scrolling box still stands because `0980cce5f` was not taken. blitz-vibey-script's caller only moved file.
- The document's clock (contract-document-core): `clock.rs` was not reached, and there is still no wall-clock read in blitz-dom outside it.
- Painting (dec-painting) and the password mask: blitz-paint was not reached. Only the owner of architecture.md:82's merge-surface residual moves.
- Stack and Technologies: no package enters, leaves or moves, Boa included.
- Directory structure under Infrastructure Patterns: no crate was added or removed.
- CI/CD and CI contracts: no `ci-leg.sh`, `agent-run.sh` or CI-script change. The new target is picked up by the workspace leg with no script change.
- The WPT runner (dec-wpt-runner, contract-wpt-runner-outputs): only the upstream WPT pin moved, and no architecture text states it.
