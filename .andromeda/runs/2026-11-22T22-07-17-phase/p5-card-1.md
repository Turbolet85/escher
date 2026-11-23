# P5 review — 2026-11-22-upstream-sync-ahead-of-the-canvas «Upstream sync ahead of the canvas»

The plan: `doc:2026-11-22-upstream-sync-ahead-of-the-canvas/plan` rev 3, rendered for an editor at
`.andromeda/cache/docs/2026-11-22-upstream-sync-ahead-of-the-canvas/plan.md`. No plan stands in the chunk's folder;
writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-upstream-sync-ahead-of-the-canvas`.

## Goal
A two-parent merge of `upstream/main` at `165d698c31d00b9614664dddfbb865ff030948b3` (10 commits past the last sync's
`547d7514b`) into `build/escher-0.4.0`, every fork as the founder ruled it (inputs#I2): E1–E6 taken, Dioxus held at
0.7.3 (D1, L9), L1–L8 kept out. Two conflicts — `wpt.yml` (our SHA pin kept beside upstream's `artifact_name`) and
`AGENTS.md` (deleted) — and no manifest or lock move. Plus CARRY `cry-rqochxka`: `deny.toml`'s RUSTSEC-2026-0192 reason
names `res-y4gizreq`.

## Top acceptance criteria
- HEAD after the pre-CI commit is the two-parent merge, second parent `165d698c3…`; no conflict marker stands.
- The rulings hold in the tree: no `AGENTS.md`, no `packages/parley_code_editor`, `Cargo.toml` and `Cargo.lock` equal
  to `31c8155fc`'s.
- `ci-leg.sh build`, `fast`, `doc`, `a11y`, `audit` exit 0; `stand-repeat` prints `stand-repeat: 3 of 3 rounds green`.
- `evidence/lock.md` names each of the 10 commits taken or left out, and each of the last sync's 14 left-out commits
  with this take's ruling; `evidence/readings.md` the eight residuals; `evidence/sink.md` the census.
- CI on the pushed merge reads `verdict: green`, its run id recorded.

## Touchpoints
- Hand-edited: `.github/workflows/wpt.yml` (the conflict), `deny.toml` (the CARRY).
- Merged clean, not edited: 17 files across blitz-dom layout, blitz-paint text, stylo_taffy, blitz-vibey-script,
  blitz-tests, `wpt/WPT_COMMIT`, the two WPT scripts; new `tests/blitz-tests/tests/ime_preedit_underline.rs`.
- Arriving and removed (on no list): `AGENTS.md`, `packages/parley_code_editor/AGENTS.md`.

## Test commands (15 entries)
1–4 merge-shape probes · 5 `build` · 6 `fast` · 7 `doc` · 8 `a11y` · 9 `audit` · 10 `stand-repeat` · 11 the CARRY's
probe · 12–15 operator: hygiene, push (clean-tree), `ci.py conclusion --wait 2700`, the second-parent read.
Baselines at `31c8155fc`: 2–10 green (`fast` 684 s, `stand-repeat` 979 s); 1 and 11 red as predicted, each with its
must-pass reading green in a scratch clone; 2, 3 and 4 each with a known positive red there
(`.andromeda/runs/2026-11-22T22-07-17-phase/controls.md`).

## Capabilities
None claimed. `v040-02` stays pooled — three later markerless entries carry it («Upstream sync ahead of inside the
canvas», «… drag, pan and zoom», «… polish and ship»); after the word a dated notes line records it, as at the last
sync. No concretization preview.

## Validation
Mechanical checks clean (planlint 0 hits; fence parsed; lists parsed, 20 items; snapshots verified). Validation-1:
aligned. The rows files (382,851 B) were read by search, past the 250,000 B whole-read bound. Graph not queried (the
two signature changes swept by name, the callers proven by the `--locked` build) — check 7's soft warning.

## Beyond the plan
- A route freight row: CARRY onto «Upstream sync ahead of inside the canvas» — re-read the Dioxus hold (D1) at that
  sync, or at the first sync after a Dioxus 0.8 release: `res-xpkmxnvl`, `res-xchmbfjr` and the `keyboard_compat`
  coverage exclusion, as `cry-ikyyzjzj` asked of this one (its re-read here: research §Dioxus 0.8 at the take-up;
  the measured move, founder-forks.md M2).

Apply? (yes / review / cancel)
