proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "In the `Session state directory` row, the seven_guis list reads \"`hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `cc-*`, `cf-*` and `ht-timer` (seven_guis) under `target/tmp/`\" — `hl-client` and `hl-client-rec` the state directories of `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`#[cfg(unix)]`, its `start crud --record` clients run at `RUST_LOG=trace` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started`) — and the row's Source column gains `examples/seven_guis/tests/host_log.rs:40-130`; the rest of the row unchanged."
    sidecar: "obs-plan §9 CI Integration — Session state directory row: seven_guis' checks' state directories gain `hl-client` and `hl-client-rec` (host_log's recording-client test), cite host_log.rs:40-130 added (2026-10-24-stand-requirement-sweep, Count 9)."
    rationale: "The report's Counts fact 9 (`state-dirs`) names `obs-plan-detail/ci-integration.md:15` as Moved: the session checks' state directories gain `hl-client` and `hl-client-rec`; Symbols / APIs (`host_log.rs`, 40-130) states the test runs its clients in those two directories under `CARGO_TARGET_TMPDIR`, and Outcome names them as two test state directories under `target/tmp/`."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:172
note: "The report's fact 9 also names a pre-existing gap — `hl-rec` / `hl-rec-kept` listed at neither this row nor occupied-resources.md:25 — left unproposed here because the report does not measure where or by which check they are made (an owner or a measured read should add them); and with 97 lines added to host_log.rs at 11-15 and 40-131, the row's existing cite `examples/seven_guis/tests/host_log.rs:156` has likely shifted — a cites re-point, not re-derived here."
