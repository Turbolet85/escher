proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables"
    change: "Add the root example multi_doc's three env reads: `MULTI_DOC_N` (parsed, a default on a parse failure), `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT` (flags), read by `examples/multi_doc.rs` alone."
    file: ".andromeda/architecture-detail/occupied-resources.md"
    near: "the body bullet «**Environment variables:** `RUST_LOG`, the filter `escher_telemetry::init` reads …»"
    detail: "`MULTI_DOC_N`, parsed with a default fallback (examples/multi_doc.rs:55-58), and the flags `MULTI_DOC_SEQ` (examples/multi_doc.rs:36-40) and `MULTI_DOC_FLAT` (examples/multi_doc.rs:22), read through `env_flag` (examples/multi_doc.rs:17-19), in the upstream root example `multi_doc` alone, which also reads `std::env::args` (examples/multi_doc.rs:48-54); since 2026-10-24-upstream-sync-polish-and-ship (from `fb297a38c`)"
    sidecar: "§Occupied Resources: registered MULTI_DOC_N · MULTI_DOC_SEQ · MULTI_DOC_FLAT, the env reads of the new root example multi_doc (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `examples/multi_doc.rs` (new) reads three env variables through two `std::env::var` sites; Expected amendments name occupied-resources.md:55 as a new registration (pattern `multi-doc` unfired, 0 detail hits). An unregistered env var is D-arch-resources drift."
    basis: "examples/multi_doc.rs:55-58"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: "Add the static `PARALLEL_TRAVERSAL_LOCK: Mutex<()>` in blitz-dom's `stylo.rs`, taken around every `StyleThreading::Parallel` traversal so Parallel resolves are serialised process-wide, its poison recovered with no panic path."
    file: ".andromeda/architecture-detail/occupied-resources.md"
    near: "the body bullet «**Process-wide state and threads:** Stylo's global rayon pool, static id counters, …»"
    detail: "a static `PARALLEL_TRAVERSAL_LOCK: Mutex<()>` (packages/blitz-dom/src/stylo.rs:29-35) taken around every `StyleThreading::Parallel` traversal (packages/blitz-dom/src/stylo.rs:157-163), so two documents resolving `Parallel` at once are serialised on Stylo's global pool rather than panicking `already mutably borrowed` (blitz issue 430), poison recovered by `unwrap_or_else(|e| e.into_inner())`; since 2026-10-24-upstream-sync-polish-and-ship (from `fb297a38c`)"
    sidecar: "§Occupied Resources: registered PARALLEL_TRAVERSAL_LOCK, the process-wide lock serialising Parallel style traversals (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is new process-wide state (stylo.rs added 29-35, 157-163); Expected amendments name occupied-resources.md:38. Unregistered process-wide state is D-arch-resources drift."
    basis: "packages/blitz-dom/src/stylo.rs:29-35"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: "Add mimalloc as the process-wide global allocator of the browser app (its `default` features now include `mimalloc`) and of the WPT runner (`#[global_allocator] static GLOBAL: mimalloc::MiMalloc`), both upstream-only binaries."
    file: ".andromeda/architecture-detail/occupied-resources.md"
    near: "the body bullet «**Process-wide state and threads:** Stylo's global rayon pool, static id counters, …»"
    detail: "a `#[global_allocator]` `mimalloc::MiMalloc` in the WPT runner (wpt/runner/src/main.rs:1-3; wpt/runner/Cargo.toml:15) and, by default since `default` gained `\"mimalloc\"`, in the browser app (apps/browser/Cargo.toml:14) — upstream-only binaries, libmimalloc-sys building vendored C through `cc`; since 2026-10-24-upstream-sync-polish-and-ship (from `9eeebb302`)"
    sidecar: "§Occupied Resources: mimalloc registered as the global allocator of apps/browser (default) and the WPT runner (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Dependencies: mimalloc now the default global allocator of `apps/browser` and a direct dependency of the WPT runner; Symbols: `#[global_allocator] static GLOBAL` (wpt/runner/src/main.rs added 1-3); Expected amendments name occupied-resources.md:38 for it."
    basis: "wpt/runner/src/main.rs:1-3"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Nodes and widgets"
    change: "Register the two new public fields: `Node::out_of_flow: AtomicBool`, recorded by the style traversal from the node's new primary style and read by the layout pass's out-of-flow test, and `ElementData::classes: ThinVec<Atom>`, filled from the `class` attribute, kept in sync by the mutator's set and remove, and read by Stylo's class matching."
    file: ".andromeda/architecture-detail/standard-contracts.md"
    near: "the body bullet «**Nodes and widgets:** `NodeData` and `SpecialElementData` …»"
    detail: "`Node::out_of_flow: AtomicBool`, a pub field (packages/blitz-dom/src/node/node.rs:125-128) written by the style traversal through the crate-private `Node::record_out_of_flow` (packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/stylo.rs:1342-1344; packages/blitz-dom/src/stylo.rs:1459; packages/blitz-dom/src/layout/construct.rs:720; packages/blitz-dom/src/layout/construct.rs:775) and read by `LayoutPassState::is_out_of_flow` alone (packages/blitz-dom/src/layout/mod.rs:856-858), `Node::is_out_of_flow` still reading the style through the crate-private `style_is_out_of_flow` (packages/blitz-dom/src/node/node.rs:2069-2076); `ElementData::classes: ThinVec<Atom>`, a pub field (packages/blitz-dom/src/node/element.rs:56-61) filled from the `class` attribute by `ElementData::new` and `Clone` (packages/blitz-dom/src/node/element.rs:422-426; packages/blitz-dom/src/node/element.rs:431; packages/blitz-dom/src/node/element.rs:313), kept in sync by `DocumentMutator::set_attribute`'s `class` arm and `remove_attribute` (packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502) and read by Stylo's `has_class` and `each_class` (packages/blitz-dom/src/stylo.rs:586-590; packages/blitz-dom/src/stylo.rs:697-699; packages/blitz-dom/src/stylo.rs:710); since 2026-10-24-upstream-sync-polish-and-ship (from `46d7cf993` and `e4ca5b9b5`)"
    sidecar: "§Standard Contracts: registered the new pub fields Node::out_of_flow and ElementData::classes (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `Node::out_of_flow` and `ElementData::classes` are new pub fields of blitz-dom's public node types; a new public API absent from §Standard Contracts is D-arch-resources drift (pattern `classes` unfired, 0 detail hits)."
    basis: "packages/blitz-dom/src/node/element.rs:56-61"
  - detector: D-retired
    severity: warning
    section: "§Established Decisions → [Style threading]"
    change: "Rewrite: `StyleThreading` derives `Default` with `#[default] Sequential`, and the `style_threading` field doc now says so too (packages/blitz-dom/src/config.rs:54); `Parallel` traversals are serialised process-wide by `PARALLEL_TRAVERSAL_LOCK` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163), so two documents resolving `Parallel` concurrently no longer panic (blitz issue 430); the windowed `dioxus_native::launch` and the browser's document loader and toolbar build their documents `Parallel` (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222; apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203), while no headless document — harness, stand or driver — sets `style_threading`; `Sequential` bypasses Stylo's global thread pool, and the thread-local `LAYOUT_CTX` keeps its multi-document FIXME (those two cites as the citation sweep re-points them) — the windowed `Parallel` launch an upstream engine change for every document, ratified by the founder (inputs#I4)."
    sidecar: "§Established Decisions [Style threading]: retired the field-doc contradiction and the two-Parallel-documents panic; Parallel now serialised by PARALLEL_TRAVERSAL_LOCK, dioxus_native::launch Parallel (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Retired: «The `StyleThreading` doc's warning that two `Parallel` documents panic is retired with the lock»; Symbols: the field doc now says the default is `Sequential` (config.rs added 13-17, 23-24, 54), `PARALLEL_TRAVERSAL_LOCK`, and `dioxus_native::launch` now `Parallel`. Expected amendments: architecture.md:44 ×4 changed. The body still states the field doc says `Parallel` and that `Parallel` panics."
    basis: "packages/blitz-dom/src/stylo.rs:157-163"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies"
    change: "Drop the clause «the flake's \"1.90.0\" pin sits below the workspace `rust-version` \"1.91.0\" its comment says to keep in sync with»: the flake's rust-bin pin now reads 1.91.0, in sync with `rust-version` 1.91.0 (flake.nix:31)."
    sidecar: "§Stack and Technologies: retired the recorded flake drift — the flake's rust-bin pin is 1.91.0, matching rust-version (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Dev-tool versions: «The flake's `rust-bin` toolchain pin 1.90.0 → 1.91.0 (`flake.nix` added 31, `f9cbb2756`)»; Expected amendments: «body :21 … the flake's rust-bin 1.91.0, retiring the recorded flake drift». architecture.md:21 still states the drift as standing."
    basis: "flake.nix:31"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies"
    change: "Add that mimalloc is the global allocator of the browser app by default and of the WPT runner (a direct dependency there), both upstream-only binaries; no package entered the lock."
    sidecar: "§Stack and Technologies: mimalloc recorded as the default global allocator of apps/browser and the WPT runner (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Dependencies: «mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, now the default global allocator of `apps/browser` (`default` gains \"mimalloc\") and a direct dependency of the WPT runner (`mimalloc = \"0.1.48\"`, one added lock edge)»; Expected amendments: «§Stack and Technologies (… body :21: mimalloc by default …)». The body names no allocator, so a library newly direct in the WPT runner and default in the browser is unrecorded."
    basis: "wpt/runner/Cargo.toml:15"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Layout]"
    change: "Add: a replaced element's natural height follows its `aspect-ratio` when the ratio is usable (finite and above 0, clamped at 0), in `compute_replaced_layout` — an upstream engine change for every document, ratified by the founder (inputs#I4)."
    file: ".andromeda/architecture-detail/established-decisions.md"
    near: "the body bullet «**[Layout]** HTML tables are laid out by Taffy's grid; replaced elements are …»"
    detail: "a replaced element's natural height is coerced through its `aspect-ratio` by `compute_replaced_layout`, keeping `is_usable_ratio` (finite and > 0) and `.max(0.0)`, its signature and its callers `layout` and `dispatch_child_layout` unchanged (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211) — an upstream engine change for every document (`50bf2f235`), ratified by the founder's «Десять коммитов (Рекомендую)» (inputs#I4); since 2026-10-24-upstream-sync-polish-and-ship"
    sidecar: "§Established Decisions [Layout]: a replaced element's natural height now follows its aspect-ratio, an upstream engine change ratified (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `compute_replaced_layout` coerces a replaced element's natural height through its `aspect-ratio` (replaced.rs added 144-145, 201-211), one of «Three engine changes for every document, ratified by the founder's word (inputs#I4)»; Expected amendments: «§Established Decisions, replaced elements (established-decisions.md:84)». The [Layout] decision records no such rule."
    basis: "packages/blitz-dom/src/layout/replaced.rs:201-211"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Scrolling and input]"
    change: "Add: a hit test skips a node dropped since the last resolve — in every hoisted and paint-children loop and the text-cluster lookup, and in the paint tree — an upstream engine change for every document, ratified by the founder (inputs#I4)."
    file: ".andromeda/architecture-detail/established-decisions.md"
    near: "the body bullet «**[Scrolling and input]** root scroll applies to the viewport; … a hit stops at the padding box of every box paint clips.»"
    detail: "a hit test skips nodes dropped since the last resolve: `Node::hit_inner` reads each child through `self.tree().get(id)` with `continue` in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1578; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1627; packages/blitz-dom/src/node/node.rs:1646-1650), and the paint tree does the same (packages/blitz-dom/src/layout/paint_tree.rs:146-147; packages/blitz-dom/src/layout/paint_tree.rs:156; packages/blitz-dom/src/layout/paint_tree.rs:161; packages/blitz-dom/src/layout/paint_tree.rs:164-172), `hit_inner`, `hit` and `hit_with_scrollbar` keeping their signatures; pinned by `hit_test_dropped_nodes` (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — an upstream engine change for every document (`f102663b8`), ratified by the founder's «Десять коммитов (Рекомендую)» (inputs#I4); since 2026-10-24-upstream-sync-polish-and-ship"
    sidecar: "§Established Decisions [Scrolling and input]: a hit test now skips nodes dropped since the last resolve, an upstream engine change ratified (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: «Hit testing (`Node::hit_inner` …) skips nodes dropped since the last resolve» and `paint_tree.rs` likewise, listed among «Three engine changes for every document, ratified by the founder's word (inputs#I4)»; Expected amendments: «hit testing skipping dropped nodes — carried». The hit rules recorded under [Scrolling and input] omit it."
    basis: "packages/blitz-dom/src/node/node.rs:1572-1574"
  - detector: D-arch-decisions
    severity: warning
    section: "§Project Intent"
    change: "Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a — the next sync's merge base, unmoved; upstream `main` last read at `f9cbb2756`, ten of its commits taken by no-commit cherry-picks; left out for the next sync: git Stylo 0.23-pre (`06ecec886` with `3954ecb85` and `1c5265a8b`), the git Taffy bumps `b8f4ba6fd` and `9babd796a` with the four Taffy-main commits (six waiting) and Dioxus 0.8.0-alpha.3 (`286874cc1`) — so Stylo stays 0.22.0, Taffy the registry \"0.15\" and Dioxus 0.7.3."
    sidecar: "§Project Intent Upstream sync: upstream read at f9cbb2756, ten commits taken, merge base unmoved; Stylo 0.23, the git Taffy bumps (six Taffy-main commits waiting) and Dioxus 0.8 left out (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Files (the ten commits taken by `git cherry-pick --no-commit`, «The merge base `23b9b26a9` is unmoved») and Cross-project (upstream read at `f9cbb2756`, the left-out commits and CARRY 1's four) and Dependencies («The pins stay: Stylo 0.22.0, taffy 0.15 from the registry, Dioxus 0.7.3»); Expected amendments: «Pattern `upstream`: 1 standing (architecture.md:286 ×3 — changed … now reads `f9cbb2756`, ten taken, six Taffy-main waiting)». The body still reads `17de049b1`, three taken, four waiting."
note: "Left to the cascade and the citation sweep, not proposed here: the standing detail groups the report names as changed — stack-and-technologies.md:60 (mimalloc «optional» → default) and :69 (rust-bin «1.90.0» and its drift clause), project-intent.md:9 (the Upstream sync record), established-decisions.md:75, :76 and :78 (CARRY 3–5 «owned by a CARRY on «Upstream sync ahead of polish and ship»» — the owner moves to the new residuals.md line of the P5 card's row 2), and the `layout/inline` path in existing-scopes.md, conventions.md and established-decisions.md; the body's §Cross-cutting Patterns → Rendering details cites `packages/blitz-dom/src/layout/inline.rs`, a path the take deleted (architecture.md:276 ×2) — owned by P2's citation sweep (`cites.py apply`, inline.rs → inline/{build,layout}.rs), so no D-retired proposal duplicates it. No new workspace crate, port, socket or listener (Crates / modules: root manifest byte-identical); tokio's 1.54.1 is a lock-only move with the manifest's \"1.42\" unchanged, no §Stack claim moved."
