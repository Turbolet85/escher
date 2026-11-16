# P5 review — 2026-11-16-taffy-past-1372 «Taffy past #1372»

Plan: `doc:2026-11-16-taffy-past-1372/plan` rev 3. Its render to open in an editor is
`.andromeda/cache/docs/2026-11-16-taffy-past-1372/plan.md`. No plan stands in a chunk's folder; writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-16-taffy-past-1372` is a diff of the chunk's records.

## Goal
Take upstream's reversed scroll origins: the seven commits `0980cce5f` · `ebacd68bd` · `35cdfc859` · `930448918` ·
`c51dd246c` · `752ed2e49` · `b4952e15f`. Git Taffy moves `e90751fd` → `a3997d55`, and a reversed box (RTL,
`row-reverse`, `column-reverse`, `wrap-reverse`) scrolls to negative offsets. The founder ratified that change for this
take (inputs#I2). escher's own-scroll bounds, hit stop and `absolute_position` readings are kept. The measured patch is
applied as it stands, not re-picked (inputs#I1).

## The take, sized (the operator's word, inputs#I1)
- **Code:** the measured patch is 16 files, +632 −120. It has 7 picks, 3 conflicts resolved to escher's side and 3
  seam edits. On top of it come the `hit_inner` coordinate fix (2 fields in one early return) and its check (one
  blitz-dom unit test, both layout modes).
- **Lock:** one line moves, the taffy `source`. It stays one git source, with no checksum and the same dependency list.
- **Local gate time:** about 25 min, from this phase's baselines on the untouched tree. `stand-repeat` takes 788 s,
  `agent-run run all` 225 s and `boot` 84 s; `fast` read 349 s in the trial; the other legs take under 20 s each.
- **CI:** about 35 min (2054 s last run). It reads the other OSes' Test jobs, the cross-OS digests and the coverage
  floors. The trial never ran coverage, so the CI run is the first reading of the floors.
- **Tallies:** predicted `test` 1501 → 1511, i.e. upstream's +9 plus step 3's +1, to be measured. The `a11y` and
  `stand-repeat` tallies are predicted unmoved.

## Top acceptance criteria
- (deps) The manifests move the taffy rev line alone (2); the lock holds one git source, taffy at `a3997d55`, and moves
  its `source` line alone (1 · 1 · 2).
- (scope) The code delta is the patch's 16 files.
- (hit) A border hit on a clipping inline root reads content-box coordinates in both layout modes. Its control, the
  fix reverted, reads the case `failed`.
- (tests) Both builds hold the take: `fast` is the workspace build (writing-mode on) and `agent-run run all` the
  per-package build (writing-mode off). `stand-repeat` reads 3 of 3 rounds green.
- (deps) `msrv` and `audit` (`advisories ok, licenses ok`) are green, and so are `doc`, `build`, `counter`, `wasm`,
  `a11y` and the windowed boot smoke.
- (ci) The pushed HEAD reads `verdict: green`.

## Touchpoints
`Cargo.toml` · `Cargo.lock` · blitz-dom `debug.rs`, `events/pointer.rs`, `layout/{inline/layout,mod,replaced,writing_mode}.rs`,
`mutator.rs`, `node/{node,scrollbar}.rs`, `scrolling.rs` · blitz-tests `display_contents.rs`,
`inline_box_scrollable_overflow.rs`, `pre_overflow_scroll.rs` · `wpt/runner/src/test_runners/attr_test.rs`. There are
no new files.

## Test commands
- Five probes check the manifests, the lock, the pin and the delta. Each has a must-pass and a must-fail control in
  `p5-controls.md`.
- The step-3 unit test.
- Local legs: `ci-leg.sh fast`, `agent-run.sh boot` and `run all`, the windowed smoke, `msrv` · `audit` · `doc` ·
  `build` · `counter` · `wasm` · `a11y` · `stand-repeat`.
- Operator legs: hygiene, push, `ci.py conclusion --wait 2400`.

## Capabilities
None claimed. `v030-11`, `v030-12` and `v030-15` are each named by another markerless entry, and this chunk builds none
of their surfaces.

## Leaned, not asked
- **This take pays the CARRY `cry-yrdxzd7k`.** Its own words: «its fix and a check owed at the take that next touches
  `hit_inner`». Step 1 rewrites `hit_inner`'s content test, so this is that take.
- **The fix moves only a coordinate.** It changes the coordinate a border hit hands its caller (text selection, the
  caret), not whether or what it hits. That keeps it inside the ratified clip stop (architecture ·
  decision/hit-stops-at-paint-clip).
- **No playbook escalate pattern applies.** No crossing widens and no upstream widening arrives.

## Beyond the plan
- Route, at the wrap: the CARRY `cry-yrdxzd7k` leaves «Upstream sync ahead of polish and ship», spent by this take.
- Route, at the wrap: the CARRY `cry-blhcfqeb` on «Upstream sync ahead of polish and ship» drops its clause that
  `0980cce5f`'s `hit_inner` hunk is a textual conflict with the non-invertible answer, since this take resolves it.
  Its other merge surfaces stand.
