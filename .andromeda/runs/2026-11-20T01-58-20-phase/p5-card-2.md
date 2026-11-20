# P5 review — 2026-11-20-stand-test-setup-shared «Stand-test setup shared»

**Plan:** `.andromeda/cache/docs/2026-11-20-stand-test-setup-shared/plan.md` (rev 2, 223 lines) — open it in an editor.
No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-stand-test-setup-shared` is a diff of the chunk's records.
Research: `doc:2026-11-20-stand-test-setup-shared/research` (rev 2) · scope rev 3 · controls `.andromeda/runs/2026-11-20T01-58-20-phase/p5-controls.md`.

## The reading, by the audit's own tool
The audit's recipe (jscpd 5.4.0 + its `sum_dup.py`) reproduces the 0.3.0 ledger exactly on the `9464034d4` (3.80 %) and
`bc7661240` (3.89 %) archives. At this chunk's base `1b0125c10`:
- **whole:** pct **4.05** — 6,722 / 166,122 lines, 612 clones, test split 367 pairs / 4,180 lines.
- **the stand checks' own** (both sides in a `stand_*.rs` or a shared module): **75 pairs / 870 lines** — 47 / 550 at
  `9464034d4`, 75 / 870 at `bc7661240`, unchanged since.

**Your hypotheses, against the tree:**
- 3.80 → 3.81 → 3.89 % · test pairs 266 → 291 → 307: **held** (ledger and replay agree).
- Growth test-to-test: **held**. "All of it stand tests": **corrected**. From 0.3.0 Epoch 4 to Epoch 6 the stand
  checks were +28 of the +41 test pairs; seven_guis' tests were +7 and other blitz-tests +5.
- Since 3.89 the stand share has not moved. The rise to 4.05 is upstream's `blitz-vibey-cdp-server` and
  `blitz-vibey-script` tests (+55 pairs), which came in with the 2026-11-19 sync. That is the founder's fork F1
  (below); least change stands: no upstream file is touched.
- First candidate `stand_act_error_text.rs:804`: **held** at HEAD. It is `serve` + `tool_call`, cloned at
  `stand_act_errors.rs:348` and `stand_order_served.rs:252`. `tool_call` has 7 copies and `serve` 4.
- Helpers under `tests/…/common`: **leaned to the masters' membership rule.** Session-holding helpers go to
  `session_common` (test-plan · decision/session-checks-share-one-module) and harness-level ones to `common`.

## Goal and steps
Move each repeated helper whose copies are **byte-equal** into the shared module the masters assign it, and fold
byte-equal in-file repeats into private `#[track_caller]` helpers of their own file. The stand reading must end
**under 47 pairs and under 550 lines** (leaned per v040-08's acceptance, "the stand tests' duplication back under its
earlier reading"). Ten steps:
1. The MCP exchange goes to `session_common`: `tool_call`, `serve_lines`, `mcp_tool_result`.
2. Seven checks call it.
3. `stand_session_state` drops its private copy of `focused` and uses the shared one.
4. The served-screen session (`lean`, `hold_served`) moves to `session_common`.
5. The step holder `Held` moves to `session_common`.
6. The geometry readers move to `session_common`.
7. The actionable-element walk moves to `common`.
8. The kept-check recording moves to `session_common`, gated `cfg(unix)`.
9. In-file repeats in `stand_load_page`, `stand_replay`, `stand_mcp_tools` and `stand_act_transformed` become private
   helpers of their own file.
10. Measure.

Copies whose body or message differs stay where they are. The fold guard refuses any changed message.

**Touchpoints:** 21 files under `tests/blitz-tests/tests/`, no new file. Two checks gain a `mod session_common;`,
predicted 44 → 46 readers. Nothing outside that directory changes: no product source, manifest, lockfile, CI script
or seven_guis test.

## Top acceptance criteria and gates
| # | entry | baseline (untouched tree) |
|---|---|---|
| 1 | `ci-leg.sh fast` (fmt · clippy -D warnings · workspace tests · ci-scripts with BlitzTestsTargetsTest · embolden) | green, 548 s |
| 2 | `ci-leg.sh stand-repeat` — `3 of 3 rounds green` | green, 861 s |
| 3 | `ci-leg.sh a11y` | green, 46 s |
| 4 | `fold_guard.py 1b0125c10…` — same files, `#[test]` names, `#[ignore]` counts, string-literal set | green; control FAIL vs `9464034d4` |
| 5 | `git diff --quiet 1b0125c10… --` product/manifest/CI/seven_guis/checks paths | green; control exit 1 |
| 6 | jscpd recipe + `stand_reading.py … 47 550` — `stand reading bar: PASS` | **red, 75 / 870** (the subject built); controls below |
| 7–9 | operator: hygiene · push · `ci.py conclusion --wait 2700` | — (OS-sensitive: the `cfg(unix)` recording item) |

**The bar's controls** (corrected wording; card 1 was shown with a misworded summary). `stand_reading.py` holds the bar only when `pairs < 47 and lines < 550`.

At the plan's own bar of 47 / 550, on synthetic reports:

| reading | result |
|---|---|
| 46 / 549 | **PASS** |
| 47 / 550 | **FAIL** |
| 48 / 551 | **FAIL** |

The earlier controls in `p5-controls.md` hold the reading fixed at the real `9464034d4` report, 47 / 550, and vary the bar:

| bar | result |
|---|---|
| under 47 / 550 | FAIL |
| under 48 / 551 | PASS (47 / 550 is under 48 / 551) |

Neither control shows an inversion. The script is unchanged.

The two instruments (`fold_guard.py`, `stand_reading.py`) were written and controlled at P4. They live in the chunk's
gitignored `evidence/`. Predictions, recorded only when measured: stand reading 75 / 870 → about 40 / 330, whole
pct 4.05 → about 3.8.

## Capability
**v040-08 is not claimed.** Working-route lines 11–13 each name v040-08, and its requirement also covers the oversize
files, the complexity ceiling and the browser's mutants. This chunk advances only its duplication leg, so the cap
stays pooled with `chunk: null`. There is no concretization preview.

## Founder forks (`.andromeda/runs/2026-11-20T01-58-20-phase/founder-forks.md`)
- **F1 — the upstream-taken tests' duplication.** Options:
  - **A**: keep it as upstream's. This is the least-change option and stands meanwhile.
  - **B**: fold it in the fork, with a standing divergence at every sync.

  Nothing in the plan presumes B.

## CI since the last wrap
`1b0125c10`: CI#44656621940 **green**, 17/17 checks, 2,341 s wall. It was in progress at take-up and at P3.

## Checks
- Mechanical: all pass. Fence parses, lists parse (21 items), planlint 0 hits, inputs 1 cited, every modify path
  exists, the jscpd output is gitignored, the graph trace is present.
- Validation-1: **aligned** with scope rev 3, whose P3 closure corrected three premises.
- Escalate patterns: none applies (a test-only fold).

## Beyond the plan
| # | proposal | owner / place |
|---|---|---|
| 1 | `WATCH:` the whole duplication reading (4.05 % at `1b0125c10`, upstream's cdp-server and vibey-script tests 55 pairs of it) and the founder's F1 on keeping them, observed until F1 is answered | «Codebase health» (existing entry, Epoch 2), as a `WATCH:` annotation |
