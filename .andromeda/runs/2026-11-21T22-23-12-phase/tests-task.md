@task_prose part=chunk
Closes the dependency audit's standing per-ID advisory ignores (sqlite, paste, memmap2, the zbus family, zip, and the ttf-parser ignore read with them), each by a reachable locked update or stated as a wait, a founder's keep, or a named limit.
Builds the verified skia prebuilt archive: a checksum recorded in the repository, a per-platform fetch script, a CI step and the local gates checking the sum and pointing the build at the verified file through SKIA_BINARIES_URL, with no line of upstream code changed.
Acts on the cancelled-runner watch (a hosted macOS or iOS job cancelled with no runner and no steps) after its recurrence, and finds or states as a limit why a CI=true fast leg on the dev host recompiles its units under new hashes.
Re-reads the codebase-health claim against the Epoch 2 boundary audit, the founder's keeps of oversize files, over-ceiling functions and duplicated test pairs standing as upstream's.
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit's configuration, advisories and licenses — edits: the advisories ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace manifest and its dependency pins, built under --locked — edits: the dependency pins an advisory update reaches
@task_file n=3 path=Cargo.lock mark=edits
what: the workspace lockfile every cargo leg runs locked against
@task_file n=4 path=apps/browser/Cargo.toml mark=edits
what: the browser app's manifest, its skia default feature, its capture serializer and its avif image feature — edits: the dependency versions an update reaches
@task_file n=5 path=packages/browser-persistence/Cargo.toml mark=edits
what: the browser persistence package's manifest, its rusqlite pin and its migration crate — edits: the rusqlite pin where the sqlite update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the CI workflow on GitHub Actions, its jobs, native platform matrix, required set and pinned actions — edits: the jobs that build skia (a new checksum-checking fetch step), the iOS and macOS jobs the cancelled-runner watch reads
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script every CI leg and the local pre-push gate run through — edits: the legs that build skia, the environment it reads under CI
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the unit tests of the CI scripts and workflow, holding its pins and invariants — edits: the pins and invariants over the new skia step and the changed jobs
@task_item n=1 part=needs
The dependency audit leg: each per-ID advisory ignore either removed by an update that leaves the vulnerable or unmaintained crate, or kept with its stated standing (a wait on a named release, the founder's keep, a named limit); the memmap2 unsound advisory stays kept out by the audit's unsound default, a measured limit, not an ignore.
@task_item n=2 part=needs
A dependency version move is a lockfile and manifest change only, run under --locked, with the audit, MSRV (Rust 1.91, verified by build) and lockfile legs green.
@task_item n=3 part=needs
Moving rusqlite to the release that takes libsqlite3-sys 0.39 or later, where reachable, keeps the browser persistence package's behaviour and its existing tests green, its migration crate on a compatible release.
@task_item n=4 part=needs
The skia checksum check: the fetched prebuilt archive is compared to the sum recorded in the repository and a mismatch or failed fetch fails the step rather than falling back silently to a source build of skia.
@task_item n=5 part=needs
The build is pointed at the verified archive through the SKIA_BINARIES_URL environment value, in CI and in the local gates alike; skia stays a default feature of the browser app and the WPT runner, and no line of upstream code changes.
@task_item n=6 part=needs
One fetch script per platform that builds skia, its archive fetched once; the recorded sum is updated by hand at each skia version move.
@task_item n=7 part=needs
The new CI fetch step follows the CI rules: actions pinned to commit SHAs, read-only permissions, no secrets, its download bounded and retried as package installs are, and the leg run through the CI leg script so the host can reproduce it.
@task_item n=8 part=needs
The skia archive fetch is a supply-chain surface: no new socket, listener, credential path or secret; the checksum file is no secret.
@task_item n=9 part=needs
The CI workflow tests carry pins and invariants for the new skia step and for every job changed, and stay green under unittest.
@task_item n=10 part=needs
The cancelled-runner watch on hosted macOS and iOS jobs acted on after its recurrence: a retry, continue-on-error, removal from the required set, or the founder's keep, with fast jobs still gating the rest.
@task_item n=11 part=needs
A CI=true fast leg run on the dev host reuses the dev host's built units, or its recompile under new hashes is stated as a limit with its cause; the build directory sweep and the shared workspace test cache stay as they hold.
@task_item n=12 part=needs
The CI cache keys and their save branches stay valid with the skia archive now fetched from a verified file.
@task_item n=13 part=needs
Codebase health claim read against the Epoch 2 boundary audit: upstream-authored oversize files, over-ceiling functions on the host-read cognitive-complexity ceiling, and upstream's duplicated test pairs kept as upstream's by the founder's word, with no test-file split entry minted.
@task_item n=14 part=needs
CI log artifacts and the leg log uploaded on failure stay in place for the changed jobs, so a failed checksum or fetch reads in the uploaded leg log.
@task_standing n=1
convention: every CI job or step pins its actions to commit SHAs, runs with read-only permissions and uses no secrets
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: every CI leg runs through the CI leg script, host-reproducible, its cargo commands run locked
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every CI download or package install is bounded and retried
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: every change to a CI script or the workflow is held by the CI Python scripts' unit tests, run under unittest
home: 1-test-scope-summary · test-scope-summary
@task_standing n=5
convention: every new test is controlled red before green and, where table-driven, asserts its row count
home: 4-unit-test-strategy · unit-test-strategy
@task_item n=1 part=not_needed
The session wire, session host, session errors, session recording and session state directory contracts: the chunk changes no session protocol or host.
@task_item n=2 part=not_needed
The driver verbs, driver refusal causes, driver session, driver log line and the driver's advance command: no driver behaviour changes.
@task_item n=3 part=not_needed
The escher-session command line, MCP role, tools list, exit codes and stderr contracts: no command or tool is added or changed.
@task_item n=4 part=not_needed
The cold-agent pipe, its verdict, events, exit codes and isolation rules: untouched.
@task_item n=5 part=not_needed
The agent-run verbs, selections, events and exit codes: the chunk adds no agent-run behaviour.
@task_item n=6 part=not_needed
The harness construction, core, input, inspection, settle, screenshot and contrast contracts: the chunk touches no harness API.
@task_item n=7 part=not_needed
Lint defects, lint findings, handler coverage limits and snapshot node contracts: no reading changes.
@task_item n=8 part=not_needed
Mocking and stand-in rules, the stand timer, virtual clock and advance decisions: no test double or time driving is added.
@task_item n=9 part=not_needed
Committed recorded checks, shared fixture pages and stand fixtures: no fixture or recorded check is added.
@task_item n=10 part=not_needed
Select list painting, chosen row colours, open pseudo-class, password caret and buffered-render decisions: no engine or paint behaviour changes, and canvas work is out of scope.
@task_item n=11 part=not_needed
The windowed end-to-end absence and the real MCP client outside CI: no end-to-end run is added.
@task_item n=12 part=not_needed
WPT status set, WPT runner CLI arguments and its outputs: the runner's behaviour is unchanged, only how its skia is fetched.
