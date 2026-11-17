# P5 review — 2026-11-17-upstream-sync-ahead-of-polish-and-ship (plan rev 4)

**Open the plan:** `.andromeda/cache/docs/2026-11-17-upstream-sync-ahead-of-polish-and-ship/plan.md`. No plan stands in a chunk's folder, and writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-upstream-sync-ahead-of-polish-and-ship` diffs the chunk's records.

## Goal
The twelfth upstream take, at `upstream/main` `e5305a4c1` (10 new commits), opening Epoch 6, in three parts:
1. **Five commits by no-commit pick**, with two hand resolutions and the shear's one line by hand: `7b8712cba` (SVG `<image>`/`<use>` external images), `e8b370a25` and `585c1869c` (WPT only), `d02c094c7` (the shear direction) and `e5305a4c1` (the scroll clamp at layout rounding).
2. **AnyRender 0.15 from crates.io**, on the founder's «Взять AnyRender 0.15 в этом sync» (`inputs#I5` §1). It carries upstream's `vello-hybrid` → `vello-gpu` rename over 26 files, and ports blitz-paint with semantics kept: `AntiAliasing::Enabled` at 53 sites, the box shadow unchanged in meaning, the filter's bounds as upstream wrote them. zip 2.4.2 → 9.6.1 cures the audit red (RUSTSEC-2026-0545/0548, `inputs#I2`), and the four standing zip ignores are dropped.
3. **The CI disk line** (`inputs#I1`): `Test [default features]` gets `df -h` before and after its test leg (the after one `if: always()`) and the same SHA-pinned free-disk step as stand-repeat.

## Measured on the trial (`target/tmp/trial-pas`)
| Reading | Five commits | + AnyRender 0.15 |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | 1546 passed · 1 failed · 15 ignored | 1546 passed · 1 failed · 15 ignored |
| The one failure | `stand_act_scroll` :248 (the clamp; re-pinned by step 7) | the same |
| clippy · doc · a11y · embolden | green | green, plus wasm and msrv 1.91 green |
| audit | red (0545, 0548) | **`advisories ok, licenses ok`** |
| Whole-screen shots of the 6 served tasks vs HEAD | byte-identical | 427–2 618 of 480 000 pixels differ, max delta ≤ 30/255 |

## The founder's words this plan rests on (`inputs#I5`, relayed by the overseer, verbatim)
- §1 AnyRender 0.15 in this sync: «Взять AnyRender 0.15 в этом sync»
- §2 `7b8712cba`, the SVG fetch, a boundary widening by upstream merge (`rul-72xhfjoz`, escalate): «Взять (Recommended)». The wrap halts once on its card citing §2, with no second question to him.
- §3 `d02c094c7`, the shear: «Взять (Recommended)»
- §4 `e5305a4c1`, the clamp: «Взять, утверждаю (Recommended)»

The four engine behaviour changes are recorded ratified under `rul-47a6zcoz` (routine, on his word).

## Two consequences of §1 the wrap will meet (yours to see now)
- **(architecture · decision/skrifa-matches-parley-vello) / decision/font-deps-match-renderers**: "skrifa matches the version parley and vello use". After the take, parley 0.12 stays on skrifa 0.44 while vello 0.12 and glifo 0.5 bring skrifa 0.49, so two versions stand in the lock. The plan amends the rows at the wrap. A wrap reading of whether §1 covers that amendment may put it as its own question.
- **(security-plan · decision/upstream-sync-dependency-set)**: a cherry-pick take "enters no package". This one enters skrifa 0.49, read-fonts 0.46, sha2 0.11 and a few more, each read name by name in `evidence/lock.md`. It is amended the same way.

## Top acceptance criteria
- The lock holds AnyRender 0.15 and zip 9.x, with no git source but Taffy's pinned rev. The four zip ignores are gone. `ci-leg.sh audit` prints `advisories ok, licenses ok`.
- `ci-leg.sh fast` is green, `stand_act_scroll` included, re-pinned to the clamped reading with its control read red then green.
- The SVG `<image>` request carries its remote document as initiator (an added case in `local_files_for_local_documents`, controlled).
- No hybrid renderer name is left. The wasm, msrv, doc and a11y legs are green, along with the agent-run `run stand` and `run all` and the windowed boot smoke.
- The shot comparison is recorded in `evidence/shots.md`. The sink census reads 3 moved and 0 added.
- Fork CI is green on the pushed commit. The per-OS tallies are re-read by test name, and the test job's two `df -h` readings are recorded.
- No capability is claimed: `v030-15` is carried by «Cold-agent bar».

## Touchpoints (62 files; plan §Codebase touchpoints)
- blitz-dom: the take's 10 files.
- blitz-paint: 9 files (port and shear).
- dioxus-native: 3 files.
- apps: 5 files.
- examples: 17 files.
- wpt runner: 5 files.
- CI: `.github/workflows/ci.yml`, `.github/scripts/ci-leg.sh`, `test_ci_workflows.py`.
- `deny.toml`, `Cargo.toml`, `Cargo.lock`.
- Docs: `justfile`, `HOWTO_WASM.md`, `docs/architecture.mmd`, `docs/parley.md`, and three `.claude/docs` pages.
- Two tests: `stand_act_scroll.rs`, `local_files_for_local_documents.rs`.

## Test commands (23 entries; baselines on the untouched tree in the plan)
- **Probes 1–7:** git sources, anyrender 0.15, zip 9, the zip ignores, hybrid names, and the census added and removed. Each has a must-pass control on a scratch clone carrying the take (`p5-controls.md` C1–C10).
- **Main legs:** `fast` (seen green) · the two narrow test targets · agent-run `boot` / `run stand` / `run all` · smoke · `doc` · `a11y` · `msrv` · `wasm` · `audit` (baseline red on exactly 0545/0548, as predicted).
- **Record check:** the shot record.
- **Operator pass:** hygiene · push · `ci.py conclusion --wait 2700`.

## Research
`doc:2026-11-17-upstream-sync-ahead-of-polish-and-ship/research` rev 4. The rows files were read by search: 383 505 B, past the 250 000 B whole-read bound, every cited row present.

## Notes for the operator
- **Disk:** 190 GB free at the take-up, with `target/` at 690 GB, and the trial adds to it. A `cargo clean` before /implement's gates is your call.
- **The trial worktree** `target/tmp/trial-pas` and the scratch clone `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/82249230-ee3f-4f07-a3ec-f62fd9a45fdd/scratchpad/ctl-clone-1` are P3/P5 instruments, removed by you after the wrap.
- **CI `c74c7f3e6`** (the route-only commit): red at «Dependency audit» on the same two advisories, which this chunk cures. `Stand checks repeated` was still running at the P3 re-read.

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | The seven carries (`cry-o26emj4y`, `cry-mnnoitsu`, `cry-hvl3d4gp`, `cry-fdvxodkv`, `cry-blhcfqeb`, `cry-7rsyw6me`, `cry-hbkcvbxr`) and the left-out delta's table (112 carried, plus `2203de767` and `c5940f038` new) ride on, but no later upstream-sync entry stands in escher-0.3.0's route. | Proposed: the wrap's route adaptation carries them to the version close, for the next version's first upstream sync, which /andromeda-route places. |

Apply? (yes / review / cancel)
