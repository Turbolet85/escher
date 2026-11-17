@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=17 class=omission
leaf-text: Dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`;
source: dec-nxphwkzn «decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.»
replace-with: Dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, except the WPT runner's direct `mimalloc` dependency;
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=57 class=omission
leaf-text: Android entry `#[unsafe(no_mangle)] pub fn android_main`.
source: dec-nqb3jdw6 «decision/android-entry-point · Android entry points — Android entry points are `android_main` calling `set_android_app`.»
replace-with: Android entry `#[unsafe(no_mangle)] pub fn android_main`, calling `set_android_app`.
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=12 class=omission
leaf-text: taffy the one crate allowed a git source)
source: dec-qxbkesvi «decision/taffy-layout · Taffy for layout — Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's
replace-with: taffy the one crate allowed a git source; Taffy is part of blitz-dom's public API through its signatures, with no re-export)
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: then the viewport — a nested box at once, the requested behaviour for the viewport alone —
source: ctr-zara73ad «contract/scroll-hit/scroll-into-view · BaseDocument scroll and hit testing · scroll_into_view — scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nes
replace-with: then the viewport — a nested box at once with no scroll event, the requested behaviour for the viewport alone —
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `BoundingRect`, the public crate-root `TRACING_COMPILED`
source: ctr-gr7trxoq «contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.»
replace-with: `BoundingRect`, `Node::try_layout_data` (public and read-only: a node's layout data, `None` before the first layout), the public crate-root `TRACING_COMPILED`
