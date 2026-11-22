@task_prose part=chunk
Closes the dependency audit's standing per-ID advisory ignores (sqlite, paste, memmap2, the zbus family, zip, and ttf-parser read with them) by a reachable version move, or states each as a wait on a named release, a keep by the founder's word, or a limit.
Adds skia's verified prebuilt archive: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL, a CI step and a fetch script per platform, no upstream code changed.
Acts on the cancelled-runner watch (a hosted macOS or iOS CI job cancelled with no runner and 0 steps) after its recurrence: retry, continue-on-error, removal from the required set, or the founder's keep.
Re-measures why a CI=true run of the CI leg script on the dev host recompiles its units under new hashes, and makes it reuse the dev host's units or states the cause as a limit.
Claims Codebase health (v040-08) against the Epoch 2 boundary audit's reading; the default-branch line of the workflow doc lands in the operator's slot, never as a code step.
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit's configuration, the cargo-deny advisory check — edits: the [advisories] ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace manifest, its dependency version pins — edits: the pins a version move reaches
@task_file n=3 path=Cargo.lock mark=edits
what: the workspace lock file read under --locked by the audit, MSRV and lock legs
@task_file n=4 path=apps/browser/Cargo.toml mark=edits
what: the browser app's manifest, its features (skia default, avif, capture) and the dependencies an update reaches
@task_file n=5 path=packages/browser-persistence/Cargo.toml mark=edits
what: the browser persistence package's manifest, its rusqlite pin
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the fork's CI on GitHub workflows, its jobs, legs and artifact uploads — edits: the skia fetch-and-verify step, the jobs that build skia, the hosted macOS and iOS jobs the cancelled-runner watch reads
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script the CI jobs and the local gates run, with its per-leg log and its env reads under CI — edits: the legs that build skia, the CI env handling
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow check, holding the workflow's pins and invariants — edits: the pins and invariants for the skia step and the changed jobs
@task_item n=1 part=needs
The fork's CI on GitHub workflows: a new step fetches skia's prebuilt archive, checks its recorded checksum and exports SKIA_BINARIES_URL to the verified file, in each job that builds skia.
@task_item n=2 part=needs
The skia fetch is a supply-chain surface: the step and the per-platform script add no credential, secret, socket or listener; the committed checksum file holds no secret and no private data.
@task_item n=3 part=needs
The per-leg CI log uploaded on failure, and the fork CI's two artifacts (that log and the coverage report on success), stay as they are across the job edits and any change to the required set.
@task_item n=4 part=needs
A failed skia fetch or checksum mismatch fails the step with a fixed diagnostic naming the phase and exit status, and never falls back to a silent source build.
@task_item n=5 part=needs
The local gates run the same checksum check through the CI leg script; local state (the fetched archive, the build directory) stays local and is never uploaded.
@task_item n=6 part=needs
The cancelled-runner watch's action changes how a hosted macOS or iOS job's no-runner cancel is read by CI: a retry, continue-on-error or removal from the required set, or no change on the founder's keep.
@task_item n=7 part=needs
The CI=true recompile on the dev host: the CI leg script's reads of CI and anything it switches under CI (flags, profile, env a build script reruns on) are measured; a change makes it reuse the dev host's units, or the cause is stated as a limit.
@task_item n=8 part=needs
The build-directory sweep's stdout line (verb, outcome, counts, cause) the CI leg script prints is kept as it is if the leg script's CI handling changes.
@task_item n=9 part=needs
Each dependency version move is a lock and manifest change under --locked with the audit, MSRV and lock legs green; no line of upstream code changes for skia.
@task_item n=10 part=needs
The CI workflow check's pins and invariants are updated for each workflow change it guards, and stay silent on success.
@task_item n=11 part=needs
A version move that reaches a crate escher's telemetry takes (tracing, the sink's dependencies) keeps escher's sink log line, its resource keys service.name and service.version, and engine tracing's cargo-feature gate unchanged.
@task_standing n=1
convention: each new or changed CI job or step runs on GitHub workflows, its leg log uploaded on failure, uploading nothing local beyond the fork CI's two artifacts
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new check or CI diagnostic line prints nothing on success, and on failure names a kind and a count or a phase and exit status in fixed text, never echoing captured content
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Driver spans, command and tool_call span fields, and refusal cause names: the chunk changes no driver code.
@task_item n=2 part=not_needed
The cold-agent and agent-run harness events, verdict fields and counted verbs: the chunk touches neither pipe.
@task_item n=3 part=not_needed
escher-session's commands, exit codes, stdout, host wire, coverage role and recording files: no session behaviour changes.
@task_item n=4 part=not_needed
PII scrubbing rows over select lists, snapshots, actions, layout and graph answers: no engine or driver behaviour changes.
@task_item n=5 part=not_needed
Script error surfaces, the window error event, WPT report messages and the WASM panic hook: no script or error-capture change.
@task_item n=6 part=not_needed
Metric emission, metrics exporters and telemetry backends: the chunk adds none.
@task_item n=7 part=not_needed
The embedder drain of script diagnostics and the escher-telemetry panic hook: neither is touched by a dependency move or CI change.
