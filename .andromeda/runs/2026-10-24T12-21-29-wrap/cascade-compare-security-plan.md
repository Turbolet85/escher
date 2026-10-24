# Cascade compare — security-plan · wrap · 2026-10-24-upstream-sync-polish-and-ship
E items 10 · held 4 · lost at classify 4 · lost at strong 0 · cascade only 2 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:10
  - detector: D-security-deps
    severity: escalate
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 10
    section: "## Dependency Security"
    change: |-
       RUSTSEC-2026-0386 (tokio 1.53.2, `AsyncFd::new` I/O safety, fixed in ≥ 1.53.3) is fixed, since 2026-10-24-upstream-sync-polish-and-ship, by the lockfile update to tokio 1.54.1 (`cargo update -p tokio`; the manifest's `tokio = "1.42"` unchanged, no package entering or leaving the lock), with no ignore added — the advisory turned the Dependency audit red alone on the base (CI#40081725681 on `a356f9086`); after the update the audit leg reads `advisories ok, licenses ok` on the dev host and the Dependency audit reads green in CI#40092092468 on `34f6f051b`, every standing ignore still encountered (no `advisory-not-detected` warning) and two warnings standing, `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA (deny.toml:46) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md
    sidecar: "Dependency Security › Audit tool: the tokio RUSTSEC-2026-0386 record — fixed by the lockfile update to 1.54.1, no ignore added, audit green (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Dependencies (tokio 1.53.2 → 1.54.1 in the lock alone, past RUSTSEC-2026-0386), Cross-project (RUSTSEC-2026-0386 cleared, audit `advisories ok, licenses ok`, gate entry 25; base CI#40081725681 red by Dependency audit alone; CI#40092092468 green) and Outcome (no ignore added, every standing ignore encountered, the two warnings); Expected amendments names the tokio record a new record in dependency-security.md (git grep 0). New detail in the shape of line 10's rustls lockfile-update record."
    basis: .andromeda/security-plan-detail/dependency-security.md:10

### error-handling#3.1 · lost at classify · .andromeda/security-plan-detail/error-handling.md:26
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block (`<!-- detail: values · errors -->` … `<!-- /detail -->`) holding one bullet: Parallel style traversal, no panic path: since the 2026-10-24 upstream sync (`fb297a38c`, #1243) a process-wide `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` is taken around every `StyleThreading::Parallel` traversal, its poison recovered by `unwrap_or_else(|e| e.into_inner())`, so two documents resolving `Parallel` at once are serialised and no longer panic with `already mutably borrowed` (blitz issue 430); the windowed `dioxus_native::launch` and the browser build their documents `Parallel`, no headless document (harness, stand, driver) sets `style_threading`, and the default stays `Sequential` (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record the PARALLEL_TRAVERSAL_LOCK (poison recovered, no panic path) that retires the two-Parallel-documents panic"
    rationale: "Report Changes → Symbols (`static PARALLEL_TRAVERSAL_LOCK`: 'Poison is recovered … so there is no panic path. Two documents resolving `Parallel` concurrently are now serialised and no longer panic'; `dioxus_native::launch` now `Parallel`) and Retired ('The `StyleThreading` doc's warning that two `Parallel` documents panic is retired with the lock') — a panic path removed and a no-panic guard added, new detail for Error Handling; no passage of this file named the old panic, so no D-retired."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:18

### error-handling#3.1 · lost at classify · .andromeda/security-plan-detail/error-handling.md:26
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block holding one bullet: Apps and examples, `multi_doc` (a root example since the 2026-10-24 upstream sync, `fb297a38c`): its mode argument from `std::env::args` panics through `panic!` on an unknown mode, while its iteration count and `MULTI_DOC_N` fall back to a default on a parse failure and `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` are read as flags (examples/multi_doc.rs:48-54; examples/multi_doc.rs:55-58; examples/multi_doc.rs:17-19; examples/multi_doc.rs:108) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record multi_doc's panic! on an unknown mode beside its defaulted numeric reads"
    rationale: "Report Changes → Symbols (`examples/multi_doc.rs` … 'a `panic!` on an unknown mode (108)', `MULTI_DOC_N` 'parsed with a default fallback') and Expected amendments (security-plan §Error Handling: '`multi_doc.rs`'s `panic!`'); new CLI-input failure handling in the Apps-and-examples panic class (body security-plan.md:369), consistent with §Input Validation's examples rule (body :53), so no violation — new detail."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24

### error-handling#3.1 · lost at classify · .andromeda/security-plan-detail/error-handling.md:26
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 26
    section: "## Error Handling"
    change: "Add after line 26 a new detail block holding one bullet: WPT runner, the attribute test runner: since the 2026-10-24 upstream sync (`f8723f495`, #1247) its new offset reads include a `get_node(node_id).unwrap()`, a panic on a node id that does not resolve, on an upstream-only path the WPT runner's per-test panic catch turns into a CRASH result (wpt/runner/src/test_runners/attr_test.rs, among its added lines 3 · 97 · 135-136 · 141-147 · 164-165) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "error-handling: record attr_test.rs's new get_node(node_id).unwrap() panic path in the WPT runner"
    rationale: "Report Changes → Symbols (WPT runner: '`attr_test.rs` gains the reads at added 3 · 97 · 135-136 · 141-147 · 164-165, including a `get_node(node_id).unwrap()` on an upstream-only path') and Expected amendments (security-plan §Error Handling: '`attr_test.rs`'s `get_node(node_id).unwrap()`'); a new panic path in the WPT-runner class (body security-plan.md:370), caught per test by the runner (body :353) — new detail; the report names no single line for the unwrap, so the cite gives its added ranges."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:25

### input-validation#1.4 · lost at classify · .andromeda/security-plan-detail/input-validation.md:19
  - detector: D-security-input
    severity: escalate
    file: .andromeda/security-plan-detail/input-validation.md
    near: 19
    section: "## Input Validation"
    change: "New row after the `preact_script` row — - **Boundary:** CLI arguments and environment (examples/multi_doc.rs) / - **What to validate:** A mode and an iteration count from argv; `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT` from the environment / - **How (stack-specific):** Since 2026-10-24-upstream-sync-polish-and-ship (upstream #1243, `fb297a38c`, upstream's text as written): `MULTI_DOC_N` parsed with `parse().ok()` and a default fallback (examples/multi_doc.rs:55-58); `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT` read as flags through `env_flag` (examples/multi_doc.rs:17-19; examples/multi_doc.rs:22; examples/multi_doc.rs:36-40); the mode and the iteration count read from `std::env::args` (examples/multi_doc.rs:48-54), an unknown mode a `panic!` (examples/multi_doc.rs:108); none of the three secret-bearing, registered beside the other env reads in §Secret Management's Environment values read — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "Input Validation: new row for examples/multi_doc.rs — argv mode and count, three MULTI_DOC_* env reads, MULTI_DOC_N parse().ok() with a default, an unknown mode panics (upstream #1243)."
    rationale: "The report's Changes → Symbols adds `examples/multi_doc.rs`, a new root example reading three env variables and `std::env::args`, with a `panic!` on an unknown mode; Coverage of new surfaces marks its validation `MULTI_DOC_N parse().ok() with a default ✓`. The boundary is validated per the category's mandated example-CLI mechanism (numeric parse with default fallback, a panic on an unresolvable argument as for `preact_script`), so no drift — but the category holds no row for it; the report's Expected amendments name the multi_doc reads for registration."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24

### secret-management#1.2 · lost at classify · .andromeda/security-plan-detail/secret-management.md:15
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 15
    section: "## Secret Management"
    change: "add after line 15, inside the values · secrets block: - **Environment values read (root example, none secret-bearing):** since 2026-10-24-upstream-sync-polish-and-ship upstream's root example `examples/multi_doc.rs` (taken from `fb297a38c`) reads three env variables through two `std::env::var` call sites — `MULTI_DOC_N`, a document count parsed with a default fallback (examples/multi_doc.rs:55-58), and the presence flags `MULTI_DOC_SEQ` (examples/multi_doc.rs:36-40) and `MULTI_DOC_FLAT` (examples/multi_doc.rs:22), both through `env_flag` (examples/multi_doc.rs:17-19) — no credential, token or key and no new secret source; it also reads `std::env::args` (examples/multi_doc.rs:48-54); registered beside the body's Environment values read list (security-plan.md:294) and arch §Occupied Resources → Environment variables"
    sidecar: "secret-management: registered the root example multi_doc's three env reads (MULTI_DOC_N · MULTI_DOC_SEQ · MULTI_DOC_FLAT), none secret-bearing (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The report's Changes → Symbols names three new env reads in examples/multi_doc.rs (MULTI_DOC_N 55-58, MULTI_DOC_SEQ 36-40, MULTI_DOC_FLAT 22, through env_flag 17-19), and Expected amendments lists them as a new registration at security-plan §Secret Management, Environment values read (security-plan.md:294), with pattern multi-doc unfired and 0 detail hits; the standing rule is that source's env reads are an enumerated, non-secret set, so an unregistered read is drift against §Secret Management."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24

## Held

- dependency-security#2.5
- input-validation#1.102
- input-validation#1.67
- logging-monitoring#1.2

## Cascade only

- input-validation#1.2
- secret-management#2.2
