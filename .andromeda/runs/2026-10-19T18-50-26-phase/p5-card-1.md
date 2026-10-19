# P5 review — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

**Plan:** `escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/plan.md` (10 steps, 34 gate entries, 11 acceptance criteria) · **Research:** `research.md` beside it · **Base:** `67f17b9d6`

## Goal
Take upstream's two commits that carry no git dependency, by no-commit cherry-pick. The merge base stays `23b9b26a9` and Taffy stays the registry 0.15. This follows your F1 answer, «Cherry-pick two», recorded as delegate and so PROVISIONAL (inputs#I2).
- `38ca4e777` (#1226) brings three changes for every document, as upstream wrote them:
  - `:focus-within` matches.
  - A node outside the document is not focused.
  - A focus move asks the shell for a redraw.
  It also brings two upstream test files, `focus_within.rs` (7 tests) and `focus_redraw.rs` (4 tests).
- `17de049b1` (#1228) is the WPT pin.
- There is one 3-line conflict in `mutator.rs::add_children_to_parent`. It is resolved by keeping both sides and recorded as the one non-additive edit.

## Top acceptance criteria
- **Exact take:** the code-path diff against the base equals upstream's two commits line for line, with our changed-set insert kept.
- **Preservation:** nothing else moves. `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical, the root manifest has no git dependency, and Taffy stays the registry 0.15.
- **Upstream tests:** `focus_within` passes 7 and `focus_redraw` passes 4, each as its own target, with `all.rs` unbuilt and 138 files in the test directory.
- **Our tests:** a focus move still marks exactly two nodes in the changed set. `stand_keyboard`, `inert_subtree`, `stand_accessibility_ids` and the focus-teardown checks stay green, and the a11y leg reads 6 · 43.
- **Legs:** fast, doc and audit are green; `run stand` passes over 44 files and `run all` passes. `host_log` and `mcp_log` hold 0 needles under both builds.
- **Readings:** CARRY 1–4 are re-read and recorded, and nothing is sent upstream.
- **CI:** green on the pushed take, with the per-OS tallies re-read against CI#39302945352.

## Touchpoints
- **Modify:**
  - `packages/blitz-dom/src/{document,mutator,stylo}.rs`
  - `events/keyboard.rs`
  - `node/node.rs`
  - `tests/blitz-tests/tests/all.rs`
  - `wpt/WPT_COMMIT`
- **New:** `tests/blitz-tests/tests/focus_within.rs`, `focus_redraw.rs`, and the chunk's `evidence/`.

## Test commands (selected)
- **Guards and probes:** the exact-take `diff`, the preservation guard, the no-git-Taffy and Taffy-0.15 greps, the fixed-sha log census, no wall-clock outside `clock.rs`, the `keyboard_compat` reads, and the `FocusWithin => false` stub gone.
- **Tests:** `cargo test -p blitz-tests --test focus_within` and `--test focus_redraw`, the changed-set lib test, and five focus and a11y targets.
- **Legs:** `ci-leg.sh fast` with its tallies, `doc`, `audit`, `a11y`; `agent-run.sh boot`, `run stand`, `run all`; `cargo test -p seven_guis`; the windowed build and smoke.
- **Operator:** hygiene, then the pre-CI commit carrying both cherry-picked-from trailers, push, CI read and per-OS tallies.

## Baselines
28 entries were baseline-run on the untouched tree. Four read red, each on the subject the take builds:
- exact-take (an empty left side)
- the `FocusWithin` stub (still present)
- the file count (136)
- the two new targets (absent)

Every other entry reads green. Controls were fired in a scratch clone where the take was done exactly as steps 2–3 write it. The conflict reproduced and the resolution held. Every must-pass control reads green, and every must-fail control reads red: `.andromeda/runs/2026-10-19T18-50-26-phase/p5-controls.md`.

## Concretization preview
None. No 0.2.0 capability is claimed, because a sync makes none verifiable.

## Open at implement
- Upstream's `focus_redraw.rs` asserts exact redraw counts, including exactly 1 on a Tab `KeyDown`. Our fork's other redraw sites on the same paths could move that count. A red is read and either fixed by a named minimal edit or recorded, never loosened (plan step 4).

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | CARRY 1 (the four Taffy-main commits, left out again because they put git Taffy back in the root manifest) moves forward. | «Upstream sync ahead of polish and ship» (`escher-0.2.0/working-route.md`, Epoch 5's first entry), by the wrap's route-resolve |
| 2 | CARRY 2 (`keyboard_compat.rs`, not retired at `17de049b1`, both coverage exclusions standing) moves forward. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
| 3 | CARRY 3 (the engine-debts merge surface, re-read per row at each sync, no upstream pull request) moves forward with this chunk's re-read cited. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
| 4 | CARRY 4 (the document clock's merge surface, re-read at each sync) moves forward with this chunk's re-read cited. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
| 5 | The take's PROVISIONAL mark: upstream's every-document `:focus-within` / not-in-document focus / focus-move redraw, taken on a delegate's answer. It stands until the founder's own word. | Discharged in the Epoch 4 boundary batch (playbook «Provisional discharge»), listed by the wrap that closes Epoch 4 |

Apply? (yes / review / cancel)
