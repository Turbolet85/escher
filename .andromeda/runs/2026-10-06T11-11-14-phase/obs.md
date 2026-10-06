# obs extract

## Relevance
partial. The chunk rewrites only the root `README.md` and adds no instrumentation (scope §Surfaces: "Telemetry: none"). Obs has one stake: any telemetry or harness-log statement the README makes, for example in an Epoch 1 Foundation line or a "try it" section, must match the as-built record and must not claim planned telemetry.

## Constraints
- The obs tier is 0 (per obs-plan §1 Obs Scope Summary). The README may not present escher as having a telemetry product, backend or export.
- escher's own sink must be described as it is recorded. `seven_guis_native` installs `escher_telemetry::init`, which writes stderr only and leaves engine `tracing` features off. The headless stand `seven_guis::stand` installs no subscriber (per obs-plan §3 Observability Harness Contract, "Logging stack"; §2 Telemetry Strategy).
- The sink's line format is one-line text, not JSON. Each line carries `service.name` / `service.version` (per obs-plan §3, "Service identity and line format"; §6 Log Coverage, "Log format (escher's sink)"). A README that names the format must not call it JSON or NDJSON.
- Several things are recorded as absent: OTel SDK init and exporters, spans, and metric emission (per obs-plan §3 "OTel SDK init"; §4 Span / Trace Coverage; §5 Metric Coverage "Absent"). The README must not list them as features. Whether the README mentions telemetry at all is P4's call, and the plan sets no requirement either way.
- The scrub covers escher's sink only. The upstream apps' `fmt::init()` and the WPT runner's `env_logger` stay unscrubbed (per obs-plan §8 PII Scrubbing & Compliance, "Scrubbing", reach paragraph). The README must not claim that escher scrubs logs workspace-wide.
- The agent-run and cold-agent JSON-line logs are harness metadata, not escher's telemetry sink (per obs-plan §3, "Agent-run harness log" / "Cold-agent pipe log"). The README must not present them as escher observability.

## Patterns to follow
- Describe any built capability by what the plan records, not by what is intended. Examples: "a stderr telemetry bootstrap with service identity, an allowlist scrub and a chaining panic hook, installed in the windowed stand binary" (per obs-plan §3 "Logging stack"; §7 Error Capture & Reporting, "Panic hooks").
- If a "try it" section names `bash scripts/agent-run.sh boot` / `run stand`, describe its output in the §6 harness shape: one JSON object per line on stdout, also appended to `target/agent-run/events.jsonl` (per obs-plan §6 Log Coverage, "Log format (the agent-run harness, §3)").
- Name the `RUST_LOG` filter and its default of `warn` only as recorded (per obs-plan §6 Log Coverage, "Log format (escher's sink)"). Do not imply any other env read.

## Anti-patterns to avoid
- Presenting spans, metrics, OTel export or a JSON log sink as escher features (per obs-plan §3 "OTel SDK init"; §4; §5 "Absent"; the §3 NOT YET MEASURED note leaves a JSON schema and a log-file location unrecorded).
- Pasting a sample log line or harness transcript into the README. Bridged `log` records carry a host path in `log.file`, and the std panic hook prints raw messages (per obs-plan §8, "Values logged as-is", last bullet).

## Contract bindings
- obs ↔ tests: any README description of `scripts/agent-run.sh` or `scripts/cold-agent.sh` output binds to the test contract of test-plan §3, as obs-plan §3 "Agent-run harness log" / "Cold-agent pipe log" and §6 record it.
- obs ↔ security: the scrub-reach statement binds to security-plan's logging rules, as obs-plan §8 "Scrubbing" records them. The contact address is a README fact, not a log field. Obs has no stake in it.

## Acceptance criteria contributions
- Wherever the README mentions telemetry, it names no OTel export, spans, metrics or JSON log format as built. Check with a read of the rewritten `README.md` (per obs-plan §3 "OTel SDK init"; §4; §5 "Absent"; §6 "Log format (escher's sink)").
- Wherever the README mentions log scrubbing, it limits the scrub to escher's own sink, in `seven_guis_native` (per obs-plan §8 "Scrubbing").
- The README contains no sample log line, harness event dump or transcript excerpt holding a host path or a content-named field value (per obs-plan §8 "Values logged as-is").
- If the README carries a "try it" step for `scripts/agent-run.sh`, its stated output matches the recorded JSON-line events (per obs-plan §6 "Log format (the agent-run harness, §3)").
