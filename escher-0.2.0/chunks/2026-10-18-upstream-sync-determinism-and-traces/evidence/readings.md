# The three CARRY re-reads — 2026-10-18-upstream-sync-determinism-and-traces

Read by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18, after the take.

## CARRY 1 — the Taffy-main commits
- `git show fb8d6c15e:Cargo.toml`, line 100: `taffy = { git = "https://github.com/DioxusLabs/taffy", rev =
  "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", default-features = false, features = [`. Upstream still pins a git Taffy.
- Ours, `git show HEAD:Cargo.toml`, line 105: `taffy = { version = "0.15", default-features = false, features = [`.
  The take leaves it byte-identical (the preservation guard and the two Taffy probes, `evidence/lock.md`).
- Taffy main at `ea07c4c3` is v0.15.0 plus 14 unreleased commits, and crates.io's newest `taffy` is 0.15.0. This is
  inputs#I3's measurement (`gh api …/compare/v0.15.0...ea07c4c3`, 2026-10-18T06:00Z) and inputs#I2's. It was not
  re-read live at /implement.
- Reading: the reason the four Taffy-main commits (`1a11761bc`, `4a5ab9cc9`, `510b592ab`, `1e5479ae6`) were left out
  stands. They wait for the next sync again, on F1 B (delegate the operator, 2026-10-18, PROVISIONAL; inputs#I5).

## CARRY 2 — `keyboard_compat.rs`
- The two entries (trail `.andromeda/runs/2026-10-18T06-16-11-implement/gate-2026-10-18-upstream-sync-determinism-and-traces.json`):
  ```
    7 probe       green · exit 0 · 0.0s · 2 B → 7.log · git ls-tree -r --name-only fb8d6c15e9ce062b3b41e71cd48988f… (128 chars)
    8 probe       green · exit 0 · 0.0s · 2 B → 8.log · grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.r… (89 chars)
  ```
  Entry 7's log reads `1`: `packages/dioxus-native-dom/src/keyboard_compat.rs` is still listed at `fb8d6c15e`. Entry 8's
  log reads `2`: both `cargo llvm-cov` lines of `.github/scripts/ci-leg.sh` keep `--ignore-filename-regex
  'keyboard_compat\.rs$'`.
- Reading: not retired upstream, so both exclusions stay.

## CARRY 3 — the engine-debts merge surface, per row
`fb8d6c15e` changes one file, `packages/blitz-dom/src/document.rs` (`git show --name-only`). Its two hunks are
`@@ -2193,11 +2193,15 @@ impl BaseDocument` (in `get_cursor`) and `@@ -2935,6 +2939,16 @@ mod hover_state_tests`,
at upstream's lines. Each row's files were read with `git diff --quiet 23b9b26a9 fb8d6c15e -- packages/blitz-dom/{file}`,
against `escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md`.

| debt | files (under `packages/blitz-dom/`) | did `fb8d6c15e` move its ground? | upstream PR |
|---|---|---|---|
| F1 | `src/node/node.rs` · `src/accessibility.rs` | no — both files untouched | no upstream PR |
| F2 | `src/accessibility.rs` | no — untouched | no upstream PR |
| F3 | none (a pin, no engine change) | no — no file | — |
| F4 | `src/node/node.rs` · `src/scrolling.rs` | no — both untouched | no upstream PR |
| F5 | `src/node/node.rs` | no — untouched | no upstream PR |
| F6 | `src/events/pointer.rs` | no — untouched | no upstream PR |
| F7 | `src/node/element.rs` · `src/events/pointer.rs` · `src/events/keyboard.rs` | no — all three untouched | no upstream PR |
| F8 | `src/document.rs` (`get_client_bounding_rect`, `node_client_rects`, `transformed_bounds`, `is_transformed_chain`, `transformed_fragment_rects`) | same file, other functions: `get_cursor` and `hover_state_tests`, none of the five. The cherry-pick merged without conflict, and the five only shift down 4 lines (`get_client_bounding_rect` 2373 → 2377, `transformed_bounds` 2410 → 2414, `is_transformed_chain` 2455 → 2459, `transformed_fragment_rects` 2465 → 2469, `node_client_rects` 2564 → 2568) | no upstream PR |

No upstream pull request was opened (the founder's standing hold; scope.md §Out of scope). The seven fixes stay
PROVISIONAL, gated on the founder's word in 2026-10-18-engine-debts. This chunk neither ratifies nor changes them.

## The take's line map for `packages/blitz-dom/src/document.rs` (for the wrap's re-pointing)
From `git diff --cached -U0` against HEAD `fb842671`, with hunks `@@ -2264,2 +2264,3 @@`, `@@ -2268 +2269,4 @@` and
`@@ -3127,0 +3132,10 @@`. The file goes from 3575 to 3589 lines.
- old 1–2263 → unchanged;
- old 2264–2268 → `get_cursor`'s changed lines (new 2264–2272);
- old 2269–3127 → **+4** (new 2273–3131); `mod hover_state_tests` 3041 → 3045;
- old 3128–3575 → **+14** (new 3142–3589); upstream's new test sits at new 3132–3141.
