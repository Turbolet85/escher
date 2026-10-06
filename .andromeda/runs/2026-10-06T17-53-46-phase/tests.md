# tests extract

## Relevance
partial — a measured no-op sync with zero source delta adds no test code; the tests domain contributes only the gate commands and the green-evidence contract.

## Constraints
- The chunk's proof is that our tests and CI survived on an unchanged tree: test-plan §9 (Pipeline facts, Local pre-push gate) makes `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) the local gate, run as CI runs it; the host-reproducible legs are the same script (per test-plan §9 Legs).
- The `doc` leg (`ci-leg.sh doc`, `cargo doc --workspace --no-deps --locked`, `-D warnings`) is a separate gate from `fast`; test-plan §9 (Legs; Local baseline) lists it as its own leg, and the project's done-rule requires both (per test-plan §9 Legs).
- Every cargo leg runs `--locked`, and each leg's merged output lands in `target/ci-logs/{leg}.log`, truncated at the leg's start. The log is the evidence file for a gate verdict (per test-plan §9 Failure logs).
- The workspace test count is the baseline a no-op must hold. test-plan §9 (Local baseline) records the last re-count as 454 passed · 0 failed · 5 ignored at 2026-10-06-accessibility-tree-identity. A tree with zero source delta must reproduce that count, and any difference needs an explanation. Whether the tree on disk still reproduces it is the implementation's measurement, not something this plan attests.
- The CI-scripts leg is part of `fast`. test-plan §4 (CI workflows and leg script; Agent-run contract; Cold-agent pipe) records it at 64 tests (`Ran 64 tests … OK`) at the last measurement. It runs test_ci_workflows.py, test_agent_run.py and test_cold_agent.py, with no live model and no credential (per test-plan §4).
- The fork CI verdict is read from the fork, not upstream. test-plan §9 (Platform) has CI run on pushes to `build/**`, and the project's session learning requires `-R Turbolet85/escher` on `gh run` reads. The scope already cites CI#37495205882 (16/16) for HEAD `42b80ad9`.
- The windows/macos/ios/android matrix legs need all four fast jobs (per test-plan §9 Fast/slow split), and the Linux host cannot reproduce them. Any claim about them rests on the CI run, not a local gate.

## Patterns to follow
- Record gate evidence as the merged leg log plus counts re-read from it (result lines, passed · failed · ignored), following the "as measured at {chunk evidence path}" form that test-plan §9 (Local baseline) uses for every re-count.
- If agent-driven proof is wanted, use `bash scripts/agent-run.sh boot` then `run stand`. test-plan §3 (Agent-run contract, Proof) records 25 `ok` stand events at the last count, with an empty run never a pass (exit 1). Treat it as an optional cross-check next to `ci-leg.sh fast`, not a substitute.
- A previous sync (2026-10-06-upstream-sync-element-identity) re-counted the baseline from its post-merge `ci-leg.sh fast` run (per test-plan §9 Local baseline). Mirror that shape here, with the count unchanged instead of +1.

## Anti-patterns to avoid
- Do not report a pass from an empty or unparsed run: test-plan §3 (`run {selection}`) says the outcome is `passed` only when cargo exited 0, no test failed and at least one test line parsed. Apply the same standard to the gate evidence, so a leg that wrote no log or no result lines is not green.
- Do not make the cold-agent live run part of this chunk's gate or of CI: test-plan §3 (Cold-agent run pipe, Proof) says the live run is never part of CI (no CI secret) and was budgeted once by the founder.
- Do not add or edit test files for a no-op sync. The ruling is that no source is edited, and test-plan §11 records no anti-pattern list to cite beyond the above.

## Contract bindings
- Harness status and log format to obs-plan §3: test-plan §3 (Stand log format; Agent-run contract Events) says `target/agent-run/` events and the stand's stderr log are harness metadata bound to obs-plan §3. An unchanged tree must leave `telemetry_stdout_silent`, `telemetry_scrub`, `telemetry_panic_hook` and `telemetry_init_idempotent` green inside the workspace test leg.
- A11y CI gate to a11y-plan: the `a11y` leg (`accessibility_hidden`, `accessibility_roles`, `focusability_updates`) and the accessibility-tree files `stand_accessibility_ids` and `accessibility_names` run inside the workspace test leg (per test-plan §9 Legs; §1). They are the same-workflow evidence that our a11y logic survived.
- Security CI gate: the `audit` leg (`cargo deny check advisories`) is a CI job on the same workflow (per test-plan §9 Legs). With no lock change it needs no re-run beyond the CI verdict.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` exits 0 on the unchanged tree, and `target/ci-logs/` shows fmt, clippy, test and ci-scripts all green (per test-plan §9 Local pre-push gate).
- `bash .github/scripts/ci-leg.sh doc` exits 0 with `-D warnings` over the whole workspace (per test-plan §9 Legs).
- The workspace test result matches the recorded baseline of 454 passed · 0 failed · 5 ignored, and the CI-scripts leg reports `Ran 64 tests … OK`. Because the delta is zero, any difference from that baseline is a finding to explain (per test-plan §9 Local baseline; §4).
- The fork's CI run on the final HEAD is green on all 16 checks, read with `gh run … -R Turbolet85/escher` (per test-plan §9 Platform; Pipeline facts).
