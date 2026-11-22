@task_prose part=session
The session settled dependency and CI debts. It added no Rust source and moved no manifest.
It recorded skia's prebuilt archives (tag 0.157.0, five targets, each with key and sha256) and added a stdlib Python script with unit tests. The script fetches one target's archive with bounded retries, verifies it against the record, and prints a file:// SKIA_BINARIES_URL template.
Every skia-building leg of the leg runner (clippy, test, build, msrv, doc, coverage, a11y) now fetches and verifies that archive before any cargo run, and so does the fast leg's closing target sweep. The native matrix job gains a verified-archive step and a witness step that reads red unless a skia-bindings build read the verified file.
FORCE_SKIA_BINARIES_DOWNLOAD was dropped: measurement showed it forces a full source build in a crates.io build. The stated fallback is now a limit: a failed read or an unrecorded key falls back to a source build, which the witness reads red.
The lock moved the yanked async-executor 1.14.0 to 1.14.1. The dependency-audit ignores keep the same six IDs, and each reason now names the release it waits on or its owning residual row. The zip ignores are closed.
After implement, the witness was rewritten to read skia-bindings' build tree by content, because Cargo 1.100 on the runners moved build output to run/stdout. CI then read green 17/17 on the final head, and the cancelled-runner watch was retired by its rule.
@task_file n=1 path=.github/skia-archives.toml
what: new: the committed record of skia's prebuilt archives, holding the release tag, the release URL template and each built target's key and sha256
@task_file n=2 path=.github/scripts/skia_archive.py
what: new: the CI Python script that fetches and verifies one target's skia archive and prints the SKIA_BINARIES_URL template, with exit codes 0, 1 and 2 and fixed stderr lines
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: new: unittest tests for the archive script, found by the ci-scripts leg's unittest discovery
@task_file n=4 path=.github/scripts/ci-leg.sh
what: edited: the CI leg script that every leg runs through; it gains the list of skia-building legs and their verified-archive arm, and the fast leg's target sweep now runs under the verified URL
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: edited: the unittest pins on the CI workflow and the leg script; new cases cover the matrix's verified-archive and witness steps, the skia legs, a refused archive ending the leg, and the fast leg's sweep (edited again after implement for the witness's content read)
@task_file n=6 path=.github/workflows/ci.yml
what: edited: the GitHub Actions CI workflow; the native platform matrix test job gains a verified-archive step after setup and a witness step after test, and the witness was rewritten after implement to read the build tree by content
@task_file n=7 path=deny.toml
what: edited: the dependency audit's advisory and license config; only ignore reasons changed, the six ignored IDs are unchanged
@task_file n=8 path=Cargo.lock
what: edited: the workspace lock that the cargo legs run locked against; only async-executor moved, from yanked 1.14.0 to 1.14.1
@task_item n=1 part=touches
CI legs run through the leg script: the leg script gains a skia arm. For seven legs it fetches and verifies the recorded archive first and exports SKIA_BINARIES_URL, and a refused archive ends the leg with a non-zero exit before any cargo runs.
@task_item n=2 part=touches
The CI leg script's exit-code contract: a leg can now also end non-zero because the archive script refused, before cargo. A row naming the leg script's exit codes may need that case.
@task_item n=3 part=touches
Native platform CI matrix: the matrix test job gains two steps on every row. One is a verified-archive step that writes SKIA_BINARIES_URL into the job environment. The other is a witness step that exits 1 unless some skia-bindings build output names a file:// read and a successful install.
@task_item n=4 part=touches
CI Python scripts unit-tested, and Python script tests under unittest: a new CI script with its own unittest file (13 cases) joins the ci-scripts leg through discovery. Rows that describe the leg as discovery over the CI scripts folder stay true.
@task_item n=5 part=touches
Dependency audit (advisories and licenses): async-executor moved off its yanked release, the zip ignores are closed, and each of the six standing ignores now states the release it waits on or the residual row that owns it. The audit reads advisories ok and licenses ok with no yanked warning.
@task_item n=6 part=touches
Cargo legs run locked: the lock moved by one entry, and the legs still run locked against it.
@task_item n=7 part=touches
Build directory held by a sweep, and target sweep safe around a build: the fast leg's sweep now runs under the verified SKIA_BINARIES_URL. A cargo run on the shared target without that variable re-runs skia-bindings' build script.
@task_item n=8 part=touches
Workspace tested in the fast CI leg: the fast leg's closing sweep now fetches the verified archive first, and its failure is still swallowed by the sweep's own or-true.
@task_item n=9 part=touches
Minimum supported Rust version and MSRV verified by build: the msrv leg is a skia-building leg, so it now builds skia from the verified archive.
@task_item n=10 part=touches
Coverage leg gated by the floors, a11y leg membership and clippy legs: each of these legs now runs the verified-archive arm before cargo. No floor, membership or clippy scope moved.
@task_item n=11 part=touches
Host-reproducible CI legs, local pre-push gate and local baseline: on the dev host the leg runner now fetches the archive over the network into a folder under target and reuses a matching local copy, so a local leg needs the archive or its release.
@task_item n=12 part=touches
Bounded, retried package installs: the archive fetch is a new bounded network fetch in CI (120 s timeout, 3 attempts 10 s apart). A rule that bounds and retries installs may now cover it, or need to say that it does not.
@task_item n=13 part=touches
CI uses no secrets, read-only permissions, and actions pinned to commit SHAs: the new matrix steps add no action, permission or secret. SKIA_BINARIES_URL holds a local path template, not a secret.
@task_item n=14 part=touches
Leg log and CI log artifacts, and leg logs uploaded on failure: the archive script's fixed stderr lines now land in the leg log or the step log. On failure, the witness prints the build outputs it found and the skia-bindings build tree to the step log.
@task_item n=15 part=touches
Shared workspace test cache and shared CI build cache: verified archives are kept under the shared target, and a cargo run without the URL on that target invalidates skia-bindings' build script result.
@task_item n=16 part=touches
Tests controlled red-before-green: the controls on the archive sum check and the URL export held, and a learning recorded on 2026-11-14 did not recur: a gate control on a ci-scripts unittest case no longer reads INDETERMINATE.
@task_fact n=1 slug=9-ci-integration
fact: The leg script's skia-building legs (clippy, test, build, msrv, doc, coverage and a11y) each fetch and verify the recorded skia archive and run every cargo call, including the fast leg's target sweep, under SKIA_BINARIES_URL. A refused archive ends the leg before cargo, and legs that build no skia set no skia variable.
@task_fact n=2 slug=9-ci-integration
fact: Every row of the native platform matrix fetches and verifies its target's recorded skia archive after setup. After test, a witness step reads red unless a skia-bindings build output names a file:// read and DOWNLOAD AND INSTALL SUCCEEDED.
@task_fact n=3 slug=9-ci-integration
fact: Skia's prebuilt fallback is a stated limit. A failed archive read or a build asking for an unrecorded key falls back to a full source build of skia with no panic. The matrix witness reads it red on CI, and the dev host's file:// output check reads it red locally. FORCE_SKIA_BINARIES_DOWNLOAD is never set, because in a crates.io build it forces a source build.
@task_fact n=4 slug=9-ci-integration
fact: The skia archive witness reads skia-bindings' build tree by content and not by a pinned path. A CI toolchain ahead of the dev host moves the layout: Cargo 1.100 writes build-script output to run/stdout under a hashed folder, while Cargo 1.99 writes the output file.
@task_fact n=5 slug=ci-integration
fact: The skia archive script's exit codes: 0 prints one SKIA_BINARIES_URL template line; 1 means no verified archive after 3 attempts, with a fixed stderr line for each attempt; 2 means a bad call or an unrecorded target, with nothing fetched.
@task_fact n=6 slug=9-ci-integration
fact: A hosted-runner cancel with no runner is a hosted-runner limit, answered by a hand re-run. The cancelled-runner watch is retired by its rule after a 60-run census.
@task_item n=1 part=untouched
The harness contract rows (construction, core, settle, input, inspection, screenshot and contrast) and the seven_guis stand boot rows: no harness or stand code changed.
@task_item n=2 part=untouched
Driver, session, session wire, recording, coverage-role, lint and refusal-cause contracts, and the escher-session command line, MCP and exit-code rows: no Rust source changed.
@task_item n=3 part=untouched
The cold-agent and agent-run rows (events, verbs, exit codes, verdict, isolation and invocation): the agent-run tooling did not move.
@task_item n=4 part=untouched
WPT runner, conformance, pinned-commit and timeout rows: the WPT workflow and runner were not reached.
@task_item n=5 part=untouched
Quality-gate floors and budgets (per-crate coverage floors, mutation runs, the complexity ceiling, the zero-failure stand budget and oversize targets kept): no threshold or count in them moved.
@task_item n=6 part=untouched
Mocking and stubbing discipline, test data and fixtures, and e2e rows: no fixture, stand-in, timer or end-to-end check changed.
@task_item n=7 part=untouched
apt-install and coverage-floor script exit codes: those scripts did not change.
