@task_prose part=chunk
Measures, from a cold build directory, which cargo invocations of the dev host's ordinary work (plan gate entries, the local CI legs, the agent-run harness, the cold-agent pipe, the dev-host mutation tier) write which artifacts and which edits mint a new metadata hash for which units, read from cargo's own output.
Names the cause from that measurement as a share per mechanism: low-crate re-hash per edit, differing feature sets or package selections, profile or flag differences, per-file integration-test binaries, debuginfo weight.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep of the build directory, less debuginfo for test binaries, or another), with every check still green.
Re-measures a working day's growth of the build directory after the mend and states it as a number of gigabytes, read before and after (capability v040-01, manual method).
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest and its build profiles, the dev profile's debuginfo level — edits: the [profile.*] tables
@task_file n=2 path=.cargo/config.toml mark=edits
what: cargo's per-repository build configuration (none stands today; a mend may add it)
@task_file n=3 path=tests/blitz-tests/Cargo.toml mark=edits
what: the blitz-tests test crate manifest, its test targets and its difference from upstream — edits: its test target list
@task_file n=4 path=tests/blitz-tests/tests mark=edits
what: the blitz-tests integration test files, one test target per file today (185 files)
@task_file n=5 path=examples/seven_guis/tests mark=edits
what: the seven_guis stand and command-line integration test files (8 files), each its own test target
@task_file n=6 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script every CI leg runs through, its legs' cargo invocations (fast and its parts, a11y, stand-repeat, embolden, coverage, doc, build) — edits: a leg's cargo invocation
@task_file n=7 path=.github/workflows/ci.yml mark=edits
what: the GitHub Actions CI workflow, its jobs, cache and leg membership
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the unit tests of the CI workflow and leg script, pinning each leg's invocation and membership
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the unit tests pinning the blitz-tests test targets and the files that join each leg
@task_file n=10 path=scripts/agent-run.sh mark=edits
what: the agent-run harness, its verbs and run selections and the cargo invocations it fires
@task_file n=11 path=.config/nextest.toml mark=edits
what: the nextest configuration, its mutants profile for the dev-host mutation tier — edits: the mutants profile
@task_file n=12 path=CLAUDE.md mark=edits
what: the project's key commands listing — edits: key commands a mend changes
@task_file n=13 path=.claude/docs/commands.md mark=edits
what: the project's command reference — edits: key commands a mend changes
@task_item n=1 part=needs
The build-directory growth measurement: each invocation's growth read by size before and after from a cold build directory, each unit's hash read from cargo's fingerprint, message-format output or deps names, never inferred.
@task_item n=2 part=needs
Feature unification and package selection across invocations: workspace-wide test runs, per-package test runs, the embolden leg's added paint feature, and the no-default-features clippy run on upstream blitz-dom, each possibly resolving shared crates to a different hash.
@task_item n=3 part=needs
Where the coverage tool (its own target subdirectory) and the mutation tool under the nextest mutants profile write their builds, and whether they count toward the growth.
@task_item n=4 part=needs
The blitz-tests one-test-target-per-file layout and its manifest's test target list, which a one-binary-per-leg mend would replace; any merge of test files into one target must keep shared test modules out of the target list.
@task_item n=5 part=needs
A test moved or merged keeps every assertion and its two layout modes; no change to what any test asserts, and the moved tests' existing checks hold them.
@task_item n=6 part=needs
Test module paths and test names kept where files are merged into one target, so the CI tally of moves stays attributed by test name and a file joining a leg keeps its legs.
@task_item n=7 part=needs
Every requirement still witnessed inside stand-repeat after any change of the stand tests' targets, and stand-repeat's three rounds and zero-failure budget kept.
@task_item n=8 part=needs
The committed lockfile and --locked on every build, test and CI leg kept; the CI legs' membership kept as the invariants state.
@task_item n=9 part=needs
A change to a leg's cargo invocation is a CI-pipeline change that goes through the CI workflow and blitz-tests target unit tests, which pin it.
@task_item n=10 part=needs
CI legs keep running through the leg script, host-reproducible, under the fast/slow split and the shared build cache; the free-disk step on the test and stand-repeat jobs remains.
@task_item n=11 part=needs
A stale-hash sweep, if chosen, touches the build directory alone and opens no network, socket, env read or secret.
@task_item n=12 part=needs
The mutation tier stays dev-host-only under its nextest profile, never a CI leg.
@task_item n=13 part=needs
Dev-profile debuginfo (line-tables-only today) and any reduction for test binaries must keep coverage floors per crate and the coverage leg's exit by the floors working.
@task_item n=14 part=needs
The agent-run harness keeps its verbs, run selections, events and exit codes while any cargo invocation inside it changes.
@task_item n=15 part=needs
The local pre-push gate and local baseline for CI comparison keep their meaning after the mend; per-package green is still not the workspace green.
@task_item n=16 part=needs
The capability v040-01: the cause measured and named, a working day's growth after the mend stated in gigabytes, read before and after, every check green.
@task_item n=17 part=needs
The test plan's harness and CI rows and architecture's test-crate layout restated where the mend changes the test crate's layout or a leg's invocation.
@task_standing n=1
convention: each new or changed CI Python script assertion is a unittest test of the CI scripts, red before green
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=2
convention: each cargo invocation a leg or script adds runs --locked
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each CI job or step added keeps actions pinned to commit SHAs, read-only permissions, no secrets and bounded retried package installs
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=4
convention: each stand or layout test moved keeps running in both layout modes on the bundled font, with no skip
home: test-scope-summary · 2-test-strategy
@task_item n=1 part=not_needed
Driver refusal causes, driver verbs, driver JSON answers and argument kinds: the chunk changes no driver behaviour.
@task_item n=2 part=not_needed
Session wire, session host, session errors, session state directory and session recording contracts: no session behaviour changes.
@task_item n=3 part=not_needed
MCP server and escher-session command line, exit status and stderr contracts: no command-line or MCP surface changes.
@task_item n=4 part=not_needed
Harness construction, core, input, inspection, settle, screenshot and contrast contracts: the headless harness's API is untouched.
@task_item n=5 part=not_needed
Lint defects, lint findings, snapshot node and handler coverage contracts: no engine reading changes.
@task_item n=6 part=not_needed
WPT runner CLI, outputs, status set and WPT pinned commit rows: the WPT workflow is upstream-only and outside the measured legs.
@task_item n=7 part=not_needed
Mocking and stubbing discipline, stand-in, virtual clock and timer tick rows: no stand-in or time driving changes.
@task_item n=8 part=not_needed
Cold-agent verdict, prompt, isolation and live-run rows: the cold-agent pipe is only measured for its build growth, its behaviour unchanged.
@task_item n=9 part=not_needed
Telemetry, log-line and privacy rows of stand logs and the driver log: the chunk adds no log line of the product.
