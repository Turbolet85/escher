# obs extract

## Relevance
partial — the chunk adds no instrumentation (obs tier 0, per obs-plan §1; no span, metric, log field or sink is asked for), but its two items each sit against a recorded obs boundary: the new textarea check against the no-subscriber statement for stand checks (obs-plan §3 → Logging stack) and the typed-text readings (obs-plan §8 → Values logged as-is), and the CI install step against the fork-CI artifact table (obs-plan §9 → Telemetry artifact handling). obs-plan §10, §11 and §12 read `> NO RECORDED INTENT`: nothing is taken from them, so no timing budget for the install bound comes from this domain.

## Constraints
- The new check installs no `tracing` subscriber, reads no env var and holds no `println!`: obs-plan §3 → Logging stack requires that the headless stand, its `stand_*` checks and their shared modules install none, with `stand_act_spans`'s two re-run children the one named exception. Whether the fixture's host file (`stand_snapshot_state.rs` or a new file) satisfies this today is research's question.
- The check's result must not hinge on the engine's `tracing` feature: obs-plan §2 → Feature wiring records that a workspace-wide build turns `blitz-dom/tracing` on while `cargo test -p blitz-tests` leaves it off, so typing into the fixture runs with engine events compiled in under one runner and out under the other; with no sink installed neither writes anything (obs-plan §3 → Logging stack).
- The check is not a typed-text log measurement and is not to be recorded as one: obs-plan §8 → Values logged as-is states typed text is measured in process only, by `stand_act_spans` under a capture, and that typed text in a sink-installing host's log is NOT measured. A check that types with no sink installed leaves both statements as they stand.
- Only the check's file stem and test name reach a harness event: obs-plan §6 → Log format (the agent-run harness) requires one `test {file, test, outcome}` event per libtest result line and reads nothing between `failures:` and the next `test result:`, and obs-plan §8 → Scrubbing requires that the harness log carries no user content — so the test function name carries no typed text or control value, and an assertion message reaches `run.log` only.
- The install step's bound and retry add no artifact and widen no upload path: obs-plan §9 → Telemetry artifact handling requires the per-leg CI log to be uploaded on failure only, from `target/ci-logs/` alone, with the coverage report the one fork-CI artifact outside it.
- The per-leg log is `ci-leg.sh`'s: obs-plan §9 → Telemetry artifact handling records each leg's merged output written by `ci-leg.sh` to `target/ci-logs/{leg}.log`. Whether an install step stated once as a script writes there, or stays in the job's step log alone (leaving nothing for the failure upload when the install is what fails), is the plan's; what `ci-leg.sh` and the upload step do today on a failed install is research's question.

## Patterns to follow
- A stand check drives its instance in process and reads results into assertions only, with no sink — the form obs-plan §3 → Logging stack records for the `stand_*` checks and their shared module `tests/blitz-tests/tests/common/mod.rs`.
- Where a check does need the sink, it is installed in a re-run child over an in-memory capture (`init_with_writer`), the parent relaying a failure message and never a captured line (obs-plan §3 → Logging stack) — recorded here so it is not copied: this chunk's check has no log to read.
- CI output that is kept goes through one place, `target/ci-logs/`, truncated at the leg's start and uploaded on failure, kept 7 days (obs-plan §9 → Telemetry artifact handling); a step whose output is not kept stays in the job log.
- Unscrubbed build output is acceptable where it holds no user data (obs-plan §9 → Telemetry artifact handling, the per-leg CI log row; obs-plan §8 → Scrubbing, the agent-run `run.log` clause) — a package manager's output is of that class.

## Anti-patterns to avoid
- Installing `escher_telemetry::init` (or any subscriber), or adding an env read, in the textarea check or in a shared test module to observe the typing — it would falsify obs-plan §3 → Logging stack for every check that reads that module.
- Fielding, logging or printing the typed text or the value read back from the snapshot — obs-plan §8 → Scrubbing lists `text` and `value` in the content-named set, and obs-plan §3 → Logging stack requires that nothing prints, logs or fields a value the screen reads.
- A new `upload-artifact` step, or a wider path on the existing ones, to capture the install's output (obs-plan §9 → Telemetry artifact handling).

## Contract bindings
- obs ↔ tests harness: the new check surfaces as `test` events of `scripts/agent-run.sh` (obs-plan §3 → Agent-run harness log; obs-plan §6 → Log format (the agent-run harness)) — binds to test-plan §3; whether `run stand` selects a new `stand_*.rs` file is the tests domain's.
- obs ↔ tests runners: the feature difference between the workspace build and `-p blitz-tests` (obs-plan §2 → Feature wiring) binds to test-plan §9 → Engine features by runner.
- obs ↔ CI pipeline: obs-plan §9 → Telemetry artifact handling cites `ci.yml:48-54`, `ci.yml:286-291` and `ci.yml:391-400`, all at or below the first of the eight install steps; an in-place edit of those steps moves the cited lines — a citation-sweep item for the chunk's wrap, not a plan step.
- obs ↔ its own counts: obs-plan §3 → Logging stack counts the stand checks by module ("15 of which read the shared module", "fifteen of the sixteen checks that hold a session"); if the textarea check lands as a new file reading `common/mod.rs` the first count is amended at the wrap, and if it joins `stand_snapshot_state.rs` nothing changes.
- obs ↔ security: no binding opened — no log field, sink or scrub set changes (obs-plan §8 → Scrubbing stands unedited).

## Acceptance criteria contributions
- (obs) The file holding the textarea check, and any shared module it reads, contains no subscriber install, no env read and no `println!` / `eprintln!` (per obs-plan §3 → Logging stack)
- (obs) The check's test function name holds no typed text, id or control value, so its agent-run `test` event carries a file stem and a name only (per obs-plan §6 → Log format (the agent-run harness))
- (obs) The check reads the same in both layout modes under the workspace build (`ci-leg.sh fast`, engine `tracing` on) and under `cargo test -p blitz-tests` (off) (per obs-plan §2 → Feature wiring)
- (obs) After the `ci.yml` edit every failure upload still reads `target/ci-logs/` alone and no artifact is added — the coverage report remains the one fork-CI artifact outside it (per obs-plan §9 → Telemetry artifact handling)
