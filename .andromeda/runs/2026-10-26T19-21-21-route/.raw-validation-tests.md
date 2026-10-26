# Tests validation — route draft

## Rewrite
- `Upstream sync ahead of engine debts`: "CI and audit green, sink allowlist re-checked" → "CI and audit green, per-OS tallies re-read, sink allowlist re-checked"
  Reason: Per test-plan §9 "Per-OS test tallies", every upstream merge re-reads the linux/macOS/windows tallies, and this sync is the only one of the five that leaves that out.
- `What can I do here`: "actionable-key check if it fits (v030-06)" → "actionable-key check if it fits; capability-statement pin and README tool count kept green (v030-06)"
  Reason: Per test-plan §4 "Capability statement", `test_capability_statement.py` in the fast `ci-scripts` leg checks every driver verb and the verb count, so a new command turns CI red long before the Epoch 6 statement chunk unless the chunk that adds it updates the pin.
- `Handler coverage`: "read from the UI graph (v030-07)" → "read from the UI graph; capability-statement pin kept green (v030-07)"
  Reason: Same cause as above (test-plan §4 "Capability statement"): each new command changes the verb tables that `ci-scripts` checks.
- `Auto-exploration`: "each path a replayable check (v030-09)" → "each path a replayable check, no screenshot step, replayed green on every CI OS; capability-statement pin kept green (v030-09)"
  Reason: Per test-plan §2 "Pixel tests" and §6 "Command line", a shot's PNG bytes differ by OS, so recorded checks hold no `screenshot` step; the pin is needed for the same reason as the two chunks above.
- `Bundled fonts`: "two hosts read one state the same way (v030-14)" → "the linux leg and the windows/macos matrix legs read one state the same way, compared by state, never shot bytes (v030-14)"
  Reason: Per test-plan §9 "Per-OS test tallies" and §2 "Pixel tests", the CI matrix is where cross-host sameness can actually be shown, and shot bytes cannot be compared across OSes.
- `Stand requirement sweep`: "coverage floors held" → "per-crate coverage floors held, agent-run `run stand` and `run all` re-counted"
  Reason: Per test-plan §3 "Proof" and §10 "Coverage floors", this adopted project already has its harness, CI legs, fixtures and gates in place (so no bootstrap chunks are needed), but `run all` has not been re-counted since early in 0.2.0, so ship is where to re-read it.
- `Cold-agent bar`: "fresh agent over 0.3.0's new capabilities via the cold-agent pipe" → "cold-agent pipe widened past its twelve verbs, contract tests kept; fresh agent over 0.3.0's new capabilities"
  Reason: Per test-plan §3 "Cold-agent run pipe" and §4 "Cold-agent pipe", the pipe's step check and its `isolated` test accept exactly the twelve current verbs, so a run over new commands would fail `not-isolated` or `check-failed`.
