group: security-plan-1
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
- leaf: .claude/docs/security-summary.md:7   class: stale
  leaf-text: «no served network API, no port and no authentication»
  source: .andromeda/security-plan.md:211 «No served network API exists — no served TCP or UDP port, two `blitz-net` tests alone binding TCP on loopback, served by no binary;»
  replace-with: «no served network API, no served TCP or UDP port — two `blitz-net` tests alone binding TCP on loopback, served by no binary — and no authentication»
- leaf: .claude/docs/security-summary.md:13   class: retired
  leaf-text: «a remote page can reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden`, on any control, not fixed, owned by a CARRY on «Upstream sync ahead of the UI graph»»
  source: .andromeda/security-plan-detail/error-handling.md:54 «a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts»
  replace-with: «a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block; focus stays and the key still acts, `DocumentMutator::flush` unedited»
- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «stale iframe responses discarded by request id;»
  source: .andromeda/security-plan-detail/input-validation.md:113 «in every document, remote pages included, a failed, empty or non-CSS stylesheet blocks no render»
  replace-with: «stale iframe responses discarded by request id; since 2026-11-01-upstream-sync-ui-graph, in every document, remote pages included, a failed, empty or non-CSS stylesheet blocks no render (a load that errs reaches `ResourceHandler<StylesheetHandler>::error`, which responds `Err(error)`, so `pending_critical_resources` loses the request) — an engine change for every document;»
- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «snapshotting a never-styled node is skipped (Stylo would panic);»
  source: .andromeda/security-plan-detail/input-validation.md:116 «an `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`, which panics»
  replace-with: «snapshotting a never-styled node is skipped (Stylo would panic); since 2026-11-01-upstream-sync-ui-graph, in every document, remote pages included, a stylesheet `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`, which panics — an engine change for every document;»
- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «since the 2026-10-17 upstream sync the accessibility builder reads»
  source: .andromeda/security-plan-detail/input-validation.md:210 «Behind the `custom-widget` feature, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value»
  replace-with: «since 2026-11-01-upstream-sync-ui-graph, behind the `custom-widget` feature, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, the value's check staying the widget's own — an engine change for every document behind the feature, ratified; since the 2026-10-17 upstream sync the accessibility builder reads»
- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «since the 2026-10-27 upstream sync `open` is read by its presence»
  source: .andromeda/security-plan-detail/input-validation.md:152 «In every document, remote pages included: page CSS paints both, validated by Stylo's parse and mapped by `convert_blend_mode`»
  replace-with: «since 2026-11-01-upstream-sync-ui-graph, in every document, remote pages included, page CSS paints `mix-blend-mode` and `isolation`, validated by Stylo's parse and mapped by `convert_blend_mode`, no fork or upstream test painting a blended element to a frame — an engine change for every document; since the 2026-10-27 upstream sync `open` is read by its presence»
- leaf: .claude/docs/security-summary.md:14   class: omission
  leaf-text: «every runtime event is `isTrusted`;»
  source: .andromeda/security-plan-detail/threat-model-summary.md:20 «A page's script also reaches `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways, through `url_search_params.js`, the first of ten bootstrap scripts»
  replace-with: «every runtime event is `isTrusted`; a page's script also reaches `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways, through `url_search_params.js`, the first of ten bootstrap scripts — a realization on the registered script crossing, no boundary widening — its native helper `__blitz_parse_search_params` deleted from `globalThis` before any page script runs;»
- leaf: .claude/docs/security-summary.md:17   class: omission
  leaf-text: «so the absent timeout cannot hang it; its outcome names a class only.»
  source: .andromeda/security-plan-detail/api-security.md:32 «a failed load ends the wait: `CountedHandler` forwards `NetHandler::error`»
  replace-with: «so the absent timeout cannot hang it; a failed load ends the wait: `CountedHandler` forwards `NetHandler::error`, a public-trait default that does nothing, so a failed stylesheet reads no `NotSettled(Loads)` for good on a harness document or on the driver, and a wrapping implementer that does not forward it drops the failure silently; its outcome names a class only.»
- leaf: .claude/docs/security-summary.md:33   class: omission
  leaf-text: «(no element id and no accessible name on stderr at `debug` or `trace`);»
  source: .andromeda/security-plan-detail/logging-monitoring.md:21 «A failed load's error text: it stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier takes it»
  replace-with: «(no element id and no accessible name on stderr at `debug` or `trace`); a failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier taking it, the sink printing `waiting_nodes` and every other field of that line, the message included, `[redacted]`;»
