proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 43
    section: "## 6. Log Coverage"
    change: "add a sub-bullet under the escher-driver emitter after its `tool_call` line — `  - the published statement `docs/capabilities.md` restates this emitter's reach: the `command` and `tool_call` spans with closed-word, bool and count fields, stderr only, no driver line at the default `warn`, and the scrub's four limits (the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, the unscrubbed upstream apps and WPT runner); its pin `.github/scripts/test_capability_statement.py` reads the verb, cause, session-error, node-key and exit-status counts and none of these claims, so a change to the span set or the scrub's limits is made in that document by hand (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)`"
    sidecar: "log-coverage: the escher-driver emitter names `docs/capabilities.md` as the published restatement of its spans and the scrub's limits, which no pin checks"
    rationale: "report Outcome (obs) — the statement names the two spans `command` and `tool_call`, stderr only, no driver line at the default `warn`, and the scrub's limits; Changes › Symbols — the pin's four cases read the verb, cause, session-error, node-key and exit-status counts and the README line only"
    basis: "escher-0.2.0/chunks/2026-10-25-capability-statement/report.md:72"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 108
    section: "## 6. Log Coverage"
    change: "add after line 108 — `- observed absent — a log site, span, field, log field or env read in the capability-statement pin `.github/scripts/test_capability_statement.py` (`NUMBER_WORDS`, the helpers `read`, `prose`, `number_word`, `verb_names`, `cause_names` and `table_length`, and `CapabilityStatementTest`'s four tests, reading `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`, `refusal.rs` and `error.rs` by path from the repository root and writing nothing) · searched: the chunk report's reading of the 108-line file, with no silence census fired over it (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)`"
    sidecar: "log-coverage: the capability-statement pin recorded silent — it reads by path, writes nothing and adds no log site, span, field or env read"
    rationale: "report Changes › Symbols / APIs — the pin «reads by path from the repository root, writes nothing, reads no env var»; no log field or span added (`guard: clean`); Coverage of new surfaces — instrumentation n/a"
    basis: "escher-0.2.0/chunks/2026-10-25-capability-statement/report.md:9"
    fact: F1
note: neither proposal fires D-obs-instrumentation, D-obs-stack or D-obs-pii — the chunk adds no runtime operation, telemetry dependency or log site, and no silence census was fired over the new pin, so F1's search clause names the report's reading rather than a probe
