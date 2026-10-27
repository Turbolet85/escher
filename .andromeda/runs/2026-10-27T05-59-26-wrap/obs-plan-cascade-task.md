# Task — obs-plan · 2026-10-27-readme-and-records

## The session
The session changed documents and records only, with no source, test, CI script, manifest or lock file (the preservation guard reads clean, changed 2).
README.md now shows 0.1.0 and 0.2.0 done and verified and 0.3.0 planned in six stages. It names 0.2.0's additions and the six served task slugs, adds `lint` to Try it, and in Later names opt-in OpenTelemetry export as out of this version.
docs/capabilities.md got three restatements. The lint at boot now says the flight booker reads one `overlap`. The host-sink part read now carries its measured reading (`serve cells` at `trace`, 0 id and 0 name needles under both builds). The run paragraph now describes the 2026-10-25 cold-agent live run (66 calls, 0 wrong, 5 % bar, one `lint`, one `screenshot`, client 2.1.308, passed), and its verdict is still stated as counts, booleans and closed words only.
The local incubator exclude was retired from `.git/info/exclude`; `.gitignore:39` ignores the incubator before and after. A run dir's planted control literal was respelled for hygiene, and the ledger entries v030-02 (gated) and v030-16 were updated.
CI run CI#40456250076 on the final HEAD read green, 17 of 17 checks.

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/inputs/
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .andromeda/runs/2026-10-27T05-19-41-implement/
- .andromeda/runs/2026-10-27T05-59-26-wrap/
- .git/info/exclude

## Touches
- The host-sink paged-part read at `trace` (`host_log`'s fourth test, `serve cells`, 0 id and 0 name needles under both builds): docs/capabilities.md now states it as measured. The obs-plan sites that already state it (the §3 Logging stack paragraph at obs-plan.md:80, log-coverage.md:123, pii-scrubbing-compliance.md:11) were re-read and are unmoved.
- The cold-agent verdict fields (log-coverage, cold-agent pipe group): the README and the statement quote `tool_calls` 66, `wrong_calls` 0, `wrong_call_bar_pct` 5, `tasks_met` six, `lint_calls` 1, `screenshot_calls` 1, `client_version` 2.1.308 and `outcome` passed. All are existing `live-verdict.json` fields; no field was added, renamed or retired, and the 5 % bar is unchanged.
- The rule that the cold-agent verdict holds only counts, booleans and closed words: the statement keeps that sentence as is, matching obs-plan's verdict-field rule, which is unmoved.
- The cold-agent carriage to the model provider (pii, the cold-agent pipe log line): six apps' screens, Cells' contents, the select's options and value, a lint's findings and one screenshot. The statement now attributes this carriage to the 2026-10-25 run; obs-plan's line already states it and is unmoved.
- The `escher-session` binary's six served slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) in §3 Logging stack: README.md now names the same six, so the claim is unmoved.
- The lint at boot (the flight booker's one `overlap`) was restated in docs/capabilities.md. obs-plan has no lint-at-boot site, and its visual-lint silence line is unmoved.
- The silence census lines, the env-read set and escher-driver's 2 `info_span!` sites are all unmoved, because no source, test or shared module changed.
- CI integration: the final HEAD's CI run is green, 17 of 17. No CI step, leg, uploaded artifact or retention moved.
- OTel SDK init (§3) is still absent. The README's Later line restates intent's out-of-version opt-in OpenTelemetry export, and no exporter or setup was added.
- The `as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md` citation (the served-stand silence line): the statement now cites the same report for the host-sink reading, and no obs-plan citation moves.
- The README's pinned line moved from `README.md:48` to `README.md:49`. Every citation of it is in architecture, none in obs-plan, so nothing here re-points.
- The local state rule (ci-integration: agent-run and cold-agent state under `target/`, session and recording directories, two committed kept checks): the incubator exclude retired from `.git/info/exclude` is no log, event or telemetry state, and the rule is unmoved.
- The run dir `p5-controls.sh` literal respelled to `/home/<user>/x` is a hygiene control in a run-dir script, not a log line or field. No scrub-set or log-field claim moves.

## New facts
- (none)

## Untouched
- escher-telemetry's subscriber, line format, closed-span record class, target allowlist, scrub sets and panic hook: no telemetry source changed.
- escher-driver's `command` and `tool_call` spans, their fields and the rule against fielding screen content: no driver source changed.
- Metric coverage (in-process timings, counters, WPT scores): nothing in the session reaches them.
- Error capture and reporting (panic hooks, error classes, the script error surface): no engine or app source changed.
- The agent-run harness log and its event shapes: no harness script or verdict shape moved.
- The cold-agent pipe's state area, `server.log` counting and replay children: `scripts/cold-agent.sh` is unchanged. Only documents quote the 2026-10-25 run's figures.
- Telemetry strategy's feature wiring and the engine's `tracing` gating: no manifest or feature changed.
- The upstream apps' `fmt::init()` and the WPT runner's `env_logger` unscrubbed-log values: unreached.
