@task_prose part=chunk
Measures from a cold build directory what grows it: which plan gate entries, local CI legs, the agent-run harness, the cold-agent pipe and the dev-host mutation tier write which artifacts, and which edits mint a new metadata hash for which units, read from cargo's own output.
Re-measures the overseer's read (deps size, daily write rate, per-file integration-test binaries and their stale hashes, feature sets differing between invocations) as a hypothesis, not a premise.
Names the cause as each mechanism's share in numbers: per-edit re-hash of a low crate, differing feature set or package selection, profile or flag difference, per-file test binaries, debuginfo weight.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep of the build directory, less test debuginfo, or another), with every check still green.
States a working day's build-directory growth after the mend in gigabytes, read before and after on the dev host.
@task_file n=1 path=Cargo.toml
what: the workspace manifest's build profiles (dev debuginfo level), the lever for test debuginfo weight
@task_file n=2 path=tests/blitz-tests/Cargo.toml
what: the stand test crate's manifest and its integration-test target layout, one binary per test file today
@task_file n=3 path=examples/seven_guis/Cargo.toml
what: the seven_guis example crate's manifest, owner of its integration-test targets
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner and each leg's cargo invocation, package selection and feature set (fast, a11y, stand-repeat, embolden, coverage, doc, build)
@task_file n=5 path=.github/workflows/ci.yml
what: the CI workflow, its legs, their membership and their uploaded artifacts
@task_file n=6 path=.github/scripts/test_ci_workflows.py
what: the CI-pipeline pin check over the workflow's legs and their cargo invocations
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py
what: the pin check over the stand test crate's test targets
@task_file n=8 path=scripts/agent-run.sh
what: the agent-run harness, its cargo builds, its state and its cleanup
@task_file n=9 path=.config/nextest.toml
what: the nextest configuration and its mutants profile for the dev-host-only mutation tier
@task_file n=10 path=CLAUDE.md
what: the project's key commands list, as a mend changes them
@task_file n=11 path=.claude/docs/commands.md
what: the key commands reference, as a mend changes them
@task_item n=1 part=needs
The build-directory measurement itself: size before and after each gate entry, CI leg, harness run and mutation-tier run from a cold start, each unit's hash read from cargo's fingerprint, message output or deps names, and the full workspace build footprint read first.
@task_item n=2 part=needs
The cause stated as numbers per mechanism, and the after-mend daily growth stated in gigabytes as a manual capability reading.
@task_item n=3 part=needs
The mend's behaviour change: one test binary per leg, a sweep of stale hashes, lower test debuginfo, or another the measurement names; whatever is chosen is a change of its own, with every check still green.
@task_item n=4 part=needs
A sweep, if chosen, touches the build directory alone, adds no network, socket, env read or secret, and removes only stale artifacts, never another tool's or the harness's own state.
@task_item n=5 part=needs
Restructure invariant where tests are merged or moved into fewer binaries: every test keeps its assertions and its two layout modes; the existing stand and seven_guis tests and the CI pin checks hold it.
@task_item n=6 part=needs
Feature unification across invocations: any change to package selection or feature sets must keep engine tracing compiled in only by the build's invocation through its cargo feature, and escher crates' tracing ungated, so the telemetry sink and driver spans in the native binaries stay as they are.
@task_item n=7 part=needs
The embolden leg's font-embolden feature and the no-default-features clippy leg keep their feature sets; a unified invocation may not drop a feature a leg exists to test.
@task_item n=8 part=needs
The committed lockfile, the locked flag on every build, test and CI leg, and the CI legs' membership stay as stated; any change to a leg's cargo invocation is a CI-pipeline change that passes through the CI pin checks.
@task_item n=9 part=needs
The mutation tier stays dev-host-only under nextest's mutants profile, never a CI leg; where it writes is measured, not moved into CI.
@task_item n=10 part=needs
The coverage leg builds in its own target subdirectory; whether it counts toward growth is measured, and its report upload on success stays as it is.
@task_item n=11 part=needs
The per-leg CI log uploaded on failure and the fork CI's two artifacts stay unchanged by a leg's new invocation; local build state is never uploaded.
@task_item n=12 part=needs
The agent-run harness and the cold-agent pipe, as build consumers measured here: their cleanup touches only their own state, their events and logs keep their fields and carry no captured output or user content.
@task_item n=13 part=needs
Checks whose empty-stderr assertion depends on RUST_LOG being unset keep that unsetting when moved into a merged binary, and headless stand tests still run with no sink installed.
@task_item n=14 part=needs
Test-crate layout as the test plan's harness and CI rows and architecture's infrastructure patterns state it, updated where the mend changes the layout.
@task_standing n=1
convention: a new harness or script log line is a JSON line encoded by python's json module, carrying no user content or captured output
home: 3-observability-harness-contract
@task_standing n=2
convention: a new check is silent on success and reports a failure by a kind and a count
home: 3-observability-harness-contract
@task_standing n=3
convention: a new or changed check that asserts empty stderr unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=4
convention: a new or changed CI leg uploads its log on failure and uploads no local state
home: ci-integration · 9-ci-integration
@task_standing n=5
convention: a new cleanup or sweep step touches only the state it owns
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
The driver's command and tool_call span contracts and their fields: the chunk changes no driver code and no span.
@task_item n=2 part=not_needed
escher's sink log line format, escaping, scrub set and resource keys: no sink code changes.
@task_item n=3 part=not_needed
The cold-agent verdict's fields, counted verbs and reasons: the pipe is only measured as a build consumer, its verdict untouched.
@task_item n=4 part=not_needed
escher-session's commands, exit codes, host wire, coverage role, stdout and error object: no session behaviour changes.
@task_item n=5 part=not_needed
Script error surfaces, window error events, script diagnostics drain and WPT report contracts: no engine or script code changes.
@task_item n=6 part=not_needed
PII rows over page content, snapshots, answers, select lists and recordings: the chunk handles no page or user content.
@task_item n=7 part=not_needed
Recording file contracts, recording bounds and the session lifecycle: no recording or session state changes.
@task_item n=8 part=not_needed
WPT score publishing from upstream only: no publishing step changes.
