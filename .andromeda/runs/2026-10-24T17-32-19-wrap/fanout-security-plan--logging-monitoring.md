proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: '## Logging & Monitoring'
    change: 'After the 2026-10-14-quality-gates reading on line 14, append: since 2026-10-24-stand-requirement-sweep a client''s own stderr is also a standing check, not only that one-off reading — `host_log`''s unix-only `a_recording_sessions_clients_log_nothing_of_it_at_trace` runs `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint` and `stop` as client commands at `RUST_LOG=trace` and reads, over all the clients'' stderr, 0 id needles, 0 name needles, 0 sentinel occurrences, 0 recording-path occurrences and 0 recorded-check needles, under both builds (`host_log` 3 as `-p seven_guis` and as `--workspace`) (examples/seven_guis/tests/host_log.rs:40-130; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)'
    sidecar: 'logging-monitoring.md:14 — a client''s own stderr gains a standing check at `trace` under `--record` (`host_log` 2 → 3), 0 needles of every class'
    rationale: 'Report Changes, Counts fact 8 (`host-log`) lists security-plan-detail/logging-monitoring.md:14 as Moved — the 2026-10-14 one-off reading of a client''s own stderr is now joined by a standing check at `trace` under `--record`; Expected amendments names security-plan §Logging & Monitoring for it.'
    basis: examples/seven_guis/tests/host_log.rs:40-130
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: '## Logging & Monitoring'
    change: 'Beside the line-14 addition, append the stated limit: a paged part read (part 2 of a screen over `SNAPSHOT_TEXT_BUDGET`) is read through the sink in process — `stand_act_spans` reads `snapshot` part 2 of ≥ 2 on the shared fixture page `fixtures::many_items` (400 items) in its sink capture, 36 calls per mode, with no id or name of that page on a line (tests/blitz-tests/tests/stand_act_spans.rs:685-693) — while a paged part read through the `escher-session` host''s own sink stays unmeasured: the host boots only the four lean tasks and none pages, so `host_log`''s and `mcp_log`''s part reads on line 13 are of a screen within the budget; its measurement owned by a `.andromeda/residuals.md` line «a paged `part` read through the escher-session host''s sink» (target: next) (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)'
    sidecar: 'logging-monitoring.md:14 — a paged part read through the sink measured in process (`stand_act_spans` 35 → 36 calls), the host''s own sink a stated, owned limit'
    rationale: 'Report Counts fact 7 (`paged-limit`): the limit is new text, placed beside fact 8''s sites (logging-monitoring.md:14 among them), owned by inputs#I2''s Beyond-the-plan row 1; fact 6 adds the in-process paged part read to the sink capture; Expected amendments name the host-role paged read as a stated limit.'
    basis: tests/blitz-tests/tests/stand_act_spans.rs:685-693
note: D-security-input, D-security-auth and D-security-deps hold — the report adds no external-input surface, no auth or secret handling and no dependency (test files and one CI leg only); the residuals line the second proposal cites is placed at this wrap's P5 route-resolve per inputs#I2.
