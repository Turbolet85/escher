proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 69
    section: "## Stack and Technologies"
    change: >-
      Build environment: "rust-bin stable "1.90.0" with rust-src, rust-analyzer, clippy, commented "Keep in sync with `rust-version` in Cargo.toml" (flake.nix:30-37)" now reads rust-bin stable "1.91.0" (since 2026-10-24-upstream-sync-polish-and-ship, upstream's `f9cbb2756`; flake.nix:31), and the closing clause "the flake's "1.90.0" pin sits below the workspace `rust-version` "1.91.0" its comment says to keep in sync with (flake.nix:31; Cargo.toml:41)" is replaced by: the flake's pin now equals the workspace `rust-version` "1.91.0", as its comment asks (flake.nix:31; Cargo.toml:41). The dev-host sentence after it stays.
    sidecar: >-
      stack-and-technologies Build environment — the flake's rust-bin pin 1.90.0 → 1.91.0 (upstream f9cbb2756), the recorded flake/rust-version drift retired.
    rationale: >-
      Report Changes → Dev-tool versions: "The flake's `rust-bin` toolchain pin 1.90.0 → 1.91.0 (`flake.nix` added 31, `f9cbb2756`)"; Expected amendments: "`rust-bin|flake drift|1\.90\.0` by `git grep`: stack-and-technologies.md 1 — changed (1.90.0 → 1.91.0)", "retiring the recorded flake drift".
    basis: .andromeda/architecture-detail/stack-and-technologies.md:69
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 60
    section: "## Stack and Technologies"
    change: >-
      Browser app extras: "optional `mimalloc` 0.1.48 (apps/browser/Cargo.toml:79; apps/browser/src/main.rs:7-9)" now reads: `mimalloc` 0.1.48 (lock 0.1.52, with libmimalloc-sys 0.1.49 building vendored C through `cc`, `links = "mimalloc"`; MIT), since 2026-10-24-upstream-sync-polish-and-ship in the browser's `default` features, so the browser's global allocator by default (apps/browser/Cargo.toml:14; apps/browser/Cargo.toml:79; apps/browser/src/main.rs:7-9).
    sidecar: >-
      stack-and-technologies Browser app extras — mimalloc no longer optional-off: in apps/browser's default features (upstream 9eeebb302), lock 0.1.52 / libmimalloc-sys 0.1.49.
    rationale: >-
      Report Changes → Dependencies: "mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, now the default global allocator of `apps/browser` (`default` gains "mimalloc")", hand review "MIT … libmimalloc-sys builds vendored C through `cc` (`build.rs`, `links = "mimalloc"`)"; New text: apps/browser/Cargo.toml added 14; Expected amendments: "§Stack and Technologies (stack-and-technologies.md:60 … mimalloc by default)".
    basis: .andromeda/architecture-detail/stack-and-technologies.md:60
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 66
    section: "## Stack and Technologies"
    change: >-
      Testing, the wpt runner's dependency list gains: mimalloc 0.1.48 (lock 0.1.52), since 2026-10-24-upstream-sync-polish-and-ship the runner's `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (wpt/runner/Cargo.toml:15; wpt/runner/src/main.rs:1-3).
    sidecar: >-
      stack-and-technologies Testing — the WPT runner takes mimalloc 0.1.48 as a direct dependency and its global allocator (upstream 9eeebb302).
    rationale: >-
      Report Changes → Dependencies: mimalloc "a direct dependency of the WPT runner (`mimalloc = "0.1.48"`, one added lock edge)"; Symbols: "WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (`wpt/runner/src/main.rs` added 1-3)"; New text: wpt/runner/Cargo.toml added 15.
    basis: .andromeda/architecture-detail/stack-and-technologies.md:66
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 55
    section: "## Stack and Technologies"
    change: >-
      Parallelism and misc, Role, after "Parallel style traversal on Stylo's global `STYLE_THREAD_POOL` (…)": since 2026-10-24-upstream-sync-polish-and-ship every `StyleThreading::Parallel` traversal is taken under the process-wide `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>`, poison recovered by `unwrap_or_else(|e| e.into_inner())`, so two documents resolving `Parallel` concurrently are serialised instead of panicking (blitz issue 430) (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163); `Parallel` is set by the windowed `dioxus_native::launch` and the browser's document loader and toolbar, by no headless document (packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222).
    sidecar: >-
      stack-and-technologies Parallelism and misc — Parallel style traversals serialised by PARALLEL_TRAVERSAL_LOCK (upstream fb297a38c); the windowed launch and the browser resolve Parallel.
    rationale: >-
      Report Changes → Symbols: "`static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` — new process-wide state in `packages/blitz-dom/src/stylo.rs` (added 29-35), taken around every `StyleThreading::Parallel` traversal (added 157-163) … Two documents resolving `Parallel` concurrently are now serialised"; "`dioxus_native::launch` … now builds its document with `style_threading: StyleThreading::Parallel` (… added 63, 222) … No headless document … sets `style_threading`".
    basis: .andromeda/architecture-detail/stack-and-technologies.md:55
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 39
    section: "## Stack and Technologies"
    change: >-
      Networking and async: after tokio "1.42" add — resolved 1.54.1 since 2026-10-24-upstream-sync-polish-and-ship (`cargo update -p tokio`, lock only, past RUSTSEC-2026-0386; the manifest's "1.42" unchanged).
    sidecar: >-
      stack-and-technologies Networking and async — tokio's resolved version recorded, 1.53.2 → 1.54.1 in the lock alone for RUSTSEC-2026-0386.
    rationale: >-
      Report Changes → Dependencies: "tokio 1.53.2 → 1.54.1 in the lock alone (`cargo update -p tokio`), past RUSTSEC-2026-0386. The manifest's `tokio = "1.42"` is unchanged." New detail of a stack value the session changed, in the file's form for a resolved version (as at line 21's "resolved 0.15.0").
    basis: .andromeda/architecture-detail/stack-and-technologies.md:39
note: the body's own Stack line (.andromeda/architecture.md:21) still states the flake's "1.90.0" pin below rust-version "1.91.0" and is not this file — the orchestrator's to retire with proposal 1; line 22's "out-of-flow layout" (Taffy's role) stays true, no change; no D-arch-resources site lands in this category (PARALLEL_TRAVERSAL_LOCK and MULTI_DOC_* belong to occupied-resources).
