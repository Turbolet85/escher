# Cascade compare — architecture · wrap · 2026-10-24-upstream-sync-polish-and-ship
E items 37 · held 25 · lost at classify 5 · lost at strong 1 · cascade only 5 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### conventions#2.1 · lost at classify · .andromeda/architecture-detail/conventions.md:15
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: '## Conventions'
    change: 'Append to the **Manifests** entry (conv-manifests), before its docs.rs clause: the WPT runner takes `mimalloc` by its own version requirement, `mimalloc = "0.1.48"`, and not through `[workspace.dependencies]`, the root manifest unchanged by the take, a measured exception to "declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`" (wpt/runner/Cargo.toml:15)'
    sidecar: 'conventions.md Manifests: the WPT runner''s direct `mimalloc = "0.1.48"` recorded as an exception to the workspace-dependency convention (2026-10-24-upstream-sync-polish-and-ship, upstream 9eeebb302)'
    rationale: 'The report''s Changes → Dependencies (report.md:31) makes mimalloc "a direct dependency of the WPT runner (`mimalloc = "0.1.48"`, one added lock edge)", and Changes → Crates / modules (report.md:27) says "the root manifest is byte-identical" — so the new declaration is a version string, not `{ workspace = true }`, while conventions.md:15 states every dependency is declared once in `[workspace.dependencies]`; New text, by line lists wpt/runner/Cargo.toml added 15'
    basis: .andromeda/architecture-detail/conventions.md:15

### established-decisions#10.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:67
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 67
    section: '## Established Decisions'
    change: 'Range interaction''s merge surface, after "— `git diff --stat 434e0661 e72650a6` over it empty, as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/merge-surface.md" insert: "; at the 2026-10-24 upstream sync upstream''s taken hunks reached `node/element.rs`, `node/mod.rs`, `layout/construct.rs` and `mutator.rs`, the take''s one non-additive edit `construct.rs`''s import list, resolved to upstream''s plus `RangeInputData` (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)".'
    sidecar: 'established-decisions.md:67 — the range model''s merge surface met by the 2026-10-24 sync, its one conflict `construct.rs`''s import list resolved with `RangeInputData` kept.'
    rationale: 'Changes → Files (upstream''s hunks modify blitz-dom `layout/construct.rs`, `node/{node,element,mod}.rs`, `mutator.rs`) and Symbols: "`construct.rs`''s import list is resolved to upstream''s plus our `RangeInputData` (the take''s one non-additive edit, `evidence/take.md`)"; the file states this surface only as read untouched at `e72650a6`.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:21

### established-decisions#10.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:69
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 69
    section: '## Established Decisions'
    change: 'Keyboard activation''s merge surface, after "— `git diff --stat 434e0661 e72650a6` over it empty, as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/merge-surface.md" insert: "; at the 2026-10-24 upstream sync upstream''s taken hunks reached `node/element.rs`, additively, with no conflict (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)".'
    sidecar: 'established-decisions.md:69 — keyboard activation''s merge surface (`node/element.rs`) met additively by the 2026-10-24 sync.'
    rationale: 'Changes → Files: upstream''s hunks modify `node/element.rs` (the `ElementData::classes` field, `parse_classes`, `Clone`, `ElementData::new`), a file of this surface; the take''s one conflict was `construct.rs`''s import line (Symbols), so element.rs took upstream''s hunks without one.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:17
    dependent-of: D-arch-decisions

### occupied-resources#6.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:49
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 49
    section: "## Occupied Resources"
    change: "New record after line 49, in the process-wide state block: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` in blitz-dom's `stylo.rs` (since 2026-10-24-upstream-sync-polish-and-ship, upstream `fb297a38c`, #1243) — process-wide, taken around every `StyleThreading::Parallel` traversal, so two documents resolving `Parallel` concurrently on Stylo's global rayon pool are serialised and no longer panic `already mutably borrowed` (blitz issue 430); poison recovered by `unwrap_or_else(|e| e.into_inner())`, no panic path, no log site; `Sequential` stays the default; the windowed `dioxus_native::launch` (`seven_guis_native`) and the browser's document loader and toolbar now build their documents `Parallel`, while no headless document — harness, stand or driver — sets `style_threading` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222; apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "occupied-resources: Process-wide state registers `PARALLEL_TRAVERSAL_LOCK` and the windowed and browser launches resolving `Parallel` under it (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is new process-wide state in stylo.rs (added 29-35, taken at 157-163), and `dioxus_native::launch` plus the browser's loader and toolbar now set `StyleThreading::Parallel`; Expected amendments names occupied-resources.md:38 (Process-wide state and threads) for it. The file's process-wide record names Stylo's global rayon pool but no lock and no `Parallel` launch. A separate record keeps line 38 under the 2 000-char entity bound."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:18

### occupied-resources#6.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:49
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 49
    section: "## Occupied Resources"
    change: "New record after the lock's record: the global allocator of two upstream-only binaries (since 2026-10-24-upstream-sync-polish-and-ship, upstream `9eeebb302`, #1239) — `mimalloc::MiMalloc` (mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, vendored C built through `cc`): the WPT runner's `#[global_allocator] static GLOBAL`, a direct dependency, and the browser app's by default, its `default` features gaining `mimalloc`; it reaches only those upstream-only binaries (wpt/runner/src/main.rs:1-3; wpt/runner/Cargo.toml:15; apps/browser/Cargo.toml:14; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "occupied-resources: Process-wide state registers mimalloc as the global allocator of the WPT runner and, by default, the browser app (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols (WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc`, main.rs added 1-3) and Dependencies (mimalloc now the default global allocator of `apps/browser` and a direct dependency of the WPT runner, reaching only upstream-only binaries); Expected amendments names occupied-resources.md:38 for mimalloc as the global allocator. A process-wide global allocator is unregistered in the file."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:25

### standard-contracts#2.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:17
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

### standard-contracts#6.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:54
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

### standard-contracts#9.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:75
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

### standard-contracts#9.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:75
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

## Held

- cross-cutting-patterns#2.1
- cross-cutting-patterns#5.1
- design-philosophy#4.1
- established-decisions#11.1
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#12.1
- established-decisions#13.1
- established-decisions#19.1
- established-decisions#21.1
- established-decisions#5.1
- existing-scopes#1.13
- existing-scopes#1.31
- existing-scopes#1.34
- occupied-resources#7.1
- project-intent#1.0
- stack-and-technologies#1.11
- stack-and-technologies#1.16
- stack-and-technologies#1.18
- stack-and-technologies#1.20
- stack-and-technologies#1.21
- standard-contracts#20.1
- standard-contracts#23.1
- standard-contracts#24.1
- standard-contracts#7.1

## Cascade only

- design-philosophy#5.1
- established-decisions#20.1
- occupied-resources#5.1
- standard-contracts#5.1
- standard-contracts#8.1
