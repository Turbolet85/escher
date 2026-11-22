@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (per-leg CI failure logs) §9 gained the row behind `decision/ci-log-artifact-on-failure`: each `ci.yml` leg's merged output written by `.github/scripts/ci-leg.sh` to `target/ci-logs/{leg}.log`, truncated at the leg's start, uploaded only on failure as `ci-log-{job id}`, kept 7 days, upload confined to `target/ci-logs/` so no artifact reaches the keystore or `Dioxus.toml`. This chunk edits `ci.yml` and `ci-leg.sh` (skia fetch step, cancelled-runner action, `CI=true` cause): any new step or leg must keep that confinement and the two-artifact set (`decision/fork-ci-two-artifacts`).

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (citations re-pointed) 8 citations into `Cargo.toml`, `wpt.yml` and `publish-browser.yml` re-pointed after the CI files moved. Precedent: a manifest or workflow edit in this chunk moves cited lines and owes a re-point.

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `coverage-report` artifact (`target/coverage/lcov.info`, success only, 7 days), the one fork-CI artifact outside `target/ci-logs/`, recorded as a boundary widening (a new upload out of fork CI), PROVISIONAL at the time. Basis for `decision/coverage-artifact-on-success`; the trap for this chunk: a new CI upload (e.g. a skia archive or checksum artifact) would be a boundary widening for the founder.

@history_item n=4 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — ratified the `coverage-report` upload and ruled no OTel export (no OTel crate, no egress, no credential path), held as residual `res-fzitzzw2`. Bears on `absence/telemetry-backend`: a dependency move in this chunk must pull in no OTel/metrics crate.

@history_item n=5 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set escher-telemetry: stderr-only non-ANSI one-line sink, `service.name`/`service.version` from `CARGO_PKG_*`, `RUST_LOG` via `EnvFilter` default `warn`, `LogTracer` bridge, and the rule that tracing is compiled only under the `tracing` feature for engine/upstream crates while escher-telemetry is ungated. Origin of `decision/telemetry-via-tracing`, `decision/engine-tracing-feature-gated` and the `contract/escher-log-line/*` and `contract/telemetry-sink/stream` rows; a `tracing`/`tracing-subscriber`/`tracing-log` version move in `Cargo.lock` must keep these.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the two sink installers (`seven_guis_native`, `escher-session`) under one `service.name=seven_guis`, the basis of `decision/scrub-sink-reach` and the `service-name` row's caveat. Trap recorded: a zero read from a host that installs no sink says nothing about one that does.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink drop every target outside `ENGINE_TARGET_PREFIXES`/`ESCHER_TARGET_PREFIXES` whole at every level (`decision/sink-target-allowlist`), the accepted cost that a third-party WARN/ERROR never prints. Rule for later chunks: a target is admitted only together with a scrub rule for it — dependency moves (zbus, accesskit, image/rav1e, zip, rusqlite) that add log targets stay dropped and must not be admitted to the allowlist without a scrub rule.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (`tracing` ungated in escher-driver) recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events — the origin of `decision/engine-tracing-by-invocation`. Directly relevant to the `CI=true` recompile carry (`cry-sqod24yl`): feature unification differs between workspace and package-alone builds, so unit hashes differ by invocation, not only by env.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (closed span's line, PROVISIONAL) defined the closed-span line shape (`span=`, own fields, `message`, `time.busy`, `time.idle` from tracing-subscriber, Debug-quoted strings), judged pair by pair by the scrub. Basis of `decision/closed-span-lines` and every `contract/span-line/*` row; a `tracing-subscriber` bump in the lock must leave the line byte-identical.

@history_item n=10 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the closed span's line as a second record class (founder, 2026-10-09). The `contract/span-line/*` and `decision/closed-span-lines(-judged-alike)` rows rest on it; a dependency move changing its format would reopen a ratified boundary.

@history_item n=11 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — recorded that CI install steps call `.github/scripts/apt-install.sh`, whose output stays in the job step log (no `ci-leg.sh` leg runs it, no artifact uploads it), its diagnostic one stderr line per failed attempt. Kept: a new artifact out of fork CI is a boundary widening that waits for the founder. Direct precedent for the skia per-platform fetch script and its CI step: keep its output in the step log, upload nothing new.

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — §9's install diagnostic phase set became `update`, `install` or `repair` (bounded `dpkg --configure -a`), line shape unchanged. Precedent for a CI script's closed diagnostic line, as a skia fetch/verify script would need.

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the `coverage-report` lcov excludes `keyboard_compat.rs` by filename, bound to test-plan's coverage leg. Part of `decision/coverage-artifact-on-success`; a coverage-leg edit in `ci-leg.sh`/`ci.yml` must keep it.

@history_item n=14 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added §9's "Agent-run harness state" row: `target/agent-run/` local only, uploaded by no CI leg. One of the facts behind `decision/local-state-never-uploaded`.

@history_item n=15 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added §9's cold-agent pipe state row: local only, gitignored, uploaded by no CI leg. Part of `decision/local-state-never-uploaded`, which any CI upload change in this chunk must keep.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added §9's "Recording directory" row (owner-only, local only, call content at rest, uploaded by no CI leg). Part of `decision/local-state-never-uploaded`.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (cold-agent verdict fields and live recording) added §9's Cold-agent recording row: local only, uploaded by no leg. Part of `decision/local-state-never-uploaded`.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (citations re-pointed) re-pointed §9's `.github/scripts/ci-leg.sh` and `.github/workflows/ci.yml` citations after `ci.yml`'s free-disk step moved. Precedent: this chunk's `ci.yml`/`ci-leg.sh` edits owe the same re-point.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (line citations moved) re-pointed citations into `ci-leg.sh` among others after added lines. Same re-point duty for this chunk's CI script edits.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed) re-pointed the `ci-leg.sh`/`ci.yml` citations of §9's Per-leg CI log and Coverage report rows. Kept: telemetry-strategy's By invocation line still names `ci-leg.sh fast` among workspace builds though `fast` ends on a `cargo test -p blitz-tests` step — bears on the `CI=true` recompile carry and `decision/engine-tracing-by-invocation`.

@history_item n=21 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (root-manifest citations re-pointed) fixed 6 root `Cargo.toml` citations a prior wrap missed after a line insert. Trap: a `Cargo.toml` edit in this chunk (dependency moves) shifts cited lines; re-point them at the wrap.

@history_item n=22 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (session host writes the command span) measured the By-invocation wiring: the workspace-built session host writes one `blitz_dom::document` INFO engine line from `info` up, the package-alone build none. Measured fact behind `decision/engine-tracing-by-invocation`.

@history_item n=23 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — re-read the workspace-built host: the `blitz_dom::document` `focussed node` INFO line prints twice (6 lines / 978 B), §2 Feature wiring updated. Readings under the two builds are what a dependency or feature move could shift.

@history_item n=24 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the `tool_call` span with closed-word fields `tool` and `ending` (founder, 2026-10-10). Basis of `decision/driver-span-fields-print`.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (failed load reaches its handler) on `Resource load failed`, `url` and `error` are in the scrub set so the sink prints them `[redacted]`. Trap: a trait default can hide a take's fix; sweep the implementers of a trait a take extends — relevant to semver-incompatible dependency moves (`rusqlite` 0.41, zbus 6, zip 9). Bears on `contract/telemetry-scrub-set/url` and `/error`.

@history_item n=26 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (take read silent) precedent that every upstream/dependency take is read for log sites, env reads, listeners and the sink re-checked (`host_log`/`mcp_log` at `trace`, both builds). A dependency version move in this chunk should carry the same re-check.

@history_item n=27 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (OTel export residual's status) brought the `otel-sdk-install` owner site to its residual row's status (`res-fzitzzw2`, re-carried, target next). Precedent: a master site naming a residual follows the row's status — this chunk closes or waits residuals `res-j4x7iiqc`, `res-kdhnfj5p`, `res-uqt2yd2y`, `res-p73cueyv`, `res-h7acz56c`, `res-q2tllspz`.

@history_item n=28 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (281 rows), each verified against the code; the picked rows' handles and form (decision/contract/absence) stem from it, and later amendments name rows by handle.

@history_item n=29 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child prints ids or digests. Part of the row's exception chain.

@history_item n=30 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `~ decision/checks-silent` except named the layout stand check's recorder. Part of the row's exception chain.

@history_item n=31 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `~ contract/escher-sink-env/rust-log` (and `contract/telemetry-sink/rust-log`): `RUST_LOG`'s `EnvFilter` applies to the sink layer alone, never the capture layer beside it. A `tracing-subscriber` move must keep that layering.

@history_item n=32 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (3. Observability Harness Contract) `~ decision/checks-silent` except: telemetry and error-text checks install escher's sink. Part of the row's exception chain.

@history_item n=33 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (8. PII, 1 row) `~ decision/escher-target-content-named-set`: `password` joined the redacted set. Picked row's current list.

@history_item n=34 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (8. PII, 3 rows) `− decision/scrub-at-sink-not-call-sites`, `+ decision/scrub-by-field-name`, `+ decision/engine-sites-fixed-messages`: scrubbing by field name in sink and capture, engine sites write fixed messages. Picked row's origin.

@history_item n=35 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (7. Error Capture) `~ contract/telemetry-scrub-set/error` and `~ contract/telemetry-scrub-set/url`: the capture keeps `url` `[redacted]` in a caused item. Picked rows.

@history_item n=36 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ contract/telemetry-scrub-set/error` (error crosses caused as written, cut at the text bound) and `~ decision/scrub-by-field-name` (capture uses the answer's own hidden set). Current text of two picked rows, ratified 2026-11-17.

@history_item n=37 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/checks-silent` except: added «the order page logs». Current text of the picked row.

@history_item n=38 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — `+ contract/target-sweep-line/{line,verb,outcome,cause,counts}`: the dev-host build-directory sweep's closed JSON stdout line. The same chunk left the `CI=true` recompile carry (`cry-sqod24yl`) this chunk takes up; any cause found there must keep the sweep line's closed key set.
