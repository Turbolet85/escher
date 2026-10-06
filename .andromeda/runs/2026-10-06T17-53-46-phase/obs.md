# obs extract

## Relevance
partial — the chunk is a measured no-op sync with zero source delta, so it adds no instrumentation; obs only bounds what the gates may emit and what stays untouched.

## Constraints
- No telemetry surface changes: no new `tracing` call site, subscriber, exporter, env read or log field is added; the sink and its env reads stay as obs-plan §3 (Logging stack) and §2 (Telemetry mechanism) record them. Whether the code already matches those records is research's question, not this extract's. (per obs-plan §2 Telemetry Strategy; §3 Logging stack)
- Engine `tracing` call sites stay `#[cfg(feature = "tracing")]` with a no-op fallback; the unchanged tree must keep that gating. (per obs-plan §2 Telemetry mechanism)
- escher's allowlist scrub (the `escher-telemetry` formatter and its content-named redaction set) is untouched by a zero-delta sync; the sync adds no user-content log fields. The upstream apps' `fmt::init()` and the WPT runner's `env_logger` remain unscrubbed, and nothing here changes that. (per obs-plan §8 Scrubbing)
- If `upstream/main` has moved by /implement, any merge is outside this chunk's ruling. A later merge that brings new engine `tracing` sites or logged values reaches the §8 "values logged as-is" surface and belongs to a separate chunk with its own scrub review. (per obs-plan §8 Values logged as-is)
- CI telemetry artifacts keep their recorded handling: the per-leg log is uploaded only on failure from `target/ci-logs/`, and `coverage-report` only on success. The sync adds no artifact, upload or retention change. (per obs-plan §9 Telemetry artifact handling)
- No OTel SDK, exporter or heartbeat is introduced. The plan records no OTel setup, and the §3 key file records that 0.1.0 has no OTel export. (per obs-plan §3 OTel SDK init; contract key `otel-sdk-install` per the contract row's labels)

## Patterns to follow
- Gate evidence is read from the `ci-leg.sh` logs under `target/ci-logs/{leg}.log`, the same artifact path obs-plan §9 records. (per obs-plan §9 Per-leg CI log)
- A harness run for the unchanged tree reads the agent-run JSON-line events (`boot`, `run.start`, `test`, `run.end`) as harness metadata, not as escher telemetry. (per obs-plan §3 Agent-run harness log; §6 Log format (the agent-run harness))

## Anti-patterns to avoid
- Do not add an unconditional `println!` or an unscrubbed new log field while resolving a gate failure. Sync-time fixes must not add engine log output. (per obs-plan §2 Telemetry mechanism; §8 Scrubbing)

## Contract bindings
- obs ↔ tests §3: the agent-run and cold-agent harness logs keep their recorded JSON-line shapes, which tests consume. A zero-delta sync must leave them unchanged. (per obs-plan §3 Agent-run harness log; Cold-agent pipe log)
- obs ↔ security: the scrub allowlist and the "no new listener, port or credential path" surface rule are unchanged. (per obs-plan §8 Scrubbing)

## Acceptance criteria contributions
- (obs) The diff over HEAD is empty for `packages/escher-telemetry/` and for every `tracing` call site, and no new `println!` or log field appears. (per obs-plan §2 Telemetry mechanism; §8 Scrubbing)
- (obs) The `fast` and `doc` gate legs write their logs under `target/ci-logs/` as recorded, and no new CI artifact upload appears in `ci.yml`. (per obs-plan §9 Telemetry artifact handling)
- (obs) `git ls-remote upstream refs/heads/main` still equals `23354585` at /implement. If it differs, the chunk halts for the operator, because new upstream log sites would be unscrubbed. (per obs-plan §8 Values logged as-is)
