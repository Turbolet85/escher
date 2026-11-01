# Apply — architecture · part 1 of 4

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:205#1, .andromeda/architecture.md:205#2
- fanout-architecture--cascade-g9.md 3 · folded into fanout-architecture.md 1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:314#1
- fanout-architecture--cascade-g9.md 11 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:177#1
- fanout-architecture--cascade-g8.md 2 · folded into fanout-architecture.md 3
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:291#1
- fanout-architecture--cascade-g9.md 9 · folded into fanout-architecture.md 4
- fanout-architecture.md 5 · applied · .andromeda/architecture.md:62#1
- fanout-architecture--cascade-g2.md 17 · folded into fanout-architecture.md 5
- fanout-architecture.md 6 · applied · .andromeda/architecture.md:60#1
- fanout-architecture--cascade-g2.md 16 · folded into fanout-architecture.md 6
- fanout-architecture.md 7 · applied · .andromeda/architecture.md:63+#1
- fanout-architecture.md 8 · applied · .andromeda/architecture.md:254#1
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/design-philosophy.md:53#1
- cascade-compare-architecture.md 2 · applied · .andromeda/architecture-detail/design-philosophy.md:25+#1
- fanout-architecture--cascade-g1.md 2 · applied · .andromeda/architecture-detail/stack-and-technologies.md:22#1
- fanout-architecture--cascade-g1.md 3 · applied · .andromeda/architecture-detail/stack-and-technologies.md:76#1
- fanout-architecture--cascade-g10.md 1 · applied · .andromeda/architecture-detail/existing-scopes.md:58#1
- fanout-architecture--cascade-g10.md 2 · applied · .andromeda/architecture-detail/existing-scopes.md:61#1
- fanout-architecture--cascade-g10.md 3 · applied · .andromeda/architecture-detail/existing-scopes.md:137#1, .andromeda/architecture-detail/existing-scopes.md:137#2
- fanout-architecture--cascade-g10.md 4 · applied · .andromeda/architecture-detail/existing-scopes.md:146#1
- fanout-architecture--cascade-g10.md 5 · applied · .andromeda/architecture-detail/existing-scopes.md:153#1

## Edits
- line: .andromeda/architecture.md:205
  old: «no network port, TCP or UDP; one local listener,»
  new: «no served network port, TCP or UDP, blitz-net's tests alone binding TCP on loopback, served by no binary and reached by no caller, so no crossing; one local listener,»
- line: .andromeda/architecture.md:205
  old: «still no second listener»
  new: «still no second served listener»
- line: .andromeda/architecture.md:314
  old: «no network (TCP or UDP) listener»
  new: «no served network (TCP or UDP) listener»
- line: .andromeda/architecture.md:177
  old: «`__blitz_` globals carry messages and synchronous fetches; a JS bootstrap defines history, fetch, DOMException and the CSSOM classes;»
  new: «`__blitz_` globals carry messages and synchronous fetches, and `__blitz_parse_search_params`, the bootstrap's own, is deleted from `globalThis` before any page script; a JS bootstrap defines history, fetch, DOMException, the CSSOM classes and `URLSearchParams` with `URL.prototype.searchParams`, `href` and `search` live both ways;»
- line: .andromeda/architecture.md:291
  old: «upstream `main` last read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and sixteen left out (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md)»
  new: «upstream `main` last read at `e42ee1f25`, thirty-six of the fifty-seven commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and twenty-one left out (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md)»
- line: .andromeda/architecture.md:62
  old: «a line owned by Epoch 3's Upstream sync ahead of the UI graph»
  new: «a line owned by Upstream sync ahead of determinism and exploration»
- line: .andromeda/architecture.md:60
  old: «the merge surface of those eight is a line owned by Epoch 3's sync,»
  new: «the merge surface of those eight is a line owned by Upstream sync ahead of determinism and exploration,»
- after: .andromeda/architecture.md:63
  anchor: «»
  next: «- The select model: every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, and a single select's choice of its chosen option fires nothing; a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge; a click, Enter and a Space with no typed text standing open a single select's list, which the engine paints over the page with a group's `label` as a header, and a bound label's click focuses the select and opens nothing; an option of a closed single select, under a group too, takes no click and stays refused `covered`; a `multiple` select's `size` hints its height, a single select's stays unread, and Ctrl+A is not modelled.»
  add: «- The twelve commits of the upstream take read at `e42ee1f25` are engine behaviour changes for every document, ratified for their take; the key-press panic's fix, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block, is one more direct engine change for every document, ratified, flagged upstreamable with nothing sent upstream, its merge surface a line owned by Upstream sync ahead of determinism and exploration.»
  add: «»
- line: .andromeda/architecture.md:254
  old: «stamped with the document's `AbortSignal` (packages/blitz-dom/src/net.rs:29-35; packages/blitz-dom/src/document.rs:643-647; packages/blitz-dom/src/config.rs:62-65);»
  new: «stamped with the document's `AbortSignal` (packages/blitz-dom/src/net.rs:29-35; packages/blitz-dom/src/document.rs:643-647; packages/blitz-dom/src/config.rs:62-65); a failed fetch reaches the handler through `NetHandler::error`, a public default that does nothing, blitz-net's error arm calling it before the waker, and `ResourceHandler<StylesheetHandler>::error` responds `Err`, so the request leaves `pending_critical_resources` and a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-traits/src/net.rs:36-38; packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316; packages/blitz-dom/src/net.rs:144-146);»
- line: .andromeda/architecture-detail/design-philosophy.md:53
  old: «packages/blitz-dom/src/layout/mod.rs:856-858), other fields living on element data»
  new: «packages/blitz-dom/src/layout/mod.rs:856-858), and one box-construction fact, the flag `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, which `collect_layout_children_with_wrap` sets and the public `Node::is_inline_with_only_block_children` reads, `render_element` skipping such a box's own shadow, background and border (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599), other fields living on element data»
- after: .andromeda/architecture-detail/design-philosophy.md:25
  anchor: «  - blitz-net provides networking (HTTP, filesystem, Data URIs) as an implementation of the `blitz_traits::net::NetProvider` trait (packages/blitz-net/src/lib.rs:1-3)»
  add: «  - `NetHandler::error`, on the public trait, is a default that does nothing (packages/blitz-traits/src/net.rs:36-38); blitz-net's error arm calls it (packages/blitz-net/src/lib.rs:313) and blitz-dom's stylesheet handler responds with the error (packages/blitz-dom/src/net.rs:144-146), so a failed stylesheet stops blocking render, and a wrapping handler must forward it or the failure stops at the wrapper, as blitz-test-harness's `CountedHandler` forwards it (packages/blitz-test-harness/src/settle.rs:121-125)»
- line: .andromeda/architecture-detail/stack-and-technologies.md:22
  old: «with no `pub use` of it, the comment at lib.rs:96-97 heading none (packages/blitz-dom/src/resolve.rs:459-463; packages/blitz-dom/src/lib.rs:96-127)»
  new: «with no `pub use` of it (packages/blitz-dom/src/resolve.rs:459-463; packages/blitz-dom/src/lib.rs:115-125)»
- line: .andromeda/architecture-detail/stack-and-technologies.md:76
  old: «(fmt → clippy → test → ci-scripts, stopping at the first red)»
  new: «(fmt → clippy → test → ci-scripts → embolden, stopping at the first red)»
- line: .andromeda/architecture-detail/existing-scopes.md:58
  old: «Modules color, debug_overlay, filters,»
  new: «Modules blend (private, its `pub(crate) convert_blend_mode` converting Stylo's blend mode: packages/blitz-paint/src/blend.rs:5-26), color, debug_overlay, filters,»
- line: .andromeda/architecture-detail/existing-scopes.md:61
  old: «integration tests tests/dom.rs, tests/preact.rs and tests/selection.rs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/preact.rs:1-2; packages/blitz-vibey-script/tests/selection.rs:1-266)»
  new: «integration tests tests/dom.rs, tests/preact.rs, tests/selection.rs and tests/url_search_params.rs, the last 4 tests of `URLSearchParams` and `URL.prototype.searchParams` (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/preact.rs:1-2; packages/blitz-vibey-script/tests/selection.rs:1-266; packages/blitz-vibey-script/tests/url_search_params.rs:15-95)»
- line: .andromeda/architecture-detail/existing-scopes.md:137
  old: «its screen 47,949 bytes in 740 lines»
  new: «its screen 47,089 bytes in 740 lines»
- line: .andromeda/architecture-detail/existing-scopes.md:137
  old: «the 5 and the 3 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «the 5 and the 3 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the 47,089 at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- line: .andromeda/architecture-detail/existing-scopes.md:146
  old: «with nine JS bootstrap scripts under `runtime/bootstrap/` — `css`,»
  new: «with ten JS bootstrap scripts under `runtime/bootstrap/` — `url_search_params`, which `BOOTSTRAP_JS` takes first, and `css`,»
- line: .andromeda/architecture-detail/existing-scopes.md:153
  old: «(tests/blitz-tests/tests/text_range_no_change_event.rs:59-97).»
  new: «(tests/blitz-tests/tests/text_range_no_change_event.rs:59-97); `key_press_after_container_lost_its_box`, 3 tests — a key on a control whose anonymous block was dropped, by `hidden`, `style="display:none"` or CSS alone, not panicking, a button's and a select's, the select's choice still moving past its dropped block, and a control in no anonymous block reading the same; no `stand_` prefix (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187).»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'no network port, TCP or UDP'

[[pattern]]
id = "arch1-02"
fixed = 'still no second listener'

[[pattern]]
id = "arch1-03"
fixed = 'no network (TCP or UDP) listener'

[[pattern]]
id = "arch1-04"
fixed = 'DOMException and the CSSOM classes'

[[pattern]]
id = "arch1-05"
fixed = 'twenty-four of the forty commits'

[[pattern]]
id = "arch1-06"
regex = 'owned by Epoch 3.s Upstream sync ahead of the UI graph'

[[pattern]]
id = "arch1-07"
regex = 'those eight is a line owned by Epoch 3.s sync'

[[pattern]]
id = "arch1-08"
fixed = 'the comment at lib.rs:96-97 heading none'

[[pattern]]
id = "arch1-09"
fixed = 'clippy → test → ci-scripts, stopping'

[[pattern]]
id = "arch1-10"
fixed = 'Modules color, debug_overlay, filters'

[[pattern]]
id = "arch1-11"
fixed = 'tests/preact.rs and tests/selection.rs'

[[pattern]]
id = "arch1-12"
fixed = 'its screen 47,949 bytes'

[[pattern]]
id = "arch1-13"
fixed = 'nine JS bootstrap scripts'
