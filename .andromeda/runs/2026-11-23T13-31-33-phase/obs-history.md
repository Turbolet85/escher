@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — per-leg CI failure logs: §9 gained the row that each ci.yml leg's merged stdout+stderr is written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix `matrix-{platform}.log`), uploaded only on failure as `ci-log-{job id}`, 7 days, unscrubbed build output with no user data; the upload path confined to `target/ci-logs/`. The origin of `decision/ci-log-artifact-on-failure` and half of `decision/fork-ci-two-artifacts`: a cross-host canvas measurement read through the macOS/Windows matrix legs must not put a shot's bytes or a temp/home path into that log, and gets diagnosability on failure without a re-run.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `coverage-report` artifact (lcov, on success, 7 days), recorded as a boundary widening because it was a new upload out of fork CI. Origin of `decision/coverage-artifact-on-success` and the "exactly two artifacts" count in `decision/fork-ci-two-artifacts`: uploading per-host canvas PNGs or rasters from CI for the measurement would be a third artifact, a widening that waits for the founder.

@history_item n=3 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — ratified the coverage-report upload (no longer PROVISIONAL) and ruled no OTel export in 0.1.0. Basis of `decision/coverage-artifact-on-success` standing ratified and of the telemetry-backend absence; the chunk adds no export or backend.

@history_item n=4 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the coverage row's lcov excludes `keyboard_compat.rs`, bound to test-plan's coverage leg. Touches `decision/coverage-artifact-on-success`; a new canvas check changes coverage counts only, no artifact.

@history_item n=5 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — package-install output stays in the step log; "Kept: no upload step was added — a new artifact out of fork CI is a boundary widening that waits for the founder." A direct precedent: a probe pushed to `build/escher-0.4.0` that wants per-host readings must read them from the step log or the failure log, not a new artifact, unless the founder rules.

@history_item n=6 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand and its in-process checks install no subscriber, no `println!`, no env read; only the windowed binary installs telemetry. Origin of `decision/headless-stand-no-sink`: the canvas cross-host check boots the stand fixture in process with no sink.

@history_item n=7 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — first exception to the stand checks' no-print: `stand_id_persistence`'s re-executed child prints to stdout that the parent captures into assertions only, never a log. Precedent behind the `decision/checks-silent` "re-run child prints" exception, should the measurement use a re-run child to print a shot digest.

@history_item n=8 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the no-subscriber census was widened to cover the checks' shared module `tests/blitz-tests/tests/common/mod.rs`. Why `decision/checks-silent` names shared modules: a canvas fixture or digest helper put in a shared module (`common`, `fixtures`) falls under the same silence census.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — a second sink installer (`escher-session`); "a headless boot has no escher sink" scoped to an in-process boot; the scrub's reach scoped. Trap it records: "a zero read from a host that installs no sink says nothing about one that does". Bears on `decision/scrub-sink-reach` and `decision/headless-stand-no-sink`: an in-process canvas shot's silence does not speak for a hosted one.

@history_item n=10 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole at every level; `ESCHER_TARGET_PREFIXES` printed with content-named fields redacted. Rule for later chunks: "a target is admitted only together with a scrub rule for it". Underlies `decision/closed-span-lines-judged-alike` and `decision/scrub-sink-reach`; no new target may be added for the measurement.

@history_item n=11 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — `Command`, `Call`, `ArgValue` derive `Debug` and print ids/text; rule: the span's cause field takes a `Cause` name, never field a schema type with `Debug` nor an argument by its schema name. Origin of `decision/debug-types-never-logged` and `decision/driver-spans-carry-no-content`.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built) — INFO, target `escher_driver`, entered before `validate`, eight fields, a never-fielded list; `tracing` ungated in escher-driver. Origin of `contract/command-span/span`, `contract/escher-driver-command-span/span`, `decision/one-span-per-command`, `contract/command-span/field-count`; the chunk must keep the eight fields (Boundaries: no new span or field).

@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line, PROVISIONAL) — the closed span's line format and a span's fields judged pair by pair by the scrub; a new record class in the sink is a boundary widening. Origin of `contract/command-span/line` and `decision/closed-span-lines-judged-alike`.

@history_item n=14 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified by the founder; diff sizes may reach a log, content never. Ratification basis of `contract/command-span/line`, `contract/escher-driver-span-lines/default-level` and `decision/closed-span-lines-judged-alike`.

@history_item n=15 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the command line writes each command's answer as JSON on stdout, an answer never a log line; standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. Also measured the host by level under both builds (package and workspace). Basis of `decision/debug-types-never-logged`'s "outcome written in one place" and of `contract/escher-driver-span-lines/default-level`.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span ratified (fields `tool`, `ending`, closed words); `command` its child; `stand_mcp_tools`' recorder an exception to "installs no subscriber"; `mcp_log` reads the role at `trace`. Origin of `contract/tool-call-span/line`, `decision/driver-span-fields-print` and the MCP exception in `decision/checks-silent` / `decision/headless-stand-no-sink`.

@history_item n=17 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the shot verb: `fonts-unavailable` cause before anything is painted; an accepted shot records `verb` alone, a refused one `verb` and `cause`; never fielded the image, its byte size or width/height; `Outcome::Shot` written by `to_json` and the MCP image block only; 0 of a shot's first 64 base64 chars on the host's and the `mcp` role's stderr at `trace`, both builds; `stand_screenshot_repeat` a re-executed child. Origin of `contract/command-span/screenshot`, `contract/command-answer/shot`, `contract/refusal-cause/fonts-unavailable` and `decision/shot-image-leaves-only-in-answer` (guards `host_log`, `mcp_log`) — the rows the canvas shot measurement rests on, and the precedent for a repeat-shot byte comparison done silently.

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — cause names nine → eight (`time-unavailable` retired), `fonts-unavailable` kept, the span keeping its eight fields. Explains the closed cause set `contract/refusal-cause/fonts-unavailable` belongs to; a per-host font difference must surface only through that existing cause.

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — ratified by the founder the eight cause names, `fonts-unavailable` among them. Ratification basis of `contract/refusal-cause/fonts-unavailable`.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing; the Recording directory row (0700, one 0600 check per session, call content at rest). Background of `contract/kept-check/answer-digest` and the kept-check rows: the kept check is a file, never a log.

@history_item n=21 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder writes the answer itself as `answer`, a shot's without `png`; the committed checks `tests/blitz-tests/checks/counter-{cli,mcp}.json` in git (public), written by `record_checks.sh`, read by `stand_replay` and the freshness check, uploaded by no CI leg. Origin of `contract/kept-check/answer`, `contract/recording-file/answer`, `decision/committed-kept-checks-clean` and `decision/committed-checks-hold-no-private-data` — why the chunk's kept check keeps no pixels and the Counter checks stay free of a `screenshot` step unless the founder amends it.

@history_item n=22 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — the seven_guis log checks' shared module states once `Needles`, `found`, `occurrences` etc., its assertion messages a kind and a count, never a needle or a captured line. Origin of `decision/log-check-failures-name-kind-count`: a canvas check's failure must name a kind and count, not the differing bytes or a host path.

@history_item n=23 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the third shared module `tests/blitz-tests/tests/fixtures/mod.rs` recorded silent (no print, tracing, subscriber, env read, file, socket). Where a canvas stand fixture booted alone would sit; `decision/checks-silent` covers it.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe's verdict fields, its live child's recording) — the live run's transcript carries a shot; the CI row records "no kept check of the live run committed (the two Counter checks the only committed ones)". Supports `decision/committed-checks-hold-no-private-data`: committed checks stay the two Counter sessions.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list read through the host, the role and a hosted shot) — `cli_commands` read a hosted shot of an open list equal to the in-process one, an answer and no log line. Precedent for a shot byte-equality comparison across carriers on one host — this chunk extends equality across OS hosts.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels, the widening ratified) — a shot carrying page text as pixels recorded as a ratified boundary widening, the shot's own fields unmoved. Bears on `decision/shot-image-leaves-only-in-answer`: canvas text painted into a shot leaves only as pixels in the answer.

@history_item n=27 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (decision/contract/absence), each verified against the code. Why the picked rows bind by handle; an amendment from this chunk names them by `~`/`+`/`−` handle.

@history_item n=28 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `decision/checks-silent` except widened: a re-run child prints ids or digests its parent reads. Precedent if the cross-host check prints a shot or raster digest from a re-run child; any other print needs a further except.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (3. Observability Harness Contract) — `decision/checks-silent` and `decision/headless-stand-no-sink` excepts widened for `stand_layout`'s TRACE recorder. Shows the pattern: any check that installs a subscriber must be named in both rows' excepts.

@history_item n=30 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (3. Observability Harness Contract) — `decision/headless-stand-no-sink` and `decision/driver-library-installs-no-subscriber` reworded: acting steps run under escher-telemetry's scoped capture, which prints nothing. Current wording of two picked rows; a shot is not an acting step, so the canvas shot runs outside the capture.

@history_item n=31 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (4. Span / Trace Coverage) — `decision/driver-emits-no-event` reworded for the scoped capture around an acting step. Current wording of a picked §4 row.

@history_item n=32 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Observability Harness Contract) — `decision/headless-stand-no-sink` except gains the error-text founder check (sink over an in-memory writer); `decision/checks-silent` except reworded; `decision/driver-library-installs-no-subscriber` gains the chaining panic hook. The rows' current text, as picked.

@history_item n=33 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Span / Trace Coverage) — `decision/driver-emits-no-event`: the capture keeps the step's records and panics in process and prints nothing. The picked row's current text.

@history_item n=34 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage) — `contract/command-span/settled` gained `layout` among the verb-alone reads (screenshot already in it); verb table fourteen → fifteen. The picked row's current form; the chunk adds no verb, so the counts stay.

@history_item n=35 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (6. Log Coverage) — `contract/command-span/verb` fifteen words and the per-verb line contract `contract/command-span/layout`, beside the screenshot line contract `contract/command-span/screenshot`. Precedent for the per-verb line form the shot row follows.

@history_item n=36 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Observability Harness Contract) — `decision/sink-host-read-at-trace` needle set widened (host and `mcp` role read at trace under both builds). The guard pattern behind `decision/shot-image-leaves-only-in-answer`'s `host_log` / `mcp_log`: a new canvas fixture reachable through the host would be read there.

@history_item n=37 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract) — `decision/checks-silent` except gains "the order page logs" (a fieldless info site in pinned code). The picked row's current except; a canvas fixture adding any log site would need the same named exception.
