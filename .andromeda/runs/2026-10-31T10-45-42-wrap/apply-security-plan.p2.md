# Apply — security-plan · part 2 of 2

## Verdicts
- fanout-security-plan--cascade-g4.md 1 · applied · .andromeda/security-plan-detail/dependency-security.md:11+#1
- fanout-security-plan--cascade-new-dependency-security.md 1 · applied · .andromeda/security-plan-detail/dependency-security.md:49+#1
- fanout-security-plan--cascade-new-dependency-security.md 2 · applied · .andromeda/security-plan-detail/dependency-security.md:49+#2
- fanout-security-plan--cascade-g4.md 2 · applied · .andromeda/security-plan-detail/error-handling.md:9#1, .andromeda/security-plan-detail/error-handling.md:10#1
- fanout-security-plan--cascade-g4.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:9#2
- fanout-security-plan--cascade-new-error-handling.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:32+#1

## Edits
- after: .andromeda/security-plan-detail/dependency-security.md:11
  anchor: « RUSTSEC-2026-0386 (tokio 1.53.2, `AsyncFd::new` I/O safety, fixed in ≥ 1.53.3) is fixed by the lockfile update to tokio 1.54.1, the manifest's `tokio = "1.42"` unchanged, with no ignore added; every standing ignore is still encountered, and the leg prints two warnings, `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA (deny.toml:46) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md.»
  add: « RUSTSEC-2026-0414 (pulldown-cmark 0.13.4) is fixed by the lockfile update to pulldown-cmark 0.13.5, in `Cargo.lock` alone, semver-compatible with no coupled pin and `deny.toml` unmoved; it was reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed (apps/readme/src/markdown/pulldown_cmark.rs:7-12), and the audit leg reads `advisories ok` and `licenses ok` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md.»
- after: .andromeda/security-plan-detail/dependency-security.md:49
  anchor: «- **Licence compliance:** the root `deny.toml` holds upstream's `[licenses]` table — 13 allowed expressions, `include-dev`, `include-build`, `confidence-threshold = 0.8` — byte-identical to upstream's (deny.toml:31-50); the audit leg runs it on every push since 2026-10-14-quality-gates — `licenses` over the whole workspace's graph, reading `licenses ok` on the dev host and in CI#38692629940 — so the fork gates on licences, the open question decided by the founder's «Проверка лицензий (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label; .github/scripts/ci-leg.sh:38); upstream's `licenses` job, taken by the 2026-10-10 merge and reverted, stays reverted; the first reading, `cargo deny --locked check licenses` over the six-target graph: exit 0, `licenses ok`, two `license-not-encountered` warnings (`NCSA`, `Unicode-DFS-2016`) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/licenses.md»
  add: «- **CI action, the free-disk step:** job `Stand checks repeated` runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false` and `swap-storage: false` (.github/workflows/ci.yml:311-316), a CI action site and not a crate; it reads `completed · success` on CI#41192433140, taking `/dev/root` from 86 G free to 109 G free on that runner, and it is not shown to be the cure for the runner out of disk (`Free space left: 5 MB`) on CI#41190771843: the green runner had 86 GB free before the step, the failed run has no `df` line, and the failed job ran on an older image than every other `ubuntu-24.04` job, that image's free disk unread — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/dependency-security.md:49
  anchor: «- **Licence compliance:** the root `deny.toml` holds upstream's `[licenses]` table — 13 allowed expressions, `include-dev`, `include-build`, `confidence-threshold = 0.8` — byte-identical to upstream's (deny.toml:31-50); the audit leg runs it on every push since 2026-10-14-quality-gates — `licenses` over the whole workspace's graph, reading `licenses ok` on the dev host and in CI#38692629940 — so the fork gates on licences, the open question decided by the founder's «Проверка лицензий (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label; .github/scripts/ci-leg.sh:38); upstream's `licenses` job, taken by the 2026-10-10 merge and reverted, stays reverted; the first reading, `cargo deny --locked check licenses` over the six-target graph: exit 0, `licenses ok`, two `license-not-encountered` warnings (`NCSA`, `Unicode-DFS-2016`) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/licenses.md»
  add: «- **CI action, the `matrix_test` free-disk step:** the same action's step on the `matrix_test` job (.github/workflows/ci.yml:392-394) is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest` (`completed · skipped` on `Test (android)`, CI#41190771843) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/error-handling.md:9
  old: «`off-screen`'s that a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows (tests/blitz-tests/tests/transformed_client_rect.rs:183-220)»
  new: «`off-screen`'s that under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view (tests/blitz-tests/tests/visible_region_transformed.rs:246-276)»
- line: .andromeda/security-plan-detail/error-handling.md:10
  old: «its no-area sentence and `off-screen`'s transformed-box sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md»
  new: «its no-area sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md, `off-screen`'s rotation sentence at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/error-handling.md:9
  old: «the first option whose label starts with it»
  new: «the first option whose label starts with it, and covers a `multiple` select: its match alone, a text opening on a space typing nothing»
- after: .andromeda/security-plan-detail/error-handling.md:32
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Mounted handles from a spawned task: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162), any app able to meet it — measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run; Cells calls its handles synchronously, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "sec2-01"
fixed = 'is read where it stands untransformed'

[[pattern]]
id = "sec2-02"
fixed = "transformed-box sentence"
