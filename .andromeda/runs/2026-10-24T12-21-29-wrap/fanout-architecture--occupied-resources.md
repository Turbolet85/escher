proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 55
    section: "## Occupied Resources"
    change: "Append to the Environment variables record, before the CI script's GitHub variables: `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the upstream root example `examples/multi_doc.rs` (since 2026-10-24-upstream-sync-polish-and-ship, upstream's text as written from `fb297a38c`) through two `std::env::var` call sites — `MULTI_DOC_N` parsed with a default fallback, `MULTI_DOC_SEQ` (choosing its documents' style threading) and `MULTI_DOC_FLAT` through its `env_flag` — example code only, no secret (examples/multi_doc.rs:17-19; examples/multi_doc.rs:22; examples/multi_doc.rs:36-40; examples/multi_doc.rs:55-58; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "occupied-resources: Environment variables registers `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT`, the new upstream example `examples/multi_doc.rs`'s env reads (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols lists `examples/multi_doc.rs` (new) reading three env variables through two `std::env::var` call sites; Expected amendments names occupied-resources.md:55 for their registration (pattern `multi-doc` unfired, 0 detail hits — a new registration). D-arch-resources: every new env var the chunk lands is registered in arch §Occupied Resources; the record at line 55 does not name them."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 49
    section: "## Occupied Resources"
    change: "New record after line 49, in the process-wide state block: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` in blitz-dom's `stylo.rs` (since 2026-10-24-upstream-sync-polish-and-ship, upstream `fb297a38c`, #1243) — process-wide, taken around every `StyleThreading::Parallel` traversal, so two documents resolving `Parallel` concurrently on Stylo's global rayon pool are serialised and no longer panic `already mutably borrowed` (blitz issue 430); poison recovered by `unwrap_or_else(|e| e.into_inner())`, no panic path, no log site; `Sequential` stays the default; the windowed `dioxus_native::launch` (`seven_guis_native`) and the browser's document loader and toolbar now build their documents `Parallel`, while no headless document — harness, stand or driver — sets `style_threading` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222; apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "occupied-resources: Process-wide state registers `PARALLEL_TRAVERSAL_LOCK` and the windowed and browser launches resolving `Parallel` under it (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is new process-wide state in stylo.rs (added 29-35, taken at 157-163), and `dioxus_native::launch` plus the browser's loader and toolbar now set `StyleThreading::Parallel`; Expected amendments names occupied-resources.md:38 (Process-wide state and threads) for it. The file's process-wide record names Stylo's global rayon pool but no lock and no `Parallel` launch. A separate record keeps line 38 under the 2 000-char entity bound."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:18
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 49
    section: "## Occupied Resources"
    change: "New record after the lock's record: the global allocator of two upstream-only binaries (since 2026-10-24-upstream-sync-polish-and-ship, upstream `9eeebb302`, #1239) — `mimalloc::MiMalloc` (mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, vendored C built through `cc`): the WPT runner's `#[global_allocator] static GLOBAL`, a direct dependency, and the browser app's by default, its `default` features gaining `mimalloc`; it reaches only those upstream-only binaries (wpt/runner/src/main.rs:1-3; wpt/runner/Cargo.toml:15; apps/browser/Cargo.toml:14; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "occupied-resources: Process-wide state registers mimalloc as the global allocator of the WPT runner and, by default, the browser app (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols (WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc`, main.rs added 1-3) and Dependencies (mimalloc now the default global allocator of `apps/browser` and a direct dependency of the WPT runner, reaching only upstream-only binaries); Expected amendments names occupied-resources.md:38 for mimalloc as the global allocator. A process-wide global allocator is unregistered in the file."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:25
note: line 38's cites `packages/blitz-dom/src/config.rs:21-24` and `packages/blitz-dom/src/stylo.rs:149-152` sit after added ranges (config.rs 13-17, stylo.rs 29-35) and are the citation sweep's digits; the body's broad lines .andromeda/architecture.md:211 and :216 may want the lock, the allocator and the example's env reads named.
