# Tests validation — route draft

## Insert
- Between `Fast-failing mutants — …` and `Upstream sync ahead of engine debts — …`: **"Shared fixture bootstrap — fixture pages beside the stand and one held network stand-in, stated once in blitz-tests' shared modules, run by agent-run"** (epoch: `Epoch 1 — Foundation`)
  Reason: per test-plan §3 Agent-run contract ("NOT YET MEASURED — the test-data bootstrap mechanism of the 5-command contract") and §8, where the held network stand-in is repeated file-private (`render_blocking_stylesheet`, `stand_settle`, `font_face_sniffed_source`); Engine debts, Load wait, Select by id and Large screen in parts all prove on a fixture, so the fixture bootstrap has to come before them.

## Rewrite
- `Upstream sync ahead of …` (all four): "tests and CI prove our logic survived" → "full CI green and per-OS test tallies re-read against the last, no escher test lost"
  Reason: per §9 Per-OS test tallies, merges have changed test counts before (e.g. dioxus-native-dom 61 → 67), so "CI green" alone can hide a dropped or un-built target, as with §1's `all` target at `test = false`.
- `Recorded-check replay`: "runs in CI as ordinary test input" → "runs as ordinary test input under agent-run `run` and in CI's test and stand-repeat legs"
  Reason: per §3 the `run {selection}` grammar and per §10 Flakiness budget, any check outside the agent-run selections and the `stand-repeat` zero-failure leg cannot be run by the agent or held to the flake budget.
- `Visual lint`: "each by stable id with its measured reason" → "each by stable id with its measured reason; proven by a fixture planting each defect and a clean stand reading none"
  Reason: per §5/§6, every other Epoch 4 capability names how it is proven, but this one doesn't, so its six detectors have no stated positive or negative test.
- `Stand requirement sweep`: "proven headless by an agent on the stand or a fixture beside it" → "proven headless through agent-run on the stand or a fixture beside it, inside `run stand` and the stand-repeat leg; coverage floors held"
  Reason: per §3 `run stand` selects only `stand_*.rs` and per §10 the `stand-repeat` leg and the three per-crate coverage floors (`.github/scripts/coverage_floor.py`) gate only what they select, so a fixture check with another name would escape both.
- `Cold-agent bar`: "a fresh agent over 0.2.0's new capabilities" → "a fresh agent over 0.2.0's new capabilities through the cold-agent pipe, its task allowlist extended to their fixtures"
  Reason: per §3 Cold-agent run pipe, `scripts/cold-agent.sh` admits only `TASKS=(stand)`, so 0.2.0's fixture-borne capabilities are out of the pipe's reach unless the allowlist is extended.
