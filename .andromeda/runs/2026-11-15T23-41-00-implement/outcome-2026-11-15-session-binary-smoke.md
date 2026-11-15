# Outcome — 2026-11-15-session-binary-smoke

**Outcome:** green + smoke (1 iteration) · run `2026-11-15T23-41-00-implement` · HEAD `4263cec1f`, tree uncommitted

## Files changed
- `scripts/agent-run.sh` — `session)` selection arm (files `cli_flow`, package `seven_guis`), the run's package
  variable `package` (default `blitz-tests`, so `stand`, `all` and a file stem call what they called), the cargo call
  `cargo test -p "$package" --locked …`, the usage line `run selections: stand · all · session · <blitz-tests file
  name>`, the header comment naming the session binary's flows.
- `.github/scripts/test_agent_run.py` — constants `SESSION_FLOW_CALL`, `PASSING_FLOWS`, `FAILING_FLOWS`, `EMPTY_FLOWS`;
  six cases appended after the fourteen standing ones.

## Gates (gate.py run, trail `.andromeda/runs/2026-11-15T23-41-00-implement/gate-2026-11-15-session-binary-smoke.json`)
- `python3 -m unittest discover -s .github/scripts -p test_agent_run.py -v` — green (Ran 20 tests, OK, the six new `... ok`)
- `bash .github/scripts/ci-leg.sh ci-scripts` — green (Ran 118 tests, OK; predicted 112 → 118)
- `bash scripts/agent-run.sh boot` — green (`"outcome": "ready", "cargo_exit": 0`, head `4263cec1f…`)
- `bash scripts/agent-run.sh run session` — green, the `role = 'smoke'` entry: `run.start` selection `session` files
  `["cli_flow"]`, two `test` events `cli_flow` ok, `run.end` passed 2 · failed 0 · ignored 0 · cargo_exit 0 · passed
- `bash scripts/agent-run.sh status` — green (`"run": {"selection": "session", "state": "passed", "passed": 2, …}`)
- `bash .github/scripts/ci-leg.sh fast` — green (exit 0, 294.51 s)
- the three `operator` entries (hygiene · push · CI read) — `not run — leg operator`: the operator pass's
- Summary: `entries 9 · green 6 · red 0 · recorded 0 · timeout 0 · not-run 3`
- No `gate.py guard` entry in the block. No deferral (no `defer` entry; every changed-surface gate ran).

## Red before green (plan step 1)
`.andromeda/cache/chunks/2026-11-15-session-binary-smoke/evidence/red-before-green.md` — against HEAD's unchanged
script: Ran 20 tests, FAILED (failures=5); five of the six new cases red; `test_run_session_with_an_extra_argument_exits_2`
green by the standing argument-count gate (the plan allowed it); `test_run_session_before_boot_exits_3` read RED
(exit 2: at HEAD `session` names no file, so usage decides first) where the plan allowed it green.

## Smoke (P3)
self-verify / smoke ran as a P2 gate ✓ (entry `bash scripts/agent-run.sh run session`, role smoke) — not re-run.

## Matrix
No `chunk == 2026-11-15-session-binary-smoke` capability (coverage query: 0 rows; the plan claims none) — no ref written.

## Deviations
- Two constants beyond the plan's named two (`PASSING_FLOWS`, `FAILING_FLOWS`): `EMPTY_FLOWS` (the empty-run case's
  canned `running 0 tests` output for `cli_flow`, the off-unix shape) and `SESSION_FLOW_CALL` (the pinned argv, read
  by two cases) — inside the listed file, serving step 1's cases.
- The header comment (`agent-run.sh:2`) rewrapped over five lines to keep the 120-column width.
- scope record: none — `gate.py scope` clean, changed 2 · listed 2 · recorded 0.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started outside the gate tool's entries | — | `gate.py census --comm escher-session --comm cargo --comm cli_flow --args agent-run.sh`: `census: none running` |

## Decisions & corrections
none

## Next
The OPERATOR PASS — the plan's `operator` entries: the hygiene entry (`gate.py hygiene`), the push entry
(`git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.3.0`) and the CI read
(`ci.py conclusion --sha HEAD --wait 2400`), its record `{work_dir}/evidence/operator-pass.md` — then
`/andromeda-wrap-session`.
