# P5 review — 2026-11-15-session-binary-smoke · «Session-binary smoke»

**The plan to read:** `.andromeda/cache/docs/2026-11-15-session-binary-smoke/plan.md` (rev 3, rendered from the
store; open it in an editor). No plan stands in the chunk's folder, and writing in that file changes no record until
a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-session-binary-smoke`
diffs the chunk's records.

## Goal
`bash scripts/agent-run.sh run session` becomes the agent-run contract's way into the session binary. One fixed
selection word, matched before the file-stem arm like `stand`/`all`, fires `cargo test -p seven_guis --locked --test
cli_flow`. That target's two `sh` flows boot the real `escher-session` on the Counter and the Flight Booker, then
read, act, refuse, stop, and leave nothing behind. A plan on the binary's call path then names `run session` as its
`role = 'smoke'` entry. That closes the Epoch 4 diagnosis's L1 (the smoke driven by hand six times). The shape and
the word are your answers to this phase's fork (inputs#I2). Five verbs, the same events, exits and state, no new
process of the script's own.

## Top acceptance criteria
- The contract cases pass: six new ones, `Ran 20 tests` · `OK`. Step 1 records them red against HEAD's script
  first.
- The word is closed and ordered: an extra argument exits 2, a run before boot exits 3 with no cargo call, and a
  blitz-tests `session.rs` does not divert it.
- A failing session flow carries no captured output, and an empty session run (what an off-unix host reads) is
  never a pass.
- `run session` after `boot` exits 0 with `run.end` `passed 2`, and `status` reads the `session` run `passed`.
- `fast` exits 0, and the operator's push reads CI `verdict: green`.

## Touchpoints
- `scripts/agent-run.sh` — the `session)` arm, the run's package variable (default `blitz-tests`), the usage line
  and header comment.
- `.github/scripts/test_agent_run.py` — two canned outputs and six cases.

## Test commands (9 entries)
1. unittest `test_agent_run.py -v` (unit) — baseline red: 14 tests, the new names absent.
2. `ci-leg.sh ci-scripts` (unit) — baseline green, 112 tests (predicted 118).
3. `agent-run.sh boot` (build) — baseline green, 72.5 s.
4. `agent-run.sh run session` (**smoke**) — baseline red, exit 2: the word is not a selection yet. Its atom forms
   were re-read from a real `run text_extent`.
5. `agent-run.sh status` (probe) — baseline red as a recency selector must be.
6. `ci-leg.sh fast` (lint) — seen green at the last wrap.
7–9. operator: hygiene · clean-tree push · `ci.py conclusion --wait 2400` (widened from 1800: the last wrap's CI wall
   was 1798 s).

The readings are in `.andromeda/runs/2026-11-15T23-03-09-phase/p5-controls.md`.

## Concretization preview
None. The pool's four unclaimed caps (`v030-10` · `v030-11` · `v030-12` · `v030-15`) each name a surface this chunk
does not build, so no capability is claimed.

## Facts beside the plan
- CI on the last wrap's commit `4263cec1f` now reads **green**, 17/17, CI#43771902054. It was in progress at take-up.
- No founder fork: the one fork was technical and yours (inputs#I1). Of the escalate patterns, «Boundary widening»
  was named in the question. A closed word in the harness's closed argv set is no new input class
  (security-plan · decision/boundary-widening-rule). `founder-forks.md` is not written.
- Expected amendments at the wrap: test-plan §3 (the `run` verb row, a `session` selection row, invocation, the
  `agent-run-holds-no-process` except, `overview/session-lifecycle`), security-plan §Input Validation (the selection
  allowlist and exit-2 rows), layout-templates §Surface: cli (the usage row), and architecture where it says
  agent-run drives blitz-tests alone. The harness files are the wrap's cascade.
- Limit, stated: `cli_flow` is `#![cfg(unix)]`, so off unix `run session` reads an empty run (exit 1). No CI leg runs
  `agent-run.sh` on windows or macOS.

## Beyond the plan
none

Apply? (yes / review / cancel)
