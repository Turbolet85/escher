## 2026-11-01-upstream-sync-ui-graph — the citations re-pointed past the take
**Section:** §Border Radius (border-radius.md) · §Color Palette, with → Core Colors and → Border Progression (color-palette.md) · §Depth Strategy (depth-strategy.md) · §Iconography (iconography.md) · §Motion (motion.md) · §Spacing (spacing.md) · §Surface: desktop-native → Tokens (surface.md) · §Typography (typography.md)
**Change:**
- Citation digits only: outside the rows the entries below name, every changed line reads as it did but for its digits, and no value moved.
- The offsets of the citations that moved: `render.rs` +2, +31, +39 or +40; `document.rs` +11, +22, +24 or +32; `node/node.rs` +11 or +13; `stylo.rs` +8; `settle.rs` +6; `bootstrap.rs` +8; `custom_widget.rs` +2; `box_shadow.rs` +1; `blitz-paint`'s `lib.rs` +1; `blitz-dom`'s `lib.rs` -2; `default.css` -4; `select_popup.rs` -4, and one of its ranges ends 4 lines sooner with its start unmoved.
**Why:** the take and the panic's fix edited each of these sources, all on the report's Modified list; among the edits, `picture`'s rule left `default.css`, `select_popup`'s row-3 skip went, `blitz-dom`'s `lib.rs` lost a two-line comment and `blitz-paint`'s `lib.rs` gained the `blend` module.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `mix-blend-mode` and `isolation` in the depth rules
**Section:** §Depth Strategy (depth-strategy.md: the stacking-root list, the open select list's paint-last row, two new rows after the opacity, filter and backdrop-filter layer row)
**Change:**
- The stacking-root list: was ending on atomic paint effects, or `isolation: isolate`; now atomic paint effects, `isolation: isolate`, or `mix-blend-mode` other than `normal`.
- The open select list's paint-last row now adds that it paints past the blend layers as well.
- New row: `mix-blend-mode` and `isolation` paint through `blitz-paint`'s private `blend` module, whose `pub(crate)` `convert_blend_mode` converts Stylo's mode; `render_element` pushes a viewport-wide blend layer and pops it last.
- New row: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; blending's witnesses are the unit tests in `layers.rs` and the fixture in `contrast.rs`.
**Why:** the report measures upstream's #1316 making both properties paint, with `Node`'s arm for `mix-blend-mode` sitting in the stacking-root check, so the list's old reading was false. Its design criterion reads the list painting last, past the blend layers, on `paint_select_list`'s call holding no hunk; its Coverage row reads no test painting a blended element to a frame. #1316 is one of the twelve taken on the founder's answer F2, option A, by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F2 A names the twelve is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10). The frame gap's placement is the operator's, a `CARRY:` on «Upstream sync ahead of determinism and exploration» (Q2, inputs#I10); the rows name no owner.
**Kept:** the atomic paint effects clause stands as opacity, filter, clip-path and mask-image: stacking is the half the report measures, it left the atomic list to the design read, and nothing in it measures `mix-blend-mode` there. surface.md takes no row of this paint order, the depth-strategy row holding it in this master.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the inline box's paint skip
**Section:** §Depth Strategy (depth-strategy.md: a new row after the inset box shadow row)
**Change:**
- New row: an inline box holding only block children, read through the public `Node::is_inline_with_only_block_children`, paints no shadow, background or border of its own (`paints_own_box`); its unit tests read the flag, and no frame test reads the skip.
**Why:** the report measures upstream's #1319 setting the flag in `collect_layout_children_with_wrap` and `render_element` skipping such a box's own shadow, background and border; its Coverage row reads four unit tests in `document.rs` and no frame test. #1319 is one of the twelve taken on the founder's answer F2, option A, by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F2 A names the twelve is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10). The frame gap's placement is the operator's, a `CARRY:` on «Upstream sync ahead of determinism and exploration» (Q2, inputs#I10); the row names no owner.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a focused select past its dropped block
**Section:** §Surface: desktop-native → Tokens (surface.md, `desktop-native-tokens`: a new row after the row on the selects that open nothing)
**Change:**
- New row: a focused select whose anonymous block was dropped as its container lost its box, by `hidden`, by `style="display:none"` or by CSS alone, keeps focus and takes a key with no panic, and the key still moves its choice.
**Why:** the report measures the panic's fix, escher's own: `BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None` on each child still in the slab naming the block, and `DocumentMutator::flush` is unedited, so focus stays and the key still acts; `key_press_after_container_lost_its_box` asserts a key still moves a select's choice. The fix stands on the founder's answer F1, option A, by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F1 A says focus stays and keys still act is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10).
**Kept:** the row records the select's state only; the limit F1 A leaves, a focused control that no longer shows keeping focus and taking keys, is not stated here, the answer card listing it under a11y-plan §5.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/
