proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 53
    section: "## Design Philosophy"
    change: "Replace the opening clause '`Node` carries only tree-structure information, other fields living on element data (packages/blitz-dom/src/node/element.rs:84-87)' with '`Node` carries tree-structure information and one recorded style fact, `Node::out_of_flow` (an `AtomicBool` the style traversal writes from the node's new primary style through `Node::record_out_of_flow`, read by `LayoutPassState::is_out_of_flow` alone, while `Node::is_out_of_flow` still reads the style), other fields living on element data — among them `ElementData::classes`, the `class` attribute's atoms, filled at construction and clone, kept in sync by `DocumentMutator::set_attribute` and `remove_attribute` and read by Stylo's `has_class` and `each_class` (packages/blitz-dom/src/node/element.rs:84-87; packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/layout/mod.rs:856-858; packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502; packages/blitz-dom/src/stylo.rs:586-590)'; the rest of the line unchanged."
    sidecar: "design-philosophy: `Node` carries the recorded `out_of_flow` flag and `ElementData` the class atoms (upstream #1238, #1240), no longer tree structure only"
    rationale: "The report's Changes → Symbols: `Node::out_of_flow: AtomicBool` is a new pub field written by the style traversal through `record_out_of_flow`, read only by `LayoutPassState::is_out_of_flow` (layout/mod.rs added 856-858); `ElementData::classes: ThinVec<Atom>` is a new pub field kept in sync by the mutator (added 375-378, 499-502) and read by Stylo's `has_class`/`each_class` (stylo.rs added 586-590). Expected amendments name §Design Philosophy :53 for both. 'only tree-structure information' is contradicted."
    basis: .andromeda/architecture-detail/design-philosophy.md:53
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 48
    section: "## Design Philosophy"
    change: "Append to the paint-tree bullet: '; hit testing and the paint tree's walks skip a node dropped since the last resolve — `tree().get(id)` and `continue`, never an index — in the hoisted positive- and negative-z loops, the paint-children loop and the text-cluster lookup (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)'."
    sidecar: "design-philosophy: hit testing and the paint tree skip nodes dropped since the last resolve (upstream #1244), pinned by `hit_test_dropped_nodes`"
    rationale: "The report's Changes → Symbols: `Node::hit_inner` (node.rs added 1414-1417 doc, 1572-1574, 1578, 1587-1589, 1621-1623, 1627, 1646-1650) and `paint_tree.rs` (added 146-147, 156, 161, 164-172) skip nodes dropped since the last resolve, an engine change for every document ratified by the founder (inputs#I4); the new target `hit_test_dropped_nodes` (82 lines, its test at 32-82) read red with the hunks reversed. The 'safe identities' entity states the paint tree's topology and dropped ids but not this."
    basis: .andromeda/architecture-detail/design-philosophy.md:48
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 62
    section: "## Design Philosophy"
    change: "Append to the paint-benchmark bullet: '; the root example `multi_doc` times `resolve` over several documents, its document count and threading read from `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` and its mode and iterations from argv (examples/multi_doc.rs:35-45; examples/multi_doc.rs:47-58; examples/multi_doc.rs:63-67)'."
    sidecar: "design-philosophy: the `multi_doc` root example (upstream #1243) joins the phase-split measurement examples"
    rationale: "The report's Changes → Files and Symbols: `examples/multi_doc.rs` is new (116 lines, from `fb297a38c`), reading `MULTI_DOC_N` (55-58), `MULTI_DOC_SEQ` (36-40) and `MULTI_DOC_FLAT` (22) and `std::env::args` (48-54), with a `timed_resolve` closure (63-67) and two `Instant::now()` (61, 64); Crates / modules: the root examples gain `multi_doc`."
    basis: .andromeda/architecture-detail/design-philosophy.md:62
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 39
    section: "## Stack and Technologies"
    change: "After 'tokio \"1.42\"' insert ' (resolved 1.54.1 since 2026-10-24-upstream-sync-polish-and-ship, past RUSTSEC-2026-0386, by a lock-only `cargo update -p tokio`)'; the rest of the line unchanged."
    sidecar: "stack: tokio resolves to 1.54.1 past RUSTSEC-2026-0386, the manifest's \"1.42\" unchanged"
    rationale: "The report's Changes → Dependencies: tokio 1.53.2 → 1.54.1 in the lock alone (`cargo update -p tokio`), past RUSTSEC-2026-0386; the manifest's `tokio = \"1.42\"` unchanged; Cross-project: the advisory cleared, audit `advisories ok, licenses ok`. The Layout entity's 'resolved 0.15.0' is the shape's precedent."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:39
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 55
    section: "## Stack and Technologies"
    change: "After the 'Parallel style traversal on Stylo's global `STYLE_THREAD_POOL` (…)' clause insert ', every `Parallel` traversal taken under the process-wide `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>`, its poison recovered, so two documents resolving `Parallel` concurrently are serialised rather than panicking (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163)', and append '; thin-vec holds `ElementData::classes` (packages/blitz-dom/src/node/element.rs:56-61)'."
    sidecar: "stack: Parallel style traversals serialised process-wide by `PARALLEL_TRAVERSAL_LOCK` (upstream #1243); thin-vec holds the class atoms"
    rationale: "The report's Changes → Symbols: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` new in stylo.rs (added 29-35), taken around every `StyleThreading::Parallel` traversal (added 157-163), poison recovered by `unwrap_or_else(|e| e.into_inner())`, two concurrent `Parallel` documents no longer panicking (blitz issue 430); `ElementData::classes: ThinVec<Atom>` (element.rs added 56-61)."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:55
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 60
    section: "## Stack and Technologies"
    change: "Replace 'optional `mimalloc` 0.1.48' with '`mimalloc` 0.1.48, in the browser's `default` features since 2026-10-24-upstream-sync-polish-and-ship (apps/browser/Cargo.toml:14)'; its existing citations and the rest of the line unchanged."
    sidecar: "stack: mimalloc is the browser's default global allocator (upstream #1239)"
    rationale: "The report's Changes → Dependencies: mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, now the default global allocator of `apps/browser` (`default` gains \"mimalloc\", apps/browser/Cargo.toml added 14). Expected amendments name stack-and-technologies.md:60."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:60
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 66
    section: "## Stack and Technologies"
    change: "In the wpt runner list, after 'serde_json 1.0.140' insert ', mimalloc \"0.1.48\", the runner's `#[global_allocator]` since 2026-10-24-upstream-sync-polish-and-ship (wpt/runner/Cargo.toml:15; wpt/runner/src/main.rs:1-3)'; the rest of the line unchanged."
    sidecar: "stack: the WPT runner takes mimalloc as a direct dependency and its global allocator (upstream #1239)"
    rationale: "The report's Changes → Dependencies: mimalloc a direct dependency of the WPT runner (`mimalloc = \"0.1.48\"`, one added lock edge; wpt/runner/Cargo.toml added 15); Symbols: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (wpt/runner/src/main.rs added 1-3). Hand review: MIT, vendored C through `cc`, reaching only upstream-only binaries."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:66
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 69
    section: "## Stack and Technologies"
    change: "Replace 'rust-bin stable \"1.90.0\"' with 'rust-bin stable \"1.91.0\" (\"1.90.0\" until 2026-10-24-upstream-sync-polish-and-ship)', and replace the sentence 'the flake's \"1.90.0\" pin sits below the workspace `rust-version` \"1.91.0\" its comment says to keep in sync with (flake.nix:31; Cargo.toml:41)' with 'the flake's pin now matches the workspace `rust-version` \"1.91.0\" its comment says to keep in sync with (flake.nix:31; Cargo.toml:41)'; the dev-host clause unchanged."
    sidecar: "stack: the flake's rust-bin pin 1.90.0 → 1.91.0 (upstream #1264), the recorded drift from `rust-version` retired"
    rationale: "The report's Changes → Dev-tool versions: the flake's `rust-bin` toolchain pin 1.90.0 → 1.91.0 (flake.nix added 31, `f9cbb2756`); Expected amendments: 'the flake's rust-bin 1.91.0, retiring the recorded flake drift', stack-and-technologies.md:69 and `rust-bin|flake drift|1\\.90\\.0` changed (1.90.0 → 1.91.0). The root manifest is byte-identical, so `rust-version` stays 1.91.0."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:69
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 21
    section: "## Stack and Technologies"
    change: "Drop the closing clause '; the flake's \"1.90.0\" pin sits below the workspace `rust-version` \"1.91.0\" its comment says to keep in sync with' so the line ends at 'the `ci-scripts` leg needs Python 3.11 or later.'"
    sidecar: "architecture body: the flake drift clause retired — the flake's pin is 1.91.0, in sync with `rust-version`"
    rationale: "The report's Changes → Dev-tool versions (flake.nix added 31: 1.90.0 → 1.91.0) and Expected amendments ('body :21 … the flake's rust-bin 1.91.0, retiring the recorded flake drift'); the body's broad line repeats the drift the stack entity's line 69 retires."
    basis: .andromeda/architecture.md:21
    dependent-of: D-cascade
note: the chunk's other new resources — `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` (Environment variables) and `PARALLEL_TRAVERSAL_LOCK` with mimalloc as a global allocator (Process-wide state and threads) — are D-arch-resources sites in occupied-resources, outside group 1; the moved `construct.rs:619-624; construct.rs:949-951` cite in design-philosophy.md:47 now lands in `layout/inline/build.rs` and is the citation sweep's.
