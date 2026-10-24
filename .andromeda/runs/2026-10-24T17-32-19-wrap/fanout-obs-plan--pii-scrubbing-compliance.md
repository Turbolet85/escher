proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 9
    section: "## 8. PII Scrubbing & Compliance"
    change: "In line 9's parenthesis, replace \"(35 calls per mode, eight verbs read back, a part read among them, its line `verb` alone)\" with \"(36 calls per mode, eight verbs read back, two part reads among them, each line `verb` alone — `part` 1 of a stand screen and, since 2026-10-24-stand-requirement-sweep, `part` 2 of a screen that pages, the shared fixture page `fixtures::many_items` held as `paged` and read as `Outcome::Part { part: 2, parts ≥ 2 }`, its 400 `item-NNN` ids joining the id needles and its `#items > button` texts the name needles, no id or name of the paged page on a line; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)\""
    sidecar: "§8 PII Scrubbing — stand_act_spans' sink capture 35 → 36 calls per mode: a paged part 2 read of fixtures::many_items, its ids and names joining the needles, 0 on a line (2026-10-24-stand-requirement-sweep)"
    rationale: "The report's Counts fact 6 names this line as moved (\"35 calls per mode … a part read among them\"): `stand_act_spans`' `child_records_each_command_at_info` now holds `fixtures::many_items` and reads `snapshot_part(2)` as `Outcome::Part { part: 2, parts ≥ 2 }`, `Needles::read` takes `#items > button` text as name needles and its 400 `item-NNN` ids join the id needles, the call count 35 → 36; the Outcome reads \"no id or name of the paged page on a line\"."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:53
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 10
    section: "## 8. PII Scrubbing & Compliance"
    change: "Append to line 10, after the 2026-10-14-stand-requirement-sweep reading: \"; since 2026-10-24-stand-requirement-sweep a client's own stderr is read by a standing check, not only the 2026-10-14 one-off reading: seven_guis `host_log`'s third test, `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`cfg(unix)`), runs each client command of a recording session through `common::run` at `RUST_LOG=trace` — `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint`, `stop`, in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR` — and reads over all the clients' stderr 0 id needles, 0 name needles, 0 sentinel occurrences, 0 of the recording path and 0 `recorded_needles`, green under `-p seven_guis` and `--workspace` (`host_log` 2 → 3; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)\""
    sidecar: "§8 PII Scrubbing — a client's own stderr now read by a standing check: host_log's third test, the start --record clients at trace, 0 id · name · sentinel · recording-path · recorded needles, both builds (2026-10-24-stand-requirement-sweep)"
    rationale: "The report's Counts fact 8 names this line among the four rows stating the 2026-10-14 one-off reading of \"a client's own stderr\", \"now joined by a standing check at `trace` under `--record`\"; the Symbols bullet for `host_log.rs` (40-130) gives the commands and the five zero readings, and the gates `cargo test -p seven_guis … --test host_log` and `cargo test --workspace … --test host_log` are green (3 tests)."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 10
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add after line 10, as its own line in the same group: \" A paged `part` read through the `escher-session` host's own sink stays unmeasured: the host boots only the four lean stand tasks and none pages, so a paged answer's log is read in process alone (`stand_act_spans`' sink capture, above), while the host and its clients are read on `snapshot --part 1` of a screen within the budget; its measurement owned by a `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next), on the operator's «yes — apply.» (inputs#I2 of escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)\""
    sidecar: "§8 PII Scrubbing — stated limit: a paged part read through the escher-session host's sink unmeasured (no lean task pages), owned by a residuals line (target: next) (2026-10-24-stand-requirement-sweep)"
    rationale: "The report's Counts fact 7 states the limit as new text \"placed beside fact 8's sites\", and fact 8 lists this file's line 10 among them; Decisions & corrections and inputs#I2's Beyond-the-plan row 1 name its owner, a new residuals line «a paged `part` read through the escher-session host's sink» (target: next), placed at this wrap."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158
note: D-obs-instrumentation, D-obs-stack and D-obs-pii hold — test files and one CI leg only, no log site, span, field, dependency or env read added (`RUST_LOG` on the client Command is the registered read); the body's §8 broad line (.andromeda/obs-plan.md:250, "0 id, name and typed-text needles … in the `escher-session` host") stays true and could name a client's own stderr beside the host if the orchestrator carries the standing check to the body.
