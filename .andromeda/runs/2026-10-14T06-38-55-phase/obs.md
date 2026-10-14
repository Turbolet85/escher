# obs extract

## Relevance
partial — the chunk adds no span, metric or log field by its scope's own boundary; obs reaches it through the silence of the stand checks it turns into assertions, the a11y CI leg's log artifact, the scrub that would judge any engine or adapter record the forks open (CARRY 2's platform adapter, CARRY 11's `aria-activedescendant` read), and a focus change's engine INFO line on the measured host flows.

## Constraints
- The headless stand and its checks install no subscriber, read no env var and print nothing, their readings going into assertions only. `stand_keyboard` and `stand_contrast` are named in that census, and so is the shared module `tests/blitz-tests/tests/common/mod.rs` they read. A check this chunk extends or adds (keyboard reach, focus order, contrast assertions, `stand_accessibility_ids`'s restated table) keeps that shape (per obs-plan §3 Logging stack).
- `Harness::text_contrast` and the settle loop open no span, emit no event, read no env var and print, panic and log nothing, and blitz-test-harness takes no `tracing`. Asserting through them, or widening them to read hover, active or non-boot states (CARRY 8), keeps that true (per obs-plan §4 Span / Trace Coverage).
- Engine telemetry stays behind the `tracing` feature: every blitz-dom log site is `#[cfg(feature = "tracing")]` with a no-op path. An engine change a founder's fork could open here, the builder reading `aria-activedescendant` (CARRY 11) or honouring `inert` (CARRY 12), adds no unconditional print. Whether either path carries a log site today is research's question (per obs-plan §2 Telemetry Strategy).
- escher's sink drops every target outside its two prefix sets at every level. Under an engine target (`accesskit_xplat` among them), it prints only the safe fields. Enabling the platform adapter on the windowed stand (CARRY 2/4) brings AccessKit crates into `seven_guis_native`'s graph. The sink judges their records by that allowlist, and which of their targets fall outside it is research's question (per obs-plan §8 Scrubbing).
- The windowed `seven_guis_native` has a measured by-level reading: one line at `info` through `trace`, with 0 id needles and 0 name needles. Enabling `accessibility` on it changes the binary that reading was taken on. Whether the reading still holds is research's question, and it gets re-read if the adapter is enabled (per obs-plan §8 Values logged as-is).
- A workspace build turns `blitz-dom/tracing` on. A focus change then emits one `INFO blitz_dom::document … message=[redacted]` ("focussed node"), and that line is counted in the measured host and `mcp` role CRUD flow readings. A stand change that moves real focus (CARRY 11's roving `tabindex` on the CRUD options, or a row click that focuses) can move those counts. Whether it does is research's question (per obs-plan §2 Feature wiring; per obs-plan §6 Logged events).
- Each `ci-leg.sh` leg writes its merged output to `target/ci-logs/{leg}.log`, which is truncated at the leg's start and uploaded only on failure as `ci-log-{job id}`, kept 7 days. The a11y leg's new test list stays inside that handling. Making it gate merges adds no new artifact outside `target/ci-logs/` (per obs-plan §9 CI Integration).

## Patterns to follow
- The silent-census shape of `stand_keyboard` / `stand_contrast`: drive the four lean tasks in process in both layout modes and read results into assertions only. Add no subscriber, env read, print or listener, so the same silence probe reads 0 over the check and the shared module (per obs-plan §3 Logging stack).
- Feature-gated engine logging: `#[cfg(feature = "tracing")]` call sites with a no-op fallback, and only safe fields (`node_id`, `status`, `waiting_nodes`, `property`) printing under engine targets (per obs-plan §2 Telemetry Strategy; per obs-plan §8 Scrubbing).
- CI legs run through `.github/scripts/ci-leg.sh`, so the log lands at `target/ci-logs/{leg}.log` and the failure-only upload picks it up (per obs-plan §9 CI Integration).

## Anti-patterns to avoid
- Do not log or field what a check or the stand reads: an element id, an accessible name, a contrast pair's colours or a control's value. A failing assertion message names a kind and a count, never screen content (per obs-plan §3 Logging stack; per obs-plan §8 Values logged as-is).
- Do not add a new log field, span field or span. Any engine attribute read added by a fork (`aria-activedescendant`, `inert`) gets no field holding the attribute's text (per obs-plan §4 Span / Trace Coverage; per obs-plan §8 Scrubbing).

## Contract bindings
- obs ↔ tests: the stand checks' silent census (obs-plan §3 Logging stack) is what test-plan's stand targets and `scripts/agent-run.sh` `run stand` events rely on. Test output never reaches a harness event, because nothing between `failures:` and the next `test result:` is read (obs-plan §6 Log format, the agent-run harness).
- obs ↔ a11y: the a11y CI leg's log and failure artifact (obs-plan §9 CI Integration) is the record of a red a11y gate. The leg's test list and its gating are a11y-plan §9's.
- obs ↔ security: enabling the platform adapter (CARRY 2) and an engine attribute read (CARRY 11) are judged by the sink's allowlist and scrub sets (obs-plan §8 Scrubbing), beside security-plan §Input Validation's closed-set rule.
- obs ↔ tests (log counts): `host_log` and `mcp_log` count the lines of a CRUD flow, including the workspace build's `blitz_dom::document` INFO focus line (obs-plan §6 Logged events). A CRUD focus change binds to those checks' readings.

## Acceptance criteria contributions
- Every stand check this chunk adds or extends, and the shared `common` module, reads 0 on the silence probe: no subscriber installed, no env var read, no print, no listener bound (per obs-plan §3 Logging stack).
- `host_log` and `mcp_log` stay green under both builds after any CRUD markup or focus change, with 0 id needles, 0 name needles and 0 typed-sentinel occurrences at `trace` (per obs-plan §8 Values logged as-is).
- If the a11y leg's test list changes, `bash .github/scripts/ci-leg.sh a11y` still writes `target/ci-logs/a11y.log` and the job uploads it on failure only (per obs-plan §9 CI Integration).
- If `accessibility` is enabled on `seven_guis_native`, its stderr by level is re-read and holds 0 id needles and 0 name needles at `trace`. Any record from an AccessKit target outside the allowlist writes no byte (per obs-plan §8 Scrubbing).
