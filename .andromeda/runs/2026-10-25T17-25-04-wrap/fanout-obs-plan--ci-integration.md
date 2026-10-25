proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 17
    section: "## 9. CI Integration"
    change: "Cold-agent pipe state row: the state set becomes `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` — `record/` the live server child's recording directory (`mcp --record`, made `mkdir -m 700`, owner-only), one kept check per accepted `start`, holding the recorded calls and their answers at rest (the stand screens' ids, names, values, snapshot text, diffs and a lint's findings, masked as on every carrier, a shot without its pixels), read by the pipe's `recording` only to set the verdict's `recorded_sessions` and `recording_kept`, never printed or copied and never committed; the replay children run without `--record`; `cold-agent.sh cleanup` removes it with the rest of the state; the rest of the row unchanged."
    sidecar: "obs-plan §9 CI Integration — the cold-agent pipe's state set gains `target/cold-agent/record/` (the live child's `--record` kept checks, owner-only, local, never printed, removed by cleanup) (2026-10-25-cold-agent-bar)."
    rationale: "The report's Schema / config names the pipe's state set as grown by `target/cold-agent/record/`, owner-only, `cleanup` removing it with the state; Symbols / APIs: the live server child runs `mcp --record \"$3\"` into `target/cold-agent/record` (made `mkdir -m 700`), replays run without `--record`, `recording` (257-271) feeds `recorded_sessions` · `recording_kept`; Coverage of new surfaces: kept checks parsed, never printed; Outcome (security): no kept check committed, `target/cold-agent/record/` uncommitted. The row's braced state set omits it."
    basis: scripts/cold-agent.sh:257-271
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "Recording directory row: its closing clause `the checks write theirs under `target/tmp/`` becomes `the checks write theirs under `target/tmp/`, and the cold-agent pipe's live server child its own under `target/cold-agent/record/` (Cold-agent pipe state, below); a full recorded Cells session keeps a check of 312,431 bytes and the load page's 1,660, under the 1,048,576-byte bound (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)`; the rest of the row unchanged."
    sidecar: "obs-plan §9 CI Integration — the recording directory row names the cold-agent live child's `target/cold-agent/record/` beside the checks' `target/tmp/`, with the measured Cells (312,431 B) and load-page (1,660 B) check sizes (2026-10-25-cold-agent-bar)."
    rationale: "The report's Symbols / APIs puts a second launcher of `--record` in the repository's own tooling (the cold-agent live child into `target/cold-agent/record`), so the row's statement of where recording directories live is incomplete; Decisions & corrections measures a full recorded Cells session's check at 312,431 bytes and the load page's at 1,660, under the 1,048,576 bound (probe through the built binary)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:361-362
    dependent-of: D-new-detail
    fact: F14
note: ci-integration.md:15's `examples/seven_guis/tests/host_log.rs:303` cite moved (F13 lists it as `citation`, host_log.rs's body shifted by 134-278) — left to the P2 citation sweep, the report stating no new number; no instrumentation, stack or PII-in-log drift in this category (no span, field, log line or CI step moved).
