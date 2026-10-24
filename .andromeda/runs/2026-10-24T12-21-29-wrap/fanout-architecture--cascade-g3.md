proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 45
    section: "## Standard Contracts"
    change: "In Scrolling, selection, tree, after the hit walk's clause ending `(packages/blitz-dom/src/node/node.rs:1424-1554; tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:187-372 — …)`, add: since 2026-10-24-upstream-sync-polish-and-ship (upstream `f102663b8`, #1244, as written) the walk skips a node dropped since the last resolve — each child id is read through `self.tree().get(id)` and a missing one passed over with `continue`, where it read `self.with(id)`, in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup — and the paint tree does the same; `hit_inner` keeps its signature and its callers `hit` and `hit_with_scrollbar` are unedited; with the engine hunks reversed the new check panics at the old read (node.rs:1572); an engine change for every document, ratified by the founder's «Десять коммитов (Рекомендую)» (inputs#I4) (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)"
    sidecar: "standard-contracts — Scrolling, selection, tree: the hit walk skips nodes dropped since the last resolve (upstream f102663b8, 2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: hit testing (`Node::hit_inner`, node.rs added 1414-1417 doc, 1572-1574, 1578, 1587-1589, 1621-1623, 1627, 1646-1650) skips nodes dropped since the last resolve, `paint_tree.rs` (added 146-147, 156, 161, 164-172) the same; listed among the three engine changes for every document ratified by inputs#I4; Deviations: the red-before-green control panicked at node.rs:1572:26."
    basis: .andromeda/architecture-detail/standard-contracts.md:45
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 143
    section: "## Standard Contracts"
    change: "In the Scrolling, selection, tree line, after `a hit stops at the padding box of a box paint clips` add `and skips a node dropped since the last resolve`."
    sidecar: "architecture body — Scrolling, selection, tree: a hit skips dropped nodes."
    rationale: "Report Changes → Symbols: `Node::hit_inner` skips nodes dropped since the last resolve, an engine change for every document (inputs#I4); the broad line the detail's hit-walk clause sits under."
    basis: .andromeda/architecture.md:143
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 60
    section: "## Standard Contracts"
    change: "In Layout, after `replaced-element API IntrinsicSizes, ReplacedContext, compute_replaced_layout (packages/blitz-dom/src/layout/replaced.rs:23-57)` add: — since 2026-10-24-upstream-sync-polish-and-ship (upstream `50bf2f235`, #1246, as written) `compute_replaced_layout` coerces a replaced element's natural height through its `aspect-ratio`, keeping `is_usable_ratio` (finite and > 0), `.max(0.0)` and its signature, so its callers `layout` and `dispatch_child_layout` see the new height — an engine change for every document, ratified by the founder's «Десять коммитов (Рекомендую)» (inputs#I4) (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211)"
    sidecar: "standard-contracts — Layout: compute_replaced_layout coerces a replaced element's natural height through aspect-ratio (upstream 50bf2f235)."
    rationale: "Report Changes → Symbols: `compute_replaced_layout` (replaced.rs added 144-145, 201-211) coerces the natural height through `aspect-ratio`, keeping `is_usable_ratio` and `.max(0.0)` and its signature; one of the three ratified engine changes (inputs#I4); Expected amendments, pattern `replaced`: standard-contracts.md 1 hit, this entity."
    basis: .andromeda/architecture-detail/standard-contracts.md:60
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 67
    section: "## Standard Contracts"
    change: "In Nodes and widgets, after `node module re-exports (packages/blitz-dom/src/node/mod.rs:17-34)` add: ; since 2026-10-24-upstream-sync-polish-and-ship two public fields, upstream's as written (`46d7cf993`, `e4ca5b9b5`): `Node::out_of_flow: AtomicBool`, written by the style traversal from the node's new primary style through the crate-private `Node::record_out_of_flow` and `style_is_out_of_flow` (`position` `absolute` or `fixed`, and not `display: none`), and read only by the crate-private `LayoutPassState::is_out_of_flow`, while `Node::is_out_of_flow` keeps its signature and its style reading (packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/node/node.rs:2069-2076; packages/blitz-dom/src/layout/mod.rs:856-858); and `ElementData::classes: ThinVec<Atom>`, the `class` attribute's atoms, filled by `ElementData::new` and `Clone` through the crate-private `parse_classes`, kept in sync by `DocumentMutator::set_attribute`'s `class` arm and `remove_attribute`, and read by Stylo's `has_class` and `each_class` (packages/blitz-dom/src/node/element.rs:45-47; packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502; packages/blitz-dom/src/stylo.rs:586-590); `ElementData` is referenced outside blitz-dom by type only, never by a struct literal, so no caller changes"
    sidecar: "standard-contracts — Nodes and widgets: the public fields Node::out_of_flow and ElementData::classes registered (upstream 46d7cf993, e4ca5b9b5)."
    rationale: "Report Changes → Symbols: `Node::out_of_flow: AtomicBool` a new pub field (node.rs added 125-128) and `ElementData::classes: ThinVec<Atom>` a new pub field (element.rs added 56-61), with their writers and readers; a new public symbol unregistered in §Standard Contracts is D-arch-resources drift."
    basis: .andromeda/architecture-detail/standard-contracts.md:67
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 192
    section: "## Standard Contracts"
    change: "In High-level launch, after `dioxus-native launch(app), launch_cfg, launch_cfg_with_props (packages/dioxus-native/src/lib.rs:94-113)` add: — since 2026-10-24-upstream-sync-polish-and-ship (upstream `fb297a38c`, #1243, as written) `launch_cfg_with_props` builds its document with `style_threading: StyleThreading::Parallel`, so the windowed path (`seven_guis_native` through `dioxus_native::launch`) resolves styles in parallel under the process-wide `PARALLEL_TRAVERSAL_LOCK` (§Occupied Resources → Process-wide state and threads), while no headless document — harness, stand or driver — sets `style_threading`; an engine change ratified by the founder's «Десять коммитов (Рекомендую)» (inputs#I4), read locally by the windowed smoke alone (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222)"
    sidecar: "standard-contracts — High-level launch: dioxus_native::launch builds its document Parallel (upstream fb297a38c)."
    rationale: "Report Changes → Symbols: `dioxus_native::launch` (`launch_cfg_with_props`) builds its document with `StyleThreading::Parallel` (lib.rs added 63, 222), the windowed path; no headless document sets `style_threading`; one of the three ratified engine changes (inputs#I4); Coverage: the windowed smoke (gate entry 35) the one local reading."
    basis: .andromeda/architecture-detail/standard-contracts.md:192
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 229
    section: "## Standard Contracts"
    change: "In CLIs, after the `preact_script` clause add: `multi_doc` (a root example since 2026-10-24-upstream-sync-polish-and-ship, upstream's `fb297a38c` as written) reads a mode and an iteration count from `std::env::args` (examples/multi_doc.rs:48-54) and its document count, layout and threading from `MULTI_DOC_N` · `MULTI_DOC_FLAT` · `MULTI_DOC_SEQ` (§Occupied Resources → Environment variables), times each resolve with `Instant::now()` (examples/multi_doc.rs:61-67), prints one summary line on stdout (examples/multi_doc.rs:111-115) and panics on an unknown mode (examples/multi_doc.rs:108);"
    sidecar: "standard-contracts — CLIs: the root example multi_doc registered (upstream fb297a38c)."
    rationale: "Report Changes → Crates / modules: the root examples gain `multi_doc`; Symbols: `examples/multi_doc.rs` reads `std::env::args` (48-54), three env variables, one `println!` (111-115), two `Instant::now()` (61, 64) and a `panic!` on an unknown mode (108)."
    basis: .andromeda/architecture-detail/standard-contracts.md:229
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 187
    section: "## Standard Contracts"
    change: "In the CLIs line, add `multi_doc` to the examples' list: `(paint_bench, screenshot, preact_script, multi_doc, browser, rdme, wgpu_texture, wpt)`."
    sidecar: "architecture body — CLIs: multi_doc added to the examples' command lines."
    rationale: "Report Changes → Crates / modules: the root examples gain `multi_doc`, a command line reading its arguments; the broad line the detail's CLIs entity sits under."
    basis: .andromeda/architecture.md:187
    dependent-of: D-arch-resources
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 237
    section: "## Standard Contracts"
    change: "In Browser app, after `make_doc_config (apps/browser/src/document_loader.rs:45-64)` add: , which since 2026-10-24-upstream-sync-polish-and-ship (upstream `fb297a38c`, as written) sets `style_threading: StyleThreading::Parallel` (apps/browser/src/document_loader.rs:62), as View Source's document does (apps/browser/src/toolbar.rs:203)"
    sidecar: "standard-contracts — Browser app: the document config and View Source's document set StyleThreading::Parallel (upstream fb297a38c)."
    rationale: "Report Changes → Symbols: the browser's `document_loader.rs` (added 62) and `toolbar.rs` (added 203) build their documents `Parallel`, as `dioxus_native::launch` does."
    basis: .andromeda/architecture-detail/standard-contracts.md:237
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 38
    section: "## Occupied Resources"
    change: "In Process-wide state and threads, after `Stylo's global rayon pool under StyleThreading::Parallel (packages/blitz-dom/src/config.rs:21-24; packages/blitz-dom/src/stylo.rs:149-152)` add: — since 2026-10-24-upstream-sync-polish-and-ship (upstream `fb297a38c`, #1243, as written) a static `PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is taken around every `Parallel` traversal, so two documents resolving `Parallel` at once are serialised and no longer panic `already mutably borrowed` (blitz issue 430), its poison recovered by `unwrap_or_else(|e| e.into_inner())`, no panic path and no log site; `Parallel` is set by `dioxus_native::launch` and the browser's document configs and by no headless document (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163)"
    sidecar: "occupied-resources — Process-wide state and threads: PARALLEL_TRAVERSAL_LOCK registered (upstream fb297a38c)."
    rationale: "Report Changes → Symbols: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` new process-wide state in stylo.rs (added 29-35), taken around every `Parallel` traversal (added 157-163), poison recovered; Coverage: no log site (`evidence/sink.md`); Expected amendments name occupied-resources.md:38."
    basis: .andromeda/architecture-detail/occupied-resources.md:38
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 38
    section: "## Occupied Resources"
    change: "In Process-wide state and threads, after the `PARALLEL_TRAVERSAL_LOCK` clause add: ; a `#[global_allocator]` `mimalloc::MiMalloc` in the WPT runner (wpt/runner/src/main.rs:1-3) and, by its now-default `mimalloc` feature, in the browser (apps/browser/Cargo.toml:14) — since 2026-10-24-upstream-sync-polish-and-ship (upstream `9eeebb302`, #1239, as written), both upstream-only binaries, mimalloc 0.1.52 / libmimalloc-sys 0.1.49 already locked, libmimalloc-sys building vendored C through `cc`; no escher binary changes allocator"
    sidecar: "occupied-resources — Process-wide state and threads: mimalloc the global allocator of the browser (default) and the WPT runner (upstream 9eeebb302)."
    rationale: "Report Changes → Symbols: WPT runner `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (main.rs added 1-3); Dependencies: mimalloc now the default global allocator of `apps/browser` (`default` gains `mimalloc`, Cargo.toml added 14) and a direct dependency of the WPT runner, reaching only upstream-only binaries; Expected amendments name occupied-resources.md:38."
    basis: .andromeda/architecture-detail/occupied-resources.md:38
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 211
    section: "## Occupied Resources"
    change: "In the Process-wide state and threads line, after `Stylo's global rayon pool` add `, its `Parallel` traversals serialised by the process-wide `PARALLEL_TRAVERSAL_LOCK`` and after `the script timer thread` add `; mimalloc the global allocator of the browser (by default) and the WPT runner`."
    sidecar: "architecture body — Process-wide state and threads: the parallel traversal lock and mimalloc named."
    rationale: "Report Changes → Symbols (`PARALLEL_TRAVERSAL_LOCK`, the WPT runner's `#[global_allocator]`) and Dependencies (mimalloc the browser's default allocator); the broad line the detail's entity sits under."
    basis: .andromeda/architecture.md:211
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 55
    section: "## Occupied Resources"
    change: "In Environment variables, after `PAINT_TREE_BENCH_HTML (tests/blitz-tests/tests/paint_tree_bench.rs:258-269);` add: `MULTI_DOC_N` (a count parsed with a default fallback; examples/multi_doc.rs:55-58), `MULTI_DOC_SEQ` (sequential style threading; examples/multi_doc.rs:36-40) and `MULTI_DOC_FLAT` (examples/multi_doc.rs:22), the last two through its `env_flag` (examples/multi_doc.rs:17-19) — read by the root example `multi_doc` alone, through two `std::env::var` call sites, since 2026-10-24-upstream-sync-polish-and-ship (upstream's `fb297a38c` as written); no library or escher binary reads them;"
    sidecar: "occupied-resources — Environment variables: MULTI_DOC_N, MULTI_DOC_SEQ, MULTI_DOC_FLAT registered (examples/multi_doc.rs)."
    rationale: "Report Changes → Symbols: `examples/multi_doc.rs` reads three env variables through two `std::env::var` call sites — `MULTI_DOC_N` (55-58), `MULTI_DOC_SEQ` (36-40), `MULTI_DOC_FLAT` (22), the last two through `env_flag` (17-19); Expected amendments: a new registration at occupied-resources.md:55 (pattern `multi-doc` unfired, 0 detail hits); an unregistered env var is D-arch-resources drift."
    basis: .andromeda/architecture-detail/occupied-resources.md:55
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 216
    section: "## Occupied Resources"
    change: "In the Environment variables line, after `PAINT_TREE_BENCH_HTML;` add `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the `multi_doc` example alone;"
    sidecar: "architecture body — Environment variables: the multi_doc example's three variables named."
    rationale: "Report Changes → Symbols: `examples/multi_doc.rs` reads `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT`; the broad line the detail's entity sits under."
    basis: .andromeda/architecture.md:216
    dependent-of: D-arch-resources
note: The dependent-of body proposals (architecture.md 143, 187, 211, 216) restate their detail proposals on the broad line; drop them if the body's doc agent proposes the same lines. Every citation into files the take moved (config.rs, element.rs, node.rs, resolve.rs, attr_test.rs, the browser and dioxus-native lib.rs) is left to the citation sweep.
