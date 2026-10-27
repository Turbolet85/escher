# P5 review card 1 — 2026-10-27-upstream-sync-engine-debts

**Plan:** `escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/plan.md` (410 lines, 35 gate entries, 10 acceptance criteria) · **Research:** `escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/research.md` · **Run dir:** `.andromeda/runs/2026-10-27T07-42-37-phase/`

## Goal
Take eleven of upstream's seventeen new commits (upstream/main `ebacd68bd`) by no-commit cherry-pick in upstream order. That is the founder's own word, «Одиннадцать коммитов (Рекомендую)» (2026-10-27, relayed verbatim by the overseer, inputs#I4). Every pin stays where it is: `Cargo.toml`, `Cargo.lock` and `deny.toml` byte-identical, Stylo 0.22.0, taffy 0.15, Dioxus 0.7.3, winit beta.3. Beside the take: re-read residual line 36's merge surfaces per row, re-check the sink allowlist, and re-read the per-OS tallies on the pushed take's CI run. Nothing is sent upstream.

## Top acceptance criteria
- The take is exact (the exact-take, file-identity and comment-carry entries); the three one-hunk resolutions are named as the take's non-additive edits.
- The pins stay (the preservation guard; no git dependency; the lock unchanged).
- `fast`, `doc`, `audit` (`advisories ok, licenses ok`) and `a11y` (`lines 14 passed 85 failed 0`) are green. The workspace tally is predicted 209 · 1125 · 0 · 12. Upstream's `pseudo_element_flow_classification` runs as its own target (3), and the stand passes over 50 files.
- Residual line 36's rows are re-read against the take, each with its owner `.andromeda/residuals.md:36`; the sink census reads its two named hits; fork CI is green on the pushed take, with the per-OS tallies explained.

## Touchpoints
blitz-vibey-script's split (`runtime/`, `dom/element/`, 23 new files; our select-by-id comment carried into `runtime/events.rs`), `layout/construct.rs`, `mutator.rs` (our range arm beside upstream's `open` arm), `node/element.rs`, `stylo.rs`, blitz-shell `lib.rs` · `window.rs`, `dioxus_document.rs` (upstream's IME test alone), `apps/browser`, `publish-browser.yml`, `wpt/WPT_COMMIT`, `tests/blitz-tests/tests/all.rs`.

## Test commands
35 entries. 29 are new, each baseline-run on the untouched tree. Every red on the base is this chunk's subject absent; must-pass readings on the trial take and must-fail inputs are recorded in `.andromeda/runs/2026-10-27T07-42-37-phase/p5-controls.md`. The four operator entries (hygiene, the push, the CI read, the per-OS tallies) run in the operator pass. The tally read's control was fired at P4 on CI#40434241601.

## Concretization preview — `v030-03` (Upstream sync), the one capability claimed
- **Current acceptance:** "Upstream's new work is in by the take the founder chose, each fork engine change's merge surface still has its owner, and every CI leg, the accessibility leg and the dependency audit read green after it."
- **Proposed concrete form:**
  - The take the founder chose is «Одиннадцать коммитов (Рекомендую)», his own word (inputs#I4): the eleven by no-commit cherry-pick, pins and lock byte-unchanged, the exact-take, file-identity and preservation-guard entries green.
  - Each merge surface keeps its owner: `evidence/readings.md` re-reads residual line 36's rows against the take and names the `.andromeda/residuals.md:36` line on each, the plan's own enumeration (by construction).
  - Every leg is green: `ci-leg.sh fast`, `doc`, `audit` and `a11y` locally, and the fork CI run on the pushed take green on all its jobs.
- Not gated: the take rests on the founder's own word, so no PROVISIONAL record holds the cap.

## Validation
- Mechanical checks 1–12 pass: the fence parses, 29 baselines sit exactly on the new entries, planlint's one listed HIT (4 (13)) is resolved by its note and control, snapshot citations read 0 unparsed, and the scope lists parse 41 items.
- Validation-1 reads aligned: the plan builds scope's fork outcome, and P3's closure had already amended the scope.
- Not sampled, so there is no `sampler:` line.

## Beyond the plan
- **Residual line 35** (the left-out upstream delta) grows by the six new left-out commits: `a09f46792`, `527394724`, `d3ed4e2c7`, `ccfe15332`, `0980cce5f` and `ebacd68bd`. Its owner moves to the existing entry «Upstream sync ahead of the UI graph — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked» (Epoch 3), with this chunk's measurements (research.md §The trial takes).
- **Residual line 36** (the merge surfaces of the engine-debts fixes, the clock and the select model) is re-read here, and its owner moves to the same Epoch 3 entry.
- **For «Engine debts» (v030-04)**: the take leaves the transformed-scrolling-box `visible_region` limit unmoved. `0980cce5f`, which would collide with escher's scrolled-box bounds and hit, is not taken — a reading for that entry, not a route insertion.
