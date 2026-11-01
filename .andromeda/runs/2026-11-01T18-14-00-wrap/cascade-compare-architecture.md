# Cascade compare — architecture · wrap · 2026-11-01-upstream-sync-ui-graph
E items 62 · held 37 · lost at classify 3 · lost at strong 5 · beside a placed fact 4 · cascade only 6 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cross-cutting-patterns#4.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 27
    section: "## Cross-cutting Patterns"
    change: "Add after line 27, in the same group, the item «- A stylesheet that fails, comes back empty or is not CSS no longer holds the redraw skip for pending critical resources, on every document: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and it stops blocking render (packages/blitz-dom/src/net.rs:144-146); the trait method it answers is §Standard Contracts → Provider traits' and the render-blocking rule §Established Decisions → Resource loading's»"
    sidecar: "Service wiring gains the failed stylesheet's release of the redraw skip for pending critical resources (f6edb9701)."
    rationale: "report.md:14 states a failed, empty or non-CSS stylesheet stops blocking render; line 21's skip while critical resources are pending stands and gains this carve-out, which no row of the report's net-error or load-wait sweep places in this file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14

### design-philosophy#2.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 25
    section: "## Design Philosophy"
    change: "New sub-bullet following this line, under «Embedder services are injected, with no-op defaults» - `NetHandler::error`, new on the public trait, is a default that does nothing (packages/blitz-traits/src/net.rs:36-38); blitz-net's error arm calls it (packages/blitz-net/src/lib.rs:313) and blitz-dom's stylesheet handler responds with the error (packages/blitz-dom/src/net.rs:144-146), so a failed stylesheet stops blocking render, and a wrapping handler must forward it or the failure stops at the wrapper, as blitz-test-harness's `CountedHandler` now forwards it (packages/blitz-test-harness/src/settle.rs:121-125)"
    sidecar: "Design Philosophy, embedder services - a failed load reaches the document through the no-op default `NetHandler::error`, which a wrapping handler must forward."
    rationale: "The report's Symbols / APIs (f6edb9701 and the `CountedHandler` bullet) and Decisions & corrections add a new method with a no-op default to the provider trait surface this group describes; the group does not state it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14

### established-decisions#11.12 · lost at classify · .andromeda/architecture-detail/established-decisions.md:87
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "New record after this line: - The key-press panic's fix stands beside them, escher's own, a direct engine change for every document, ratified: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so every reader of a stale `layout_parent` is cured with it, the id no longer stale, and a key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, an inline `display: none` and CSS alone, in both layout modes (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187; tests/blitz-tests/tests/select_popup.rs:513). Focus stays and the key still acts: `DocumentMutator::flush` is unedited, still reading `is_inert()` alone ([DOM semantics]). Flagged upstreamable, upstream's own tree panicking the same way (`node/node.rs:1740:27` at `e42ee1f25`), and nothing sent; its merge surface is carried by «Upstream sync ahead of determinism and exploration» — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md."
    sidecar: "established-decisions: the panic's fix recorded as a direct engine change for every document, ratified; the founder's word, inputs#I3: «Убрать падение, фокус остаётся (Рекомендую)». Playbook pattern: An engine behaviour change for every document, routine."
    rationale: "Expected amendments name it for §Established Decisions; Symbols / APIs state the fix, its reach and that focus stays; For P2's halt puts his words in the sidecar and the bare `ratified` in the body."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23

### established-decisions#22.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:167
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 167
    section: "## Established Decisions"
    change: "New record after this line: - `URLSearchParams` is a global class and `URL.prototype.searchParams`, `href` and `search` live both ways, its script `url_search_params.js` first in `BOOTSTRAP_JS`, and its native `parse_search_params` (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184), registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4), held by `url_search_params` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95) — an upstream change, a realization on the registered script-to-DOM crossing; two WPT files joined `timeout-quarantine.txt` with it (wpt/runner/timeout-quarantine.txt:105; wpt/runner/timeout-quarantine.txt:107)."
    sidecar: "established-decisions: JavaScript records `URLSearchParams` and its native helper's deletion before any page script, upstream #1314."
    rationale: "Symbols / APIs, `91ef65e21`; Expected amendments read it as a realization on the registered crossing; Coverage of new surfaces holds the deletion and the four tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17

### established-decisions#23.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:173
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 173
    section: "## Established Decisions"
    change: "New record after this line: - `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239) — an upstream engine change for every document; upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0, and returns with `286874cc1`."
    sidecar: "established-decisions: Dioxus integration records the mounted event's guard, upstream #1305, and why its test stays out on Dioxus 0.7."
    rationale: "Symbols / APIs, `6ef4b532d`, and inputs#I6's third row, which ties the test's return to `286874cc1`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20

### occupied-resources#2.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: After this line add the record «- blitz-net's `#[cfg(test)] mod tests` holds two test-only TCP binds on loopback since 2026-11-01-upstream-sync-ui-graph, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, each `std::net::TcpListener::bind("127.0.0.1:0")` — served by no binary and reached by no caller, so no crossing; no served port or listener is added (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»
    sidecar: Registered blitz-net's two test-only TCP loopback binds under Network ports and listeners, as no crossing.
    rationale: The report brings two new test-only binds and owes their registration in arch §Occupied Resources (report.md:28, report.md:93); the tests holding them are read from the new-text ranges 522-530 and 532-545 (report.md:288-289), and the detail exceeds what line 8 can take under the 2 000-char line limit.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28

### occupied-resources#10.3 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:79
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 79
    section: "## Occupied Resources"
    change: After this line add the record «- `init_stylo_prefs` (`pub(crate)`) in blitz-dom's `document.rs`, process-wide since 2026-11-01-upstream-sync-ui-graph — behind a `Once`, `INIT`, it enables Stylo's prefs once in a process, before the user-agent sheet parses, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 names; no env read, print or `tracing` site (packages/blitz-dom/src/document.rs:87-102; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
    sidecar: Registered the `Once` behind `init_stylo_prefs` as new process-wide state in blitz-dom.
    rationale: The take adds a new process-wide guard (report.md:16, report.md:193-194), the sweep's `stylo-prefs` control fired on no master (report.md:59), and this category's process-wide group lists such statics, `PARALLEL_TRAVERSAL_LOCK` among them at line 78.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16

### standard-contracts#2.7 · lost at strong · beside a placed fact F20 (near 95, standard-contracts#9.8) · .andromeda/architecture-detail/standard-contracts.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 22
    section: "## Standard Contracts"
    change: "A new record follows, «- `init_stylo_prefs`, `pub(crate)` and behind a `Once`, sets Stylo's prefs before the user-agent sheet parses: `UserAgentStylesheet::parse` and `BaseDocument::new` both call it, its body the 0.22 names (packages/blitz-dom/src/document.rs:87-102).»"
    sidecar: "Document core: new record, `init_stylo_prefs`."
    rationale: "report.md:16 lands the function; report.md:59 reads `stylo-prefs` unfired, so no master row names it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16

### standard-contracts#3.4 · lost at classify · beside a placed fact F12 (near 58, standard-contracts#5.1) · .andromeda/architecture-detail/standard-contracts.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 36
    section: "## Standard Contracts"
    change: "A new record follows, «- Restyle narrowing: `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion; `BaseDocument::snapshot_node` takes `attr: &LocalName`, a public signature changed, its two callers both in `mutator.rs`; `TElement::borrow_data` filters data with no primary style; the changed-set insert in `remove_and_drop_all_children` stays first; an engine change for every document (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807; packages/blitz-dom/src/mutator.rs:288; packages/blitz-dom/src/mutator.rs:429; packages/blitz-dom/src/stylo.rs:771-776).»"
    sidecar: "Mutation, query and CSSOM: new record, the restyle helpers and `snapshot_node`'s changed signature."
    rationale: "report.md:51 says `standard-contracts.md` owes the restyle helpers a line; report.md:11 states them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:51

### standard-contracts#9.10 · lost at strong · .andromeda/architecture-detail/standard-contracts.md:97
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 97
    section: "## Standard Contracts"
    change: "A new record follows, «- `Node::is_inline_with_only_block_children(&self) -> bool`, public, reads `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, set in `collect_layout_children_with_wrap`; `render_element` skips such a box's own shadow, background and border (`paints_own_box`), and `append_text_to_node` also damages the parent; an engine change for every document (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/mutator.rs:257-259; packages/blitz-dom/src/document.rs:4096-4296).»"
    sidecar: "Nodes and widgets: new record, the public reader `is_inline_with_only_block_children` and its flag."
    rationale: "report.md:19 lands the public reader and flag; report.md:59 reads `inline-flag` unfired, so no master row names them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19

### standard-contracts#12.3 · lost at classify · beside a placed fact F18 (near 75, standard-contracts#7.1) · .andromeda/architecture-detail/standard-contracts.md:118
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 118
    section: "## Standard Contracts"
    change: "A new record follows, «- `mix-blend-mode` and `isolation` paint in every document: blitz-paint's private module `blend` holds the `pub(crate)` `convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, `render_element` pushes a viewport-wide blend layer and pops it last, and `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an open select's list paints last, past the blend layers; `layers.rs`'s unit tests hold the layers, and no test paints a blended element to a frame (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/render.rs:485-506; packages/blitz-paint/src/layers.rs:104-181; packages/blitz-dom/src/node/node.rs:1382-1384).»"
    sidecar: "Paint and shell: new record, the blend module and what paints `mix-blend-mode` and `isolation`."
    rationale: "report.md:94 asks a new sentence for the paint contract's blend module, no row standing; report.md:15 states it, report.md:104 the missing frame test, report.md:147 the list painting last."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:15

### standard-contracts#25.1 · lost at classify · beside a placed fact F16 (near 116, standard-contracts#12.1) · .andromeda/architecture-detail/standard-contracts.md:253
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 253
    section: "## Standard Contracts"
    change: "A new record follows, «- `URLSearchParams` is a global class beside `URL.prototype.searchParams`, `href` and `search` live both ways, from the bootstrap script `url_search_params.js`; its native `parse_search_params` is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script, on the registered script-to-DOM crossing, no carrier added (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/tests/url_search_params.rs:15-95).»"
    sidecar: "JavaScript: new record, `URLSearchParams`, `URL.prototype.searchParams` and the native helper deleted before any page script."
    rationale: "report.md:17 lands the surface, report.md:102 its native helper and tests, report.md:92 reads it a realization on the registered crossing."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17

## Held

- cross-cutting-patterns#5.1
- cross-cutting-patterns#6.1
- design-philosophy#4.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#11.10
- established-decisions#11.13
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#11.4
- established-decisions#13.1
- established-decisions#14.3
- established-decisions#21.1
- established-decisions#6.1
- established-decisions#7.1
- established-decisions#9.1
- existing-scopes#1.17
- existing-scopes#1.18
- existing-scopes#2.1
- existing-scopes#3.4
- existing-scopes#4.1
- occupied-resources#1.1
- occupied-resources#15.1
- project-intent#1.0
- stack-and-technologies#1.23
- stack-and-technologies#1.5
- standard-contracts#1.1
- standard-contracts#10.1
- standard-contracts#11.1
- standard-contracts#14.1
- standard-contracts#21.4
- standard-contracts#23.1
- standard-contracts#27.1
- standard-contracts#3.1
- standard-contracts#32.1
- standard-contracts#32.2
- standard-contracts#8.1

## Cascade only

- occupied-resources#10.2
- standard-contracts#12.1
- standard-contracts#21.3
- standard-contracts#5.1
- standard-contracts#7.1
- standard-contracts#9.8
