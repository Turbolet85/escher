proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 11
    section: "## 8. PII Scrubbing & Compliance"
    change: "In line 11 replace the clause from 'a paged `part` read through the `escher-session` host's own sink stays unmeasured' to '(target: next)' with: since 2026-10-25-cold-agent-bar the paged `part` read through the host's own sink is measured — the host serves six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), and `host_log`'s fourth test (`a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`) runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell, then reads `snapshot --part 2`: 0 id and 0 name needles under both builds (`host_log` 4), the `.andromeda/residuals.md` line that owned it discharged (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md); keep the 2026-10-24 reading of the third test as it stands."
    sidecar: "pii-scrubbing-compliance:11 — the host-sink paged part read is measured (serve cells, part 2, 0 id and 0 name needles, both builds); the host serves six tasks; the residual discharged."
    rationale: "The report's F13 (host-sink-part) and F1 (served-tasks) each list pii-scrubbing-compliance.md:11 stale; Spec claims disproved names «the host boots only the four lean tasks and none pages» false since `serve cells`; inputs#I4 row 1 discharges residuals.md:38 by this measurement."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157-164
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 32
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a line after 32, in the same group: since 2026-10-25-cold-agent-bar the live run's `transcript.jsonl` also carries the stand's new screens to the model provider — Cells' contents, the flight booker's select with its options and value, a lint's findings and a shot — on the founder's «Запускать сейчас (Рекомендую)» (inputs#I6) within «Один запуск, до $3 (Рекомендую)» (inputs#I3); the one run of 2026-10-25 (6 tasks, 66 calls, 0 wrong) keeps the transcript unprinted, its committed copy host-path-masked, and its events and verdict carry counts and closed words only (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "pii-scrubbing-compliance:32+ — the 2026-10-25 live run's carriage of the new screens (Cells, the select, a lint, a shot) to the model provider, on the founder's I6 within I3."
    rationale: "The report's F15 (live-run) lists pii-scrubbing-compliance.md:32 stale: the carriage line names only the 2026-10-15 run and the 0.1.0 screens; Coverage of new surfaces states the pipe's PII redacted (events and verdict counts and closed words, planted-marker cases green)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:175-181
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 32
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a line after 32 (after the F15 line), in the same group: since 2026-10-25-cold-agent-bar the live server child runs `mcp --record` into `target/cold-agent/record/` (made `mkdir -m 700`, removed by `cleanup` with the state), so each accepted `start` leaves one kept check holding that session's calls and answers — the screens' readings at rest, masked as on every carrier — never printed, copied or committed; the replay children run without `--record`; the verdict's new fields `wrong_call_bar_pct` · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept` and reasons `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar` are counts, booleans and closed words, none content-named (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "pii-scrubbing-compliance:32+ — the pipe's `--record` kept checks under target/cold-agent/record/ (owner-only, never printed) and the verdict's seven new count/closed-word fields."
    rationale: "The report's F14 (pipe) and Schema / config state the live child's `--record` into owner-only `target/cold-agent/record/` and the verdict's seven fields and three reasons; Outcome (obs) states the kept checks never printed or copied and Outcome (security) no kept check committed."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:79-83
    fact: F14
note: "No detector fires as violated — the chunk adds no log line, span field or dependency and its new boots carry no instrumentation by design (Coverage of new surfaces); the body's §8 cold-agent line (.andromeda/obs-plan.md:262) and its host-reading line (.andromeda/obs-plan.md:250) are the body's own and may want the same F13/F14 broad line."
