# P5 review — 2026-11-19-upstream-sync-ahead-of-the-debts («Upstream sync ahead of the debts»)

The plan: `doc:2026-11-19-upstream-sync-ahead-of-the-debts/plan` rev 3. Its render, to open in an editor: `.andromeda/cache/docs/2026-11-19-upstream-sync-ahead-of-the-debts/plan.md`. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`. A diff of the chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-upstream-sync-ahead-of-the-debts`.

## Goal
A real two-parent merge of `upstream/main` at `547d7514b` into `build/escher-0.4.0`. It is the first merge since `3eda07bc8`; the 0.3.0 syncs were cherry-picks.
- Upstream moved during the take-up: `547d7514b`, 212 commits. Your reading was `5b8e7a268`, 209 commits; the 3 new ones are `f0d7aeac9`, `18ce65e97` and `547d7514b`.
- The trial merge on `d9af170dc` measures 52 conflicted files and 164 hunks:
  - about 40 are the same change made twice, the earlier picks;
  - about 30 come from the pin moves;
  - about 45 are real behaviour clashes: the event queue against the `click_*` arms, select and keyboard; `has_changes` against the changed set; float placement.
- **`position: sticky` (blitz#887) is not in the take.** The only sticky commit in the delta is `f9d0e8c76`, which handles paint order. A sticky box stays laid out as relative.

## Your rulings at this review
1. **The pin set (my lean).**
   - Take Stylo 0.24, selectors 0.43, parley 0.13, skrifa 0.49, winit `=0.31.0-beta.4` and svgtypes 0.16.2 as one coupled registry set. Holding Stylo would mean undoing 34 later blitz-dom commits.
   - Hold Dioxus at 0.7.3, because it is a pre-release. Intent finding 4 says: «One that hangs on another's release (RUSTSEC, Dioxus 0.8) is stated as such, not forced».
   - The undo of `286874cc1` is named in the left-out table. These are a dependency set, not a widening, so they are yours to rule here.
2. **The order of the CDP security amendment (validation-1's one unclear point).**
   - `res-ipcah32k` says «a security-plan amendment first … before the take lands».
   - Masters change only at the wrap, which comes after the pre-CI merge commit.
   - The plan therefore gates on the founder's W1 answer before any merge step, and the registration rides the wrap. Under W1 C nothing is registered.
   - Say whether that order satisfies «first», or whether the take should wait for a separate amendment.

## The founder's forks — one file, not answered here
`.andromeda/cache/chunks/2026-11-19-upstream-sync-ahead-of-the-debts/evidence/founder-forks.md` holds nine forks for relay. Each one lists what crosses, file:line, the rule of ours it meets, test coverage, and the options.
- W1: the CDP server, with its TCP listener, `BLITZ_CDP_PORT`, `BLITZ_DEVTOOLS_FRONTEND_DIR` and `webbrowser`, on by default in the browser.
- W2: the `floem-editor-core` git dependency, which arrives through `parley_code_editor`.
- W3: rustls with `aws-lc-sys`, and the `CDLA-Permissive-2.0` licence.
- W4: skia built by default in the browser and the WPT runner.
- W5: focus changes fire DOM events. This contradicts the ratified silences on the inert clear and on the driver's select focus.
- W6: script-dispatched events read `isTrusted = false`.
- W7: a cancelled mouse pointerup still clicks.
- W8: SVG presentation attributes and `hr` attributes are read as style hints.
- W9: `visibility: collapse` leaves the accessibility tree and the snapshot.

**The plan is blocked on those answers.** `/andromeda-implement` starts only after the founder's word is relayed and snapped. Step 1 stops before the merge if any fork is unanswered.

## Top acceptance criteria
- HEAD after the pre-CI commit is a two-parent merge, and `HEAD^2` is `547d7514b` (entry 14).
- No conflict marker stands, `AGENTS.md` is absent, and the README is escher's (entries 2 and 3).
- Taffy is the lock's only git source (entry 4). The expected count becomes 2 only if the founder picks W2 B.
- Every W1–W9 change sits in the tree as the founder ruled it. Each take is recorded PROVISIONAL for ratification.
- The `build`, `fast`, `doc`, `a11y`, `audit` and `stand-repeat` legs are green. A moved stand reading is re-measured and recorded, never loosened.
- `evidence/lock.md` names every upstream change the merged tree does not keep, with its reason, and re-reads the 102 rows of `res-7qgqygtj`. Each new crate is reviewed by hand.
- The residuals are re-read in `evidence/readings.md`, the sink and env census is in `evidence/sink.md`, and sticky and the Dioxus hold are stated in `evidence/limits.md`.
- Fork CI reads green on the pushed merge with every job settled. A no-runner iOS or macOS cancel gets a re-run (inputs#I2).

## Touchpoints
- The merge's own set: `packages/**`, `apps/**`, `examples/**`, `tests/**` and `wpt/**`, all derived by `git merge`.
- The root files: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `README.md`, `.github/workflows/ci.yml` and `wpt.yml`.
- `.github/scripts/test_blitz_tests_targets.py`, and the 8 new upstream test targets.
- The named resolution sites, among them:
  - `blitz-dom` `document.rs`, `events/pointer.rs`, `events/keyboard.rs`, `mutator.rs`, `accessibility.rs`, `net.rs` and `layout/inline/layout.rs`;
  - `blitz-shell/src/window.rs` and `blitz-vibey-script/src/runtime/events.rs`;
  - `dioxus-native-dom/src/dioxus_document.rs` and `mutation_buffer.rs`;
  - `blitz-paint/src/render.rs`.
- The chunk's evidence files: `lock.md`, `readings.md`, `sink.md` and `limits.md`.

## Test commands (14 entries, baselined at `d9af170dc`)
1. `MERGE_HEAD` = `547d7514b`. Baseline red: no merge stands yet, which is the state the merge moves.
2. No conflict markers. Baseline green.
3. No `AGENTS.md`. Baseline green.
4. One git source in the lock. Baseline green.
5. `ci-leg.sh build`. Baseline green, 11 s.
6. `fast`. Baseline green, 549 s.
7. `doc`. Baseline green, 13 s.
8. `a11y`. Baseline green, 45 s.
9. `audit`. Baseline green, 1 s.
10. `stand-repeat`. Baseline green, 889 s, 3 of 3 rounds.
11. Hygiene (operator).
12. Push, clean-tree form (operator). The merge commit before it is the pre-CI commit, with the subject `chore(2026-11-19-upstream-sync-ahead-of-the-debts): operator pre-CI commit — merge upstream/main 547d7514b, for the run this chunk's verdict reads`.
13. `ci.py conclusion --wait 2400` (operator).
14. `HEAD^2` = `547d7514b` (operator).

## Capabilities — no claim, no concretization preview
`v040-02`'s acceptance covers the first entry of every epoch, and four later upstream-sync entries name it. This chunk advances it without making it verifiable. After the word, one dated `notes` line records that, and the capability stays pooled.

## Reads behind this plan, stated
- **Rows files:** read by search, because their sum is about 484 KB, past the 250,000 B whole-read bound. Every row the plan cites is present in its rows file.
- **History files:** security was read whole; architecture and tests by a search for sync entries. The design, layouts, obs and a11y histories were not read.
- **P3:** two read-only survey agents did the conflict classification and the widening survey. Their output is kept as `evidence/collisions-survey.md` and `evidence/widenings-survey.md`, and I spot-checked five of their claims.
- **CI on `d9af170dc`** (CI#44558104404) is red only on the no-runner `Test (ios)` cancel. It is recorded as the recurrence of WATCH `res-q2tllspz` on your word (inputs#I2), and you are re-running that job.

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | The Dioxus 0.8 move stays out of this take (held at 0.7.3). `res-xpkmxnvl`, `res-xchmbfjr` and the `keyboard_compat` coverage exclusion stand, re-read at the next upstream sync. | «Upstream sync ahead of the canvas» (existing entry), or the first sync after a Dioxus 0.8 release |
| 2 | The iOS no-runner cancel on `d9af170dc` (CI#44558104404) is a recurrence of WATCH `res-q2tllspz`. Its retire clause reads a recurrence as a trigger to act. | «Dependency and CI debts» (existing entry, v040-07) |

Apply? (yes / review / cancel)
