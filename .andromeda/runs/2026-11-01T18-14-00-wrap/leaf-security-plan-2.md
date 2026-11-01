group: security-plan-2
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »
- leaf: .claude/rules/security.md:12   class: retired
  leaf-text: «A remote page can reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden` — not fixed, owned by a CARRY on «Upstream sync ahead of the UI graph».»
  source: .andromeda/security-plan-detail/error-handling.md:54 «a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts»
  replace-with: «A key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone: `BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None` on each child still in the slab naming the block, so the id does not go stale; focus stays and the key still acts.»
- leaf: .claude/rules/security.md:13   class: omission
  leaf-text: «without a decision recorded in the security plan.»
  source: .andromeda/security-plan-detail/error-handling.md:66 «a default that does nothing (packages/blitz-traits/src/net.rs:36-38), so a wrapping implementer that does not forward it compiles and drops the failure silently»
  replace-with: «without a decision recorded in the security plan. A failed load reaches `NetHandler::error`, on the public trait a default that does nothing, so a wrapping implementer that does not forward it compiles and drops the failure silently — forward it.»
- leaf: .claude/rules/security.md:15   class: omission
  leaf-text: «same-origin/CORS checks are absent and every runtime event has `isTrusted = true`.»
  source: .andromeda/security-plan-detail/threat-model-summary.md:20 «its native helper `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs»
  replace-with: «same-origin/CORS checks are absent and every runtime event has `isTrusted = true`. A page's script also reaches `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways, through `url_search_params.js`, the first of ten bootstrap scripts — a realization on this registered crossing, no boundary widening; its native helper `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs.»
- leaf: .claude/rules/security.md:16   class: omission
  leaf-text: «invalid CSS declarations are dropped, not errors»
  source: .andromeda/security-plan-detail/input-validation.md:113 «in every document, remote pages included, a failed, empty or non-CSS stylesheet blocks no render»
  replace-with: «invalid CSS declarations are dropped, not errors; in every document, remote pages included, a failed, empty or non-CSS stylesheet blocks no render (`ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request), an engine change for every document»
- leaf: .claude/rules/security.md:16   class: omission
  leaf-text: «on new input paths.»
  source: .andromeda/security-plan-detail/input-validation.md:116 «an `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`, which panics»
  replace-with: «on new input paths. In every document, remote pages included, an `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`, which panics — an engine change for every document.»
- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «ratified as to the list through a group and as to the `label` as a name.»
  source: .andromeda/security-plan-detail/input-validation.md:210 «`ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value»
  replace-with: «ratified as to the list through a group and as to the `label` as a name. Behind the `custom-widget` feature, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, so a widget receives them at set-up and not only on a later change, the value's check staying the widget's own — an engine change for every document behind the feature, ratified.»
- leaf: .claude/rules/security.md:20   class: omission
  leaf-text: «No served network API, port or auth exists.»
  source: .andromeda/security-plan.md:211 «no served TCP or UDP port, two `blitz-net` tests alone binding TCP on loopback, served by no binary»
  replace-with: «No served network API, port or auth exists — two `blitz-net` tests alone bind TCP on loopback, in the crate's `#[cfg(test)] mod tests`, served by no binary, reached by no caller, so no crossing, registered as test-only.»
