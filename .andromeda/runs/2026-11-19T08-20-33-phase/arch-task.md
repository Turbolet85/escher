@task_prose part=chunk
Measures, from a cold build directory, what grows target/ on the dev host: which cargo invocations of ordinary work (plan gate entries, the local CI legs fast, a11y, stand-repeat, embolden, coverage, doc, build, the agent-run harness, the cold-agent pipe, the dev-host mutation tier) write which artifacts, and which edits mint a new metadata hash for which units, read from cargo's own output.
Names the cause as numbers: the share of re-hash per low-crate edit, feature set or package selection differing between invocations, profile or flag differences, per-file integration-test binaries, and debuginfo weight.
Re-measures the overseer's figures (773 GB in deps, 193 test binaries of about 145 MB, 30 to 65 stale hashes each) as hypotheses, not premises.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep confined to target/, less debuginfo for test binaries, or another), with every check still green.
States a working day's growth after the mend as a number of gigabytes, read before and after on the dev host (capability v040-01, method manual).
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace root manifest holding the build profiles and their debuginfo level — edits: the [profile.*] tables
@task_file n=2 path=tests/blitz-tests/Cargo.toml mark=edits
what: the blitz-tests integration-test crate manifest declaring its test targets — edits: the test target table
@task_file n=3 path=.github/scripts/ci-leg.sh mark=edits
what: the one CI leg runner, each leg's cargo invocation — edits: the legs' cargo invocations
@task_file n=4 path=.github/workflows/ci.yml mark=edits
what: the GitHub Actions CI workflow and its jobs that call the leg runner
@task_file n=5 path=.github/scripts/test_ci_workflows.py mark=edits
what: the unittest check pinning the CI workflow contract and the legs' invocations — edits: the pins of a changed leg
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the unittest check pinning the blitz-tests test-target layout — edits: the target layout pins
@task_file n=7 path=scripts/agent-run.sh mark=edits
what: the agent-run harness script, its cargo build and its state directory
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest configuration holding the mutants profile of the dev-host mutation tier — edits: the mutants profile
@task_file n=9 path=CLAUDE.md mark=edits
what: the project's agent instructions listing the key build and test commands — edits: the key commands a mend changes
@task_file n=10 path=.claude/docs/commands.md mark=edits
what: the key commands reference for build, test and CI legs — edits: the commands a mend changes
@task_item n=1 part=needs
The build directory's growth per cargo invocation, measured from a cold target/ by size before and after each step, with each unit's metadata hash read from cargo's fingerprint, message-format output or the deps names, not inferred.
@task_item n=2 part=needs
The feature unification and package selection of each leg: workspace-wide test, the blitz-tests-only legs, the embolden leg's added font-embolden feature, the seven_guis test, and the blitz-dom feature-subset clippy line, with in-repo crates built without default features and default features fixed per crate; whether each resolves shared crates to a different hash.
@task_item n=3 part=needs
The debuginfo level: line-tables-only for dev and test, one debuginfo level for dev host and CI, and build profiles left unrewritten; a lowered debuginfo for test binaries must act on these rules or change them openly.
@task_item n=4 part=needs
The blitz-tests test-target layout: integration tests live in blitz-tests, one test target per file, the manifest's single test table, and its pin; one binary per leg changes this layout and its pin.
@task_item n=5 part=needs
Tests moved or merged keep every assertion and run in both layout modes; one behaviour per integration test file, shared test modules in subdirectories, and stand-check helpers in non-target modules hold where files move.
@task_item n=6 part=needs
The committed lockfile and --locked on every build, test and CI cargo leg stay as stated.
@task_item n=7 part=needs
The CI legs' membership and the one leg runner stay; a change to a leg's cargo invocation is a CI-pipeline change pinned by the CI contract tests and goes through them; the fast/slow split, the a11y leg, the stand-repeat zero-failure budget and the embolden local witness keep their meaning.
@task_item n=8 part=needs
The coverage leg under cargo llvm-cov and its per-crate coverage floors: where it builds, whether that counts toward target/ growth, and that the floors still pass after the mend.
@task_item n=9 part=needs
The mutation tier: cargo-mutants under nextest's mutants profile stays dev-host-only, never a CI leg, demo shells still excluded; where its scratch builds land is measured.
@task_item n=10 part=needs
A stale-hash sweep, if chosen, touches target/ alone and adds no network, socket, listener, environment read or secret.
@task_item n=11 part=needs
The agent-run harness and the cold-agent pipe as build consumers: their cargo builds are measured, and their events, exit codes and state directories stay as they are.
@task_item n=12 part=needs
The doc and build legs, with rustdoc warnings as errors and the fmt and clippy gates, stay green after the mend.
@task_item n=13 part=needs
The CI rust cache on compiling jobs and the shared test cache restored but never saved: a change to profiles or test layout keeps those cache rules.
@task_item n=14 part=needs
The key commands in the agent instructions and the commands reference, and the plan's statement of the test crate's layout in Infrastructure Patterns, follow the mend where it changes them.
@task_item n=15 part=needs
The capability v040-01 statement: the cause measured and named, a day's growth after the mend stated in gigabytes read before and after, every check green.
@task_item n=16 part=needs
Sharing the stand tests' setup is a separate entry, out of this chunk, unless the chosen mend moves those files anyway, which the plan then states.
@task_standing n=1
convention: a new or changed CI helper or sweep script is written on the Python standard library, within the Python 3.11 floor of the ci-scripts leg
home: stack-and-technologies
@task_standing n=2
convention: a new or changed check of a CI script uses unittest and descriptive test names
home: conventions
@task_standing n=3
convention: workflow actions stay pinned to commit SHAs and CI permissions stay read-only in any edited workflow job
home: infrastructure-patterns
@task_standing n=4
convention: dependencies are declared once in the workspace and crates inherit the workspace package metadata in any edited manifest
home: conventions
@task_item n=1 part=not_needed
Engine behaviour decisions and contracts (style, layout, paint, scrolling, hit testing, select lists, event dispatch, focus): the chunk changes no engine code and no test assertion.
@task_item n=2 part=not_needed
Driver verbs, session wire, socket host, MCP server, refusal causes, snapshot, diff and kept-check contracts: no carrier or answer changes.
@task_item n=3 part=not_needed
Accessibility tree and AccessKit adapter contracts: untouched by build layout or profile changes.
@task_item n=4 part=not_needed
WPT runner, its outputs, result kinds, statuses and PR diff contracts: the WPT runner is outside this chunk's measurement and mend.
@task_item n=5 part=not_needed
Browser app decisions and contracts (history, about pages, toolbar, persistence): no app behaviour changes.
@task_item n=6 part=not_needed
JavaScript runtime, script globals and script fetcher contracts: no script code changes.
@task_item n=7 part=not_needed
Telemetry line, scrub and bootstrap contracts: the chunk adds no log line to any binary.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and upstream-take decisions: the chunk takes nothing from another tree.
@task_item n=9 part=not_needed
Cold-agent verdict fields and wrong-call bar: the pipe is only measured as a build consumer, its verdict unchanged.
