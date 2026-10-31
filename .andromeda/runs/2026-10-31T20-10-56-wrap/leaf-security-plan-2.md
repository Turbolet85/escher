group: security-plan-2
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «keep the recursion caps (`MAX_SUBDOCUMENT_DEPTH = 10`, `MAX_IMPORT_DEPTH = 16`) and the stale-iframe-response discard by request id.»
  source: .andromeda/security-plan-detail/threat-model-summary.md:11 «A remote page can also reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped»
  replace-with: «keep the recursion caps (`MAX_SUBDOCUMENT_DEPTH = 10`, `MAX_IMPORT_DEPTH = 16`) and the stale-iframe-response discard by request id. A remote page can reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden` — not fixed, owned by a CARRY on «Upstream sync ahead of the UI graph».»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «`aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-disabled` moves no snapshot reading)»
  source: .andromeda/security-plan-detail/input-validation.md:225 «`aria-expanded` a boolean on every element but a single select, which writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`»
  replace-with: «`aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-disabled` moves no snapshot reading; a single select reads no `aria-expanded`, its `expanded` the closed bool of the engine's own state, and an author's `open` on a select steers nothing, `flush_open_state` reading `open` by presence on a `details` or a `dialog` alone)»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «focused and not `ElementData::is_disabled`, `ArrowDown` · `ArrowUp` · `Home` · `End` move it and a `Key::Character` with no Ctrl, Alt or Meta is typeahead — a closed key match,»
  source: .andromeda/security-plan-detail/input-validation.md:260 «a `Key::Character` with no Ctrl, Alt or Meta is typeahead, Enter opens its list, a Space with no typed text standing opens it (`press_select_key`) and every other key does nothing — a closed key match»
  replace-with: «focused, not `ElementData::is_disabled` and its list closed, `ArrowDown` · `ArrowUp` · `Home` · `End` move it, a `Key::Character` with no Ctrl, Alt or Meta is typeahead, Enter opens its list and a Space with no typed text standing opens it (`press_select_key`) — a closed key match,»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «a click on a select focuses it;»
  source: .andromeda/security-plan-detail/input-validation.md:261 «a click on a select, past `handle_click`'s `is_disabled` walk gate, clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing»
  replace-with: «a click on a select clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing, a bound label's click focusing a single select and opening nothing;»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «and on a `multiple` select `focus_multiple_select` focusing with no click and clearing the typeahead, the text choosing its match alone and a text opening on a space typing nothing;»
  source: .andromeda/security-plan-detail/input-validation.md:286 «`focus_select` focuses any select with no click, which on a `multiple` select would land on the row at the centre, so `type` fires no DOM focus events on a single select either»
  replace-with: «reached with `focus_select` on any select — focusing with no click, so no DOM focus events on a single select either, clearing the typeahead and closing any list — on a `multiple` select the text choosing its match alone, and a text opening on a space typing nothing on any select;»
- leaf: .claude/rules/security.md:17   class: retired
  leaf-text: «its stated limits: no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read.»
  source: .andromeda/security-plan-detail/input-validation.md:261 «stated limits: an option of a closed single select has no click arm and stays refused `covered`, under a group too, and a single select's `size` is not read»
  replace-with: «its stated limits: an option of a closed single select has no click arm and stays refused `covered`, under a group too, and a single select's `size` is not read.»
- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «Since 2026-10-31-engine-debts, in every document — remote pages included — a `select` with `multiple` takes a closed key match»
  source: .andromeda/security-plan-detail/threat-model-summary.md:11 «A page's markup, a remote page's included, also reaches the engine-painted select list on every document: its option labels as the list's rows, its group labels as their headers»
  replace-with: «Since 2026-10-31-select-popup, in every document — remote pages included — a single select's list is engine-painted, its rows a page's option labels and its headers group labels: a disabled, inert or hidden select or one with no row to show opens none (`select_list_allowed`), a disabled row changes nothing and a header chooses nothing; an open list's keys pass one closed match (`press_open_list_key`), Tab choosing and closing before focus moves, every choice through `apply_select_choice`; a press elsewhere, a page scroll or a box move closes it, as does the select's removal or its turning disabled, hidden or inert, with no panic; at most `MAX_ROWS` 20 rows show, hostile `size`, 2,000 options and a 5,500-character label keeping every box finite; its colours are the engine's own, no page value; it holds no log site, span or field, `SelectListRow.label` page text never logged, and a label leaves only as a painted pixel and the snapshot's existing `name` — an engine change for every document and a boundary widening, ratified. Since 2026-10-31-engine-debts, in every document — remote pages included — a `select` with `multiple` takes a closed key match»
- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «An `optgroup` is read by a closed element-name match,»
  source: .andromeda/security-plan-detail/input-validation.md:179 «`size_rows` parses it through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint, and a valid value hints a `height` of `size` × 1.2em, carrying no text out»
  replace-with: «`size` on a `multiple` select is read by `size_rows` through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint and a valid value hinting a `height` of `size` × 1.2em, carrying no text out — an engine change for every document and a boundary widening, ratified. An `optgroup` is read by a closed element-name match,»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «its non-empty `label` its name in the tree, unpainted, inside the ratified name crossing»
  source: .andromeda/security-plan-detail/input-validation.md:280 «its non-empty `label` is its name in the tree, inside the ratified name crossing, and a header painted in an open list, leaving also as pixels in a `screenshot`»
  replace-with: «its non-empty `label` its name in the tree, inside the ratified name crossing, and a header painted in an open list, leaving also as pixels in a `screenshot`, a boundary widening ratified»
- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «ratified as to the list through a group and not as to the `label` as a name.»
  source: .andromeda/security-plan-detail/input-validation.md:280 «ratified as to the list through a group and as to the `label` as a name»
  replace-with: «ratified as to the list through a group and as to the `label` as a name.»
- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «a password input in it painted masked, under the unchanged answer bound (the in-band image the founder chose, 2026-10-12)»
  source: .andromeda/security-plan-detail/threat-model-summary.md:29 «An answer's `screenshot` of an open list, on the socket and MCP carriers alike, also takes a page's option labels and group `label`s out as pixels»
  replace-with: «a password input in it painted masked, a shot of an open select list also taking a page's option labels and group `label`s out as pixels with the shot's fields unmoved (a boundary widening, ratified), under the unchanged answer bound (the in-band image the founder chose, 2026-10-12)»
- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «a changed node's `differs` cross as closed words and booleans»
  source: .andromeda/security-plan-detail/threat-model-summary.md:30 «The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing) and as pixels in a shot»
  replace-with: «a changed node's `differs` cross as closed words and booleans, the select list crossing only as a row's own `bounds` in the snapshot and the diff (`differs` naming `bounds`, a closed set) and as pixels in a shot — `NodeState` holding no field of it, `NODE_FIELDS` thirteen, no `expanded` node key, its highlighted row in no snapshot and no tree»
- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «(a PNG, base64, a password input in it painted masked; the in-band image the founder chose, 2026-10-12)»
  source: .andromeda/security-plan-detail/threat-model-summary.md:41 «A result's `image` block of an open list also holds a page's option labels and group `label`s as pixels, a boundary widening ratified»
  replace-with: «(a PNG, base64, a password input in it painted masked, an open list's option labels and group `label`s in it as pixels, a boundary widening ratified; the in-band image the founder chose, 2026-10-12)»
- leaf: .claude/rules/security.md:26   class: omission
  leaf-text: «paste (RUSTSEC-2024-0436) reaches it and is ignored by ID, its drop owned by a `.andromeda/residuals.md` line;»
  source: .andromeda/security-plan-detail/dependency-security.md:8 «RUSTSEC-2026-0192 (ttf-parser 0.25.1, unmaintained, no patched release), reached only through winit 0.31.0-beta.3 → winit-wayland → sctk-adwaita → ab_glyph → owned_ttf_parser»
  replace-with: «paste (RUSTSEC-2024-0436) reaches it and is ignored by ID, its drop owned by a `.andromeda/residuals.md` line; ttf-parser 0.25.1's RUSTSEC-2026-0192 (unmaintained, no patched release, reached only through winit 0.31.0-beta.3 → winit-wayland → sctk-adwaita → ab_glyph → owned_ttf_parser, no fix reachable without a lone bump of the coupled winit pin) is ignored by ID, a bounded deferral;»
