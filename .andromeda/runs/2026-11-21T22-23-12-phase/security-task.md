@task_prose part=chunk
Closes the dependency audit's standing per-ID advisory ignores (libsqlite3-sys, paste, memmap2, the zbus family, zip, and ttf-parser read alongside) by a reachable version update under locked builds, or states each as a wait on a named release of another project, a founder keep, or a limit.
Makes skia's prebuilt archive verified: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL, with a CI step and a fetch script per platform; no upstream code line changes.
Acts on the recurred cancelled-runner watch for the hosted macOS/iOS CI jobs, and finds why a CI=true run of the CI leg script on the dev host recompiles under new hashes, fixing it or stating the cause as a limit.
Re-reads the advisory chains and the project source's reading of CI, and registers skia's fetch and each advisory's standing in the security plan's Dependencies section.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny audit configuration, the dependency audit's advisory ignore list with a written reason per ID — edits: the [advisories] ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the root manifest, the workspace-inherited dependency versions and the engine dependency pins — edits: a dependency version move where an update reaches it
@task_file n=3 path=Cargo.lock mark=edits
what: the lockfile the locked builds and the audit leg read — edits: the resolved versions an update moves
@task_file n=4 path=apps/browser mark=edits
what: the browser app, a skia default build, its history store and capture — edits: its manifest where an update reaches it
@task_file n=5 path=packages/browser-persistence mark=edits
what: the browser's history storage in sqlite through rusqlite — edits: its rusqlite pin where an update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the CI workflow, its jobs matrix, audit, MSRV and lock legs and the hosted macOS and iOS jobs — edits: a skia fetch-and-verify step, the cancelled-runner handling of the hosted macOS/iOS jobs
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script the CI jobs and the local gates run — edits: the skia checksum check and SKIA_BINARIES_URL pointing, the CI=true path that rebuilds units on the dev host
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow tests holding the workflow's pins and invariants — edits: pins and invariants
@task_item n=1 part=needs
Advisory ignores stay per ID with a written reason: each ignore this chunk keeps carries its stated wait, keep or limit; each it closes leaves the ignore list.
@task_item n=2 part=needs
A dependency version move is a manifest and lockfile change under locked builds with the dependency audit CI gate, the MSRV leg, the lock leg and the licence gate green.
@task_item n=3 part=needs
Skia built by default with a build-time fetch changes: the prebuilt archive is fetched once and verified against a checksum committed in the repository, the build pointed at the verified file through SKIA_BINARIES_URL, the sum updated by hand at each skia version move; a failed fetch must not fall back silently to an unverified source.
@task_item n=4 part=needs
Skia's fetch is a supply-chain surface: registered in the security plan's Dependencies section with its checksum verification; the checksum file is no secret.
@task_item n=5 part=needs
No line of upstream's code changes for skia; the engine dependency pins and the upstream sync's whole dependency set are moved only where a version update is reachable and compatible (AccessKit 0.25 and winit beta pins coupled).
@task_item n=6 part=needs
The new CI step and per-platform fetch script: actions pinned to commit SHAs, read-only CI token, no new secret, credential path, socket, port or listener.
@task_item n=7 part=needs
The CI environment variable: what reading CI=true changes in the CI leg script, the build profile or flags, or a build script's rerun on env change, measured and either removed as a recompile cause or stated as a limit.
@task_item n=8 part=needs
SKIA_BINARIES_URL becomes an environment value the CI and local gates set, pointing at a local verified file.
@task_item n=9 part=needs
A rusqlite move for the libsqlite3-sys advisory keeps browser history stored in sqlite with SQL values bound through placeholders.
@task_item n=10 part=needs
The zip advisories' standing reads the capture path through anyrender_serialize, which reads entries by name and never extracts.
@task_item n=11 part=needs
The memmap2 unsound advisory stays kept out by cargo-deny's unsound default as a measured limit, not an ignore, unless a compatible fixed version is reachable.
@task_item n=12 part=needs
The cancelled-runner watch: CI matrix runs every job; acting on the hosted macOS/iOS no-runner cancel (a retry, a continue-on-error, a removal from the required set, or a keep) changes what a required CI job may be.
@task_item n=13 part=needs
Run the audit leg locally before a push; the local gates verify the skia sum the same way CI does.
@task_item n=14 part=needs
Manifests keep no git dependency and workspace-inherited dependency versions after any version move.
@task_item n=15 part=needs
CI's GitHub API calls, where acting on the cancelled-runner watch reads runs, go through the gh CLI.
@task_standing n=1
convention: each new CI step pins its actions to commit SHAs and runs under a read-only token
home: dependency-security · authentication-authorization
@task_standing n=2
convention: each new build-time fetch is registered as a supply-chain surface in the Dependencies section with its integrity check
home: dependency-security
@task_standing n=3
convention: each advisory ignore kept or added is per ID with a written reason
home: dependency-security
@task_standing n=4
convention: each new CI script or step holds no secret; secrets stay in GitHub Actions secrets and vars
home: secret-management
@task_item n=1 part=not_needed
The driver session socket, its wire, requests, replies and bounds: no driver or session code changes.
@task_item n=2 part=not_needed
The MCP server, its transport, request and result contracts and protocol errors: untouched.
@task_item n=3 part=not_needed
Select lists, accessibility, stable ids, snapshot text, control states and other engine input rules: no engine behaviour changes.
@task_item n=4 part=not_needed
Kept checks, recorded checks, cold-agent runs and coverage answer lines: untouched.
@task_item n=5 part=not_needed
Refusal causes, session error tables and refusal JSON: no refusal path changes.
@task_item n=6 part=not_needed
Log sinks, span lines, telemetry allowlists and log target rules: the chunk adds no log line.
@task_item n=7 part=not_needed
Crossing decisions of the threat model summary (answers, screens, select lists crossing to providers): no carrier changes.
@task_item n=8 part=not_needed
Outbound HTTP, CORS, same-origin, request size and timeout absences at runtime: skia's fetch is a build-time fetch, not a runtime request.
@task_item n=9 part=not_needed
Canvas work, the upstream syncs and the whole-mode leaves pass: out of this chunk's scope.
