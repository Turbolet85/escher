# obs-plan · the rows 2026-11-21-dependency-and-ci-debts touches — picked 6 of 295 · rule any · pin 3029

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
absence/metrics-export — Metrics exporters and metric emission (counters, gauges, histograms) · r1
overview/5-metric-coverage — In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process: the frame and phase timings behind the log-times features, blitz-dom's resolve phases, debug_timer, blitz-paint's layer counters, the bench, screenshot and multi_doc examples and the WPT runner's figures; the WPT scores are published to Pages on upstream DioxusLabs/blitz only. · r1

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/wpt-scores-upstream-only · WPT scores published from upstream only — WPT scores are computed and published to Pages only on the upstream DioxusLabs/blitz repository, the wpt job being repository-guarded. · r1 · stale
absence/metric-emission — Metric emission: counters, gauges or histograms · r1
absence/metrics-exporters — Metrics exporters or telemetry backends (OpenTelemetry, Prometheus, Sentry) · r1

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
> NOT YET MEASURED — snapshot artifact upload and CI resource attributes: the reading recorded none
decision/ci-platform-github-workflows · CI on GitHub workflows — CI runs as GitHub workflows under .github/workflows/. · r1 · add-to · F2
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret. · r1
decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data. · r1
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg. · r1
decision/wpt-publishing-upstream-only · WPT results published upstream only — The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork. · r1
contract/apt-install-diagnostic/attempt · The package-install diagnostic line · attempt — One stderr line per failed attempt names the attempt number, and nothing of a user's. · r1 · add-to · F1
contract/apt-install-diagnostic/exit-status · The package-install diagnostic line · exit status — The line names the failed attempt's exit status. · r1
contract/apt-install-diagnostic/phase · The package-install diagnostic line · phase — The line names the phase as one of update, install or repair. · r1 · add-to · F1

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/agent-run-cleanup-scoped · Agent-run cleanup touches only its own state — The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/. · r1
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data. · r1 · add-to · F1
decision/cold-agent-cleanup-scoped · Cold-agent cleanup touches only its own state — The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits. · r1
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording. · r1
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public. · r1
decision/coverage-artifact-on-success · Coverage report uploaded on success — The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days. · r1 · add-to · F4
decision/recording-bounds · Recording bounds — A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording. · r1
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only. · r1
decision/session-state-dir-private · Session state directory private and empty of content — A session's state directory is created 0700 by the host and holds the socket file alone while a session is up and nothing after stop or idle expiry; no log, event or screen content is written into it. · r1
contract/record-refusals/record-dir-not-private · Session recording refusals · record-dir-not-private — A recording directory open to its group or others is refused with record-dir-not-private. · r1
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png. · r1
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename. · r1
contract/recording-file/typed-text · The recording file · typed text — Typed text is held unmasked, except a type to a masked or unresolved control, held as masked. · r1
contract/session-lifecycle/idle-expiry · The session lifecycle · idle expiry — The host's idle expiry ends a session exactly as stop does. · r1
contract/target-sweep-line/cause · The build-directory sweep's stdout line · cause — A refused or failed line adds cause, one of build-in-flight, no-build-directory, unsupported-host or keep-run. · r1
contract/target-sweep-line/counts · The build-directory sweep's stdout line · counts — A swept or planned line adds the counts units_kept, units_removed (run) or units_to_remove (plan), bytes_before, bytes_after and seconds. · r1
contract/target-sweep-line/line · The build-directory sweep's stdout line · line — Each ending of the dev-host build-directory sweep prints exactly one JSON line on stdout, a closed key set per outcome, holding no path, unit name or key. · r1
contract/target-sweep-line/outcome · The build-directory sweep's stdout line · outcome — Every line carries outcome, one of swept, planned, refused or failed. · r1
contract/target-sweep-line/verb · The build-directory sweep's stdout line · verb — Every line carries verb, one of plan or run. · r1
