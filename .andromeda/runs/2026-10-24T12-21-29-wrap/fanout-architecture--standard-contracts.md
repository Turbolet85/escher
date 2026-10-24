proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: >-
      - `StyleThreading` since the 2026-10-24 upstream sync (upstream #1243, `fb297a38c`; ratified by the founder, inputs#I4): every `StyleThreading::Parallel` traversal takes the process-wide static `PARALLEL_TRAVERSAL_LOCK: Mutex<()>`, its poison recovered by `unwrap_or_else(|e| e.into_inner())` with no panic path, so `Parallel` traversals are serialised process-wide and two documents resolving `Parallel` concurrently no longer panic with `already mutably borrowed` (blitz issue 430); the enum doc says so, and the `style_threading` field doc now names `Sequential` as the default, as the enum's `#[default]` already was — doc comments only, the `clock` field untouched; the windowed `dioxus_native::launch` and the browser set `Parallel`, and no headless document — harness, stand or driver — sets `style_threading` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/blitz-dom/src/config.rs:13-17; packages/blitz-dom/src/config.rs:23-24; packages/blitz-dom/src/config.rs:54) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Document core: StyleThreading's Parallel traversals serialised process-wide by PARALLEL_TRAVERSAL_LOCK; style_threading field doc names Sequential the default.
    rationale: >-
      The report's Changes → Symbols names a changed public config contract (the StyleThreading and style_threading docs) and new process-wide state taken around every Parallel traversal; the file's Document core lists `style_threading` among DocumentConfig's fields with no statement of its default or of Parallel's concurrency rule, so the changed contract is unregistered here.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:18
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 54
    section: "## Standard Contracts"
    change: >-
      - Dropped nodes in the hit walk (an engine change for every document, ratified by the founder, inputs#I4; upstream #1244, `f102663b8`): `Node::hit_inner` skips a node dropped since the last resolve — `self.with(id)` became `self.tree().get(id)` with `continue` in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup — and the paint tree skips such a node the same way; `hit_inner` keeps its signature and its callers `hit` and `hit_with_scrollbar` are unedited (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — the check read red with those engine hunks reversed, as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Scrolling, selection, tree: the hit walk and the paint tree skip nodes dropped since the last resolve (hit_test_dropped_nodes).
    rationale: >-
      The report's Changes → Symbols and its third ratified engine change: hit testing now skips dropped nodes; the file's Scrolling, selection, tree contract describes the hit walk (`Node::hit`, `hit_inner`) in full and says nothing of dropped nodes.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:22
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 61
    section: "## Standard Contracts"
    change: >-
      - A replaced element's natural height (an engine change for every document, ratified by the founder, inputs#I4; upstream #1246, `50bf2f235`): `compute_replaced_layout` coerces a replaced element's natural height through its `aspect-ratio`, keeping `is_usable_ratio` (finite and > 0), `.max(0.0)` and its signature; its callers `layout` and `dispatch_child_layout` see the new height (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Layout: compute_replaced_layout coerces a replaced element's natural height through aspect-ratio, signature and degenerate-ratio guard kept.
    rationale: >-
      The report's Changes → Symbols (`compute_replaced_layout`) and its first ratified engine change; the report's `replaced` pattern locates this file's one site, the Layout contract naming `compute_replaced_layout` as API with no statement of the new behaviour.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:23
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 75
    section: "## Standard Contracts"
    change: >-
      - `Node::out_of_flow: AtomicBool`, a public field since the 2026-10-24 upstream sync (upstream #1238, `46d7cf993`): written by the style traversal from the node's new primary style through the crate-private `Node::record_out_of_flow`, called from `stylo.rs` and `construct.rs`, beside the crate-private `style_is_out_of_flow(&ComputedValues) -> bool` — `position` `absolute` or `fixed` and not `display: none`; `Node::is_out_of_flow` keeps its signature and its style reading, now through `style_is_out_of_flow`, as do its other callers (`resolve.rs`, `writing_mode.rs`, `dispatch_child_layout`), while `LayoutPassState::is_out_of_flow` alone reads the recorded flag (packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/node/node.rs:2069-2076; packages/blitz-dom/src/layout/mod.rs:856-858) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Nodes and widgets: new public field Node::out_of_flow, recorded by the style traversal and read by LayoutPassState::is_out_of_flow alone.
    rationale: >-
      The report's Changes → Symbols names `Node::out_of_flow` a new pub field; the file's Nodes and widgets contract registers the node module's public surface and does not name it.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:16
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 75
    section: "## Standard Contracts"
    change: >-
      - `ElementData::classes: ThinVec<Atom>`, a public field since the 2026-10-24 upstream sync (upstream #1240, `e4ca5b9b5`): filled from the `class` attribute by `ElementData::new` and `Clone` through the crate-private `parse_classes`, re-exported `pub(crate)` from the node module; `DocumentMutator::set_attribute`'s `class` arm (after the `id` arm, ahead of the `value` arm) and `remove_attribute`'s `classes.clear()` keep it in sync, and Stylo's `has_class` and `each_class` read it; `ElementData` is named outside blitz-dom by type only, never by a struct literal, so no caller changes (packages/blitz-dom/src/node/element.rs:45-47; packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/node/element.rs:422-426; packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502; packages/blitz-dom/src/stylo.rs:586-590; packages/blitz-dom/src/stylo.rs:697-699) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Nodes and widgets: new public field ElementData::classes, kept in sync by the mutator and read by Stylo's class matching.
    rationale: >-
      The report's Changes → Symbols names `ElementData::classes` a new pub field; the file's Nodes and widgets contract registers `ElementData`'s public accessors and does not name it.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:17
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 192
    section: "## Standard Contracts"
    change: >-
      - `dioxus_native::launch`, through `launch_cfg_with_props`, builds its document with `style_threading: StyleThreading::Parallel` since the 2026-10-24 upstream sync (upstream #1243, `fb297a38c`; an engine change ratified by the founder, inputs#I4) — the windowed path, `seven_guis_native` among its callers, its style traversals serialised under the process-wide lock (Document core above); no headless document sets `style_threading`, and the windowed smoke is its one local reading (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      High-level launch: dioxus_native::launch now resolves styles Parallel under the process-wide lock.
    rationale: >-
      The report's Changes → Symbols and its third ratified engine change: the windowed `dioxus_native::launch` resolves styles `Parallel`; the file's High-level launch contract states `launch`, `launch_cfg` and `launch_cfg_with_props` with no document threading.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:20
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 231
    section: "## Standard Contracts"
    change: >-
      - `multi_doc`, a root example since the 2026-10-24 upstream sync (upstream's text from `fb297a38c`, `examples/multi_doc.rs`, run by `cargo run --example multi_doc`): it times document resolves (`timed_resolve`), its first two arguments a mode and an iteration count, and reads three environment variables through two `std::env::var` call sites — `MULTI_DOC_N` (parsed, a default on a parse failure), `MULTI_DOC_SEQ` (choosing the documents' `style_threading`) and `MULTI_DOC_FLAT` (shaping the generated page), the last two through `env_flag`; it prints one line on stdout, reads `Instant::now()` twice and panics on an unknown mode (examples/multi_doc.rs:17-19; examples/multi_doc.rs:22; examples/multi_doc.rs:36-40; examples/multi_doc.rs:48-54; examples/multi_doc.rs:55-58; examples/multi_doc.rs:63-67; examples/multi_doc.rs:108; examples/multi_doc.rs:111-115) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      CLIs: the new root example multi_doc, its arguments, its three MULTI_DOC_* env reads, its stdout line and its panic on an unknown mode.
    rationale: >-
      The report's Changes → Symbols and Crates / modules: the root examples gain `multi_doc`, reading `std::env::args` and three env variables; the file's CLIs contract registers every example's command line (paint_bench, screenshot, url, preact_script, wgpu_texture …) and does not name it.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 237
    section: "## Standard Contracts"
    change: >-
      - The browser's document configs — `make_doc_config` in `document_loader.rs` and the one `toolbar.rs` builds — set `style_threading: StyleThreading::Parallel` since the 2026-10-24 upstream sync (`fb297a38c`), their style traversals serialised under the process-wide lock (Document core above) (apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: >-
      Browser app: make_doc_config and the toolbar's document config set Parallel style threading.
    rationale: >-
      The report's Changes → Symbols: the browser's `document_loader.rs` (added 62) and `toolbar.rs` (added 203) set `Parallel`; the file's Browser app contract names `make_doc_config` with no threading.
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:20
note: No D-retired or D-arch-decisions drift in this file (it never states the retired Parallel-panic warning, and mimalloc/tokio belong to Stack); citation digit moves into the taken files are left to P2's cites.py sweep; the body's own stale [Style threading] line (architecture.md:44) and Upstream sync line (architecture.md:286) lie outside this category.
