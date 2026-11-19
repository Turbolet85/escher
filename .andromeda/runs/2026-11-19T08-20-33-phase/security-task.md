@task_prose part=chunk
Measures, from a cold build directory, what grows target/ on the dev host: which gate entries, CI legs run locally, the agent-run harness, the cold-agent pipe and the dev-host-only mutation tier write which artifacts, and which edits mint a new metadata hash, read from cargo's own output.
Names the cause as numbers per mechanism: a re-hash per low-crate edit, differing feature sets or package selections between invocations, profile or flag differences, per-file integration-test binaries, debuginfo weight.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep confined to target/, or less debuginfo for test binaries), with every check still green.
Re-measures a working day's growth after the mend as a number of gigabytes, read before and after on the dev host.
Re-asserts the committed lockfile, --locked on every build, test and CI leg, and the CI legs' membership; changes no test assertion.
@task_file n=1 path=Cargo.toml mark=edits
what: the root workspace manifest holding the build profiles — edits: the [profile.*] tables (dev debuginfo level)
@task_file n=2 path=.cargo/config.toml mark=edits
what: a cargo build configuration file for the workspace, none standing today; created only if the mend sets build flags there
@task_file n=3 path=tests/blitz-tests/Cargo.toml mark=edits
what: the unpublished blitz-tests integration-test crate's manifest — edits: its test target list and its tests/ layout
@task_file n=4 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner that fires each leg's locked cargo invocation (fast, a11y, stand-repeat, embolden, coverage, doc, build) — edits: a leg's cargo invocation where the mend changes it
@task_file n=5 path=.github/workflows/ci.yml mark=edits
what: the CI workflow running every leg as a matrix job with SHA-pinned actions and read-only tokens — edits: a leg's step where the mend changes it
@task_file n=6 path=scripts/agent-run.sh mark=edits
what: the agent-run harness that runs a selected test stem through cargo — edits: its cargo invocation where the mend changes it
@task_file n=7 path=.config/nextest.toml mark=edits
what: the nextest configuration — edits: the mutants profile used by the dev-host-only mutation tier
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the check pinning the CI workflow's jobs and each leg's cargo invocation — edits: the pins of a changed leg
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning the blitz-tests crate's test targets and the legs that run them — edits: the pinned target list
@task_file n=10 path=CLAUDE.md mark=edits
what: the project instructions' key commands — edits: the key commands a mend changes
@task_file n=11 path=.claude/docs/commands.md mark=edits
what: the commands reference — edits: the key commands a mend changes
@task_item n=1 part=needs
Locked builds: the committed Cargo.lock stays, and every build, test and CI leg keeps --locked, including any leg whose cargo invocation the mend rewrites.
@task_item n=2 part=needs
The font-embolden feature stays tested by its own locked fast leg; a mend unifying feature sets between legs keeps that leg and its feature.
@task_item n=3 part=needs
The CI matrix keeps running every job and every leg's membership; a changed leg's invocation goes through the pinning checks of the CI workflow and the blitz-tests targets.
@task_item n=4 part=needs
A changed workflow step keeps actions pinned to commit SHAs and read-only CI tokens; no new secret reaches any step.
@task_item n=5 part=needs
Coverage stays measured by cargo-llvm-cov under the per-crate line coverage floors; where llvm-cov builds and what it writes is measured, not changed in meaning.
@task_item n=6 part=needs
The blitz-tests crate stays unpublished, workspace-inherited in its dependency versions, with no git dependency added; its test harness takes no tracing and no style.
@task_item n=7 part=needs
agent-run.sh selection still reaches cargo only as a test stem from its allowlist; a merged test binary leaves that allowlist check intact.
@task_item n=8 part=needs
A stale-hash sweep, if chosen, removes only superseded artifacts inside target/ and nothing outside it: no following of links out of it, no removal of foreign files.
@task_item n=9 part=needs
No new network access, socket, environment read or secret in any script, profile or harness the mend touches.
@task_item n=10 part=needs
The mutation tier (cargo-mutants under nextest) stays dev-host-only and never becomes a CI leg.
@task_item n=11 part=needs
The cold-agent pipe is measured only for what it writes to the build directory; its recordings, transcripts and checks stay owner-only and unprinted as they are.
@task_item n=12 part=needs
Restructure invariant: integration-test files moved or merged into one binary per leg keep every assertion and both layout modes, held by the existing test suite and the blitz-tests targets check.
@task_item n=13 part=needs
The measurement's output is sizes, hashes and counts of build artifacts only; it records no credentials and no page or screen content.
@task_standing n=1
convention: every new or changed cargo invocation in a CI leg or script runs --locked against the committed lockfile
home: dependency-security
@task_standing n=2
convention: every new or changed CI workflow step uses actions pinned to commit SHAs and a read-only token
home: authentication-authorization · dependency-security
@task_standing n=3
convention: a new script validates its arguments before acting and touches no environment value or secret
home: input-validation
@task_item n=1 part=not_needed
Driver session socket, MCP server and escher-session wire contracts: request grammar, replies, exit codes, flags and refusal causes; the chunk changes no runtime binary.
@task_item n=2 part=not_needed
Snapshot text, stable element id, actions and graph answers, and accessibility crossings; no engine behaviour changes.
@task_item n=3 part=not_needed
Select list, range, password masking and other form-control behaviour rows.
@task_item n=4 part=not_needed
Outbound HTTP, fetch method limits, CORS and same-origin absences, sub-resource loading and navigation rows.
@task_item n=5 part=not_needed
Browser history, cookie store, HTTP cache, clipboard, file dialog and TLS data-handling rows.
@task_item n=6 part=not_needed
Engine and escher telemetry sink, span lines and log-target allowlists; the chunk adds no runtime log line.
@task_item n=7 part=not_needed
WPT runner, WPT post-results workflow and quarantine rows, and release publishing and signing rows.
@task_item n=8 part=not_needed
CSS parsing, HTML parsing, SVG serialization, script errors and Dioxus native bridge rows.
