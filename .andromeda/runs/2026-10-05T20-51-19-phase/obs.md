# obs extract

## Relevance
partial — the chunk builds CI pipeline config only (no instrumentation); obs ties in through §9 CI telemetry-artifact handling, the content of uploaded failure artifacts (§6/§7/§8) and the `tracing`-feature build surface (§2). Obs tier 0 (per obs-plan §1).

## Constraints
- obs-plan §9 CI Integration records the WPT report and `wptscores.json` as CI telemetry artifacts archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on `main` (wpt.yml:71-74, :94-118 at the reading). Classifying `wpt.yml` as fast / slow / excluded on the fork must keep that publish/dispatch path from firing from `build/**` pushes; whether the current trigger/guard already prevents it is research's question (per obs-plan §9).
- obs-plan §9 / §6 record publish builds logging at `CARGO_LOG: info` with `--verbose --trace` (publish-browser.yml:33, :154 at the reading); that verbose build log belongs to the signing-secret workflow this chunk excludes — the exclusion must not relocate that logging into a build-branch leg (per obs-plan §9).
- "Failure artifacts uploaded" is new CI telemetry-artifact handling: obs-plan §9 marks log-file/snapshot artifact upload, CI resource attributes and artifact retention as NOT YET MEASURED, so the chunk sets this surface with no prior plan contract — the retention and contents it picks are a §9 amendment candidate, not a reading of existing intent (per obs-plan §9).
- Diagnostics the failure artifacts would carry are unstructured: obs-plan §6 records test diagnostics as `println!`/`eprintln!` to stdout/stderr and wpt/runner log messages as free-form strings (no log JSON schema — §6 Log format NOT YET MEASURED); obs-plan §7 records the wpt runner panic hook capturing message/file/line/column and a forced backtrace. Artifact capture is therefore stdout/stderr/report files, not a structured-log sink (per obs-plan §6, §7).
- obs-plan §8 records URLs, attribute values, outer HTML and text-node contents logged as-is with no scrubbing anywhere; any log/test-output artifact this chunk uploads publishes that content to the Actions artifact store unscrubbed (per obs-plan §8).
- `tracing` is a default feature of `blitz` and `log-phase-times`/`log-frame-times` forward through the renderer/DOM crates (per obs-plan §2 Feature wiring); any feature-matrix or fast/slow split of compiling jobs must keep both the feature-on and feature-off `#[cfg(feature = "tracing")]` paths compiled somewhere in the pipeline, since each call site has a no-op fallback (per obs-plan §2 Telemetry mechanism). Whether today's feature jobs already cover both is research's question.

## Patterns to follow
- Telemetry stays behind each crate's `tracing` cargo feature with a no-op path when off (per obs-plan §2 Telemetry mechanism) — a reduced-debuginfo `[profile.dev]`/CI change touches build config only, never these gates.
- The only existing CI telemetry artifacts are WPT report/scores routed via `wpt.yml` (per obs-plan §9 Telemetry artifact handling) — failure-artifact upload follows the same "artifact produced by the leg, uploaded by the workflow" shape rather than a new telemetry backend (none exists: per obs-plan §2 Absent, §3 OTel SDK init).
- The wpt runner's printed per-run statistics and panic-hook backtraces (per obs-plan §5, §7) are the existing diagnosis payload for a failing WPT leg — the natural failure-artifact content for that leg.

## Anti-patterns to avoid
- Adding a telemetry backend, exporter, OTel SDK or scrub layer inside a CI chunk — those are the bootstrap items `otel-sdk-install` / `pii-scrubbing-wire` (per obs-plan §3 Bootstrap phases (derive for route / setup-project)), owned by the later "Telemetry bootstrap" chunk.
- Uploading verbose/trace build logs or test output carrying user content as artifacts without a recorded decision, given no scrubbing exists (per obs-plan §8 Scrubbing (absent)).

## Contract bindings
- obs ↔ security: failure-artifact contents (test stdout/stderr, wpt reports) carry unscrubbed URLs/attributes/outer HTML (per obs-plan §8) — binds to security-plan's secrets/logging rules; artifacts must also never include signing material or env dumps (security owns that rule).
- obs ↔ tests: test diagnostics via `println!`/`eprintln!` (per obs-plan §6 tests/blitz-tests) and the `log-phase-times` timing output (per obs-plan §5 `paint_tree_bench`) are what a failing test leg's artifact captures — binds to test-plan's CI legs / local-baseline sections.
- obs ↔ arch: the WPT publish/dispatch path to `DioxusLabs/blitz-wpt-results` (per obs-plan §9) is an upstream-owned destination the fork's trigger classification must not reach.

## Acceptance criteria contributions
- (obs) A deliberately failing leg on the build branch produces an uploaded artifact sufficient to diagnose it without a re-run (test output and, for WPT, the report/panic backtrace), and a passing run uploads none or only by explicit design (per obs-plan §9, §7).
- (obs) A push to `build/escher-0.1.0` does not publish WPT results to Pages or dispatch to `DioxusLabs/blitz-wpt-results` (per obs-plan §9 Telemetry artifact handling).
- (obs) No workflow change adds a telemetry backend/exporter or a new user-content log field, and the `tracing` feature-gated paths still compile in at least one CI leg after the fast/slow split (per obs-plan §2, §3 Bootstrap phases).
