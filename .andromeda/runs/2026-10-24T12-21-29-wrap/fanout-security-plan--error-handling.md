proposals:
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block (`<!-- detail: values · errors -->` … `<!-- /detail -->`) holding one bullet: Parallel style traversal, no panic path: since the 2026-10-24 upstream sync (`fb297a38c`, #1243) a process-wide `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is taken around every `StyleThreading::Parallel` traversal, its poison recovered by `unwrap_or_else(|e| e.into_inner())`, so two documents resolving `Parallel` at once are serialised and no longer panic with `already mutably borrowed` (blitz issue 430); the windowed `dioxus_native::launch` and the browser build their documents `Parallel`, no headless document (harness, stand, driver) sets `style_threading`, and the default stays `Sequential` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record the PARALLEL_TRAVERSAL_LOCK (poison recovered, no panic path) that retires the two-Parallel-documents panic"
    rationale: "Report Changes → Symbols (`static PARALLEL_TRAVERSAL_LOCK`: 'Poison is recovered … so there is no panic path. Two documents resolving `Parallel` concurrently are now serialised and no longer panic'; `dioxus_native::launch` now `Parallel`) and Retired ('The `StyleThreading` doc's warning that two `Parallel` documents panic is retired with the lock') — a panic path removed and a no-panic guard added, new detail for Error Handling; no passage of this file named the old panic, so no D-retired."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:18
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block holding one bullet: Apps and examples, `multi_doc` (a root example since the 2026-10-24 upstream sync, `fb297a38c`): its mode argument from `std::env::args` panics through `panic!` on an unknown mode, while its iteration count and `MULTI_DOC_N` fall back to a default on a parse failure and `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` are read as flags (examples/multi_doc.rs:48-54; examples/multi_doc.rs:55-58; examples/multi_doc.rs:17-19; examples/multi_doc.rs:108) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record multi_doc's panic! on an unknown mode beside its defaulted numeric reads"
    rationale: "Report Changes → Symbols (`examples/multi_doc.rs` … 'a `panic!` on an unknown mode (108)', `MULTI_DOC_N` 'parsed with a default fallback') and Expected amendments (security-plan §Error Handling: '`multi_doc.rs`'s `panic!`'); new CLI-input failure handling in the Apps-and-examples panic class (body security-plan.md:369), consistent with §Input Validation's examples rule (body :53), so no violation — new detail."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block holding one bullet: WPT runner, the attribute test runner: since the 2026-10-24 upstream sync (`f8723f495`, #1247) its new offset reads include a `get_node(node_id).unwrap()`, a panic on a node id that does not resolve, on an upstream-only path the WPT runner's per-test panic catch turns into a CRASH result (wpt/runner/src/test_runners/attr_test.rs, among its added lines 3 · 97 · 135-136 · 141-147 · 164-165) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record attr_test.rs's new get_node(node_id).unwrap() panic path in the WPT runner"
    rationale: "Report Changes → Symbols (WPT runner: '`attr_test.rs` gains the reads at added 3 · 97 · 135-136 · 141-147 · 164-165, including a `get_node(node_id).unwrap()` on an upstream-only path') and Expected amendments (security-plan §Error Handling: '`attr_test.rs`'s `get_node(node_id).unwrap()`'); a new panic path in the WPT-runner class (body security-plan.md:370), caught per test by the runner (body :353) — new detail; the report names no single line for the unwrap, so the cite gives its added ranges."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:25
note: "The three new facts' broad lines stand in the body (security-plan.md:342-370, Graceful degradation / Panic paths → Apps and examples :369, WPT runner :370), which this detector cannot edit — the orchestrator may prefer a body line over a detail block; this file's stylo.rs and dioxus-native lib.rs citations (lines 18, 24) shift with the take and are the citation sweep's; D-security-auth and D-security-deps (mimalloc, no ban list) find nothing in this category."
