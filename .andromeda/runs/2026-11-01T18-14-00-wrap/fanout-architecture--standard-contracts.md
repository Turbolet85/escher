proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 327
    section: "## Standard Contracts"
    change: "Where it reads «· `fast` (fmt → clippy → test → ci-scripts, stopping at the first red: the local pre-push gate)» it reads «· `embolden` (`cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, no CI job, macOS exercising the feature there; .github/scripts/ci-leg.sh:28-29) · `fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red: the local pre-push gate; its last step pinned by `test_fast_leg_ends_on_the_font_embolden_witness`, .github/scripts/test_ci_workflows.py:361-375)»."
    sidecar: "CI contracts: the new leg `embolden` registered, and `fast` now ends on it, with its pin."
    rationale: "report.md:41 marks `standard-contracts.md:327` changed, fmt → clippy → test → ci-scripts → embolden; report.md:64 lands the leg in `LEGS` and last in `FAST_LEGS`, with no CI job."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 324
    section: "## Standard Contracts"
    change: "Where it reads «`.github/scripts/ci-leg.sh {leg}` — legs `fmt`» it reads «`.github/scripts/ci-leg.sh {leg}` — `embolden` joining `LEGS`, last in `FAST_LEGS` (.github/scripts/ci-leg.sh:8-9), and described with `fast` below — legs `fmt`»."
    sidecar: "CI contracts: the leg list's opening names `embolden` among `LEGS` and last in `FAST_LEGS`."
    rationale: "report.md:41 marks `standard-contracts.md:324` changed beside `:327`; report.md:64 states `ci-leg.sh:8-9`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:41
    dependent-of: D-arch-resources
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 334
    section: "## Standard Contracts"
    change: "The record reads «- `blitz-paint/font-embolden` has its local gate since 2026-11-01-upstream-sync-ui-graph, the `embolden` step of `fast`: `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests`, `--test select_popup_painted`, reproduces macOS's two readings on linux, CI's macOS leg exercising the feature and no CI job running the step (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).», the gap's sentence and its owner clause gone, since the owed thing landed."
    sidecar: "CI contracts: the «no local gate builds `blitz-paint/font-embolden`» gap retired, now the `embolden` step of `fast`; its owner clause dropped."
    rationale: "report.md:33 Retired (3) names this gap in arch §Standard Contracts; report.md:42 marks `standard-contracts.md:334` changed; report.md:62 answers the line's owner row, to nothing where the owed thing landed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 103
    section: "## Standard Contracts"
    change: "After the `NetHandler::bytes(self: Box<Self>, resolved_url: String, bytes: Bytes)` clause and its citation it adds «; `NetHandler::error(self: Box<Self>, url: String, error: String)`, new on the public trait, a default that does nothing (packages/blitz-traits/src/net.rs:36-38)»."
    sidecar: "Provider traits: `NetHandler::error` registered beside `bytes`."
    rationale: "report.md:14 lands the method on the public trait; report.md:48 marks `standard-contracts.md:103` changed, `error` beside `bytes`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 9
    section: "## Standard Contracts"
    change: "Where it reads «`NetHandler::bytes(resolved_url, bytes)` per resource kind» it reads «`NetHandler::bytes(resolved_url, bytes)` per resource kind, and a stylesheet's `error` (below)»."
    sidecar: "Document core: the per-kind handler sentence names a stylesheet's `error`."
    rationale: "report.md:48 marks `standard-contracts.md:9` changed, per resource kind; report.md:14 states `ResourceHandler<StylesheetHandler>::error`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:48
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 109
    section: "## Standard Contracts"
    change: "Where it reads «waking the waker for `doc_id` then calling `handler.bytes`» it reads «calling `handler.bytes`, or on a failed fetch `handler.error(url, e.to_string())`, then waking the waker for `doc_id` (packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316)»."
    sidecar: "HTML and networking crates: blitz-net's fetch calls the handler first, `error` on a failure, and wakes after it."
    rationale: "report.md:14 states blitz-net's error arm calls `handler.error` at `:313` and the waker moves after the handler at `:316`, which retires this line's order; the `net-error` row in report.md:48 did not reach it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 135
    section: "## Standard Contracts"
    change: "Where it reads «a crate-private counting `NetProvider` that forwards `fetch` and `is_noop`, counts a request in flight until its handler is consumed or dropped» it reads «a crate-private counting `NetProvider` that forwards `fetch` and `is_noop`, its handler forwarding `NetHandler::error`, without which a failed stylesheet read `NotSettled(Loads)` for good on a harness document (packages/blitz-test-harness/src/settle.rs:121-125; packages/blitz-test-harness/src/settle.rs:456-473), counts a request in flight until its handler is consumed or dropped»."
    sidecar: "Test harness: the load counter's handler forwards `NetHandler::error`."
    rationale: "report.md:25 states `CountedHandler` forwards `error`; report.md:48 marks `standard-contracts.md:135` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 227
    section: "## Standard Contracts"
    change: "Where it reads «until the load is answered or `LOAD_WAIT` has passed» it reads «until the load is answered — a failed load among them since 2026-11-01-upstream-sync-ui-graph, so it ends the wait (packages/blitz-test-harness/src/settle.rs:121-125) — or `LOAD_WAIT` has passed»."
    sidecar: "Driver session: a failed load ends the load wait, `settle_waiting` unedited."
    rationale: "report.md:25 says a failed load now ends the wait, `settle_waiting` unedited, and the harness and driver contracts owe that sentence; report.md:49 marks `standard-contracts.md:227` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 10
    section: "## Standard Contracts"
    change: "A new record follows, «- A failed stylesheet: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet stops blocking render; a bad `href` starts no load, `resolve_url` giving way to `doc.url.resolve_relative`; an engine change for every document (packages/blitz-dom/src/net.rs:144-146; packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107).»"
    sidecar: "Document core: new record, what a failed stylesheet does to the render block."
    rationale: "report.md:14 states `f6edb9701`'s behaviour; report.md:10 reads each of the twelve as an engine change for every document."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 22
    section: "## Standard Contracts"
    change: "A new record follows, «- `init_stylo_prefs`, `pub(crate)` and behind a `Once`, sets Stylo's prefs before the user-agent sheet parses: `UserAgentStylesheet::parse` and `BaseDocument::new` both call it, its body the 0.22 names (packages/blitz-dom/src/document.rs:87-102).»"
    sidecar: "Document core: new record, `init_stylo_prefs`."
    rationale: "report.md:16 lands the function; report.md:59 reads `stylo-prefs` unfired, so no master row names it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: "## Standard Contracts"
    change: "After «used values for layout-dependent ones» and its citation it adds «, `width` and `height` sub-pixel, read from `unrounded_layout` (packages/blitz-dom/src/resolved_style.rs:468)»."
    sidecar: "Mutation, query and CSSOM: the resolved `width` and `height` read sub-pixel."
    rationale: "report.md:21 states the computed sizes read `node.unrounded_layout()`; report.md:53 marks `standard-contracts.md:28` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:53
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 36
    section: "## Standard Contracts"
    change: "A new record follows, «- Restyle narrowing: `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion; `BaseDocument::snapshot_node` takes `attr: &LocalName`, a public signature changed, its two callers both in `mutator.rs`; `TElement::borrow_data` filters data with no primary style; the changed-set insert in `remove_and_drop_all_children` stays first; an engine change for every document (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807; packages/blitz-dom/src/mutator.rs:288; packages/blitz-dom/src/mutator.rs:429; packages/blitz-dom/src/stylo.rs:771-776).»"
    sidecar: "Mutation, query and CSSOM: new record, the restyle helpers and `snapshot_node`'s changed signature."
    rationale: "report.md:51 says `standard-contracts.md` owes the restyle helpers a line; report.md:11 states them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 82
    section: "## Standard Contracts"
    change: "After the `Widget` trait's method list and its citations it adds «; `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget` (packages/blitz-dom/src/node/element.rs:902-906)»."
    sidecar: "Nodes and widgets: `set_custom_widget` replays existing attributes to the widget."
    rationale: "report.md:18 states the replay; report.md:52 marks `standard-contracts.md:82` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:52
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 97
    section: "## Standard Contracts"
    change: "A new record follows, «- `Node::is_inline_with_only_block_children(&self) -> bool`, public, reads `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, set in `collect_layout_children_with_wrap`; `render_element` skips such a box's own shadow, background and border (`paints_own_box`), and `append_text_to_node` also damages the parent; an engine change for every document (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/mutator.rs:257-259; packages/blitz-dom/src/document.rs:4096-4296).»"
    sidecar: "Nodes and widgets: new record, the public reader `is_inline_with_only_block_children` and its flag."
    rationale: "report.md:19 lands the public reader and flag; report.md:59 reads `inline-flag` unfired, so no master row names them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 118
    section: "## Standard Contracts"
    change: "A new record follows, «- `mix-blend-mode` and `isolation` paint in every document: blitz-paint's private module `blend` holds the `pub(crate)` `convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, `render_element` pushes a viewport-wide blend layer and pops it last, and `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an open select's list paints last, past the blend layers; `layers.rs`'s unit tests hold the layers, and no test paints a blended element to a frame (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/render.rs:485-506; packages/blitz-paint/src/layers.rs:104-181; packages/blitz-dom/src/node/node.rs:1382-1384).»"
    sidecar: "Paint and shell: new record, the blend module and what paints `mix-blend-mode` and `isolation`."
    rationale: "report.md:94 asks a new sentence for the paint contract's blend module, no row standing; report.md:15 states it, report.md:104 the missing frame test, report.md:147 the list painting last."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 131
    section: "## Standard Contracts"
    change: "Where it reads «whose `filter` list is not empty or whose `background-image` holds a layer other than `none`» it reads «whose `filter` list is not empty, whose `background-image` holds a layer other than `none` or whose `mix-blend-mode` is other than `normal` (packages/blitz-test-harness/src/contrast.rs:154-156)»."
    sidecar: "Test harness: `text_contrast` reads `None` past a blended element."
    rationale: "report.md:24 adds the condition; report.md:44 marks `standard-contracts.md:131` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 240
    section: "## Standard Contracts"
    change: "Where it reads «a `concat!` of nine `include_str!` scripts under `runtime/bootstrap/`, defines history» it reads «a `concat!` of ten `include_str!` scripts under `runtime/bootstrap/`, `url_search_params` first, defines `URLSearchParams`, history»."
    sidecar: "JavaScript: `BOOTSTRAP_JS` holds ten scripts, `url_search_params` first, and the globals it defines name `URLSearchParams`."
    rationale: "report.md:17 moves nine to ten with `url_search_params` first; report.md:46 marks `standard-contracts.md:240` changed where it lists globals."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 253
    section: "## Standard Contracts"
    change: "A new record follows, «- `URLSearchParams` is a global class beside `URL.prototype.searchParams`, `href` and `search` live both ways, from the bootstrap script `url_search_params.js`; its native `parse_search_params` is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script, on the registered script-to-DOM crossing, no carrier added (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/tests/url_search_params.rs:15-95).»"
    sidecar: "JavaScript: new record, `URLSearchParams`, `URL.prototype.searchParams` and the native helper deleted before any page script."
    rationale: "report.md:17 lands the surface, report.md:102 its native helper and tests, report.md:92 reads it a realization on the registered crossing."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 266
    section: "## Standard Contracts"
    change: "After «`vdom_state.try_element_to_node_id(ElementId)`» and its citations it adds «, the guard `flush_queued_mounted_events` reads to skip an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)»."
    sidecar: "Dioxus DOM bridge: `try_element_to_node_id` guards the mounted events' flush."
    rationale: "report.md:20 states the guard; report.md:56 marks `standard-contracts.md:266` changed, the guard's reader."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:56
note: "Lines 9, 28, 131, 227 and 266 are estimated near the 2 000-char line bound once edited, so a validator's refusal there means splitting the line at its nearest clause; D-arch-decisions fired nothing here, and the free-disk steps, test-only binds and the panic's fix have no row in this file."
