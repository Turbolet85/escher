proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: ## 5. Integration Test Strategy
    change: After the 2026-10-14-quality-gates clause that reads a client's own stderr by the evidence instrument, add — since 2026-10-24-stand-requirement-sweep a client's own stderr is also read by a standing check, seven_guis `host_log`'s third test `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`cfg(unix)`), which runs `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint` and `stop` as client processes through `common::run` at `RUST_LOG=trace`, in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR` guarded by `common::Started`, and reads over all the clients' stderr 0 ids, 0 names, 0 sentinel occurrences, 0 recording-path occurrences and 0 `recorded_needles`, `host_log` 2 → 3 (examples/seven_guis/tests/host_log.rs:40-130)
    sidecar: integration-test-strategy — a client's own stderr under `start --record` now read at `trace` by a standing check, `host_log` 3 (2026-10-24-stand-requirement-sweep)
    rationale: The report's Counts fact 8 (`host-log`) names this row as moved — the 2026-10-14 one-off reading of a client's own stderr is now joined by a standing check at `trace` under `--record`; Symbols / APIs gives the test, its commands, state directories and needle classes, and Outcome reads `host_log` 3 green under both builds.
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:162
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: ## 5. Integration Test Strategy
    change: Beside the in-process `stand_act_spans` reading, add — since 2026-10-24-stand-requirement-sweep its sink capture also holds a paged part read, `snapshot_part(2)` on `fixtures::hold("paged", fixtures::many_items, …)` read as `Outcome::Part` with part 2 of at least 2, the paged page's 400 `item-NNN` ids and its item buttons' texts joining the id and name needles, 36 calls per mode where 35 stood, still 0 needles on any line (tests/blitz-tests/tests/stand_act_spans.rs:685-693)
    sidecar: integration-test-strategy — `stand_act_spans` reads a paged part 2 in its sink capture, 36 calls per mode (2026-10-24-stand-requirement-sweep)
    rationale: The report's Counts fact 6 (`spans-calls`) and Symbols / APIs state the paged part read added to the in-process capture and the 35 → 36 call count; Outcome reads it met, no id or name of the paged page on a line.
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:53
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: ## 5. Integration Test Strategy
    change: Add the stated limit — a paged `part` read through the `escher-session` host's own sink stays unmeasured, because the host boots only the four lean tasks and none pages, so a paged read is measured in process alone (`stand_act_spans`); its measurement owned by the `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next)
    sidecar: integration-test-strategy — the host-role paged part read stated as an owned limit (2026-10-24-stand-requirement-sweep)
    rationale: The report's Counts fact 7 (`paged-limit`) says the limit is new text placed beside fact 8's sites, which include this file's line 22; inputs#I2's Beyond-the-plan row 1, approved by the operator's «yes — apply.», names the residuals line as its owner.
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 21
    section: ## 5. Integration Test Strategy
    change: Read «its count (2 each) unmoved» as that chunk's reading — «its count (2 each) unmoved then; `host_log` 3 since 2026-10-24-stand-requirement-sweep, `mcp_log` still 2»
    sidecar: integration-test-strategy — `host_log`'s stated count 2 dated, 3 since 2026-10-24-stand-requirement-sweep
    rationale: The report's Counts fact 8 moves `host_log` 2 → 3 and Outcome reads `host_log` 3 · `mcp_log` 2; this clause still states 2 for both as the standing count.
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:21
    dependent-of: D-new-detail
note: The `examples/seven_guis/tests/host_log.rs` citations at lines 20 and 22 (`:251-264`, `:28-345`, `:34-38`) predate the 97 lines added at 11-15 and 40-131 and likely point at moved text — a re-point for the cites tool, not proposed here; no detector of D-tests-coverage, D-tests-framework or D-tests-obs-harness fires (test code only, `cargo test` unchanged, the harness and log format unchanged).
