proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 103
    section: "## Standard Contracts"
    change: "Add a continuation line after this one: since 2026-11-01-upstream-sync-ui-graph, `NetHandler::error(self: Box<Self>, url: String, error: String)` stands beside `bytes` on the public trait, a default that does nothing (packages/blitz-traits/src/net.rs:36-38); blitz-net's error arm calls it, `handler.error(url, e.to_string())` (packages/blitz-net/src/lib.rs:313), and its waker moves after the handler (packages/blitz-net/src/lib.rs:316)."
    sidecar: "Provider traits: `NetHandler::error` recorded beside `bytes`, a do-nothing default, blitz-net's error arm its caller."
    rationale: "report.md:14 states the new public trait method and blitz-net's call; report.md:48 marks standard-contracts line 103 changed, `error` beside `bytes`."
    basis: packages/blitz-traits/src/net.rs:36-38
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 109
    section: "## Standard Contracts"
    change: "In blitz-net's `Provider`, replace «waking the waker for `doc_id` then calling `handler.bytes`» with «calling `handler.bytes` on a success or `handler.error` on a failure, then waking the waker for `doc_id`»."
    sidecar: "HTML and networking crates: blitz-net's `Provider` calls the handler first, `bytes` or `error`, and wakes after it."
    rationale: "report.md:14: `f6edb9701` gives blitz-net's error arm `handler.error` and moves the waker after the handler, so the row's order contradicts the changed file."
    basis: packages/blitz-net/src/lib.rs:316
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 116
    section: "## Standard Contracts"
    change: "Add a continuation line: since 2026-11-01-upstream-sync-ui-graph, `mix-blend-mode` and `isolation` paint in every document: blitz-paint's private module `blend` holds `pub(crate) convert_blend_mode` (packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/blend.rs:5-26), `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, and `render_element` pushes a viewport-wide blend layer and pops it last (packages/blitz-paint/src/render.rs:485-506); `paint_select_list`'s call is unchanged, so an open list still paints last, past the blend layers."
    sidecar: "Paint and shell: `mix-blend-mode` and `isolation` now paint, through the private `blend` module and a viewport-wide blend layer."
    rationale: "report.md:15 states the blend paint; report.md:94 says no paint-contract row holds it, a new sentence; report.md:147 holds the list painting last, past the blend layers."
    basis: packages/blitz-paint/src/blend.rs:5-26
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 116
    section: "## Standard Contracts"
    change: "Add a continuation line after the blend line: the blend has no frame witness: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; its unit tests are the 3 of `packages/blitz-paint/src/layers.rs` (packages/blitz-paint/src/layers.rs:104-181)."
    sidecar: "Paint and shell: blending's measured limit recorded, 3 unit tests in `layers.rs` and no frame test."
    rationale: "report.md:104 reads ✗ on a frame test for `mix-blend-mode` and `isolation`; report.md:172 gives that ✗ an owner at P5, which this row does not name."
    basis: packages/blitz-paint/src/layers.rs:104-181
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 116
    section: "## Standard Contracts"
    change: "Add a continuation line after the blend line: `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`, so such an element is a stacking root (packages/blitz-dom/src/node/node.rs:1382-1384)."
    sidecar: "Paint and shell: a `mix-blend-mode` other than `normal` makes a stacking root, the `Node` arm that replaces its TODO."
    rationale: "report.md:15 states the arm and report.md:33 retires `Node`'s `TODO: mix-blend-mode`; report.md:45 binds it with design-system's depth-strategy line 20, the other side of this fact."
    basis: packages/blitz-dom/src/node/node.rs:1382-1384
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 116
    section: "## Standard Contracts"
    change: "Add a continuation line: `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, set in `collect_layout_children_with_wrap` (packages/blitz-dom/src/layout/construct.rs:587-599), has the public reader `Node::is_inline_with_only_block_children` (packages/blitz-dom/src/node/node.rs:531-537), and `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test, the flag held by the unit tests `inline_with_only_block_children_tests` (packages/blitz-dom/src/document.rs:4096-4296)."
    sidecar: "Paint and shell: an inline box with only block children paints no box of its own, read through a new public `Node` reader."
    rationale: "report.md:19 states the flag, the reader and the paint skip; report.md:106 reads ✗ on a frame test."
    basis: packages/blitz-dom/src/node/node.rs:531-537
    fact: F20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 131
    section: "## Standard Contracts"
    change: "In `text_contrast`'s `None` list, read «whose `opacity` is below 1, whose `filter` list is not empty, whose `background-image` holds a layer other than `none` or whose `mix-blend-mode` is other than `normal` (`paints_more_than_colour`, packages/blitz-test-harness/src/contrast.rs:154-156)», and add that `Session::lint`'s `contrast` reads the same, no second formula, and no stand row moved; a line past 2 000 chars wraps as this entity's lines already wrap."
    sidecar: "Test harness: `text_contrast` reads `None` past a `mix-blend-mode` other than `normal`, `Session::lint`'s `contrast` with it."
    rationale: "report.md:24 states the new `None` and the lint's reading; report.md:44 marks standard-contracts line 131 changed where it states the conditions."
    basis: packages/blitz-test-harness/src/contrast.rs:154-156
    fact: F19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 135
    section: "## Standard Contracts"
    change: "After «counts a request in flight until its handler is consumed or dropped» add: its handler `CountedHandler` forwards `NetHandler::error` (packages/blitz-test-harness/src/settle.rs:121-125), so a failed load reaches the document on a harness or driver session and ends the wait, returning where it would have run to `LOAD_WAIT`, rather than reading `NotSettled(Loads)` for good; `settle_waiting` is unedited."
    sidecar: "Test harness: the counter's handler forwards `NetHandler::error`, so a failed load ends the wait on a harness or driver session."
    rationale: "report.md:25 states the forward and says the harness and driver contracts owe that sentence; report.md:48 marks standard-contracts line 135 changed; report.md:85 records the fix stopping at `CountedHandler` until this forward."
    basis: packages/blitz-test-harness/src/settle.rs:121-125
    fact: F15
note: architecture.md:163 names `text_contrast` and states no `None` condition, so it stays true and no body edit is proposed despite report.md:44 and the task's Touches; F11, F12, F14 and F21-F24 belong beside entities not picked here; standard-contracts#14.1 already stands past the 8 KB entity limit before these additions.
