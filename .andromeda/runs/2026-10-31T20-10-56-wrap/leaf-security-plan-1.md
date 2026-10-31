group: security-plan-1
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «snapshotting a never-styled node is skipped (Stylo would panic);»
  source: .andromeda/security-plan-detail/threat-model-summary.md:11 «A remote page can also reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped»
  replace-with: «snapshotting a never-styled node is skipped (Stylo would panic); a remote page can reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden`, on any control, not fixed, owned by a CARRY on «Upstream sync ahead of the UI graph»;»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «`aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-disabled` moves no snapshot reading)»
  source: .andromeda/security-plan-detail/input-validation.md:225 «`aria-expanded` a boolean on every element but a single select, which writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`»
  replace-with: «`aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-expanded` read on every element but a single select, whose `expanded` is the closed bool of the engine's own state, an author's `aria-expanded` or `open` on it steering nothing; `aria-disabled` moves no snapshot reading)»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «focused and not read disabled by `ElementData::is_disabled`, `ArrowDown` · `ArrowUp` · `Home` · `End` move it and a `Key::Character` with no Ctrl, Alt or Meta is typeahead — a closed key match»
  source: .andromeda/security-plan-detail/input-validation.md:260 «Enter opens its list, a Space with no typed text standing opens it (`press_select_key`) and every other key does nothing»
  replace-with: «focused, not read disabled by `ElementData::is_disabled` and its list closed, `ArrowDown` · `ArrowUp` · `Home` · `End` move it, a `Key::Character` with no Ctrl, Alt or Meta is typeahead, Enter opens its list and a Space with no typed text standing opens it, every other key doing nothing — a closed key match»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «a click on a select focuses it;»
  source: .andromeda/security-plan-detail/input-validation.md:261 «clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing»
  replace-with: «a click on a select focuses it and, on a single select, opens its list or closes one standing;»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «the driver's `type` on a single select is the same typeahead, its help one fixed sentence more, and on a `multiple` select `focus_multiple_select` focuses with no click and clears the typeahead, the text choosing its match alone and a text opening on a space typing nothing, `Target` holding `multiple`»
  source: .andromeda/security-plan-detail/input-validation.md:286 «`focus_select` focuses any select with no click»
  replace-with: «the driver's `type` on any select goes through `focus_select`, focusing with no click, so firing no DOM focus events on a single select either, clearing the typeahead and closing any list, a text opening on a space typing nothing on any select; on a single select it is the same typeahead and on a `multiple` select the text chooses its match alone, its help fixed text, `Target` holding `select`»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «its name in the tree, unpainted, inside the ratified name crossing»
  source: .andromeda/security-plan-detail/input-validation.md:280 «a header painted in an open list, leaving also as pixels in a `screenshot`»
  replace-with: «its name in the tree, inside the ratified name crossing, and a header painted in an open list, leaving also as pixels in a `screenshot`, a boundary widening ratified»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «(ratified as to the list through a group and not as to the `label` as a name)»
  source: .andromeda/security-plan-detail/input-validation.md:280 «ratified as to the list through a group and as to the `label` as a name»
  replace-with: «(ratified as to the list through a group and as to the `label` as a name)»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «stated limits: no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read»
  source: .andromeda/security-plan-detail/input-validation.md:261 «stated limits: an option of a closed single select has no click arm and stays refused `covered`, under a group too, and a single select's `size` is not read»
  replace-with: «since 2026-10-31-select-popup Enter, a Space with no typed text standing or a click opens a single select's engine-painted list, on every document, remote pages included: its rows the options' labels and its headers the groups' `label`s, page text that leaves also as pixels in a `screenshot`, its colours the engine's own constants, at most `MAX_ROWS` 20 rows shown and those past that scrolled, a hostile `size`, 2,000 options and a 5,500-character label keeping every box finite and panicking nowhere; a disabled, inert or hidden select or one with no row to show opens none, a disabled row changes nothing and leaves the list open, a group header chooses nothing; its keys pass one closed match (`press_open_list_key`, Tab choosing and closing before focus moves) and `click_list_row` and `choose_current_and_close` write only through `apply_select_choice`; a press elsewhere, a page scroll or a box move closes it, and a wheel elsewhere closes it and then scrolls; a select removed, disabled, hidden or made inert with its list open closes it with no panic; the list holds no log site, span or field, and its highlighted row crosses no structured carrier; `size` on a `multiple` select is read through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value no hint, carrying no text out; stated limits: an option of a closed single select has no click arm and stays refused `covered`, its meaning stating `an option of a closed select lies under it: click the select first, then the option`, under a group too, and a single select's `size` is not read»

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «a changed node's `differs`, a list of closed words, never an attribute's value as typed»
  source: .andromeda/security-plan-detail/threat-model-summary.md:30 «The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing) and as pixels in a shot»
  replace-with: «a changed node's `differs`, a list of closed words, never an attribute's value as typed, an open select list's row reading its own `bounds` in the text and the diff, `differs` naming `bounds`, `NodeState` holding no field of the list, `NODE_FIELDS` thirteen and no `expanded` node key standing»

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «a password input in it painted masked, under the unchanged 1,048,576-byte bound (the in-band image the founder chose, 2026-10-12)»
  source: .andromeda/security-plan-detail/threat-model-summary.md:29 «An answer's `screenshot` of an open list, on the socket and MCP carriers alike, also takes a page's option labels and group `label`s out as pixels»
  replace-with: «a password input in it painted masked, a shot of an open select list also taking a page's option labels and group `label`s out as pixels, the shot's fields unmoved (a boundary widening, ratified), under the unchanged 1,048,576-byte bound (the in-band image the founder chose, 2026-10-12)»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «the shot's line without `png`, a password input in it painted masked — the pixels in band»
  source: .andromeda/security-plan-detail/threat-model-summary.md:41 «A result's `image` block of an open list also holds a page's option labels and group `label`s as pixels, a boundary widening ratified»
  replace-with: «the shot's line without `png`, a password input in it painted masked, the `image` block of an open select list also holding a page's option labels and group `label`s as pixels, a boundary widening ratified, and a snapshot or diff a row's own `bounds` — the pixels in band»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «`BaseDocument::visible_region` reads each clipping box through its transforms, exact under a translate and a scale»
  source: .andromeda/security-plan-detail/input-validation.md:311 «for a row or a group of an open select list it answers the list's scrollport instead»
  replace-with: «`BaseDocument::visible_region` reads each clipping box through its transforms, for a row or a group of an open select list answering the list's scrollport instead, exact under a translate and a scale»
