proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 32
    section: "## Established Decisions"
    change: >-
      In [Default features], the browser's list gains `mimalloc` — "browser: hybrid, cookies, cache, screenshot, apple-font-embolden, scrollbars, mimalloc (apps/browser/Cargo.toml:14; apps/browser/Cargo.toml:19-27)" — upstream #1239, since 2026-10-24-upstream-sync-polish-and-ship, the feature making mimalloc the browser's global allocator ([Browser app]).
    sidecar: >-
      established-decisions [Default features]: browser defaults gain `mimalloc` (upstream #1239, 2026-10-24-upstream-sync-polish-and-ship).
    rationale: >-
      Report Changes → Dependencies: mimalloc is "now the default global allocator of `apps/browser` (`default` gains \"mimalloc\")"; New text lists apps/browser/Cargo.toml added 14. The entity's browser list omits it.
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: "## Established Decisions"
    change: >-
      Replace "their merge surface, by file, is owned by a CARRY on «Upstream sync ahead of polish and ship» (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md)" with "their merge surface, by file (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md), re-read at 2026-10-24-upstream-sync-polish-and-ship with nothing sent upstream, is owned by a `.andromeda/residuals.md` line (target: next), re-read at the next sync".
    sidecar: >-
      established-decisions :75: the engine-debts merge surface's owner moves from the closed CARRY on «Upstream sync ahead of polish and ship» to a residuals line (target next).
    rationale: >-
      Report Expected amendments, «The CARRY 3–5 owners»: this chunk closes the entry that owns them; lines :75, :76 and :78 "each say owned by a CARRY on «Upstream sync ahead of polish and ship»" — "Changed: the owner moves to the new `.andromeda/residuals.md` line of the card's row 2 (target next, inputs#I5)". Outcome: CARRY 3–5 re-read, "Nothing sent upstream."
    basis: .andromeda/architecture-detail/established-decisions.md:75
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: "## Established Decisions"
    change: >-
      Replace "owned by the same CARRY on «Upstream sync ahead of polish and ship»" with "re-read at 2026-10-24-upstream-sync-polish-and-ship with nothing sent upstream, owned by the same `.andromeda/residuals.md` line (target: next)".
    sidecar: >-
      established-decisions :76: the clock surface's owner moves to the same residuals line (target next).
    rationale: >-
      Report Expected amendments, «The CARRY 3–5 owners»: line :76 (one-virtual-clock) changes owner with :75 and :78 (inputs#I5 row 2). The report's Spec claims disproved note that :76 lists `resolve.rs` and "says nothing of reach", so no reach wording is added.
    basis: .andromeda/architecture-detail/established-decisions.md:76
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: >-
      Replace "is owned by the same CARRY on «Upstream sync ahead of polish and ship»" with "re-read at 2026-10-24-upstream-sync-polish-and-ship with nothing sent upstream, is owned by the same `.andromeda/residuals.md` line (target: next)".
    sidecar: >-
      established-decisions :78: the select model's merge-surface owner moves to the same residuals line (target next).
    rationale: >-
      Report Expected amendments, «The CARRY 3–5 owners»: line :78 (select-by-id) changes owner with :75 and :76 (inputs#I5 row 2).
    basis: .andromeda/architecture-detail/established-decisions.md:78
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 84
    section: "## Established Decisions"
    change: >-
      In [Layout], after "default object size 300x150, an unloaded image zero (…)", add "; a replaced element's natural height is coerced through its `aspect-ratio` in `compute_replaced_layout` when the ratio is usable (`is_usable_ratio`: finite and > 0), clamped at 0 (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211) — upstream #1246, merged at 2026-10-24-upstream-sync-polish-and-ship, an engine change for every document ratified by the founder's «Десять коммитов (Рекомендую)»".
    sidecar: >-
      established-decisions [Layout]: a replaced element's natural height follows its `aspect-ratio` (upstream #1246).
    rationale: >-
      Report Changes → Symbols: `compute_replaced_layout` (replaced.rs added 144-145, 201-211) "coerces a replaced element's natural height through its `aspect-ratio`", keeping `is_usable_ratio` and `.max(0.0)`; one of three engine changes for every document ratified by inputs#I4. Expected amendments name established-decisions.md:84.
    basis: .andromeda/architecture-detail/established-decisions.md:84
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: >-
      In [Layout], after "out-of-flow boxes are painted and hit-tested by their containing block (…)", add "; the layout pass (`LayoutPassState::is_out_of_flow`) reads a box's out-of-flow-ness from the pub `Node::out_of_flow: AtomicBool`, which the style traversal records from the node's new primary style through `Node::record_out_of_flow` (`style_is_out_of_flow`: `position` `absolute` or `fixed` and not `display: none`), while `Node::is_out_of_flow` and its other callers still read the style (packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279; packages/blitz-dom/src/node/node.rs:2069-2076; packages/blitz-dom/src/layout/mod.rs:856-858; packages/blitz-dom/src/stylo.rs:1342-1344; packages/blitz-dom/src/stylo.rs:1459) — upstream #1238, since 2026-10-24-upstream-sync-polish-and-ship".
    sidecar: >-
      established-decisions [Layout]: the layout pass reads the recorded `Node::out_of_flow` flag (upstream #1238).
    rationale: >-
      Report Changes → Symbols: `Node::out_of_flow` (node.rs added 125-128) written through `record_out_of_flow` (1271-1279) from `stylo.rs` 1342-1344 and 1459; "Only `LayoutPassState::is_out_of_flow` (`layout/mod.rs` added 856-858) now reads the recorded flag"; `style_is_out_of_flow` at 2069-2076.
    basis: .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 92
    section: "## Established Decisions"
    change: >-
      In [DOM semantics], after "hit-test results canonicalize anonymous blocks and pseudo-element subtrees to their element, hover re-resolved after each `resolve` (…)", add "; a hit test skips a node dropped since the last resolve — `Node::hit_inner`'s positive-z hoisted, paint-children and negative-z hoisted loops and its text-cluster lookup, and the paint tree's walk, read `tree().get(id)` and pass over an id that no longer resolves, `hit_inner`'s signature unchanged (packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — upstream #1244, merged at 2026-10-24-upstream-sync-polish-and-ship, an engine change for every document ratified by the founder's «Десять коммитов (Рекомендую)»".
    sidecar: >-
      established-decisions [DOM semantics]: a hit test skips nodes dropped since the last resolve (upstream #1244).
    rationale: >-
      Report Changes → Symbols: hit testing (`Node::hit_inner`, node.rs added 1572-1574 … 1646-1650) "skips nodes dropped since the last resolve" in the four loops, `paint_tree.rs` likewise (164-172); one of the three engine changes ratified by inputs#I4; new target `hit_test_dropped_nodes` (32-82), red with the hunks reversed.
    basis: .andromeda/architecture-detail/established-decisions.md:92
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 131
    section: "## Established Decisions"
    change: >-
      In [Dioxus integration], add "; `launch` (`launch_cfg_with_props`) builds its document with `style_threading: StyleThreading::Parallel`, serialised process-wide by `PARALLEL_TRAVERSAL_LOCK` ([Style threading]) — the windowed path (`seven_guis_native`), no headless document, harness, stand or driver setting it (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222) — upstream #1243, since 2026-10-24-upstream-sync-polish-and-ship, an engine change ratified by the founder's «Десять коммитов (Рекомендую)»".
    sidecar: >-
      established-decisions [Dioxus integration]: the windowed `launch` resolves styles `Parallel` under the process-wide lock (upstream #1243).
    rationale: >-
      Report Changes → Symbols: "`dioxus_native::launch` (`launch_cfg_with_props`) now builds its document with `style_threading: StyleThreading::Parallel` (lib.rs added 63, 222). This is the windowed path … No headless document — harness, stand or driver — sets `style_threading`"; listed among the three engine changes ratified by inputs#I4.
    basis: .andromeda/architecture-detail/established-decisions.md:131
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 137
    section: "## Established Decisions"
    change: >-
      In [Browser app], add "; mimalloc is the browser's global allocator through its default `mimalloc` feature (apps/browser/Cargo.toml:14) — upstream #1239 — and its documents, the loader's and the toolbar's, resolve styles `Parallel` (apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203) — upstream #1243, both since 2026-10-24-upstream-sync-polish-and-ship".
    sidecar: >-
      established-decisions [Browser app]: mimalloc the default allocator; loader and toolbar documents `Parallel` (upstream #1239, #1243).
    rationale: >-
      Report Changes → Dependencies: mimalloc "now the default global allocator of `apps/browser` (`default` gains \"mimalloc\")" (Cargo.toml added 14); Changes → Symbols: "The browser's `document_loader.rs` (added 62) and `toolbar.rs` (added 203) do the same" (`Parallel`).
    basis: .andromeda/architecture-detail/established-decisions.md:137
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 143
    section: "## Established Decisions"
    change: >-
      In [WPT runner], add "; its global allocator is mimalloc — `#[global_allocator] static GLOBAL: mimalloc::MiMalloc`, a direct `mimalloc = \"0.1.48\"` dependency (wpt/runner/src/main.rs:1-3; wpt/runner/Cargo.toml:15) — upstream #1239, since 2026-10-24-upstream-sync-polish-and-ship".
    sidecar: >-
      established-decisions [WPT runner]: mimalloc the runner's global allocator (upstream #1239).
    rationale: >-
      Report Changes → Symbols: "WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (`wpt/runner/src/main.rs` added 1-3)"; Dependencies: "a direct dependency of the WPT runner (`mimalloc = \"0.1.48\"`, one added lock edge)"; New text lists wpt/runner/Cargo.toml added 15.
    basis: .andromeda/architecture-detail/established-decisions.md:143
note: "the `layout/inline.rs` → `inline/{layout,build}.rs` path re-points (established-decisions #9.1, #12.1; conventions #4.1, #7.1) and every shifted line number in this group are left to the citation sweep (`cites.py apply`), as the report assigns; `ElementData::classes`, `PARALLEL_TRAVERSAL_LOCK` and the three `MULTI_DOC_*` env reads (D-arch-resources) have no picked entity in this group and are the body's or another group's"
