@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (signing and PR-token jobs repository-guarded; CI legs locked) made `--locked` cover every ci.yml cargo leg (the leg script and the matrix) beside the flake and `dx bundle`, `examples/wasm_hello` excepted, with ci.yml referencing no `secrets.`, pinned by a unit test. This is the origin of decision/locked-builds: any mend that rewrites a leg's cargo invocation in `.github/scripts/ci-leg.sh` or ci.yml (one test binary per leg, a profile flag, a sweep step) must keep `--locked` and add no secret reference.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (cargo-deny audit leg, SHA-pinned ci.yml, a read-only ci.yml token) set ci.yml's workflow-level `permissions: contents: read` with no job-level grant (decision/ci-token-read-only), pinned every ci.yml `uses:` to a 40-hex SHA (decision/ci-actions-pinned-to-sha), installed cargo-llvm-cov 0.9.1 through `taiki-e/install-action` (decision/cargo-llvm-cov-coverage-tool) and extended `--locked` to the audit, a11y and coverage legs. A mend touching ci.yml (a sweep or cache step, a new action) must keep the read-only token and SHA pins; the coverage leg's own target subdirectory is one of the invocations this chunk measures.

@history_item n=3 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — (the agent-run argv boundary and harness log) added the `agent-run.sh` CLI row (usage exit 2 before any cargo call; selection only `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem) and recorded `target/agent-run/run.log` as raw cargo/libtest output, unscrubbed like `target/ci-logs/`, never printed, gitignored. Both rows bind this chunk: merging per-file test binaries changes what a "test-file stem" selects, and a sweep of `target/` touches these raw logs, which must stay unprinted and uncopied (decision/agent-run-raw-log-unprinted).

@history_item n=4 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — (the cold-agent login, input rows and log) registered the `cold-agent.sh` CLI row (usage exit 2 before any precondition or `claude` call), the operator's own Claude Code login as the pipe's only credential path (no env read, no CI secret), and the transcript as raw by design, never printed, gitignored. Constraint for measuring the cold-agent pipe's `target/` growth: measure the build side only, add no env read or secret, and never print the raw transcript or server log living under `target/cold-agent/`.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (input rows re-read on the merged engine) recorded that the `env!("CARGO_MANIFEST_DIR")` build-time read also sits in upstream's `tests/all.rs` of blitz-tests, a target held at `test = false` and never built. A precedent for this chunk's test layout: restructuring `tests/blitz-tests/tests/` into one binary per leg must keep `all.rs` unbuilt (or re-register its env read) and add no runtime env read.

@history_item n=6 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — (blitz-test-harness takes `color`, reviewed) recorded every dependency an escher crate takes as a reviewed Pinning bullet and restated that blitz-test-harness takes no `tracing` and no `style` and blitz-tests gains no dependency (decision/test-harness-no-tracing-no-style, decision/workspace-inherited-dependency-versions). A mend reshaping `tests/blitz-tests/Cargo.toml` or the harness to merge test binaries must add no dependency without that review and must not pull `tracing` or `style` into the harness.

@history_item n=7 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (the whole-workspace audit, the licence gate, the owed checks discharged) widened the audit to `cargo deny --locked --workspace --all-features` (decision/dependency-audit-cargo-deny), listed the `stand-repeat` leg and both clippy lines under Lockfile verification, added the `coverage_floor.py` row (malformed or repeated floor exit 2, prints no source or lcov path: decision/coverage-floor-input-validation), and registered `TMPDIR` as a test-only env read. Each of these legs is an invocation this chunk measures; a mend changing them must keep `--locked`, the coverage floors and their validation, and must add no env read beyond the registered ones.

@history_item n=8 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — (the cold-agent pipe on the escher server) set the task allowlist to `stand` (decision/cold-agent-call-validation), validated the agent-written check (≤ 65,536 bytes, ≤ 400 steps, closed keys; decision/cold-agent-check-validation), recorded that `server.log` and `check.json` are never printed, and added a precondition that `target/debug/escher-session` already exists (exit 3, the pipe builds nothing). Trap for a sweep: removing `target/debug/escher-session` or the pipe's state under `target/` breaks this precondition. The pipe itself adds no build, so its `target/` growth comes from the build step before it.

@history_item n=9 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the kept check at rest, its directory, its reader and `--record`) created the Driver recording directory rows: a launcher-named directory created `0700` or refused `RecordDirNotPrivate` with nothing written or spawned, `check-{n}.json` written `0600` with `create_new` and never replacing a file, a file the recorder did not create never removed, unix only, the directory named by no tool call or field and never logged. These are the origin of decision/recording-dir-owner-only, decision/recording-directory-owner-only, decision/recording-dir-never-named and decision/kept-check-owner-only. A `target/` sweep must neither touch nor print a recording directory nor loosen its mode.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (wire v3, fifteen session errors, `call_of`'s home, serde_json and `CARGO_MANIFEST_DIR`) recorded that escher-driver's unit tests also read `CARGO_MANIFEST_DIR` at build time to reach the workspace `target/tmp`. Test scratch therefore lives inside `target/`: measure it as part of the growth, and keep it working under any sweep or `.cargo/config.toml` target-dir change.

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (each kept answer whole at rest, the committed checks, the replay) admitted `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` as the only public exception to owner-only kept checks, written by `record_checks.sh` and compared byte for byte with a fresh recording by `stand_replay` (the `except` of decision/kept-check-owner-only and decision/kept-checks-owner-only). A layout change to `tests/blitz-tests` (merging test files into one binary per leg) must keep these committed checks, their reader and that comparison intact.

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the live child's kept checks and their second reader, the paged decode) made the live server child record with `--record target/cold-agent/record` (`mkdir -m 700`), the pipe's `cleanup` removing it with its state and the replays running without `--record`. The pipe reads those checks for bools and counts only, reads an unreadable directory as no check kept, and never prints, copies or commits them. Those checks live under `target/`, so a sweep of stale build hashes must not reach `target/cold-agent/record/` or must remove it only as cleanup does: never reading, printing or copying it (decision/cold-agent-checks-read-for-counts-only, decision/cold-agent-records-private, decision/cold-agent-recordings-owner-only).

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`TMPDIR` read at a second test-only site) recorded `TMPDIR` read through `std::env::temp_dir()` at two test-only sites, the escher-driver client test and blitz-vibey-script's `temp_dir_with_files`, against which every env read is measured. This is the register the scope's "no new env read" boundary is checked against: a sweep script or a cargo-config change must add no env read outside it.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the free-disk action at its two CI sites) added `jlumbroso/free-disk-space@54081f13…` to `Stand checks repeated` (SHA-pinned, a CI action site and not a crate) after that leg ran out of runner disk (`Free space left: 5 MB`), recorded as not shown to be the cure, with the matrix_test copy skipped on every row. This is the direct precedent for build-directory growth hitting CI. A mend that shrinks build output may make this step unnecessary, but any change to it goes through the SHA pin and `test_ci_workflows.py`.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the citations the chunk's edits moved) re-anchored citations after edits to `ci.yml`, `ci-leg.sh` and `test_ci_workflows.py`, the master moving by citations alone. Trap: editing the CI leg script or workflow shifts the security rows' cited lines and guards, which this chunk's wrap must re-point.

@history_item n=16 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `matrix_test` free-disk step retired; the `embolden` leg, locked) removed the skipped free-disk step from `matrix_test`, `test_only_stand_repeat_frees_disk` pinning the action to `Stand checks repeated`. It also added the `embolden` leg: `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, in `LEGS` and last in `FAST_LEGS`, with no CI job of its own (decision/embolden-locked-fast-leg). That leg is one of the feature sets this chunk suspects of minting a second hash, and its `--test` file-stem form depends on the per-file binary layout a mend might merge.

@history_item n=17 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (advisories published after a green audit) set the standing rule that the local `fast` leg runs no audit and `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push. When this chunk measures local CI legs and pushes a leg change, the audit leg is run locally first, and any sweep must not disturb what it reads.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (the masters as rows) turned the security master into rows verified against the code, guards re-pointed. This is why the picked rows (locked builds, SHA pins, CI token, coverage floors, agent-run allowlist, recording directory) carry guards such as `test_ci_workflows.py`, the coverage leg and `CoverageFloorTest`, which a CI-pipeline change in this chunk must keep green.

@history_item n=19 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Dependency Security: 1 row) re-pointed decision/ci-actions-pinned-to-sha's guard to `WorkflowPinTest in test_ci_workflows.py, reading every workflow file`. Any action added or changed by a mend (a cache or sweep step) in any workflow is checked by that test.

@history_item n=20 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (Data Protection: 3 rows) widened contract/kept-check-file/medium to `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `0600` `check-{n}.json` per session, each explored path its own, in a `0700` directory the launcher named. Exploration runs in the test legs may write such directories. A sweep or layout change must leave their modes and lifecycle (contract/kept-check-file/lifecycle: never removed by escher but for its own check) intact.

@history_item n=21 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Dependency Security: 3 rows) gave decision/no-git-dependency its Taffy-only exception and amended decision/upstream-sync-dependency-set. Bears on the picked no-git-dependency row: a mend must not change the git source or the lock, and the Taffy git pin still has to pass the lock, audit and MSRV legs this chunk measures.

@history_item n=22 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — (Dependency Security: 4 rows) took the exception: Taffy is a git pin at a full commit rev with no patch section and no lock checksum, and every other crate is a registry version. This is the current wording of decision/no-git-dependency picked for this chunk. The committed `Cargo.lock` stays an invariant (scope boundary), so the mend must not touch it.

@history_item n=23 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — (Input Validation: 2 rows) amended decision/agent-run-selection-allowlist and contract/agent-run-cli/2 to admit the fixed word `session`, matched before the file-stem arm, beside `stand`, `all` and an existing test-file stem. This is the current allowlist: if a mend merges per-file test binaries, the file-stem arm's meaning changes and these rows must be amended in step, still exiting 2 before any cargo call.

@history_item n=24 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (Dependency Security: 1 row, the retired row) retired decision/free-disk-only-on-stand-repeat because the default-features test job now frees runner disk too, and its guard was renamed, the rule's home moving to the test plan's CI integration. This is the latest state of the CI disk-pressure workaround that this chunk's build-directory mend may bear on: it now sits in test-plan, not security.
