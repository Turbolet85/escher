# Apply — architecture · part 2 of 4

## Verdicts
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:39#1
- fanout-architecture--cascade-g2.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:45#1
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:57+#1
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:67+#1
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:69+#1
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:75#1
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:76#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:76+#1
- fanout-architecture--cascade-g2.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:77#1
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture-detail/established-decisions.md:79#1
- fanout-architecture--cascade-g2.md 11 · applied · .andromeda/architecture-detail/established-decisions.md:85#1
- fanout-architecture--cascade-g2.md 12 · applied · .andromeda/architecture-detail/established-decisions.md:88+#1
- fanout-architecture--cascade-g2.md 13 · applied · .andromeda/architecture-detail/established-decisions.md:88+#2
- fanout-architecture--cascade-g2.md 14 · applied · .andromeda/architecture-detail/established-decisions.md:101#1
- fanout-architecture--cascade-g2.md 15 · applied · .andromeda/architecture-detail/established-decisions.md:101#2
- fanout-architecture--cascade-g3.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:163+#1

## Edits
- line: .andromeda/architecture-detail/established-decisions.md:39
  old: «with `layout.threads` -1 (packages/blitz-dom/src/document.rs:90-100);»
  new: «with `layout.threads` -1 (packages/blitz-dom/src/document.rs:90-100), `pub(crate) fn init_stylo_prefs`, behind a `Once`, setting them, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 pref names, upstream's 0.23 `set_pref!` names not written (packages/blitz-dom/src/document.rs:87-102);»
- line: .andromeda/architecture-detail/established-decisions.md:45
  old: «tests/blitz-tests/tests/render_blocking_stylesheet.rs:122-124); a `<link>` starts loading once it has both `rel=stylesheet` and an `href` (tests/blitz-tests/tests/link_rel_attribute.rs:1-7);»
  new: «tests/blitz-tests/tests/render_blocking_stylesheet.rs:122-124), and a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-dom/src/net.rs:144-146; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91); a `<link>` starts loading once it has both `rel=stylesheet` and an `href` (tests/blitz-tests/tests/link_rel_attribute.rs:1-7), and a bad `href`, `doc.url.resolve_relative` refusing it, starts no load (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107);»
- after: .andromeda/architecture-detail/established-decisions.md:57
  anchor: «- **[Unsupported features]** Shadow DOM not implemented (packages/blitz-dom/src/stylo.rs:278-287; packages/blitz-dom/src/stylo.rs:359-362; packages/blitz-dom/src/stylo.rs:634-643); container queries disabled by a default size (packages/blitz-dom/src/stylo.rs:1236-1242); 3D transforms dropped, z-axis rotate applied as planar (packages/blitz-dom/src/stylo_to_kurbo.rs:40-46; packages/blitz-dom/src/stylo_to_kurbo.rs:64-73; tests/blitz-tests/tests/rotate_z_axis.rs:1-8; tests/blitz-tests/tests/rotate_z_axis.rs:59-63); CSS `offset` not supported in transforms (packages/blitz-dom/src/stylo_to_kurbo.rs:81-82); visited-link styling disabled (packages/blitz-dom/src/stylo.rs:133; packages/blitz-dom/src/stylo.rs:213); `:focus-visible`, `:valid`, `:invalid`, `:required`, `:target` always match false (packages/blitz-dom/src/stylo.rs:463-511), while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (packages/blitz-dom/src/stylo.rs:480-482; [DOM semantics]); anchor positioning "flagged off for time being" with size variants hitting `unreachable!()` (packages/stylo_taffy/src/convert.rs:122-124; packages/stylo_taffy/src/convert.rs:142-144; packages/stylo_taffy/src/convert.rs:197-200); grid subgrid and masonry convert to none / empty (packages/stylo_taffy/src/wrapper.rs:791-793; packages/stylo_taffy/src/convert.rs:811-813); `text-indent`'s `hanging` and `each-line` flags are read from the style and handed to Parley (packages/blitz-dom/src/layout/inline/layout.rs:486-489), and that they do not work, their parsing cfg'd out in Stylo, is recorded, not established: the upstream comment that stated it left the file at the 2026-10-10 sync, stylo unmoved at 0.22.0, and no reading of either flag was taken; pseudo-element `content` supports only strings (packages/blitz-dom/src/layout/construct.rs:663-680); anonymous table wrapper boxes are not generated (packages/blitz-dom/src/layout/construct.rs:585-604)»
  add: « ; `background-clip: text` is not supported, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22, and upstream `ac6f9f582`, which paints it, stays left out (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- after: .andromeda/architecture-detail/established-decisions.md:67
  anchor: « flagged upstreamable against upstream `main` at 434e0661, which has no range interaction, and no upstream pull request is open; its merge surface is `node/element.rs`, `node/mod.rs`, `layout/construct.rs`, `mutator.rs`, `events/keyboard.rs`, `events/pointer.rs` and `form.rs`, with the new `node/range.rs`, read at upstream `main` `e72650a6` on 2026-10-15, three commits past that base, and untouched by upstream since it — `git diff --stat 434e0661 e72650a6` over it empty, as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/merge-surface.md — as measured at escher-0.1.0/chunks/2026-10-13-range-input-interaction/report.md; at upstream `main` `ebacd68bd` the taken commits reach `node/element.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) as a second independent `if` (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md); three more direct engine changes for every document stand beside those since 2026-10-13-stand-keyboard-harness, the sixth to the eighth: keyboard activation — a pressed `Enter` whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it is no disabled form control ([DOM semantics]) and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`, and a released Space on the focused node dispatches the same Click, through the new `handle_key_up`, for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled — a link takes no Space, and a text input and a range input take neither key there — the Click's data built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers run before the default action `handle_click` (packages/blitz-dom/src/events/keyboard.rs:124-129; packages/blitz-dom/src/events/keyboard.rs:134-154; packages/blitz-dom/src/events/keyboard.rs:329-344; packages/blitz-dom/src/events/keyboard.rs:346-360; packages/blitz-dom/src/events/keyboard.rs:362-372;»
  add: « the range model's merge surface at upstream `main` `e42ee1f25`: the twelve commits taken reach `node/element.rs`, `layout/construct.rs` and `mutator.rs` (packages/blitz-dom/src/node/element.rs:902-906; packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/mutator.rs:689-735);»
- after: .andromeda/architecture-detail/established-decisions.md:69
  anchor: « — `git diff --stat 434e0661 e72650a6` over it empty, as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/merge-surface.md — as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md; at upstream `main` `ebacd68bd` the taken commits reach `node/element.rs` (`ElementData::flush_open_state` and its call from `ElementData::new`), with no conflict, and leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md); a user scroll aborts a smooth scroll (packages/blitz-dom/src/scrolling.rs:424-426; packages/blitz-dom/src/scrolling.rs:135-139); smooth scrolls last 300 ms, cubic ease-in-out (packages/blitz-dom/src/scrolling.rs:112-122; packages/blitz-dom/src/scrolling.rs:432-433); fling decelerates 0.95 per 60 fps frame and stops below 0.1 (packages/blitz-dom/src/scrolling.rs:894-908), velocity from the last 100 ms of pan samples, doubled (packages/blitz-dom/src/events/pointer.rs:113-154); wheel line deltas × 20 (packages/blitz-dom/src/events/pointer.rs:1189-1192), a wheel over an open select list scrolling the list alone and a wheel elsewhere closing the list and then scrolling (packages/blitz-dom/src/events/pointer.rs:1194-1202); drags start beyond 2 px (packages/blitz-dom/src/events/pointer.rs:222-229); repeated clicks within 500 ms and 2 px, read on the document's clock (packages/blitz-dom/src/events/pointer.rs:397-407); platform action modifier `META` on macOS (`SUPER` until keyboard-types 0.8, merged at the 2026-10-17 upstream sync), `CONTROL` elsewhere (packages/blitz-dom/src/util.rs:7-10); fragment-only link changes scroll in page (packages/blitz-dom/src/events/pointer.rs:1009-1022; packages/blitz-dom/src/url.rs:23-31); empty or unmatched `top` fragments scroll to top (packages/blitz-dom/src/scrolling.rs:840-844)»
  add: « ; keyboard activation's merge surface at upstream `main` `e42ee1f25`: the twelve commits taken reach `node/element.rs` (`ElementData::set_custom_widget`'s replay) and leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched (packages/blitz-dom/src/node/element.rs:902-906)»
- line: .andromeda/architecture-detail/established-decisions.md:75
  old: «re-read row by row with its owner quoted (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/readings.md), its owner Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  new: «re-read row by row, each row given a named outcome and nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md), its owner «Upstream sync ahead of determinism and exploration».»
- line: .andromeda/architecture-detail/established-decisions.md:76
  old: «with the non-invertible answer for Epoch 3's sync, their merge surface a line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md).»
  new: «with the non-invertible answer for the next sync, `0980cce5f` being left out at `e42ee1f25` (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md), their merge surface a line owned by «Upstream sync ahead of determinism and exploration» (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md).»
- after: .andromeda/architecture-detail/established-decisions.md:76
  anchor: «- Eight more ratified direct engine changes for every document stand beside those seven: `visible_region` reading a clipping box through its transforms and a non-invertible transform taking no hit (§Standard Contracts → Scrolling, selection, tree); the focus fixup ([DOM semantics]); the tree's focus falling back to the window and a native control's `toggled` on a checkable role (§Standard Contracts → Mutation, query and CSSOM); the `multiple` select (below) and `optgroup` ([DOM semantics]); and the engine's `change` on a checkbox and a radio (§Standard Contracts → Events). Each is flagged upstreamable and nothing is sent; upstream `0980cce5f` has no hunk on `visible_region`, and its `hit_inner` hunk is a textual conflict with the non-invertible answer for Epoch 3's sync, their merge surface a line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md).»
  add: «- A dropped anonymous block clearing its children's `layout_parent` stands beside those eight, a direct engine change for every document, ratified: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, and every reader of a stale `layout_parent` is cured with it, the id no longer stale; focus stays and the key still acts, `DocumentMutator::flush` reading `is_inert()` alone. Flagged upstreamable and nothing is sent; its merge surface a line owned by «Upstream sync ahead of determinism and exploration» (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
- line: .andromeda/architecture-detail/established-decisions.md:77
  old: «`packages/blitz-dom/src/events/pointer.rs` — owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  new: «`packages/blitz-dom/src/events/pointer.rs` — owned by the same `.andromeda/residuals.md` line, whose owner is «Upstream sync ahead of determinism and exploration».»
- line: .andromeda/architecture-detail/established-decisions.md:79
  old: «`runtime/events.rs` — is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  new: «`runtime/events.rs` — is owned by the same `.andromeda/residuals.md` line, whose owner is «Upstream sync ahead of determinism and exploration».»
- line: .andromeda/architecture-detail/established-decisions.md:85
  old: «its line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at»
  new: «its line owned by «Upstream sync ahead of determinism and exploration» (as measured at»
- after: .andromeda/architecture-detail/established-decisions.md:88
  anchor: «- Six behaviour changes for every document are upstream's and are not counted among escher's direct engine changes above: `:open` ([DOM semantics]); the `::before` / `::after` flow vote and IME at a focused input's editor construction ([Layout]); the windowed shell's redraw route ([Shell and windowing]); the `file:` script-load refusal (§Standard Contracts → JavaScript); and the CSS property accessors on `CSSStyleDeclaration`. Each stands ratified for its take under the playbook pattern for an engine behaviour change for every document; the `file:` refusal is a narrowing and the accessors a realization on the registered script-to-DOM crossing (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md).»
  add: «- Twelve more behaviour changes for every document are upstream's and are not counted among escher's direct engine changes above, each ratified for its take, by its commit, under the playbook pattern for an engine behaviour change for every document: `fe33b778b` `8b1a0a3ce` `dee854c76` `f6edb9701` `6fca6308d` `7a3dd8918` `91ef65e21` `a26c6ddeb` `f76777766` `6ef4b532d` `ae774d7f4` `e42ee1f25`. No boundary is widened in them, `URLSearchParams` a realization on the registered script crossing (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
- after: .andromeda/architecture-detail/established-decisions.md:88
  anchor: «- Six behaviour changes for every document are upstream's and are not counted among escher's direct engine changes above: `:open` ([DOM semantics]); the `::before` / `::after` flow vote and IME at a focused input's editor construction ([Layout]); the windowed shell's redraw route ([Shell and windowing]); the `file:` script-load refusal (§Standard Contracts → JavaScript); and the CSS property accessors on `CSSStyleDeclaration`. Each stands ratified for its take under the playbook pattern for an engine behaviour change for every document; the `file:` refusal is a narrowing and the accessors a realization on the registered script-to-DOM crossing (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md).»
  add: «- `AGENTS.md` from upstream `22f0cd000` stays out of the tree: no build reads it and its instructions contradict this fork's (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
- line: .andromeda/architecture-detail/established-decisions.md:101
  old: «hoisted offsets computed at most once per geometry generation (tests/blitz-tests/tests/paint_tree_incremental.rs:275-316)»
  new: «hoisted offsets computed at most once per geometry generation (tests/blitz-tests/tests/paint_tree_incremental.rs:275-316); `<picture>` is not `display: contents`, `packages/blitz-dom/assets/default.css` holding no `picture` rule and every select rule standing (packages/blitz-dom/src/document.rs:4061-4094)»
- line: .andromeda/architecture-detail/established-decisions.md:101
  old: «text-shaping changes yield `ALL_DAMAGE`,»
  new: «text-shaping changes and, through `compute_layout_damage`'s `table_needs_rebuild`, a table box's position, padding or border change (packages/blitz-dom/src/layout/damage.rs:442-458) yield `ALL_DAMAGE`,»
- after: .andromeda/architecture-detail/established-decisions.md:163
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-painting -->»
  add: «»
  add: «- `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform — an upstream engine change for every document, upstream #1342, a port of `e42ee1f25`, upstream's own standing on the untaken `ac6f9f582` (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = 'its owner Epoch 3'

[[pattern]]
id = "arch2-02"
fixed = 'answer for Epoch 3'

[[pattern]]
id = "arch2-03"
fixed = 'owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3'

[[pattern]]
id = "arch2-04"
fixed = 'is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3'

[[pattern]]
id = "arch2-05"
fixed = 'its line owned by Epoch 3'
