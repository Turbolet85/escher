# Cascade compare — obs-plan · wrap · 2026-10-25-cold-agent-bar
E items 10 · held 6 · lost at classify 2 · lost at strong 0 · beside a placed fact 1 · cascade only 4 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### log-coverage#8.4 · lost at strong · beside a placed fact F2 (near 113, log-coverage#8.2) · .andromeda/obs-plan-detail/log-coverage.md:115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 115
    section: "## 6. Log Coverage"
    change: "- observed absent — a print, an env read, a subscriber, a socket, a listener or `blitz_net` in the stand's two new boots `boot_cells` and `boot_load` and the load page's in-process network stand-in `LoadNet` (each `fetch` answered with a fixed sheet from a thread of its own after `LOAD_ANSWER_DELAY`, 200 ms; no `is_noop` override, so the harness counts its load), every other boot keeping `net_provider: None`; no log site, span, span field or log line added and `packages/` unchanged · searched: the silence census over `examples/seven_guis/src/stand.rs`, reading 0, and the `git diff` probe over `packages` reading 0 (examples/seven_guis/src/stand.rs:131-243; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its silence-census entry 10)"
    sidecar: "log-coverage.md — absence record for `boot_cells`, `boot_load` and `LoadNet`: no print, env read, subscriber, socket, listener or `blitz_net`, census 0 (2026-10-25-cold-agent-bar, F17)"
    rationale: "Symbols / APIs: `LoadNet` holds no `blitz_net`, print, env read, socket or listener — silence census entry 10 reads 0; Coverage of new surfaces names `boot_cells` and `boot_load` instrumentation n/a; Schema / config: no span field or log line moves; the gates read the stand.rs census and the packages diff probe at 0; the absence group at lines 110-117 records each such chunk's silent additions."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:36
    fact: F17

### span-trace-coverage#2.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:29
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

### span-trace-coverage#3.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:36
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

## Held

- ci-integration#1.7
- ci-integration#1.8
- log-coverage#2.1
- log-coverage#5.1
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#4.1

## Cascade only

- ci-integration#1.6
- log-coverage#4.1
- log-coverage#8.2
- log-coverage#8.3
