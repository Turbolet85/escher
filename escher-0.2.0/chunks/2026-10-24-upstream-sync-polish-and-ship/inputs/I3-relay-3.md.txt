# Relay 3 — P3 measurement outside this repository (the orchestrator, dev host, 2026-10-24T10:50Z–11:06Z)

Read during P3's trial takes, in a throwaway worktree under `target/tmp/` with its own target dir; the main tree
untouched. Each fact below is what an external source answered at that time.

1. crates.io index, through `cargo update -p tokio` (cargo 1.99.0) on the take's tree: `Updating tokio v1.53.2 ->
   v1.54.1` ("Locking 1 package to highest compatible version"); `cargo update -p tokio --precise 1.53.3` also
   resolves (`Updating tokio v1.53.2 -> v1.53.3`). Either form also re-points twelve dependency edges from
   `windows-sys 0.61.2` to an older `windows-sys` already in the lock (0.60.2 · 0.59.0 · 0.52.0: anstyle-query,
   anstyle-wincon, dirs-sys, errno, nu-ansi-term, rustix, socket2, tempfile, term, uds_windows, winapi-util) and one
   edge of `cssparser-macros` from `syn 3.0.6` to `syn 2.0.119`; no package enters or leaves the lock.
2. The RustSec advisory database, through cargo-deny 0.20.2 (`cargo deny --locked --workspace --all-features check
   advisories licenses`) on the take's tree with tokio 1.54.1: `advisories ok, licenses ok`; `warning[yanked]`
   clipboard-win 5.4.1 (← arboard 3.6.1 ← blitz-shell) stands as a warning.
3. crates.io, Dioxus `0.8.0-alpha.3` (trial 3 — the take plus `286874cc1`, its `Cargo.lock` conflict resolved by
   cargo's re-lock): the crates download; `cargo check --workspace --all-targets --locked` exits 101 —
   dioxus-native-dom (lib) 10 errors and (lib test) 15, every site in escher's own `element_id.rs` and
   `dioxus_document_tests.rs` (dioxus-core 0.8's `VNode` and `DioxusState` reshaped: `template`, `key`,
   `dynamic_nodes`, `mounted_root`, `DynamicNode::Placeholder`, `TemplateNode`, `node_id_mapping`, the private
   `stack`) and one `rustc_hash` import, the dependency `286874cc1` drops from the manifest.
4. GitHub, git Stylo (DioxusLabs/stylo `d6502e00…`) and git Taffy (`802e497b…`) (trial 4 — the whole merge of
   `upstream/main` `f9cbb2756`, four conflicts resolved, the lock taken as upstream's then re-locked): the sources
   fetch; `cargo check` exits 101 — blitz-dom (lib) 1 error, escher's `contain: paint` hit stop calling
   `clone_contain` on `style.get_box()` (`packages/blitz-dom/src/node/node.rs:1549` on that tree), which Stylo 0.23
   does not have; no crate past blitz-dom reached. No `Instant::now()` / `SystemTime::now()` stands in blitz-dom
   outside `clock.rs` on that tree.
5. GitHub Actions: CI#40081725681 on `a356f9086` — completed, failure, 17/17 checks, Dependency audit alone failed,
   first fail +404 s, wall 967 s (`ci.py conclusion`, trail `.andromeda/runs/2026-10-24T09-31-30-phase/ci-no-marker.json`).
