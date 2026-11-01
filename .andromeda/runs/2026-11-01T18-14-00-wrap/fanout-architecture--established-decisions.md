proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: "## Established Decisions"
    change: "Replace «its owner Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)» with «its owner «Upstream sync ahead of determinism and exploration» (inputs#I6 of 2026-11-01-upstream-sync-ui-graph)»."
    sidecar: "established-decisions: the seven's merge-surface line moves its owner to «Upstream sync ahead of determinism and exploration», this chunk being the entry that held it."
    rationale: "The report's sweep re-points this owner row (Counts, Sweep reach: established-decisions.md:75) and inputs#I6's second row carries the merge surfaces to the next sync's entry, with residuals.md:35 and :36 moving too."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: "## Established Decisions"
    change: "Replace «a textual conflict with the non-invertible answer for Epoch 3's sync, their merge surface a line owned by Epoch 3's «Upstream sync ahead of the UI graph»» with «a textual conflict with the non-invertible answer for the next sync, `0980cce5f` left out at upstream `main` `e42ee1f25`, their merge surface a line owned by «Upstream sync ahead of determinism and exploration»»."
    sidecar: "established-decisions: the eight's merge-surface line and the `hit_inner` conflict move to the next sync's entry, `0980cce5f` left out of this take."
    rationale: "Sweep reach re-points established-decisions.md:76; inputs#I6's first row lists `0980cce5f` among the twenty-one left out and its second row carries the merge surfaces to that entry."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:78
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 77
    section: "## Established Decisions"
    change: "Replace «whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)» with «whose owner is «Upstream sync ahead of determinism and exploration» (inputs#I6 of 2026-11-01-upstream-sync-ui-graph)»."
    sidecar: "established-decisions: the document clock's merge-surface owner moves to the next sync's entry."
    rationale: "Sweep reach re-points established-decisions.md:77, the same residuals line as :75."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "Replace «whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)» with «whose owner is «Upstream sync ahead of determinism and exploration» (inputs#I6 of 2026-11-01-upstream-sync-ui-graph)»."
    sidecar: "established-decisions: the select model's merge-surface owner moves to the next sync's entry."
    rationale: "Sweep reach re-points established-decisions.md:79, the same residuals line as :75."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "Replace «its line owned by Epoch 3's «Upstream sync ahead of the UI graph»» with «re-read at 2026-11-01-upstream-sync-ui-graph, its line owned by «Upstream sync ahead of determinism and exploration»»."
    sidecar: "established-decisions: the select list's merge surface, re-read at this take, moves its owner to the next sync's entry."
    rationale: "Sweep reach re-points established-decisions.md:85; For P5 discharges the select list's surface by evidence/readings.md and inputs#I6's second row carries the surfaces on."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:131
    dependent-of: D-arch-decisions
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 69
    section: "## Established Decisions"
    change: "New record after this line: - At upstream `main` `e42ee1f25` the merge base stays `23b9b26a9` and twelve more commits are taken by no-commit cherry-pick, thirty-six of the 57 past it taken and twenty-one left out (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md); this take's resolutions and the key-press panic's fix stand beside the three merge-surface records, every row of the three re-read to a named outcome (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md), each flagged upstreamable and nothing sent, the whole carried by «Upstream sync ahead of determinism and exploration»."
    sidecar: "established-decisions: this take's sentence on the merge surfaces, beside the last take's history at the range input and keyboard records."
    rationale: "Counts, sync-line: established-decisions.md:67 and :69 stay as history with a new sentence for this take beside them; the Outcome and inputs#I6's second row state the re-read and its owner."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "New record after this line: - The key-press panic's fix stands beside them, escher's own, a direct engine change for every document, ratified: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so every reader of a stale `layout_parent` is cured with it, the id no longer stale, and a key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, an inline `display: none` and CSS alone, in both layout modes (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187; tests/blitz-tests/tests/select_popup.rs:513). Focus stays and the key still acts: `DocumentMutator::flush` is unedited, still reading `is_inert()` alone ([DOM semantics]). Flagged upstreamable, upstream's own tree panicking the same way (`node/node.rs:1740:27` at `e42ee1f25`), and nothing sent; its merge surface is carried by «Upstream sync ahead of determinism and exploration» — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md."
    sidecar: "established-decisions: the panic's fix recorded as a direct engine change for every document, ratified; the founder's word, inputs#I3: «Убрать падение, фокус остаётся (Рекомендую)». Playbook pattern: An engine behaviour change for every document, routine."
    rationale: "Expected amendments name it for §Established Decisions; Symbols / APIs state the fix, its reach and that focus stays; For P2's halt puts his words in the sidecar and the bare `ratified` in the body."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 88
    section: "## Established Decisions"
    change: "New record after this line: - Twelve more behaviour changes for every document are upstream's, taken at upstream `main` `e42ee1f25`, and are not counted among escher's direct engine changes above: `restyle_for_attribute_change` and `restyle_for_child_list_change` in place of the blanket `restyle_subtree` on an attribute write, a removal and an insertion (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807); a table box's rebuild and `<picture>` ([Layout]); a failed stylesheet ([Resource loading]); `mix-blend-mode` and `isolation` and the culling bounds ([Painting]); the prefs before the user-agent sheet ([Document defaults]); `URLSearchParams` ([JavaScript]); a `CustomWidget`'s attribute replay, behind `custom-widget` (packages/blitz-dom/src/node/element.rs:902-906); the inline box of only block children ([Layout]); a mounted event's guard ([Dioxus integration]); and sub-pixel resolved `width` and `height` (packages/blitz-dom/src/resolved_style.rs:468). Each stands ratified for its take under the playbook pattern for an engine behaviour change for every document; `URLSearchParams` is a realization on the registered script-to-DOM crossing, and no boundary widening was read in the twelve (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "established-decisions: the twelve upstream behaviour changes recorded, ratified for their take; the founder's word, inputs#I3: «Двенадцать коммитов (Рекомендую)»."
    rationale: "Expected amendments name the twelve for §Established Decisions, in the form line 88 holds for the last take's six; For P2's halt keeps his words to the sidecar."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:92
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 39
    section: "## Established Decisions"
    change: "After the Stylo prefs clause, ending at `layout.threads` -1 and its citation, add: «, set once, behind a `Once`, by the crate-private `init_stylo_prefs`, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, so they stand before the user-agent sheet parses, its body the 0.22 names (packages/blitz-dom/src/document.rs:87-102)»."
    sidecar: "established-decisions: Document defaults states when the Stylo prefs are set, upstream #1310."
    rationale: "Symbols / APIs, `7a3dd8918`: the prefs now precede the user-agent sheet, and Reverted facts keep the 0.22 names; the stylo-prefs pattern is unfired, so no standing row carries it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 45
    section: "## Established Decisions"
    change: "After the render-blocking clause add: «; a failed, empty or non-CSS stylesheet stops blocking render, `ResourceHandler<StylesheetHandler>::error` responding `Err(error)` so that `pending_critical_resources` loses the request (packages/blitz-dom/src/net.rs:144-146; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-91), the new `NetHandler::error` a default that does nothing (packages/blitz-traits/src/net.rs:36-38), forwarded by the harness's `CountedHandler` (packages/blitz-test-harness/src/settle.rs:121-125)». After the `<link>` clause add: «, and a bad `href` starts no load (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107)»."
    sidecar: "established-decisions: Resource loading states that a failed stylesheet no longer blocks render and a bad href starts no load, upstream #1318 with escher's companion forward."
    rationale: "Symbols / APIs, `f6edb9701` and the `CountedHandler` bullet: line 45 states the block with no failure arm and the load on any `href`, both now narrower."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 57
    section: "## Established Decisions"
    change: "New record after this line: - Upstream's `background-clip: text` (`ac6f9f582`) was tried and left out: `BackgroundClip::Text` is `gecko`-only on Stylo 0.22 (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "established-decisions: Unsupported features records why upstream's `background-clip: text` stays out on Stylo 0.22."
    rationale: "Reverted / negative API facts state it; it binds to the Stylo pin the report says stands."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:84
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 101
    section: "## Established Decisions"
    change: "After «display/float/position/contain/visibility/font/text-shaping changes yield `ALL_DAMAGE`» add: «, a table box's position, padding or border change too (`table_needs_rebuild`)»."
    sidecar: "established-decisions: Layout's damage rule gains the table box's rebuild, upstream #1312."
    rationale: "Symbols / APIs, `8b1a0a3ce`: the line's «else `RELAYOUT`» no longer holds for those table box changes; `table_needs_rebuild` stands at layout/damage.rs:442-458."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 109
    section: "## Established Decisions"
    change: "New record after this line: - `<picture>` is no longer `display: contents`: upstream's `picture` rule left the default sheet and every select rule stays (packages/blitz-dom/src/document.rs:4061-4094) — an upstream engine change for every document."
    sidecar: "established-decisions: Layout records `<picture>` leaving `display: contents`, upstream #1317."
    rationale: "Symbols / APIs, `dee854c76`; the picture pattern is unfired, so no standing row carries it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 109
    section: "## Established Decisions"
    change: "New record after this line: - An inline box whose children are only blocks carries `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, read through the public `Node::is_inline_with_only_block_children` (packages/blitz-dom/src/node/node.rs:531-537) and set in `collect_layout_children_with_wrap` (packages/blitz-dom/src/layout/construct.rs:587-599); `render_element` skips such a box's own shadow, background and border (`paints_own_box`), and `append_text_to_node` also damages the parent (packages/blitz-dom/src/mutator.rs:257-259), held by `inline_with_only_block_children_tests` with no frame test (packages/blitz-dom/src/document.rs:4096-4296) — an upstream engine change for every document."
    sidecar: "established-decisions: Layout records the inline box of only block children, upstream #1319."
    rationale: "Symbols / APIs, `f76777766`, and Coverage of new surfaces for its unit tests and the frame's ✗; the inline-flag pattern is unfired."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 161
    section: "## Established Decisions"
    change: "New record after this line: - `mix-blend-mode` and `isolation` paint: blitz-paint's private `blend` module holds `convert_blend_mode` (packages/blitz-paint/src/blend.rs:5-26), `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, `render_element` pushes a viewport-wide blend layer and pops it last (packages/blitz-paint/src/render.rs:485-506), and `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal` (packages/blitz-dom/src/node/node.rs:1382-1384); an open select's list paints last, past the blend layers, and `text_contrast` reads `None` past a blended element (packages/blitz-test-harness/src/contrast.rs:154-156). A measured limit: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, `layers.rs`'s three unit tests its witness (packages/blitz-paint/src/layers.rs:104-181) — an upstream engine change for every document."
    sidecar: "established-decisions: Painting records `mix-blend-mode` and `isolation` now painting, upstream #1316, with no frame test, that gap's owner named at P5."
    rationale: "Symbols / APIs, `6fca6308d`, and Coverage of new surfaces' ✗; line 161's own «mask isolation» is another sense and stays, per Counts, blend-mod."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 161
    section: "## Established Decisions"
    change: "New record after this line: - `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform — a port of upstream's `e42ee1f25`, whose own lines stand on the untaken `ac6f9f582`, and an upstream engine change for every document, the workspace its witness (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "established-decisions: Painting records the culling bounds on the scaled size, upstream #1342, ported."
    rationale: "Symbols / APIs, `e42ee1f25`, and Coverage of new surfaces, which names it among the commits with no test of their own."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 167
    section: "## Established Decisions"
    change: "New record after this line: - `URLSearchParams` is a global class and `URL.prototype.searchParams`, `href` and `search` live both ways, its script `url_search_params.js` first in `BOOTSTRAP_JS`, and its native `parse_search_params` (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184), registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4), held by `url_search_params` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95) — an upstream change, a realization on the registered script-to-DOM crossing; two WPT files joined `timeout-quarantine.txt` with it (wpt/runner/timeout-quarantine.txt:105; wpt/runner/timeout-quarantine.txt:107)."
    sidecar: "established-decisions: JavaScript records `URLSearchParams` and its native helper's deletion before any page script, upstream #1314."
    rationale: "Symbols / APIs, `91ef65e21`; Expected amendments read it as a realization on the registered crossing; Coverage of new surfaces holds the deletion and the four tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 173
    section: "## Established Decisions"
    change: "New record after this line: - `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239) — an upstream engine change for every document; upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0, and returns with `286874cc1`."
    sidecar: "established-decisions: Dioxus integration records the mounted event's guard, upstream #1305, and why its test stays out on Dioxus 0.7."
    rationale: "Symbols / APIs, `6ef4b532d`, and inputs#I6's third row, which ties the test's return to `286874cc1`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
note: The twelve and the panic's fix go on P2's answer card (report.md:92, :130), so their records hold the bare `ratified` and only the sidecars his words; architecture.md:60 also names «Epoch 3's sync» as a merge-surface owner though the owner sweep lists only architecture.md:62, and the body is not this file's edit.
