# obs extract

## Relevance
partial — the chunk's `logs` verb and JSON-line log are an obs surface (format, sink, scrub, service identity). The rest is the tests harness contract, and the PREREQ doc fix is outside obs. The plan's JSON log schema and log-file location read NOT YET MEASURED (§3, §6), so the chunk defines them and the wrap records them.

## Constraints
- obs-plan §3 Logging stack records that the headless stand (`seven_guis::stand`) installs no subscriber and has no escher sink. Whether the stand checks gain a subscriber (escher-telemetry or a JSON layer) or the JSON-line log is harness-level only is a P4 fork. Whether the code still installs none at HEAD is research's question (per obs-plan §3 Logging stack).
- obs-plan §3 Service identity and line format requires every line of escher's sink to carry `service.name` / `service.version`, the OTel resource keys, read by `service_identity!()`. A JSON format added inside escher-telemetry must keep these keys. A harness-level JSON log that wants to stay convertible to the sink should reuse the same key names (per obs-plan §3 Service identity and line format).
- obs-plan §6 Log format defines escher's sink as one-line text on stderr with no ANSI, level filtered by `RUST_LOG` through `EnvFilter` (default `warn`). A JSON-line format is a new format beside it, not a description of what exists, and it lands as a §3/§6 amendment at the wrap (per obs-plan §6 Log format; §3 NOT YET MEASURED line).
- obs-plan §8 Scrubbing requires every field on escher's sink to pass the allowlist scrub: engine-prefixed targets print only the safe fields, and `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload` are redacted at any target. Any JSON-line log that carries engine or panic events must keep that scrub and add no new user-content field (per obs-plan §8 Scrubbing).
- obs-plan §8 Values logged as-is records that the chained std panic hook prints the raw panic message to stderr past the scrub. A harness that captures raw test stderr or libtest output into its log carries those unscrubbed values. The log's reach has to be stated, not assumed scrubbed (per obs-plan §8 Values logged as-is).
- obs-plan §3 Bootstrap phases (`otel-sdk-install`) defers opt-in OTel export to "Driver command spans". The chunk adds no exporter, OTLP endpoint or `OTEL_*` env read (per obs-plan §3 Bootstrap phases).

## Patterns to follow
- `ci-leg.sh` writes each leg's merged stdout+stderr to `target/ci-logs/{leg}.log` and truncates it when the leg starts. That build output is unscrubbed and holds no user data. The harness can follow the same pattern: a per-run log file under the gitignored `target/`, reset each run (per obs-plan §9 Telemetry artifact handling).
- `seven_guis_native` installs `escher_telemetry::init(escher_telemetry::service_identity!())`, reports an `Err` with `eprintln!` and continues. If the chunk wires a subscriber into the stand path, this is the install shape (per obs-plan §3 Logging stack).
- The scrub is centralized in the formatter (`packages/escher-telemetry/src/format.rs`) and no engine call site is edited. A JSON formatter added to escher-telemetry should reuse that allowlist instead of re-implementing it (per obs-plan §8 Scrubbing).
- blitz-tests write diagnostics with `println!`/`eprintln!`, which libtest captures. Per-test outcomes are therefore parsed from the test runner's output, not from a telemetry sink (per obs-plan §6 Logged events → tests/blitz-tests).

## Anti-patterns to avoid
- Routing escher-binary telemetry to stdout. escher's sink writes to stderr only, and the stdout JSON lines from `agent-run logs` / `status` are the harness's output channel, kept separate from any binary's log sink (per obs-plan §3 Logging stack).
- Putting engine events into the JSON-line log through an unscrubbed path, such as `tracing_subscriber::fmt::init()` or `env_logger`. These are the upstream sinks that log URLs, attribute values and outer HTML as-is (per obs-plan §8 Values logged as-is; §8 Scrubbing).
- Adding OTel export or a telemetry backend in this chunk (per obs-plan §3 Bootstrap phases).

## Contract bindings
- obs ↔ tests §3. The 5-command `logs` verb uses a JSON-line schema that obs-plan §3/§6 leave NOT YET MEASURED. The schema the chunk defines (fields, `service.*` keys, file location) becomes the obs-plan §3/§6 amendment at the wrap, alongside test-plan §3's amendment.
- obs ↔ security §8. The allowlist scrub (obs-plan §8 Scrubbing) is the redaction mechanism for any engine or panic content in the log. The wrap amends security-plan §Logging if the harness log carries raw test stderr.
- obs ↔ arch §Occupied Resources. A log-file path or a log-related env var (for example the template's `HARNESS_LOGFILE`) has to be registered. obs-plan §3 lists `RUST_LOG` as the telemetry's only env read (§6 Log format).

## Acceptance criteria contributions
- Every line `agent-run logs` prints parses as exactly one JSON object. Lines that come from escher's sink carry `service.name` and `service.version` (per obs-plan §3 Service identity and line format).
- After a stand-check run with `RUST_LOG=trace`, the harness log contains no unredacted value for any field obs-plan §8 redacts at any target (`url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload`) (per obs-plan §8 Scrubbing).
- No escher-telemetry line appears on stdout during `agent-run run`. Telemetry stays on stderr or in the harness log file, and only the harness's own result lines use stdout (per obs-plan §3 Logging stack).
- The harness log file lives under the gitignored `target/` and is reset at each run's start, and `agent-run cleanup` removes it (per obs-plan §9 Telemetry artifact handling).
