@task_prose part=session
The session paid dependency and CI debts with no Rust source change. Every manifest is byte-equal, and the lock moved one yanked entry, async-executor 1.14.0 to 1.14.1.
It added a verified prebuilt skia archive. A committed record holds the release tag 0.157.0, the release URL template and a key and sha256 for five targets. A stdlib Python script fetches the archive once per platform (3 attempts 10 s apart, 120 s timeout) and checks it against the record. It then prints a SKIA_BINARIES_URL file:// template with {key} left for skia-bindings to fill.
The CI leg runner gained a skia arm: the seven skia-building legs (clippy test build msrv doc coverage a11y) take the verified archive before any cargo runs, and the fast leg's closing target sweep now runs under it. The matrix test job gained two steps, the verified-archive step and a witness that reads skia-bindings' build tree by content.
Measured: FORCE_SKIA_BINARIES_DOWNLOAD forces a full source build in a crates.io build, so it is set nowhere. A failed read or an unrecorded key falls back to a source build, which the witness reads red. Cargo 1.100 on the CI runners moves the build-script output to build/skia-bindings/{hash}/run/stdout, while Cargo 1.99 keeps build/skia-bindings-{hash}/output.
The cargo-deny ignore reasons were rewritten so each names the release it waits on. The six IDs are unchanged and the zip ignores are gone. The cancelled-runner watch was retired by its rule. CI went green 17/17 on the final HEAD.
@task_file n=1 path=.github/skia-archives.toml
what: added; the committed record of skia's prebuilt binary archives: release tag, release URL template, and per target triple the key and sha256 (five targets)
@task_file n=2 path=.github/scripts/skia_archive.py
what: added; the CI helper script that fetches and verifies skia's prebuilt archive for one target and prints the SKIA_BINARIES_URL template, with exit codes 0, 1 and 2 and fixed stderr lines
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: added; the unittest suite of the skia archive script, run by the ci-scripts leg's discovery
@task_file n=4 path=.github/scripts/ci-leg.sh
what: edited; the one CI leg runner: its legs, its skia arm over the skia-building legs, and the fast leg's closing target sweep
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: edited; the tests that pin the CI workflow contract and the leg runner's behaviour (skia legs, the matrix's verified-archive and witness steps, the sweep under the verified URL)
@task_file n=6 path=.github/workflows/ci.yml
what: edited; the GitHub Actions CI workflow, its matrix test job across linux, macos, windows, ios and android rows, gaining the verified skia archive step and the witness step
@task_file n=7 path=deny.toml
what: edited; the cargo-deny dependency and licence audit policy and its advisory ignore list with reasons
@task_file n=8 path=Cargo.lock
what: edited; the workspace lockfile that the locked cargo legs build against
@task_item n=1 part=touches
The ci-leg.sh legs contracts for clippy, test, build, msrv, doc, coverage and a11y: each now fetches and verifies skia's archive and exports SKIA_BINARIES_URL first. A refused archive ends the leg with a non-zero exit before any cargo runs.
@task_item n=2 part=touches
The fast leg's contract: its closing target sweep now runs under the verified skia URL. If the archive is refused, the sweep is skipped and the leg still returns 0.
@task_item n=3 part=touches
The ci-leg.sh invocation contracts (exit status, leg log): a new failure cause, the skia archive script's exit 1 or 2 and its fixed stderr lines, now appears in a leg's log and ends the leg.
@task_item n=4 part=touches
The one-CI-leg-runner decision: the runner gains a named list of skia-building legs and a skia arm. This belongs beside it.
@task_item n=5 part=touches
The decision that the CI workflow contract is pinned by tests gains new pins: the matrix builds skia from the verified archive on every row, the skia legs run under the verified URL, a caller's URL is overwritten, and FORCE_SKIA_BINARIES_DOWNLOAD stays unset.
@task_item n=6 part=touches
The CI helper scripts on the Python stdlib and the Python 3.11 floor for the ci-scripts leg: a new stdlib script relies on tomllib, which is what sets the 3.11 floor.
@task_item n=7 part=touches
The convention that CI Python tests use unittest: the new suite is unittest, 13 cases.
@task_item n=8 part=touches
The dependency and licence audit, the cargo-deny advisory gate and the workspace dependency audit: the ignore reasons are rewritten, each naming the release it waits on. The six IDs are unchanged, the zip ignores are gone, and the audit reads advisories ok and licenses ok with no yanked warning.
@task_item n=9 part=touches
Locked cargo legs and cargo legs run locked: the lockfile moved one entry, the yanked async-executor 1.14.0 to 1.14.1. No crate was added and no manifest changed.
@task_item n=10 part=touches
The accesskit_xplat async features on Unix and the decision that the driver graph holds no platform adapter, indirectly through the async-executor lock move. The stand's adapter probe still reads 0.
@task_item n=11 part=touches
The matrix test job's rules (read-only CI permissions, SHA-pinned actions, the Rust cache on compiling jobs): two steps were added to it with no action, permission or secret.
@task_item n=12 part=touches
Upstream CI resources reached only from upstream, and any occupied-resource rule on network reach at build time: CI and the dev host now fetch skia's prebuilt archive from the rust-skia skia-binaries GitHub release at tag 0.157.0, verified by sha256.
@task_item n=13 part=touches
The anyrender-over-Skia stack choice: Skia's binaries are now taken from a verified prebuilt archive pinned to the locked skia-bindings version. A failed read falls back to a full source build, which is stated as a limit.
@task_item n=14 part=touches
The target-sweep.py name and its exit-code contracts: the sweep is now called under use_verified_skia in the fast leg. Its own exit codes and its rule of reading no environment value of its own stay true.
@task_item n=15 part=touches
Occupied names under other: SKIA_BINARIES_URL is newly taken, set by the leg runner and the matrix step. FORCE_SKIA_BINARIES_DOWNLOAD is deliberately left unset. The rows naming CI and RUST_LOG sit beside it.
@task_item n=16 part=touches
The latest-stable-toolchain and MSRV policy decisions, and the MSRV job that only builds: the msrv leg now builds skia from the verified archive. The runners' toolchain (rustc 1.100) is ahead of the dev host (cargo 1.99) and moved Cargo's build-directory layout, which the witness reads by content.
@task_item n=17 part=touches
The a11y leg widening in place and the per-crate coverage floors: the a11y and coverage legs join the skia arm. No floor or threshold value moved.
@task_item n=18 part=touches
The decision that measured limits are stated where an agent reads the tool: the archive fallback to a source build is stated as a measured limit, read red by the witness step.
@task_item n=19 part=touches
The bounded, retried package install with repair before retry: the archive fetch follows the same bounded-retry shape (3 attempts, 10 s apart, a fixed attempt-failed stderr line). This belongs beside it.
@task_fact n=1 slug=standard-contracts
fact: The skia archive script takes TARGET DIR. Exit 0 prints one stdout line, the SKIA_BINARIES_URL template file://{DIR}/skia-binaries-{key}.tar.gz. Exit 1 prints skia-archive: attempt {n} of 3 failed in {fetch|verify}, then skia-archive: no verified archive. Exit 2 prints skia-archive: usage or skia-archive: no recorded archive for this target, with nothing fetched. A matching local copy is reused, and a fetched copy is renamed into place only when its sha256 equals the record.
@task_fact n=2 slug=standard-contracts
fact: The leg runner's SKIA_LEGS are clippy, test, build, msrv, doc, coverage and a11y. Each first verifies skia's archive for the rustc host into target/skia-archives and exports SKIA_BINARIES_URL, and a refused archive ends the leg before any cargo runs. The fast leg's closing target sweep runs under the same verified URL.
@task_fact n=3 slug=occupied-resources
fact: SKIA_BINARIES_URL is an occupied environment variable. The leg runner and the matrix test step set it, and skia-bindings' build script reads it. It holds a local file:// path template and no secret.
@task_fact n=4 slug=infrastructure-patterns
fact: Skia's prebuilt archive is fetched once per platform from the rust-skia skia-binaries release at the locked skia-bindings tag and verified against a committed sha256 per target (five targets). FORCE_SKIA_BINARIES_DOWNLOAD is never set, because in a crates.io build it forces a full source build. A failed read or an unrecorded key falls back to a source build of skia, a stated limit.
@task_fact n=5 slug=infrastructure-patterns
fact: The matrix test job verifies skia's archive after Setup and appends SKIA_BINARIES_URL to GITHUB_ENV. After the tests, a witness step reads every file under the target triple's skia-bindings build paths by content, with out/ pruned and only files under 1024 KiB. It passes only when a file holds both FROM: file:// and DOWNLOAD AND INSTALL SUCCEEDED, printing skia-archive: build output {path}:{line}. Otherwise it prints skia-archive: no skia-bindings build read the verified archive with the build tree and exits 1.
@task_fact n=6 slug=infrastructure-patterns
fact: Any cargo run on the shared target directory without SKIA_BINARIES_URL re-runs skia-bindings' build script, because the script declares rerun-if-env-changed. Every cargo call in a skia-building leg, its sweep included, must therefore run under the verified URL.
@task_fact n=7 slug=stack-and-technologies
fact: Cargo 1.100 (the CI runners) puts a package's build-script output at build/skia-bindings/{hash}/run/stdout, while Cargo 1.99 (the dev host) keeps build/skia-bindings-{hash}/output. A check of build output must not pin one layout.
@task_item n=1 part=untouched
Engine design and established decisions on layout, style, paint, text, scrolling, hit testing, select lists and forms: no Rust source changed.
@task_item n=2 part=untouched
The driver, session socket, MCP role, kept checks, snapshot, diff, UI graph and stable element id contracts: no carrier, verb or wire changed.
@task_item n=3 part=untouched
The accessibility tree contracts and AccessKit adapter contracts: no reading or adapter call changed, only a lock entry beneath them.
@task_item n=4 part=untouched
Telemetry bootstrap, scrub and line contracts, and the RUST_LOG rows: nothing in logging moved.
@task_item n=5 part=untouched
WPT runner contracts, result kinds and statuses, and the wpt diff PR tooling: not reached.
@task_item n=6 part=untouched
Crate and binary names, workspace package metadata, publishing and versioning decisions: no crate was added, renamed or published.
@task_item n=7 part=untouched
The cold-agent and agent-run scripts' events, exit codes and verdict contracts: not reached.
@task_item n=8 part=untouched
Project intent, the capability statement and the root README rows: no capability of the framework moved.
@task_item n=9 part=untouched
Browser app decisions (history, persistence, about pages, the publish workflow): not reached.
