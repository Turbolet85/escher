proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 26
    section: "## 4. Span / Trace Coverage"
    change: "Re-point `tests/blitz-tests/tests/stand_act_spans.rs:555-567`, the busy-class `command` line reading, to the same block. The 3 lines added at 6-8 and the 1 added at 243 come before it and shift it down; cites.py resolves the new range. The sentence around it stays."
    sidecar: "span-trace-coverage — the `stand_act_spans.rs` busy-class citation re-pointed past the lines 2026-10-24-stand-requirement-sweep added at 6-8 and 243"
    rationale: "The report lists `stand_act_spans.rs` as changed and gives its new text at 6-8 · 243 · 685-693 · 727 (report `## New text, by line`). The two earlier ranges sit above the cited 555-567, so the citation no longer points at the reading it names. The claim itself is unchanged: report fact 6 keeps the span rule as no-change."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:425
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: "## 6. Log Coverage"
    change: "Re-point `examples/seven_guis/tests/host_log.rs:251-264`, the shot-needle reading, to the same block. The 97 lines added at 11-15 and 40-131 come before it and shift it down; cites.py resolves the new range. Leave `mcp_log.rs:141-154` and the line's text as they stand: the part-read line's shape is unchanged."
    sidecar: "log-coverage — the `host_log.rs` shot-needle citation re-pointed past the lines 2026-10-24-stand-requirement-sweep added at 11-15 and 40-131"
    rationale: "The report gives `host_log.rs`'s new text at 11-15 · 40-131 (the third test at 40-130). Both ranges sit above the cited 251-264, so the citation moves. Report fact 6 keeps `log-coverage.md:41`'s part-read shape as no-change. `mcp_log` is untouched."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:407-409
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 107
    section: "## 6. Log Coverage"
    change: "Add after it: \"- The stand requirement sweep's checks are silent: `stand_requirement_sweep` (its second table `WITNESSES_020` and the leg script `.github/scripts/ci-leg.sh`, read through `include_str!`), `stand_lint`'s in-file page `reading_page` and its seventh test, `text_extent`'s helper `extent` and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS` (moved in from `stand_snapshot_parts`) and seven_guis `host_log`'s third test add no log site, span, field, log field, env read or listener — the chunk's silence census reading 0 (its gate entry 8); `packages/` is unchanged, so escher-telemetry and escher-driver keep their sites, and the `RUST_LOG=trace` set on `host_log`'s client commands is the registered `RUST_LOG` read (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md).\""
    sidecar: "log-coverage — absence · logs gains the stand requirement sweep's silence line (census 0, gate entry 8)"
    rationale: "Task F4. The report's Coverage of new surfaces names no new hot-path op ('test files and one CI leg only'). Its Outcome reads the silence census at 0 (gate entry 8). It records no engine, driver or telemetry change, and `RUST_LOG` on the client `Command` as the registered read."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:330-336
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 9
    section: "## 8. PII Scrubbing & Compliance"
    change: "\"(35 calls per mode, eight verbs read back, a part read among them, its line `verb` alone)\" → \"(36 calls per mode, eight verbs read back, two part reads among them — part 1, and since 2026-10-24-stand-requirement-sweep part 2 of a screen that pages — each line `verb` alone)\""
    sidecar: "pii-scrubbing-compliance — `stand_act_spans` 35 → 36 calls per mode, a paged part read added to the sink capture"
    rationale: "Report fact 6 (`spans-calls`) lists `obs-plan-detail/pii-scrubbing-compliance.md:9` as moved: the call count goes 35 → 36 (stand_act_spans.rs:727) once `snapshot_part(2)` is read on `fixtures::many_items` (685-693)."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:151-154
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 9
    section: "## 8. PII Scrubbing & Compliance"
    change: "Append to line 9: \"Since 2026-10-24-stand-requirement-sweep the paged read is `fixtures::many_items` held as `paged` and read through `snapshot_part(2)` as `Outcome::Part` part 2 of at least 2. The page's 400 `item-NNN` ids join the id needles and its buttons' text joins the name needles (`Needles::read`). No id or name of the page appears on a captured line (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md).\""
    sidecar: "pii-scrubbing-compliance — the paged part read's needles (400 ids, the buttons' text) read 0 on the children's captured lines"
    rationale: "Task F3. In `stand_act_spans.rs`, report Symbols puts the paged read at 685-693 and the needle intake at 243. Outcome: 'stand_act_spans 36 calls per mode with a paged part 2 of ≥ 2, no id or name of the paged page on a line — met'."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:53-56
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 10
    section: "## 8. PII Scrubbing & Compliance"
    change: "In \"since 2026-10-14-quality-gates the three readings that left out are taken too — a client's own stderr, …\", mark the client's reading as joined by a standing check: \"a client's own stderr (a one-off reading, joined since 2026-10-24-stand-requirement-sweep by `host_log`'s standing check at `trace` under `--record`, below)\"."
    sidecar: "pii-scrubbing-compliance — the one-off 2026-10-14 client-stderr reading now joined by a standing `host_log` check"
    rationale: "Report fact 8 (`host-log`) lists `obs-plan-detail/pii-scrubbing-compliance.md:10` as moved. It is one of the four rows stating the 2026-10-14 one-off reading of a client's own stderr, 'now joined by a standing check at `trace` under `--record`'."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:162-168
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 10
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add after line 10: \" A recording session's clients are read by a standing check since 2026-10-24-stand-requirement-sweep. `host_log`'s third test, `a_recording_sessions_clients_log_nothing_of_it_at_trace` (built on unix only), runs each client command through `common::run` at `RUST_LOG=trace`: `start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop`. It runs in the state directories `hl-client` and `hl-client-rec`, guarded by `common::Started`. Over all the clients' own stderr it reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences, under both builds (`host_log` 3 green as `-p seven_guis` and as `--workspace`; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md).\""
    sidecar: "pii-scrubbing-compliance — `host_log`'s third test, a recording session's clients' stderr at `trace`, 0 needles"
    rationale: "Task F1. Report Symbols gives `host_log.rs` 40-130. The gates show `-p seven_guis` and `--workspace` both reading host_log 3. Outcome: 'the new test reading 0 of each needle class — met'."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59-63
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 10
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add after the standing-check sentence: \" A paged `part` read through the `escher-session` host's own sink stays unmeasured. The host boots only the four lean tasks, and none of them pages. This stated limit is owned by a `.andromeda/residuals.md` line, «a paged `part` read through the escher-session host's sink» (target: next), placed at 2026-10-24-stand-requirement-sweep's wrap on the operator's word (inputs#I2).\""
    sidecar: "pii-scrubbing-compliance — the host-role paged part read stated as an unmeasured limit with a residuals owner"
    rationale: "Task F2. Report fact 7 (`paged-limit`) calls it new text, a stated limit with an owner. The operator approved its Beyond-the-plan row 1 (inputs#I2): a new residuals line, target next, placed at this chunk's wrap."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158-161
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "Re-point `.github/scripts/ci-leg.sh:88-89`, the leg log's truncation and write, to the same lines. The 5 lines added at 54-58 (the `stand-repeat` named list) come before them and shift them down; cites.py resolves the new range. The row's text stays: the per-leg log handling is unchanged."
    sidecar: "ci-integration — the per-leg log's `ci-leg.sh` citation re-pointed past the stand-repeat list added at 54-58"
    rationale: "Report Schema / config and `## New text, by line` show `ci-leg.sh` gaining 54-58, which sits before the cited 88-89. The report changes only the `stand-repeat` selection, so the log handling still holds."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:403-404
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "\"`hb-*`, `hl-trace`, `cc-*`, `cf-*` and `ht-timer` (seven_guis)\" → \"`hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `cc-*`, `cf-*` and `ht-timer` (seven_guis)\". Also re-point the citation `examples/seven_guis/tests/host_log.rs:156`, which the lines added at 11-15 and 40-131 shift down; cites.py resolves the new line."
    sidecar: "ci-integration — the session state-directory list gains `hl-client` and `hl-client-rec` (host_log's third test)"
    rationale: "Report fact 9 (`state-dirs`) lists `obs-plan-detail/ci-integration.md:15` as moved. Symbols places the new test's state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR` (host_log.rs 40-130), which sit above the cited line 156."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:172-175
note: Not proposed here (outside the picked entities) — the report's fact 9 says `hl-rec` / `hl-rec-kept` are missing from ci-integration.md:15, a gap from before this chunk; the task's Touches on body lines (obs-plan.md:72, 77, 79, 80, 88, 90, 91, 92, 95) are for the body pass.
