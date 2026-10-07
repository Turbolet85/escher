# obs extract

## Relevance
partial — a measured no-op sync adds no instrumented entity, log call site, metric or sink; obs touches it only through the gate runs' log artifacts (obs-plan §9) and through the telemetry surface the sync must leave as it stands (obs-plan §2, §3).

## Constraints
- Obs tier is 0 (per obs-plan §1 Obs Scope Summary): no instrumentation depth is owed, and a chunk with no source edit owes no span, metric or log of its own.
- obs-plan §2 Telemetry Strategy requires engine telemetry to stay `tracing` events compiled in only under each crate's `tracing` feature, each with a no-op path when the feature is off. With 0 upstream commits to merge the chunk brings no call site under that rule; if the re-read at /implement finds upstream moved, the inherited call sites of a non-empty merge fall under it — that is the halt's question, not this chunk's.
- obs-plan §3 Observability Harness Contract (Logging stack) requires the headless stand and its `stand_*` checks to install no subscriber and read no env var, so the workspace-test leg has no escher sink: the proof that our logic survived is read from test and CI verdicts, never from telemetry lines.
- obs-plan §3 Bootstrap phases (`otel-sdk-install`) requires escher 0.1.0 to carry no OTel crate, no egress and no exporter credential path; the scope's "no dependency or lock change" keeps it, and a later non-empty merge would have to be read against it.
- obs-plan §9 CI Integration (Per-leg CI log) requires each leg run through `ci-leg.sh` to write its merged stdout+stderr to `target/ci-logs/{leg}.log`, truncated at the leg's start — the artifact both local legs (`fast`, `doc`) leave. Whether `fast` leaves one log or one per sub-leg is research's question.
- obs-plan §8 PII Scrubbing & Compliance and §9 class that per-leg log as unscrubbed build output holding no user data, sitting under the gitignored `target/`; it stays local.

## Patterns to follow
- Read each local gate's verdict from the leg's exit status with its `target/ci-logs/{leg}.log` as the record (per obs-plan §9 Per-leg CI log).
- Read the fork-CI verdict on HEAD from the run's own check results: the `ci-log-{job id}` artifact is uploaded only on failure and kept 7 days, so a green run carries no log artifact (per obs-plan §9 Per-leg CI log).
- If any stand check is run through the agent-run contract rather than a leg, its JSON-line events and `target/agent-run/` state are harness metadata, not a telemetry sink (per obs-plan §3 Agent-run harness log · §6 Log format, the agent-run harness).

## Anti-patterns to avoid
- obs-plan §11 Obs Anti-Patterns records no intent, so no ban is cited from it; the two below follow from the measured sections.
- Adding a subscriber, an `escher_telemetry::init` call, an env read or a print to a check or to the stand in order to witness the sync (per obs-plan §3 Logging stack — the headless boot has no escher sink).
- Citing a `target/ci-logs/{leg}.log` that this chunk's own run did not write: the file is truncated at each leg's start, so one found on disk may belong to an earlier HEAD (per obs-plan §9 Per-leg CI log).

## Contract bindings
- obs §9 Per-leg CI log ↔ tests: the tests domain owns the gate commands (`ci-leg.sh fast`, `ci-leg.sh doc`); obs owns the log artifact they leave and its failure-only upload.
- obs §3 Logging stack ↔ tests §3: the stand checks the workspace-test leg runs install no sink — the binding holds only while the chunk edits no check.
- obs §8 ↔ security (logging and scrub): not exercised — the chunk adds no log field and no user-content path.

## Acceptance criteria contributions
- (obs) The chunk's diff adds no telemetry dependency, subscriber, log call site or env read — zero source and lock delta (per obs-plan §2 Telemetry Strategy · §3 Bootstrap phases `otel-sdk-install`).
- (obs) Each local leg's verdict is taken from a run made on the chunk's HEAD, with that run's `target/ci-logs/{leg}.log` as its record, not from a log left by an earlier run (per obs-plan §9 Per-leg CI log).
- (obs) Nothing under `target/ci-logs/` is staged or committed as evidence; committed evidence states verdicts and counts only (per obs-plan §8 PII Scrubbing & Compliance · §9 Per-leg CI log).
- (obs) The fork-CI verdict on HEAD is recorded from the run's checks; the absence of a `ci-log-*` artifact on a green run is not read as a gap (per obs-plan §9 Per-leg CI log).
