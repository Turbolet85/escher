proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 8
    section: '## API Security'
    change: 'Open the row: No served network API exists — no served TCP or UDP port: two `blitz-net` tests each call `std::net::TcpListener::bind("127.0.0.1:0")` (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534) inside its `#[cfg(test)] mod tests`, served by no binary, reached by no caller, so no crossing, registered as test-only under the session rule of 2026-10-15; the one listener is local: the driver session socket, the rest of the row unedited.'
    sidecar: 'API Security: «no TCP or UDP port» reads as no served one; the two `blitz-net` test-only TCP loopback binds registered.'
    rationale: 'The report measures two new test-only binds and rules that this claim, read as no served port, changes by a word saying the two tests bind; session rule 2026-10-15 owes the registration in the security plan (report.md:28, report.md:47, report.md:88). Its other side is architecture §Occupied Resources.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 209
    section: '## API Security'
    change: 'Open the broad line: No served network API exists — no served TCP or UDP port, two `blitz-net` tests alone binding TCP on loopback, served by no binary; the one listener is local, the rest of the line unedited.'
    sidecar: 'API Security broad line: no served port, the two test-only TCP loopback binds named.'
    rationale: 'The broad line that api-security.md:8 details states the same «no TCP or UDP port» (report.md:28, report.md:47).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:47
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 31
    section: '## API Security'
    change: 'Follow the Configuration continuation with its own continuation line, the two lines before it standing near the 2 000-char bound: — a failed load ends the wait since 2026-11-01-upstream-sync-ui-graph: `CountedHandler` forwards `NetHandler::error` (packages/blitz-test-harness/src/settle.rs:121-125), where a failed stylesheet otherwise read `NotSettled(Loads)` for good on a harness document and on the driver; `settle_waiting` is unedited and returns where it would otherwise have run to `LOAD_WAIT` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'API Security, settle and loads in flight: a failed load ends the wait through the forward in `CountedHandler`.'
    rationale: 'The report names this owed sentence: without the forward the fix in `f6edb9701` stopped at the wrapper; `settle_waiting` is unedited (report.md:25, report.md:112).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 24
    section: '## Dependency Security'
    change: 'Follow the row, which stays as the history of the last take, with: - The upstream sync read at `e42ee1f25` took twelve commits, `fe33b778b` · `8b1a0a3ce` · `dee854c76` · `f6edb9701` · `6fca6308d` · `7a3dd8918` · `91ef65e21` · `a26c6ddeb` · `f76777766` · `6ef4b532d` · `ae774d7f4` · `e42ee1f25`, by no-commit cherry-picks (`45c333f16`), thirty-six of 57 now taken and twenty-one left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Dependency Security: the lock record of this take beside the last take, its history kept.'
    rationale: 'The report rules line 24 no-change as history with a new sentence for this take beside it; Dependencies reads none, the pins stand and the audit is green (report.md:30, report.md:35, report.md:96, report.md:137, report.md:141). D-security-deps: no new dependency, so no drift.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:30
    fact: F3
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 52
    section: '## Dependency Security › ### Supply chain integrity'
    change: 'Replace the row with: - **CI action, the `matrix_test` free-disk step:** removed since 2026-11-01-upstream-sync-ui-graph: `Test (android)` read no such step in CI#41333172440, and `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115) pins the action to `Stand checks repeated` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Dependency Security, supply chain: the `matrix_test` free-disk step retired, removed from ci.yml.'
    rationale: 'The Retired bullet names the step and the report rules this line changed, removed; the step of `Stand checks repeated` at line 51 is no-change (report.md:33, report.md:43, report.md:64, report.md:144).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 52
    section: '## Dependency Security › ### Supply chain integrity'
    change: 'Follow the row with: - **Lockfile verification, the `embolden` leg:** `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` (.github/scripts/ci-leg.sh:28-29) is a locked cargo leg, in `LEGS` and last in `FAST_LEGS` (.github/scripts/ci-leg.sh:8-9), with no CI job of its own, macOS exercising the feature there — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Dependency Security, supply chain: the new `embolden` leg recorded as a locked cargo leg with no CI job.'
    rationale: 'The report adds the leg to `ci-leg.sh` with `--locked`, last in `FAST_LEGS` and with no CI job (report.md:64, report.md:143).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 42
    section: '## Error Handling'
    change: 'Follow the row with: - Mounted event of a removed element: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239), an engine behaviour change for every document; the upstream test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Error Handling: the mounted-event guard of `6ef4b532d` recorded, with its upstream test left out.'
    rationale: 'The report takes `6ef4b532d` and leaves its test out; the row at line 42, on a spawned task, stays true beside it (report.md:20, report.md:80).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
    fact: F11
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 48
    section: '## Error Handling'
    change: 'Replace the row with: - Key press after a hidden ancestor: retired since 2026-11-01-upstream-sync-ui-graph: a key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, `style="display:none"` and CSS alone, in both layout modes; focus stays and the key still acts, `DocumentMutator::flush` unedited — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md. The «not fixed» clause and its owner go with it.'
    sidecar: 'Error Handling: the key-press panic retired, together with its boundary sentence in the threat model; the owner clause dropped.'
    rationale: 'The Retired bullet names the panic, and the report rules this line changed and retired together with threat-model-summary.md:11; its owner row is answered to nothing, the owed fix having landed (report.md:23, report.md:33, report.md:50, report.md:62, report.md:139).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 48
    section: '## Error Handling'
    change: 'Follow the row with: - Anonymous block dropped: `BaseDocument::deallocate_anonymous_block` takes the `layout_children` of the block and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so the id stops being stale and every reader of a stale `layout_parent` is cured with it; witnessed by `key_press_after_container_lost_its_box`, 3 tests in both layout modes, and by row 3 of `select_popup`, both red before the fix — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Error Handling: the fix to `deallocate_anonymous_block` and its two witnesses recorded.'
    rationale: 'The report states the fix, its cure of every reader and its witnesses, red before (report.md:23, report.md:27, report.md:139).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 54
    section: '## Error Handling'
    change: 'Replace the clause «this holds of the close alone, a key pressed after an ancestor gained `hidden` being the panic of this section row `Key press after a hidden ancestor`» as the row spells it with: a key pressed after an ancestor gained `hidden` no longer panics, `select_popup` pressing it in every row (tests/blitz-tests/tests/select_popup.rs:513), as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; the rest of the row unedited.'
    sidecar: 'Error Handling, select list closing: the clause pointing at the retired key-press panic replaced, the row-3 skip gone.'
    rationale: 'The Retired bullet names the panic and the row-3 skip of `select_popup`; line 513 is now one `harness.press` for every row, so this clause no longer holds (report.md:26, report.md:33).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: '## Logging & Monitoring'
    change: 'Follow the row with: - A failed load error text, since 2026-11-01-upstream-sync-ui-graph: it stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier takes it; on that line `url` and `error` are in the scrub set of the sink above (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59) and `waiting_nodes` prints; no `tracing` site is added — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md'
    sidecar: 'Logging & Monitoring: where the error text of a failed load stops, under the unchanged scrub set.'
    rationale: 'The report states the stop and the scrub set, with no schema or scrub set moved (report.md:28, report.md:31, report.md:103). It also reads «and the message», left out here: see note.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
    fact: F12
note: 'For the orchestrator: (1) F12 and report.md:31 say the sink prints the message of `Resource load failed`, while logging-monitoring.md:8 says an engine target prints its message as `[redacted]`, and `escher-telemetry` holds no diff, so the proposal leaves «and the message» out and this conflict is unsettled by any read here; (2) F9 has no picked entity to carry it, so it goes to the new-detail pass, and security-plan.md:326 «the handler is not called on error» is contradicted by report.md:14, with security-plan.md:324 also moved, a failed stylesheet now being answered; (3) neither is a line a picked group details, and neither is security-plan.md:264, whose list of locked commands omits `embolden` (F4 stands in the detail); all three body lines need an owner; (4) threat-model-summary.md:11 retires with error-handling.md:48 (report.md:50) in another group.'
