# Apply — design-system

## Verdicts
- fanout-design-system--cascade-g3.md 1 · applied · .andromeda/design-system-detail/depth-strategy.md:20#1
- fanout-design-system--cascade-g3.md 2 · applied · .andromeda/design-system-detail/depth-strategy.md:36#1
- fanout-design-system--cascade-g3.md 3 · applied · .andromeda/design-system-detail/depth-strategy.md:38+#1
- fanout-design-system--cascade-g3.md 4 · applied · .andromeda/design-system-detail/depth-strategy.md:39+#1
- fanout-design-system--cascade-g3.md 5 · applied · .andromeda/design-system-detail/depth-strategy.md:39+#2
- cascade-compare-design-system.md 2 · applied · .andromeda/design-system-detail/surface.md:19+#1

## Edits
- line: .andromeda/design-system-detail/depth-strategy.md:20
  old: «atomic paint effects, or `isolation: isolate`»
  new: «atomic paint effects, `isolation: isolate`, or `mix-blend-mode` other than `normal`»
- line: .andromeda/design-system-detail/depth-strategy.md:36
  old: «select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  new: «select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md), past the blend layers as well (packages/blitz-paint/src/render.rs:485-506; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:38
  anchor: «- Inset box shadows fill the padding box then cut a blurred hole with Compose::DestOut (packages/blitz-paint/src/render/box_shadow.rs:90-157)»
  add: «- An inline box holding only block children, read through the public `Node::is_inline_with_only_block_children`, paints no shadow, background or border of its own (`paints_own_box`); its unit tests read the flag, and no frame test reads the skip (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/document.rs:4096-4296; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:39
  anchor: «- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:530-563)»
  add: «- `mix-blend-mode` and `isolation` paint through `blitz-paint`'s private `blend` module, whose `pub(crate)` `convert_blend_mode` converts Stylo's mode; `render_element` pushes a viewport-wide blend layer and pops it last (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/render.rs:485-506; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:39
  anchor: «- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:530-563)»
  add: «- No fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; blending's witnesses are the unit tests in `layers.rs` and the fixture in `contrast.rs` (packages/blitz-paint/src/layers.rs:104-181; packages/blitz-test-harness/src/contrast.rs:304-315; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- after: .andromeda/design-system-detail/surface.md:19
  anchor: «- A disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing (tests/blitz-tests/tests/select_popup.rs:428-441; tests/blitz-tests/tests/select_popup.rs:443-472; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «- A focused select whose anonymous block was dropped as its container lost its box, by `hidden`, by `style="display:none"` or by CSS alone, keeps focus and takes a key with no panic, and the key still moves its choice (packages/blitz-dom/src/document.rs:1493-1503; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»

## Patterns
[[pattern]]
id = "ds-01"
fixed = 'atomic paint effects, or `isolation: isolate`'
