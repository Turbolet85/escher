group: design-system
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «whole past an `opacity` ancestor or a box that clips its select»
  source: .andromeda/design-system-detail/depth-strategy.md:36 «, past the blend layers as well (packages/blitz-paint/src/render.rs:485-506; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
  replace-with: «whole past an `opacity` ancestor, the blend layers or a box that clips its select»
- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «Engine facts: default generic size 16px (13px monospace)»
  source: .andromeda/design-system-detail/depth-strategy.md:41 «`mix-blend-mode` and `isolation` paint through `blitz-paint`'s private `blend` module»
  replace-with: «Engine facts: `mix-blend-mode` other than `normal` makes a stacking-context root and, with `isolation`, paints through `blitz-paint`'s `blend` module in a viewport-wide layer, no test painting a blended element to a frame; default generic size 16px (13px monospace)»
- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «so a timer or an animation does not run during the wait.»
  source: .andromeda/design-system-detail/depth-strategy.md:39 «An inline box holding only block children, read through the public `Node::is_inline_with_only_block_children`, paints no shadow, background or border of its own (`paints_own_box`)»
  replace-with: «so a timer or an animation does not run during the wait; an inline box holding only block children paints no shadow, background or border of its own, unit tests reading the flag and no frame test reading the skip.»
