# obs extract

## Relevance
partial — the chunk adds no engine instrumentation, but its invocation surface, verdict record, transcript and state area extend the agent-run harness log contract (obs-plan §3 / §6 / §8 / §9), and a stub tool built as an escher binary would fall under the escher sink rules (§3 Logging stack).

## Constraints
- obs-plan §3 (Agent-run harness log) sets out how the harness log works. Events are JSON lines, one object per event, encoded by python3's `json` module and never assembled in bash. They go to stdout and are appended to a state file under `target/agent-run/`. That log is harness metadata, not a telemetry sink, and no `tracing` subscriber is involved. If the pipe joins `scripts/agent-run.sh` as a verb, or sits beside it as its own script, its events and verdict record follow the same encoding and stay outside escher's sink.
- obs-plan §6 (Log format, the agent-run harness) lists the event set (`boot`, `run.start`, `test`, `run.end`, `status`, `cleanup`) and its fields. Any new pipe event (for example a verdict event) adds named, bounded fields alongside them. The raw-output rule is in the same section: a run's raw merged output goes to a state-area log file that is never printed, and nothing after a `failures:` marker is read into an event. That is the precedent for keeping raw transcript or model text out of the stdout event stream.
- obs-plan §8 (Scrubbing) requires the harness log to carry no user content. Its events may hold no key from the content-named scrub set (`url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload`) and no captured test output. A verdict carries "the transcript's path", which would add a `path`-named key, so the field name and placement need a decision (for example, keep it in the verdict file only, under a non-scrub-set name). Model output and tool I/O belong in the transcript artifact, never in an event field.
- obs-plan §3 (Logging stack) requires two things. An escher binary installs `escher_telemetry::init(escher_telemetry::service_identity!())`, which writes to stderr only. The headless stand `seven_guis::stand` installs no subscriber. If the stub tool is a Rust binary, and especially an MCP server over stdio where stdout is the protocol channel, its telemetry must stay on stderr. If the stub boots the stand, it does not gain an escher sink on that account. Whether the stub is a Rust binary at all is P4's fork.
- obs-plan §9 (Telemetry artifact handling) fixes the agent-run state as local only, gitignored under `target/`, uploaded by no CI leg, and removed by `agent-run.sh cleanup`, which touches nothing else under `target/`. Pipe state (transcripts, stub call log, verdicts) under `target/` takes the same handling, and a cleanup verb must not reach past the pipe's own area. The committed evidence copy of the green run is a new artifact row there, registered at the wrap.
- obs-plan §3 (Service identity and line format) uses the OTel resource keys `service.name` / `service.version`. Bootstrap phases `otel-sdk-install` (contract key) says OTel export is deferred and carried to "Driver command spans". The verdict's "client and model identity" is a harness field, not a reason to add OTel export or a new egress here.

## Patterns to follow
- The agent-run.sh event grammar (obs-plan §3 / §6). It has one JSON object per line on stdout, a mirrored `events.jsonl`, a `status.json`, and a raw log that is never printed. The pipe's events, its verdict record and its `logs` / `status` reads should take the same shape.
- Raw output kept in a state file and summarised into bounded events (obs-plan §6, the `run.log` vs `test` events split). The transcript plays the `run.log` role: written in full to a file and referenced, never streamed into events. The wrong-call count and pass/fail are derived counts in the verdict.
- The census evidence method (obs-plan §8, "a live `logs` read held 0 scrub-set keys"). Prove the pipe's events and verdict with the same kind of measured read.
- escher-telemetry `init` + `service_identity!()` for any new escher Rust binary, stderr only (obs-plan §3 Logging stack). This applies only if the stub is built as one.

## Anti-patterns to avoid
- Putting model output, tool arguments/results or the task prompt into a stdout event or an escher `tracing` field. The transcript is an artifact, and §8 Scrubbing keeps the harness log free of user content.
- Writing telemetry or diagnostics to stdout from a stub tool whose stdout is a protocol or event channel. §3 Logging stack keeps escher's sink stderr only, and the upstream `fmt::init()` stdout subscribers are the unscrubbed counter-example (§8).
- Uploading pipe state from any CI leg, or widening a CI artifact path to include it, without a §9 amendment. Today only `target/ci-logs/` and `target/coverage/` are uploaded.

## Contract bindings
- obs ↔ tests: the pipe's invocation surface and event grammar extend the test-plan §3 agent-run contract. The obs side owns the event encoding, field set and state-area handling (obs-plan §3 / §6 / §9), and its contract tests assert them.
- obs ↔ security: no scrub-set keys or user content in events, and no credential in transcripts, logs or verdicts (obs-plan §8 Scrubbing). The provider credential and egress decision is security's, and the obs deferral of OTel export stays carried (Bootstrap phases `otel-sdk-install`).
- obs ↔ arch: transcript, verdict and state paths under `target/`, plus the committed evidence copy, become new §9 artifact rows. They pair with the arch §Occupied Resources → Filesystem registration at the wrap.

## Acceptance criteria contributions
- A live run of the pipe's event stream and of its `logs` read yields valid JSON lines only, each encoded by a JSON library rather than assembled in bash. A census of every event and the verdict record finds 0 keys from the content-named scrub set and no model or tool text (per obs-plan §3 Agent-run harness log; §8 Scrubbing).
- The transcript, stub call log and verdict live in the pipe's own area under `target/` (gitignored), and no CI leg uploads them. The pipe's cleanup removes only that area and leaves `target/agent-run/` and `target/ci-logs/` intact (per obs-plan §9 Telemetry artifact handling).
- If the stub tool is a Rust binary, a stdio run shows its telemetry on stderr only. If it is an MCP stdio server, stdout carries only protocol frames (per obs-plan §3 Logging stack).
