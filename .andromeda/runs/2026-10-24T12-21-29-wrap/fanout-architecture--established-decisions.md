proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: '## Established Decisions'
    change: 'At :75 replace "their merge surface, by file, is owned by a CARRY on «Upstream sync ahead of polish and ship» (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md)" with "their merge surface, by file (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md), re-read at the 2026-10-24 upstream sync with nothing sent upstream, is owned by a `.andromeda/residuals.md` line (target: next), origin 2026-10-24-upstream-sync-polish-and-ship, that re-reads the CARRY 3–5 merge surfaces at the next sync".'
    sidecar: 'established-decisions.md:75 — the seven engine changes'' merge surface re-owned from the closed «Upstream sync ahead of polish and ship» CARRY to the new residuals.md line (target: next).'
    rationale: 'The report closes the entry that owned CARRY 3–5 and moves the owner to a new `.andromeda/residuals.md` line of the P5 card''s row 2 (target next, approved by the operator, inputs#I5); it names :75 (engine-debts) as changed, and records the CARRY 3–5 readings re-read green with nothing sent upstream.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:97
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: '## Established Decisions'
    change: 'At :76 replace "owned by the same CARRY on «Upstream sync ahead of polish and ship»" with "owned by the same `.andromeda/residuals.md` line (target: next), origin 2026-10-24-upstream-sync-polish-and-ship".'
    sidecar: 'established-decisions.md:76 — the document clock''s merge surface re-owned to the new residuals.md line (target: next).'
    rationale: 'Same owner move as :75 — the report names :76 (one-virtual-clock) among the three CARRY-owner lines that change (Changes → Expected amendments, The CARRY 3–5 owners; card row 2, inputs#I5).'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:97
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: '## Established Decisions'
    change: 'At :78 replace "is owned by the same CARRY on «Upstream sync ahead of polish and ship»" with "is owned by the same `.andromeda/residuals.md` line (target: next), origin 2026-10-24-upstream-sync-polish-and-ship".'
    sidecar: 'established-decisions.md:78 — the select model''s merge surface re-owned to the new residuals.md line (target: next).'
    rationale: 'Same owner move as :75 — the report names :78 (select-by-id) among the three CARRY-owner lines that change (Changes → Expected amendments, The CARRY 3–5 owners; card row 2, inputs#I5).'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:97
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 84
    section: '## Established Decisions'
    change: '[Layout], after "replaced elements are `img`, `svg`, `canvas`, `video`, `embed`, `iframe`, `<object>` excluded so fallback children render (packages/blitz-dom/src/layout/replaced.rs:7-17);" insert: "a replaced element''s natural height is coerced through its `aspect-ratio` by `compute_replaced_layout`, a degenerate ratio still refused by `is_usable_ratio` (finite and > 0) and the height floored by `.max(0.0)`, its signature unchanged (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211) — upstream #1246 (`50bf2f235`), taken at 2026-10-24-upstream-sync-polish-and-ship, an engine change for every document ratified by the founder (2026-10-24, «Десять коммитов (Рекомендую)», inputs#I4);"'
    sidecar: 'established-decisions.md:84 [Layout] — a replaced element''s natural height through `aspect-ratio` (upstream #1246), ratified engine change for every document.'
    rationale: 'Changes → Symbols: `compute_replaced_layout` coerces the natural height through `aspect-ratio`, keeping `is_usable_ratio` and `.max(0.0)`; listed among the three engine changes for every document ratified by the founder (inputs#I4); Expected amendments names established-decisions.md:84 (replaced elements) for it.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:23
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 92
    section: '## Established Decisions'
    change: '[DOM semantics], after "hit-test results canonicalize anonymous blocks and pseudo-element subtrees to their element, hover re-resolved after each `resolve` (tests/blitz-tests/tests/interaction_state_canonicalization.rs:5-10);" insert: "a hit test skips a node dropped since the last resolve — `Node::hit_inner` reads each child through `tree().get(id)` and passes over one that no longer resolves, in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup, the paint tree doing the same, `hit_inner`, `hit` and `hit_with_scrollbar` keeping their signatures (packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — upstream #1244 (`f102663b8`), taken at 2026-10-24-upstream-sync-polish-and-ship, an engine change for every document ratified by the founder (2026-10-24, inputs#I4);"'
    sidecar: 'established-decisions.md:92 [DOM semantics] — a hit test skips nodes dropped since the last resolve (upstream #1244), ratified engine change for every document, pinned by `hit_test_dropped_nodes`.'
    rationale: 'Changes → Symbols: hit testing (`Node::hit_inner`, the four loops) and `paint_tree.rs` skip nodes dropped since the last resolve; one of the three engine changes for every document ratified by the founder (inputs#I4); its new target `hit_test_dropped_nodes` read red with the hunks reversed and green restored (Coverage of new surfaces).'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:22
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 131
    section: '## Established Decisions'
    change: '[Dioxus integration], after "hot reload only under `hot-reload` with `debug_assertions` off wasm32 (packages/dioxus-native/src/lib.rs:166-175);" insert: "`launch_cfg_with_props` builds its document with `style_threading: StyleThreading::Parallel`, so the windowed `dioxus_native::launch` — `seven_guis_native` its stand caller — resolves styles in parallel, serialised process-wide by blitz-dom''s `PARALLEL_TRAVERSAL_LOCK`, while no headless document (harness, stand or driver) sets `style_threading` and each stays `Sequential` (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222) — upstream #1243 (`fb297a38c`), taken at 2026-10-24-upstream-sync-polish-and-ship, an engine change ratified by the founder (2026-10-24, inputs#I4);"'
    sidecar: 'established-decisions.md:131 [Dioxus integration] — the windowed launch resolves styles `Parallel` under the process-wide lock (upstream #1243); headless documents stay `Sequential`.'
    rationale: 'Changes → Symbols: `dioxus_native::launch` (`launch_cfg_with_props`) now builds its document `Parallel` (lib.rs added 63, 222), the windowed path, while no headless document sets `style_threading`; `PARALLEL_TRAVERSAL_LOCK` serialises `Parallel` traversals; listed among the three ratified engine changes (inputs#I4).'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:20
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 32
    section: '## Established Decisions'
    change: '[Default features], replace "browser: hybrid, cookies, cache, screenshot, apple-font-embolden, scrollbars (apps/browser/Cargo.toml:14;" with "browser: hybrid, cookies, cache, screenshot, apple-font-embolden, scrollbars and, since 2026-10-24-upstream-sync-polish-and-ship (upstream #1239, `9eeebb302`), `mimalloc`, its global allocator by default (apps/browser/Cargo.toml:14;".'
    sidecar: 'established-decisions.md:32 [Default features] — the browser''s default list gains `mimalloc` (upstream #1239).'
    rationale: 'Changes → Dependencies: mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, is now the default global allocator of `apps/browser` (`default` gains `"mimalloc"`, apps/browser/Cargo.toml added 14); the file''s browser default list omits it.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:31
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 143
    section: '## Established Decisions'
    change: '[WPT runner], open the entry with: "the runner''s global allocator is mimalloc — `#[global_allocator] static GLOBAL: mimalloc::MiMalloc`, a direct dependency `mimalloc = "0.1.48"` (wpt/runner/src/main.rs:1-3; wpt/runner/Cargo.toml:15) — upstream #1239 (`9eeebb302`), since 2026-10-24-upstream-sync-polish-and-ship;" ahead of "Render target 800x600 at scale 1.0".'
    sidecar: 'established-decisions.md:143 [WPT runner] — mimalloc the runner''s global allocator (upstream #1239).'
    rationale: 'Changes → Symbols (WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc`, main.rs added 1-3) and Dependencies (mimalloc a direct dependency of the WPT runner, `mimalloc = "0.1.48"`, one added lock edge; hand review re-read, reaching only upstream-only binaries).'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:25
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 67
    section: '## Established Decisions'
    change: 'Range interaction''s merge surface, after "— `git diff --stat 434e0661 e72650a6` over it empty, as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/merge-surface.md" insert: "; at the 2026-10-24 upstream sync upstream''s taken hunks reached `node/element.rs`, `node/mod.rs`, `layout/construct.rs` and `mutator.rs`, the take''s one non-additive edit `construct.rs`''s import list, resolved to upstream''s plus `RangeInputData` (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)".'
    sidecar: 'established-decisions.md:67 — the range model''s merge surface met by the 2026-10-24 sync, its one conflict `construct.rs`''s import list resolved with `RangeInputData` kept.'
    rationale: 'Changes → Files (upstream''s hunks modify blitz-dom `layout/construct.rs`, `node/{node,element,mod}.rs`, `mutator.rs`) and Symbols: "`construct.rs`''s import list is resolved to upstream''s plus our `RangeInputData` (the take''s one non-additive edit, `evidence/take.md`)"; the file states this surface only as read untouched at `e72650a6`.'
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:21
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
note: 'Not this file''s: body architecture.md:44 [Style threading] is retired as written (PARALLEL_TRAVERSAL_LOCK, field doc `Sequential`, panic warning retired) and has no detail block here; D-arch-resources registrations (PARALLEL_TRAVERSAL_LOCK, MULTI_DOC_N/SEQ/FLAT, Node::out_of_flow, ElementData::classes) fall to occupied-resources/standard-contracts; the `layout/inline.rs` cites at :57 and :85 and shifted digits (node.rs, element.rs, wpt main.rs, dioxus-native lib.rs) are the citation sweep''s.'
