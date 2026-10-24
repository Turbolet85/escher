# /andromeda-implement — 2026-10-24-upstream-sync-polish-and-ship

**Outcome: green + smoke.** 2026-10-24T11:53Z · base `a356f9086` · run dir `.andromeda/runs/2026-10-24T11-39-00-implement/`

## What was done
- **Take (step 2):** ten `git cherry-pick --no-commit`, in the plan's order. Nine picked clean. `28f848d3b` conflicted in `construct.rs`'s import list alone, and git's rerere staged P3's trial resolution (`Staged … using previous resolution`). The file was read after the pick: upstream's line with `RangeInputData` after `NodeKind`, no conflict marker, no other line off upstream's hunk (`evidence/take.md`).
- **Lock (step 3):** `cargo update -p tokio`, 1.53.2 → 1.54.1. `Cargo.lock` +15 −14 as forecast: tokio's version and checksum, eleven `windows-sys` edges and one `syn` edge re-pointed to locked versions, and `9eeebb302`'s `mimalloc` edge. No package enters or leaves (987 → 987). The mimalloc / libmimalloc-sys hand review was re-read, and each `deny.toml` ignore is still encountered (`evidence/lock.md`).
- **Evidence:** `before.md`, `take.md`, `lock.md`, `readings.md` (CARRY 1–5 per row), `sink.md`, `counts.md` under `escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/`. `ci.md` is the operator pass's.

## Files changed (27, exactly research's lists; `gate.py scope`: clean — changed 27 · listed 27 · recorded 0)
- New: `packages/blitz-dom/src/layout/inline/{build,mod,layout}.rs`, `examples/multi_doc.rs`, `tests/blitz-tests/tests/hit_test_dropped_nodes.rs`.
- Removed (renamed): `packages/blitz-dom/src/layout/inline.rs`.
- Modified: blitz-dom `layout/{construct,mod,paint_tree,replaced}.rs`, `node/{node,element,mod}.rs`, `mutator.rs`, `stylo.rs`, `config.rs`, `resolve.rs`; `packages/dioxus-native/src/lib.rs`; `apps/browser/{Cargo.toml,src/document_loader.rs,src/toolbar.rs}`; `wpt/runner/{Cargo.toml,src/main.rs,src/test_runners/attr_test.rs}`; `tests/blitz-tests/tests/all.rs`; `flake.nix`; `Cargo.lock`.
- Staged in the index by the picks; nothing committed.

## Gates (trail `.andromeda/runs/2026-10-24T11-39-00-implement/gate-2026-10-24-upstream-sync-polish-and-ship.json`)
`entries 39 · green 34 · red 0 · recorded 1 · timeout 0 · not-run 4` — one firing, one iteration, no fix made. The not-run four are entries 36–39, `leg = 'operator'`.
- workspace tally (20): `lines 206 passed 1099 failed 0 ignored 12` — +1 line and +1 passed against the base, both `hit_test_dropped_nodes`.
- `run stand` (29): run.end passed 195 · failed 0 · ignored 7, unmoved. `run all` (31): passed 617 · failed 0 · ignored 12, +1.
- a11y (27): `lines 6 passed 43 failed 0`. Audit (25): `advisories ok, licenses ok`. doc (24) green. CI scripts (23, recorded): `Ran 100 tests` · `OK`.
- smoke (35): green — `service.name=seven_guis` logged, the window up to the 10 s bound (WAYLAND_DISPLAY set).
- No gate deferred: the take changes Rust source and the lockfile. No `gate.py guard` entry is in the plan.
- Matrix: `matrix.py show` — claimed 0, so no ref is written (a sync claims none).

## Deviations from plan (with justification)
- **A red-before-green control was taken for `hit_test_dropped_nodes`** (entry 16's baseline names "the implement red-before-green run"). `f102663b8`'s hunks to `node/node.rs` and `layout/paint_tree.rs` were reversed in the working tree, and the test read red: exit 101, panic at `node.rs:1572:26`. The saved bytes were restored (sha256 equal, worktree equal to the index), and the test read green again. Recorded in `evidence/counts.md`; no file is left changed.
- **CARRY 4 reading corrected:** plan step 5 says only `config.rs` is reached. `resolve.rs`, also in CARRY 4's file list, is reached too, by `28f848d3b`'s import list alone (+2 −2, no time read). Recorded in `evidence/readings.md`.
- **The trial worktree:** the plan's implementation notes say `target/tmp/trial-wt` stays registered for the operator to remove. `git worktree list` at Setup lists only the main tree, as the handoff says (already removed).
- Scope record: none needed (no out-of-list edit).

## Surfaced / stuck
Nothing.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (entries 16–19, 24–26, 28–34) | this run (gate entries) | terminated |
| `agent-run.sh` boot · run stand · run all (28, 29, 31) | this run (gate entries) | terminated |
| `seven_guis_native` (35, under `timeout 10`) | this run (gate entry) | terminated |
| cargo (the red-before-green control, twice) | this run (by hand) | terminated |
Measured: `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args agent-run.sh` at 11:52:35Z → `census: none running`.

## Decisions & corrections
- Guard refusal: bash-guard refused `cd {skill references dir} && grep …` (the cd arm); re-issued once with an absolute path.
- Hook: the stamp-ahead PostToolUse hook refused an estimated `11:45Z` stamp in `evidence/readings.md` while the clock read 11:41:17Z. Corrected to the read clock; later stamps were read from `date -u` or dropped.
- Standing learning met: rerere's recorded P3 resolution was applied by itself and read before going on, as the operator's word asked (handoff).

## Next
The **operator pass**: entry 36 (hygiene, `gate.py hygiene`), then the pre-CI commit, its subject exactly `chore(2026-10-24-upstream-sync-polish-and-ship): operator pre-CI commit, for the run this chunk's verdict reads`. Its body names the take on the founder's «Десять коммитов (Рекомендую)» and carries the ten `(cherry picked from commit …)` trailers in step 2's order. Then entry 37 (fast + clean-tree guard + push), entry 38 (`ci.py conclusion --sha HEAD --wait 2400`) and entry 39 (the per-OS tallies into `evidence/ci.md` against CI#39410865604). Then `/andromeda-wrap-session`.
