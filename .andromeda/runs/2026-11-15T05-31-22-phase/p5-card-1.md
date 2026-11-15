# P5 review — 2026-11-15-upstream-sync-errors-and-explanation

**Plan to open in an editor:** `.andromeda/cache/docs/2026-11-15-upstream-sync-errors-and-explanation/plan.md` (rev 2).
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-upstream-sync-errors-and-explanation` shows a diff of the chunk's records.

## Goal
This is the eleventh upstream take, at `upstream/main` `ea07ee716`, by no-commit cherry-pick onto the registry pins.
- **Taken:** three commits — `f5ea1caf1` (`@font-face` `font-stretch` passed to fontique), `c30096724` (WPT pin) and `d2ee20b98` (WPT quarantine) — plus **taffy 0.15.0 → 0.15.1 from crates.io**, a lock-only move.
- **The Taffy carry:** 0.15.1 carries 13 of the 14 Taffy changes upstream's four Taffy-main commits bring. #1303, the caller-defined float id, is not on 0.15.x. So the founder's 22:09Z word is met with no master rule broken.
- **Left out:** the other 13 new commits, each with its measured reason. Six move a git AnyRender pin, six conflict on the stylo-split and SVG-paint chains left out last time, and one is build-red without one of those. 112 left out in all, each with its reason.

## The Taffy measurement (research §The Taffy carry)

| | Git Taffy (the four commits) | Registry 0.15.1 |
|---|---|---|
| Picks and edits | 4 clean; 3 files +3 −3 | `Cargo.lock` 2 lines |
| fast · audit · msrv | green · `advisories ok, licenses ok` · green | green · `advisories ok, licenses ok` · green |
| Tests | 1461 passed · 0 failed · 13 ignored | the same |
| Rule broken | architecture `decision/no-git-dependencies` | none |

The git take is stopped by the master rule alone, which only the founder can lift. It goes to him as **`founder-forks.md` Q1**, with the numbers above. It is answered PROVISIONAL: the rule stands and the registry carrier is taken (`inputs#I1`).

## Top acceptance criteria
- No manifest moves, no git source in the lock, and `taffy` 0.15.1 in the lock (probes 1–3).
- No new log site, print, env read or `Request::get` in the taken delta (probe 4).
- These legs pass: `fast`, `agent-run boot` / `run stand` / `run all`, the windowed boot smoke, and the `doc`, `a11y`, `msrv` and `audit` legs.
- Fork CI on the pushed commit reads green. The per-OS and stand-repeat tallies are re-read by test name into `evidence/ci.md`.
- All ten carries are read in `evidence/readings.md`. `cry-r2cidrqy` (the Taffy word) and `cry-qrse4pmp` (the left-out delta) are discharged; eight ride on.
- The take's two behaviour changes for every document (`f5ea1caf1` and Taffy 0.15.1's layout fixes) are recorded PROVISIONAL (**`founder-forks.md` Q2**).

## Touchpoints
- `packages/blitz-dom/src/document.rs`, `packages/blitz-dom/src/net.rs` — `f5ea1caf1`, as upstream wrote it
- `wpt/WPT_COMMIT`, `wpt/runner/timeout-quarantine.txt` — `c30096724`, `d2ee20b98`
- `Cargo.lock` — `cargo update -p taffy --precise 0.15.1` alone

## Test commands (16 entries, baselines on the untouched tree)
- 1–4 probes. 1, 2 and 4 read green, each vacuous until the take. 3 reads red: the lock holds 0.15.0, the subject step 2 builds. The controls in `p5-controls.md` read each one green on the take and red on its fail input.
- 5 `ci-leg.sh fast` — seen green at the last wrap.
- 6–8 the agent-run contract: boot 34 s; `run stand` 294 passed · 0 failed · 8 ignored; `run all` 825 · 0 · 13.
- 9 windowed smoke — green.
- 10–13 `doc` · `a11y` · `msrv` · `audit` — green.
- 14–16 operator: hygiene, push, and `ci.py conclusion --sha HEAD --wait 2100`.

## Capabilities
None claimed. The pool's four, `v030-10`, `v030-11`, `v030-12` and `v030-15`, are each named by a later markerless entry. No concretization preview.

## Checks
- Mechanical checks 1–12: pass.
- Check 7: WARN (soft). No code-graph query was recorded, because the take adds no escher symbol and changes no signature.
- Validation-1: aligned with `doc:2026-11-15-upstream-sync-errors-and-explanation/scope` rev 4, its P3 premise corrections included.

## For the founder (away) — `.andromeda/runs/2026-11-15T05-31-22-phase/founder-forks.md`
- **Q1** — Lift "the workspace takes no git dependency" for the git Taffy? PROVISIONAL: no. The registry carrier is taken, and #1303 with the later git chain waits.
- **Q2** — Ratify `f5ea1caf1` and taffy 0.15.1's layout fixes as upstream behaviour for every document? PROVISIONAL: both taken, each marked PROVISIONAL until his word.

## Beyond the plan

| # | Proposal | Owner / place |
|---|---|---|
| 1 | Carries `cry-irb7ecz4`, `cry-h4dkkdhq`, `cry-2hvzi4ns`, `cry-lrzp6e2g`, `cry-nnm3rzya`, `cry-oglnc26y` and `cry-p2xvtfy2` ride on, untouched by this take | «Upstream sync ahead of polish and ship» |
| 2 | Carry `cry-eyudppev` (the browser's unjustified `expect_used` allow) rides on | «Browser corrective» |
| 3 | The git-Taffy remainder (#1303, `1a11761bc`'s `inline.rs` line, and `b8f4ba6fd` … `b4952e15f` toward `a3997d55`) waits on Q1's answer or a registry release | «Upstream sync ahead of polish and ship» |
| 4 | The founder's Q1 and Q2 answers, applied PROVISIONAL now and discharged at the epoch boundary by the Provisional discharge rule | the wrap of this chunk, as escalations |
| 5 | Remove the P3 trial worktrees `target/tmp/trial-eae` and `target/tmp/trial-eae-git` with their build dirs `target/trial-eae` and `target/trial-eae-git` after the wrap | the operator |

Apply? (yes / review / cancel)
