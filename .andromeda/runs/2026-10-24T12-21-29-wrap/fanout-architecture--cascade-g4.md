proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: "## Project Intent"
    change: "Replace the Upstream sync line with: Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a (merge commit 3eda07bc, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync's merge base, unmoved; upstream `main` last read at `f9cbb2756` (`f9cbb275628b8f3017df8f76150f5f32df218620`), 16 commits past `17de049b1`; taken by no-commit cherry-picks of upstream's text, each with its `(cherry picked from commit …)` trailer: first three, `fb8d6c15e` (the default cursor over no box, upstream #1220) alone (`275950dc0`), then `38ca4e777` (`:focus-within`, upstream #1226) and `17de049b1` (the WPT pin, upstream #1228) together (`79603d1ef`), and then, at 2026-10-24-upstream-sync-polish-and-ship, ten together on `34f6f051b`: `28f848d3b` (#1233, the inline layout module split) · `9eeebb302` (#1239, mimalloc in the browser and the WPT runner) · `e4ca5b9b5` (#1240, class atoms on `ElementData`) · `46d7cf993` (#1238, out-of-flow-ness on `Node`) · `fb297a38c` (#1243, `Parallel` style traversals serialised; `examples/multi_doc.rs`) · `50bf2f235` (#1246, a replaced element's natural height through `aspect-ratio`) · `f8723f495` (#1247, WPT runner) · `21edc92d9` (#1258) · `f102663b8` (#1244, dropped nodes skipped in hit testing) · `f9cbb2756` (#1264, the flake's toolchain); left out, waiting for escher's next version and owned by a `.andromeda/residuals.md` line (target: next): `06ecec886` (git Stylo 0.23-pre) with `3954ecb85` and `1c5265a8b`, the six Taffy-main commits — `b8f4ba6fd` and `9babd796a` (git Taffy `802e497b…`) with the four `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6` — and `286874cc1` (Dioxus 0.8.0-alpha.3), so Stylo stays \"0.22.0\", Taffy the registry \"0.15\" and Dioxus \"0.7.3\" with no git dependency; every partial take ratified, the last on the founder's «Десять коммитов (Рекомендую)» (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)"
    sidecar: "project-intent: Upstream sync now reads upstream `main` last read at `f9cbb2756`, ten more commits cherry-picked on `34f6f051b`, the merge base unmoved, Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8 left out for next (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Files lists the ten no-commit cherry-picks on `34f6f051b` with the merge base `23b9b26a9` unmoved (report.md:14); Cross-project reads upstream at `f9cbb2756`, 16 past `17de049b1`, and names the six left out plus CARRY 1's four (report.md:54); Expected amendments name project-intent.md:9 as changed, now `f9cbb2756`, ten taken, six Taffy-main waiting (report.md:76); the pins stay (report.md:32); the left-out delta becomes a residuals line, target next, on the operator's approval (report.md:66)"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:54
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 46
    section: "## Existing Scopes"
    change: "In blitz-dom: layout's What it is, replace \"`inline` (packages/blitz-dom/src/layout/inline.rs:84-195)\" with \"`inline`, a directory since 2026-10-24-upstream-sync-polish-and-ship (upstream #1233): `layout`, the inline layout pass (packages/blitz-dom/src/layout/inline/layout.rs:85-195), and `build`, `build_inline_layout_into` moved there from `construct` (packages/blitz-dom/src/layout/inline/build.rs:23-303), both re-exported `pub(crate)` by `inline/mod.rs`\""
    sidecar: "existing-scopes: blitz-dom layout's `inline` module is now the directory `inline/{build,layout,mod}.rs`, `build_inline_layout_into` moved from `construct` (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Files: `layout/inline.rs` renamed to `layout/inline/layout.rs`, new `inline/build.rs` and `inline/mod.rs` (report.md:9-10); Symbols: `build_inline_layout_into` moved from `construct.rs` to `inline/build.rs` (23-303), re-exported `pub(crate)` by `inline/mod.rs` (report.md:21); Crates/modules: `blitz-dom::layout::inline` now a directory (report.md:27); Expected amendments name existing-scopes.md:46 (report.md:89); New text lists `compute_inline_layout` at inline/layout.rs 85-195"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:21
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 103
    section: "## Existing Scopes"
    change: "In blitz-tests' What it is, after \"paint tree and hit testing (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4; tests/blitz-tests/tests/pointer_events.rs:1-4)\" add: \"and, since 2026-10-24-upstream-sync-polish-and-ship, upstream's `hit_test_dropped_nodes`, 1 test, its own target — a hit test skipping nodes dropped since the last resolve, read red with `f102663b8`'s engine hunks reversed (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)\""
    sidecar: "existing-scopes: blitz-tests gains upstream's `hit_test_dropped_nodes` target, 1 test (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Files: new `tests/blitz-tests/tests/hit_test_dropped_nodes.rs`, upstream's text (report.md:9); Counts: `hit_test_dropped_nodes` a new blitz-tests target, 1 test, 0 detail-file hits (report.md:48); Coverage: red with `f102663b8`'s engine hunks reversed, green restored (report.md:102, 106); New text lists `hit_test_skips_dropped_nodes` at 32-82"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:48
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 123
    section: "## Existing Scopes"
    change: "In Root examples' What it is, insert `multi_doc` between `inner_html` and `mutations` in the list with its cite `examples/multi_doc.rs:47-116` in order, and add to the exercised list: \"multi-document style resolution, `Parallel` or `Sequential` by `MULTI_DOC_SEQ`, each resolve timed, since 2026-10-24-upstream-sync-polish-and-ship (examples/multi_doc.rs:35-45; examples/multi_doc.rs:63-67)\""
    sidecar: "existing-scopes: the root examples gain `multi_doc`, upstream's multi-document style-resolution example (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Files: new `examples/multi_doc.rs` (116 lines, from `fb297a38c`) (report.md:9); Crates/modules: the root examples gain `multi_doc` (report.md:27); Expected amendments name existing-scopes.md:123 (report.md:89); New text lists `config` 35-45 with the `MULTI_DOC_SEQ` style-threading choice at 36-40, `timed_resolve` 63-67 and `main` 47-116"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:27
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 15
    section: "## Cross-cutting Patterns"
    change: "In Logging and timing, after the `paint_bench` item add: \"; `multi_doc` times each resolve with two `Instant::now()` reads around its `timed_resolve` closure and prints the result with one `println!` on stdout (examples/multi_doc.rs:61; examples/multi_doc.rs:64; examples/multi_doc.rs:63-67; examples/multi_doc.rs:111-115)\""
    sidecar: "cross-cutting-patterns: Logging and timing names `multi_doc`'s resolve timing and its one `println!` (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Symbols: `examples/multi_doc.rs` has one `println!` (111-115) and two `Instant::now()` (61, 64) (report.md:24); Coverage: its `println!` is example output on stdout (report.md:99); Expected amendments carry obs-plan §6's examples emitter for the same site (report.md:92)"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: "## Cross-cutting Patterns"
    change: "In Invalidation and state integrity, after \"stale slab ids tolerated by `get`/`contains_key` (…)\" add: \"; hit testing and the paint tree skip a node dropped since the last resolve — `Node::hit_inner` reads each id through `tree().get(id)` and continues past a missing one in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup, and the paint tree does the same — an engine change for every document ratified by the founder's «Десять коммитов (Рекомендую)» (2026-10-24) (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)\""
    sidecar: "cross-cutting-patterns: Invalidation and state integrity — hit testing and the paint tree skip nodes dropped since the last resolve (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Symbols: `Node::hit_inner` skips nodes dropped since the last resolve, `self.with(id)` → `self.tree().get(id)` with `continue` in four places, `paint_tree.rs` the same (report.md:22); one of three engine changes for every document ratified by inputs#I4 (report.md:26); tested by `hit_test_dropped_nodes` (report.md:102)"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:22
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: "## Cross-cutting Patterns"
    change: "In Invalidation and state integrity, after \"new Stylo element data marked `ALL_DAMAGE` (…)\" add: \"; two derived values are kept in step with their sources — `ElementData::classes`, the `class` attribute's atoms, filled by `ElementData::new` and kept in sync by `DocumentMutator::set_attribute`'s `class` arm and `remove_attribute`'s `classes.clear()`, read by Stylo's `has_class` and `each_class`; and `Node::out_of_flow`, recorded from the node's new primary style by `Node::record_out_of_flow` in the style traversal and at box construction, read by `LayoutPassState::is_out_of_flow` alone while `Node::is_out_of_flow` still reads the style (packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/node/element.rs:422-426; packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502; packages/blitz-dom/src/stylo.rs:586-590; packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/stylo.rs:1342-1344; packages/blitz-dom/src/layout/construct.rs:720; packages/blitz-dom/src/layout/mod.rs:856-858)\""
    sidecar: "cross-cutting-patterns: Invalidation and state integrity — `ElementData::classes` and `Node::out_of_flow` kept in step with the `class` attribute and the primary style (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Symbols: `Node::out_of_flow` written by the style traversal through `record_out_of_flow`, callers in `stylo.rs` and `construct.rs`, read only by `LayoutPassState::is_out_of_flow` (report.md:16); `ElementData::classes` filled by `ElementData::new`, kept in sync by the mutator's `class` arm and `remove_attribute`'s `classes.clear()`, read by `has_class`/`each_class` (report.md:17)"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:17
note: D-arch-resources sites for this chunk (`MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT`, `PARALLEL_TRAVERSAL_LOCK`, mimalloc as the browser's and WPT runner's global allocator) sit in Occupied Resources, outside group 4; the body's Project Intent line (architecture.md:286) moves with the project-intent proposal and is the body agent's.
