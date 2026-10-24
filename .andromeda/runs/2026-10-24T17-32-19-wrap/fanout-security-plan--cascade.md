proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "After the 2026-10-14 reading of a client's own stderr, add: since 2026-10-24-stand-requirement-sweep that one-off reading is joined by a standing check, seven_guis `host_log`'s unix-only `a_recording_sessions_clients_log_nothing_of_it_at_trace` — a recorded session's clients (`start crud --record`, a `type` of a sentinel into `crud-name`, `snapshot`, `snapshot --part 1`, `lint`, `stop`) run at `RUST_LOG=trace` in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, and over all their stderr it reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded needles, under both builds (examples/seven_guis/tests/host_log.rs:40-130; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)."
    sidecar: "logging-monitoring#1.1: the client's own stderr gains a standing check at trace under --record (host_log 2 → 3), beside the 2026-10-14 one-off reading."
    rationale: "Report Count 8 moves security-plan-detail/logging-monitoring.md:14 — the 2026-10-14 one-off reading of a client's own stderr, now joined by a standing check at trace under --record (report.md:162-168); the test and its needles at report.md:59-63; under both builds at report.md:327-328. This carries Count 8's moved row in this file and the task's F1."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "Add, after the F1 sentence: the in-process sink capture also reads a paged part — `stand_act_spans` reads part 2 of a screen that pages (the shared fixture page `many_items`, its 400 item ids and button names among the needles, none on a line), 36 calls per mode (tests/blitz-tests/tests/stand_act_spans.rs:685-693) — while a paged `part` read through the `escher-session` host's own sink stays unmeasured, a stated limit: the host boots only the four lean tasks and none pages; owned by a `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next), on the operator's word on P5 card 2 (2026-10-24)."
    sidecar: "logging-monitoring#1.1: a paged part read is measured in the in-process capture (stand_act_spans, part 2) and stated unmeasured through the host's sink, a residuals line its owner."
    rationale: "Report Count 7 states the host-sink paged read a new limit, 'placed beside fact 8's sites' (report.md:158-161), with its owner the operator's Beyond-the-plan row 1 (report.md:207-215, 282-284); Count 6 and the Symbols give the in-process paged read part 2, 36 calls per mode (report.md:53-56, 151-153). Line 13's part-read claim cites host_log and mcp_log, whose part reads page nothing, so the limit qualifies it here."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 13
    section: "## Logging & Monitoring"
    change: "Re-point the citation examples/seven_guis/tests/host_log.rs:251-264 (the shot's base64 needle reading) to the lines it holds after this chunk's two added blocks, host_log.rs:11-15 and 40-131 (97 lines added before it; +97 where nothing was removed — re-read at apply); the claim itself stands."
    sidecar: "logging-monitoring#1.1: host_log.rs citation re-pointed past the blocks this chunk added at 11-15 and 40-131."
    rationale: "host_log.rs is a reached file; the report's New text lists its added ranges 11-15 · 40-131 (report.md:407-408), both before the cited 251-264, so the cited lines have moved. The reading the citation backs (0 shot-base64 needles at trace) is unmoved — report reads host_log 3 green under both builds (report.md:327-328)."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:408
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 28
    section: "## Input Validation"
    change: "Re-point the citations .github/scripts/test_ci_workflows.py:422-456 and .github/scripts/test_ci_workflows.py:476-492 (the `InstallScriptTest` cases) to the lines they hold after the four lines this chunk added at test_ci_workflows.py:359-362 (+4 where nothing was removed — re-read at apply); the apt-install.sh citations and the row's claims stand."
    sidecar: "input-validation#1.7: InstallScriptTest citations re-pointed past the stand-repeat pin's lines added at 359-362."
    rationale: "test_ci_workflows.py is a reached file; the report's New text lists its one added range 359-362 (report.md:405-406), before both cited ranges, so they have moved. Fact 2 rules the other test_ci_workflows.py rows' content no-change (report.md:113-115) — the invariants hold, only the line numbers moved."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:406
note: "Unpicked body lines carry the same line drift — security-plan.md:236 and :264 cite .github/scripts/ci-leg.sh:20-65, which gained 54-58 inside the range, and security-plan.md:301 cites examples/seven_guis/tests/host_log.rs:22, past the block added at 11-15 — for the orchestrator or the citation re-point run to place."
