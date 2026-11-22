@task_prose part=session
The session paid dependency and CI debts with no Rust source change and every manifest byte-equal.
Skia's prebuilt archive is now fetched once per platform and verified against a committed sha256 record for the locked tag 0.157.0, and the skia build is pointed at the verified local copy through a SKIA_BINARIES_URL file:// template with a literal {key}.
The leg runner does this before cargo in its seven skia-building legs and before the fast leg's closing sweep, and the CI matrix job gains a verified-archive step and a witness step that fails a row whose skia build did not read the verified archive.
FORCE_SKIA_BINARIES_DOWNLOAD was measured to force a full source build in a crates.io build, so it is set nowhere. A failed read or an unrecorded key falls back to a source build with no panic, and the witness reads that red.
The cargo-deny ignore reasons were rewritten with the same six advisory IDs, each now naming the release it waits on or its owning residual row. The yanked async-executor 1.14.0 moved to 1.14.1 in the lockfile, so the audit reads no yanked warning.
CI went green on the final HEAD after two witness fixes, which read the build output by content because the runners' newer Cargo lays out the build directory differently.
@task_file n=1 path=.github/skia-archives.toml
what: new; the committed record of skia's prebuilt archives: the release tag, the release URL template, and one key and sha256 per built target (five targets).
@task_file n=2 path=.github/scripts/skia_archive.py
what: new; the build-time fetch and verify of skia's prebuilt archive: reads the record, fetches with bounded retries, refuses an archive whose sha256 differs and leaves no file, prints the SKIA_BINARIES_URL template, exits 0, 1 or 2 with fixed stderr lines, and reads no environment variable.
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: new; the unit tests of the skia archive fetch-and-verify script and of the committed archive record, run by the ci-scripts leg.
@task_file n=4 path=.github/scripts/ci-leg.sh
what: edited; the CI leg runner: it gains the list of skia-building legs and exports the verified SKIA_BINARIES_URL before cargo in those legs and before the fast leg's build-directory sweep, ending the leg when the archive is refused.
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: edited; the unit pins on the CI workflow and leg runner: the matrix's verified-archive and witness steps, the skia legs under the verified URL, a caller's URL overwritten, legs building no skia setting no skia variable, and the sweep under the verified archive.
@task_file n=6 path=.github/workflows/ci.yml
what: edited; the CI workflow's matrix test job: it gains a verified skia archive step that appends SKIA_BINARIES_URL to the job environment, and a witness step that fails the row unless the skia build's output read the verified file. No action, permission or secret was added.
@task_file n=7 path=deny.toml
what: edited; the cargo-deny audit config: the ignore reasons only, with the same six advisory IDs, each reason naming the release it waits on or the residual row that owns it.
@task_file n=8 path=Cargo.lock
what: edited; the lockfile: only async-executor moved, from the yanked 1.14.0 to 1.14.1 with its checksum.
@task_item n=1 part=touches
decision/skia-default-build-fetch is stale where it says the build-time fetch has no content check. The archive is now verified against a committed sha256 per target and served to the build as a local file:// {key} template. Its fallback is a limit: a failed read or an unrecorded key builds skia from source, with no panic.
@task_item n=2 part=touches
decision/dependency-audit-cargo-deny gains its memmap2 exception: advise_range is unused by cacache 13.1.0, a wait on a cacache release.
@task_item n=3 part=touches
decision/audit-ignores-per-id-with-reason and decision/advisory-ignores-by-id: the reasons were rewritten and the six IDs held. Each reason now states its wait: RUSTSEC-2026-0459 is owned by a residual row and drops when an upstream sync brings rusqlite 0.41 or later. RUSTSEC-2026-0192 waits on a winit 0.31 beta chain release that leaves ttf-parser 0.25.1. The zip ignores are gone.
@task_item n=4 part=touches
decision/dependency-audit-ci-gate and decision/local-audit-before-push: the audit leg now reads advisories ok and licenses ok, with no yanked warning, after the lock move.
@task_item n=5 part=touches
decision/locked-builds and decision/engine-dependency-pins: one lock entry moved (async-executor, yanked 1.14.0 to 1.14.1). Every manifest is byte-equal and no crate was added.
@task_item n=6 part=touches
decision/upstream-sync-dependency-set: two advisory waits are now tied to upstream syncs (rusqlite 0.41 or later, and the winit 0.31 beta chain past ttf-parser 0.25.1).
@task_item n=7 part=touches
The Secret Management environment-variable contract items: a new variable, SKIA_BINARIES_URL, is set by the leg runner and the CI matrix step and read by skia's build script. It holds no secret, only a local file:// path template. FORCE_SKIA_BINARIES_DOWNLOAD is set nowhere.
@task_item n=8 part=touches
decision/target-sweep-reads-no-env, indirectly through the leg runner: the build-directory sweep still reads no environment of its own, but the fast leg now runs it under the exported SKIA_BINARIES_URL.
@task_item n=9 part=touches
decision/ci-token-read-only, decision/ci-secrets-in-actions and decision/ci-secrets-in-github-actions, indirectly through the CI workflow: the matrix job gained two shell steps and no permission, secret or token use.
@task_item n=10 part=touches
decision/ci-actions-pinned-to-sha, indirectly through the CI workflow: the two new matrix steps are run steps, and no action was added.
@task_item n=11 part=touches
decision/ci-matrix-runs-every-job: every matrix row now runs a verified-archive step after setup and a witness step after its tests. The witness exits 1 when no skia build output on the row read the verified file.
@task_item n=12 part=touches
decision/no-git-dependencies and decision/no-git-dependency, indirectly: the skia archive is a release asset fetched by URL and pinned by sha256, not a git dependency.
@task_item n=13 part=touches
The Dependency Security overview's supply-chain integrity line: an external build-time artifact is now integrity-checked against a committed record, while SBOM generation and image scanning stay unmeasured.
@task_fact n=1 slug=dependency-security
fact: Skia's prebuilt archive for the locked skia-bindings tag is fetched once per platform and verified against a committed per-target sha256 record. An archive whose sum differs is refused and leaves no file, and the build reads only the verified local copy through a file:// {key} template.
@task_fact n=2 slug=dependency-security
fact: The skia archive fallback is a stated limit: a failed read of the archive, or a build asking for an unrecorded key, falls back to a full source build of skia with no panic. The CI matrix witness step, and the dev host's gate, read that red.
@task_fact n=3 slug=dependency-security
fact: FORCE_SKIA_BINARIES_DOWNLOAD is not used: in a crates.io build it routes skia-bindings to a full source build instead of a forced download.
@task_fact n=4 slug=dependency-security
fact: Every cargo-deny advisory ignore states its outcome in its reason: the upstream release it waits on, or the residual row that owns it.
@task_fact n=5 slug=secret-management
fact: SKIA_BINARIES_URL is set by the CI leg runner, only in the skia-building legs and before the fast leg's sweep, and by the CI matrix step, and is read by skia's build script. It holds no secret, a local file:// path template, and the leg runner overwrites any value a caller set.
@task_fact n=6 slug=input-validation
fact: The skia archive script takes exactly a target and a directory. It exits 2 with a fixed line on a bad call or an unrecorded target before any fetch, exits 1 with fixed attempt lines when no verified archive is obtained, and reads no environment variable.
@task_item n=1 part=untouched
The driver session, its socket, the MCP server and the escher-session command line: no verb, reply, refusal cause, exit code, bound or label changed.
@task_item n=2 part=untouched
Page-content input validation: HTML, CSS, select lists, password masking, range inputs, stable element ids, accessibility crossings and focus rules are untouched. No Rust source changed.
@task_item n=3 part=untouched
Data Protection: history, cookies, the HTTP cache, TLS through rustls, kept check files and cold-agent recordings did not move.
@task_item n=4 part=untouched
API Security's outbound requests, load waits, per-host limits, the CDP devtools server and the served-network-API absence: no runtime network path changed. The only new fetch is a build-time CI fetch of a release asset.
@task_item n=5 part=untouched
Logging and Monitoring: the telemetry sink, the log target allowlists, the span lines and the cold-agent and agent-run logs did not change.
@task_item n=6 part=untouched
Error Handling's refusal causes, session error tables, MCP protocol errors and WPT runner results did not change.
@task_item n=7 part=untouched
Authentication and authorization absences, the cookie feature and the threat-model crossing decisions are not reached.
@task_item n=8 part=untouched
The rows on the color, png, serde_json, base64 and coverage-tool crate choices: no crate was added, removed or re-chosen.
@task_item n=9 part=untouched
The WPT workflows, the coverage floors and the cold-agent and agent-run harness scripts were not changed.
