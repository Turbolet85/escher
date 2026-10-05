# obs extract

## Relevance
partial — the chunk adds CI legs and CI artifacts, which fall under obs-plan §9 CI Integration (per-leg logs, artifact handling). It adds no runtime instrumentation, spans, metrics or log events. The keyed contract "Bootstrap phases" (otel-sdk-install, pii-scrubbing-wire) belongs to the later "Telemetry bootstrap" chunk and does not bind here.

## Constraints
- Every new leg (dependency audit, coverage, named a11y, rustdoc gate) should produce the same per-leg log as the existing legs. obs-plan §9 (Telemetry artifact handling, "Per-leg CI log" row) requires that `ci-leg.sh` write each leg's merged stdout+stderr to `target/ci-logs/{leg}.log`, truncated at the leg's start. Whether `ci-leg.sh` already routes arbitrary new leg names this way is research's question.
- A failed new job should keep the failure-log contract in obs-plan §9 ("Per-leg CI log" row): upload only on failure, as artifact `ci-log-{job id}`, with `if-no-files-found: ignore`, a 7-day retention, and files from `target/ci-logs/` alone.
- The coverage report is a new CI telemetry artifact. obs-plan §9 tabulates every CI telemetry artifact by storage and source, so the coverage artifact's storage, retention and upload condition need a row there. That row is an obs-plan amendment, recorded at wrap.
- Per obs-plan §8 (Values logged as-is) and §9 ("Per-leg CI log" row: "unscrubbed build output (no user data — §8)"), CI leg logs and artifacts are unscrubbed. New leg output (audit advisories, coverage summaries, rustdoc diagnostics) must stay build or tool output and carry no user content.
- Rustdoc-gate fixes in engine crates (blitz-dom, blitz-vibey-script) may touch files that hold `tracing` call sites. obs-plan §2 (Telemetry mechanism) requires each such call to stay compiled only under `#[cfg(feature = "tracing")]`, with its no-op path when the feature is off. A doc-only fix must leave that gating as it is.
- obs-plan §2 (Feature wiring) records `tracing` and `log-phase-times` as per-crate optional features, with `tracing` a default feature of `blitz`. Which feature set `cargo doc --workspace --no-deps` (and the coverage build) compiles determines which feature-gated items rustdoc sees. P3 should check whether the rustdoc errors depend on features.

## Patterns to follow
- One leg script for CI and host: each new leg is a `ci-leg.sh {leg}` entry that writes to `target/ci-logs/{leg}.log`, so a red CI leg can be reproduced locally with the same log (per obs-plan §9 "Per-leg CI log" row).
- Failure-only log upload as `ci-log-{job id}`, kept 7 days and pulled from `target/ci-logs/` alone. New jobs reuse the existing upload step shape rather than adding a new one (per obs-plan §9).
- CI telemetry artifacts are recorded in the §9 artifact table with storage and source. The coverage report follows that convention (per obs-plan §9 Telemetry artifact handling).

## Anti-patterns to avoid
- Do not widen the failure-log upload beyond `target/ci-logs/`, for example by sweeping coverage profiles or `target/doc` into `ci-log-*`. The coverage report goes in its own, separately named artifact (per obs-plan §9 "Per-leg CI log" row: "from `target/ci-logs/` alone").
- Do not add `println!`, or remove `cfg` gates, on `tracing` call sites while fixing doc comments or intra-doc links (per obs-plan §2 Telemetry mechanism).

## Contract bindings
- obs ↔ tests: the coverage report measures the workspace tests (test-plan §9 records coverage tooling as absent). Obs owns how the report is stored and kept as a CI artifact (§9 artifact table). Tests own what is measured and any threshold, and this chunk sets none.
- obs ↔ a11y: the named a11y leg's log follows the per-leg log contract (§9). No a11y violation-JSON schema binds here, because obs-plan §3 and §6 record the log format JSON schema as NOT YET MEASURED.
- obs ↔ security: the audit leg's advisory output is unscrubbed tool output in the per-leg log (§9). It carries no secrets or user data (§8).

## Acceptance criteria contributions
- Each new leg (audit, coverage, a11y, doc) run through `bash .github/scripts/ci-leg.sh {leg}` writes its merged output to `target/ci-logs/{leg}.log` (per obs-plan §9 Telemetry artifact handling).
- Each new ci.yml job uploads `target/ci-logs/` as `ci-log-{job id}` only on failure, with 7-day retention and `if-no-files-found: ignore`, and the upload path stays limited to `target/ci-logs/` (per obs-plan §9 Telemetry artifact handling).
- The coverage report is uploaded as its own named artifact (not inside `ci-log-*`), with explicit retention, and is recorded as a row in obs-plan §9's artifact table at wrap (per obs-plan §9 Telemetry artifact handling).
- The rustdoc-gate source diff changes no `#[cfg(feature = "tracing")]` gate or `tracing` call site and adds no `println!` (per obs-plan §2 Telemetry mechanism).
