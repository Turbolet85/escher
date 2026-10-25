proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 29
    section: "## 4. Span / Trace Coverage"
    change: "add, as its own bullet after line 29 inside the first `values · spans` group: - `packages/escher-driver` — a hosted part read, measured since 2026-10-25-cold-agent-bar: `escher-session serve cells` at `RUST_LOG=trace` answers `snapshot --part 2` through `Session::run` and nothing else, so its spans are the `command` spans above, a part read recording `verb` alone; seven_guis `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` counts the host's closed `command` span lines and reads 0 id and 0 name needles under both builds; no span, field or log line added (examples/seven_guis/tests/host_log.rs:134-277; examples/seven_guis/tests/host_log.rs:259-268; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "span-trace-coverage: the hosted Cells part read's `command` spans measured by `host_log`'s fourth test, no span or field added (2026-10-25-cold-agent-bar, F13)"
    rationale: "Report Changes → Counts F13 (host_log 3 → 4 tests; `serve cells` at `RUST_LOG=trace`, `snapshot --part 2`, 0 id and 0 name needles under both builds) and Schema / config ('No driver schema, verb, argument, cause, session error, span field or log line moves'); New text lists host_log.rs:134-277 with its `command_spans` count at 259-268. The file states a part read records `verb` alone (line 25) and the hosted span measured only over three CRUD calls (line 26); the hosted part read is new span-coverage detail. Binds §6 Log coverage / §8 PII (the needle reading) — name both sides."
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:26
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 36
    section: "## 4. Span / Trace Coverage"
    change: "add, as its own bullet after line 36 inside the `tool_call` `values · spans` group: - `tool_call`'s fields read by the cold-agent pipe, since 2026-10-25-cold-agent-bar: `scripts/cold-agent.sh`'s `span_counts` also counts the accepted `tool_call` lines per `COUNTED` verb (`lint` · `screenshot` · `advance`) in the live server child's `server.log`, giving the verdict's `lint_calls` · `screenshot_calls` · `advance_calls` (reason `capability-unused` when one reads 0), beside the refused and error counts that decide `wrong_calls`; the span keeps its two fields, `tool` and `ending`, none added (scripts/cold-agent.sh:483-484; relation: obs-plan §6 → Log format (the cold-agent pipe); as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "span-trace-coverage: the cold-agent pipe now reads `tool_call`'s `tool` and `ending` to count accepted lint · screenshot · advance calls; no span field added (2026-10-25-cold-agent-bar, F14)"
    rationale: "Report Changes → Symbols / APIs (the pipe: '`span_counts` also returns accepted `tool_call` lines per `COUNTED` verb (`lint` · `screenshot` · `advance`)', verdict fields `lint_calls` · `screenshot_calls` · `advance_calls`, reason `capability-unused`), Coverage of new surfaces ('instrumentation ✓ (counts from `tool_call` span lines)') and F14; New text scripts/cold-agent.sh:483-484. The `tool_call` group (lines 35-36) names no consumer of its fields; the pipe's new per-verb reading is new detail on it. Binds obs-plan §6 Log format (the cold-agent pipe, obs-plan.md:169-171) — name both sides."
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:35
    fact: F14
note: D-obs-instrumentation, D-obs-stack and D-obs-pii read clean for this category — boot_cells / boot_load / LoadNet open no span (report Coverage, instrumentation n/a), no dependency moved, host_log reads 0 needles; no Retired item (the flight booker's mode buttons) and no F1–F21 stale or citation site falls in this file.
