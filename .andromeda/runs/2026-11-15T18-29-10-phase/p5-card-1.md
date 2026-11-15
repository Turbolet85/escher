# P5 review — 2026-11-15-taffy-from-upstream-git: "Taffy from upstream's git"

The plan to open in an editor: `.andromeda/cache/docs/2026-11-15-taffy-from-upstream-git/plan.md` (rev 3, 314 lines).
No plan stands in the chunk's folder. Writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-taffy-from-upstream-git`.

## Goal
Taffy moves from registry `0.15.1` to git Taffy pinned at `e90751fd` (the founder's F1 C, `inputs#I2`), by no-commit
cherry-pick of upstream's eight bumps `1a11761bc` … `ccfe15332`. That carries #1303 through one `place_floated_box`
line. The eight commits' layout changes are ratified for this take by his F2 «Ратифицировать взятое (Recommended)»,
on the condition that the gates, the stand, the a11y leg and CI stay green. Option A (`0980cce5f`, `ebacd68bd` and
the five later bumps) becomes its own entry after this chunk.

## Measured before planning (`evidence/measurement.md`)
- The pin: 8 of 8 picks apply, `Cargo.lock` the only conflict. 3 files change, +4 −4. The lock moves the `taffy`
  package alone.
- `fast` · `msrv` · `audit` · `doc` · `build` · `counter` · `wasm` · `a11y` · `stand-repeat` all exit 0 on the trial.
  The tallies equal HEAD's: 239 binaries · 1464 passed · 0 failed · 13 ignored; embolden 6; a11y 14 · 115.

## Top acceptance criteria
- The manifests move one line, the taffy line, to `rev = "e90751fd…"`. The lock holds exactly one git source, taffy
  at that rev, and moves only its three lines (diff count 5).
- The `place_floated_box` call passes `crate::taffy_node_id(node_id).into()` (#1303).
- The code delta is the three files.
- Green: `fast`; `run stand` (`"outcome": "passed"`); `stand-repeat: 3 of 3 rounds green`; `a11y`; `msrv`; `audit`
  (`advisories ok, licenses ok`); `doc`, `build`, `counter`, `wasm`; the windowed stand smoke.
- CI reads `verdict: green` on the pushed HEAD (the operator pass).

## Touchpoints
`Cargo.toml` · `Cargo.lock` · `packages/blitz-dom/src/layout/inline/layout.rs`

## Test commands
21 `[[gate]]` entries:
- 6 probes (manifest line count, one git source, the taffy rev, the lock delta, the #1303 line, the three-file delta);
- `fast`, then the agent-run `boot` and `run stand`, the windowed smoke, `msrv`, `audit`, `doc`, `build`, `counter`,
  `wasm`, `a11y`, `stand-repeat`;
- the operator's hygiene, push and `ci.py` read.

Baselines on the untouched tree: the 6 probes red at count 0, as predicted. Their must-pass (green on the pin) and
must-fail controls are in `.andromeda/runs/2026-11-15T18-29-10-phase/p5-controls.md`. The 11 legs that need a
baseline all read green.

## Capabilities
None claimed. The four unclaimed (`v030-10` · `v030-11` · `v030-12` · `v030-15`) name surfaces this chunk does not
build.

## Leans and checks to note
- **Lean:** security-plan · decision/upstream-sync-dependency-set («takes git-pinned content only by a lock-only,
  semver-compatible update of a pinned registry crate») reads as written for a registry crate. This take's git source
  is the Taffy-only exception's own subject (architecture · decision/no-git-dependencies, its except). So the wrap
  restates that rule with the Taffy except, rather than lifting a rule.
- **Expected amendments (wrap):**
  - architecture · decision/taffy-layout, decision/no-git-dependencies, decision/deferred-upstream-dependency-bumps,
    and decision/upstream-take-engine-changes (the eight commits ratified, the founder's F2);
  - security-plan · decision/engine-dependency-pins, decision/no-git-dependencies, and
    decision/upstream-sync-dependency-set.
- **Check 7 warning:** the code graph was not queried. The whole-workspace compile and test run on the trial reach
  every site, and `place_floated_box` has one caller.
- **Host fact:** the `wasm32-unknown-unknown` target was added to this host's stable toolchain for the `wasm` reading
  and stays.

## Beyond the plan
- A new route entry directly after this chunk, the founder's F1 option A (`inputs#I2`): «Taffy past #1372 — reversed
  scroll origins». It covers upstream's `0980cce5f` and its panic fix `ebacd68bd`, reconciled with escher's
  scrolled-box bounds, hit and absolute-position changes, with the five later bumps `35cdfc859` · `930448918` ·
  `c51dd246c` · `752ed2e49` · `b4952e15f` behind it. Its reversed-scroll-origin behaviour change is asked at its own
  take. Minted by this chunk's wrap (route-resolve).
- The master record's desc names upstream's pin `a3997d55` («Taffy moved to upstream's git pin a3997d55 with #1303
  and the left-out chain, measured before planned»), written before F1. The take is `e90751fd`. The wrap's flip
  restates it; phase never edits a record.

Apply? (yes / review / cancel)
