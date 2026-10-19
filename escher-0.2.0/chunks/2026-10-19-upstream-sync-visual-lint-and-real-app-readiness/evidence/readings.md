# The four CARRY re-reads and the engine behaviour — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Read by /andromeda-implement, run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19, after the take
(no-commit cherry-picks of `17de049b1` and `38ca4e777` onto the base `67f17b9d6`). Gate lines are quoted from that
run's trail `.andromeda/runs/2026-10-19T19-16-25-implement/gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`.

## CARRY 1 — the Taffy-main commits
- `git show 17de049b1:Cargo.toml`, line 100: `taffy = { git = "https://github.com/DioxusLabs/taffy", rev =
  "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", default-features = false, features = [`. Upstream still pins a git Taffy.
- Ours, `git show HEAD:Cargo.toml`, line 105: `taffy = { version = "0.15", default-features = false, features = [`.
  The take leaves `Cargo.toml`, `Cargo.lock` and `deny.toml` byte-identical (`evidence/lock.md`).
- crates.io's newest `taffy` is 0.15.0 (inputs#I1, measured at take-up; not re-read live at /implement).
- Reading: the four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) are left out again, on
  the operator's answer to F1 «Cherry-pick two (Recommended)», given as delegate and PROVISIONAL (inputs#I2). The CARRY
  moves to the next sync's entry, «Upstream sync ahead of polish and ship».

## CARRY 2 — `keyboard_compat.rs`
- `git ls-tree -r --name-only 17de049b1 -- packages/dioxus-native-dom/src/keyboard_compat.rs` lists the file: upstream
  has not retired it at the read sha.
- Both `cargo llvm-cov` lines of `.github/scripts/ci-leg.sh` keep `--ignore-filename-regex 'keyboard_compat\.rs$'`.
- The two entries:
  ```
    7 probe       green · exit 0 · 0.01s · 2 B → 7.log · git ls-tree -r --name-only 17de049b11095808e0d093be2a8f052… (128 chars)
    8 probe       green · exit 0 · 0.0s · 2 B → 8.log · grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.r… (89 chars)
  ```
  Entry 7's log reads `1`, entry 8's reads `2`. The coverage leg was not run locally (plan §Test Commands: the take
  changes no line of the three floored crates); its floors are CI's "Coverage report" job, read in `evidence/ci.md`.
- Reading: not retired upstream, so both exclusions stay. The CARRY moves to «Upstream sync ahead of polish and ship».

## CARRY 3 — the engine-debts merge surface, per row
`38ca4e777` changes five engine files (`git show --stat`): `src/document.rs`, `src/events/keyboard.rs`,
`src/mutator.rs`, `src/node/node.rs`, `src/stylo.rs`; `17de049b1` changes `wpt/WPT_COMMIT` alone. Each row's files are
read against `escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md`; the functions upstream touched are
read from `git show 38ca4e777 -- {file}`'s hunks, and the row's named functions are located with `grep -n 'fn {name}'`
in `HEAD` and in the taken tree. Untouched files read `git diff --quiet HEAD -- {file}` exit 0 after the take.

| debt | files (under `packages/blitz-dom/`) | did `38ca4e777` move its ground? | upstream PR |
|---|---|---|---|
| F1 | `src/node/node.rs` (`is_focussable`, `is_inert`, `hit_inner`) · `src/accessibility.rs` | no. In `node.rs` upstream adds three methods, `add_focus_within`, `remove_focus_within` and `contains_focus`, after `is_hovered` and before `focus` (one hunk, `@@ -715,6 +715,29 @@` upstream, new lines 736–758 here). It touches none of the three. `is_focussable` 553 and `is_inert` 563 stay where they are; `hit_inner` shifts 1392 → 1415. `accessibility.rs` is untouched. | no upstream PR |
| F2 | `src/accessibility.rs` | no — untouched | no upstream PR |
| F3 | none (a pin, no engine change) | no — no file | — |
| F4 | `src/node/node.rs` (`absolute_position`, `position_in_containing_block`) · `src/scrolling.rs` | no. The `node.rs` hunk above touches neither function, which shift +23: `absolute_position` 1720 → 1743, `position_in_containing_block` 1728 → 1751. `scrolling.rs` is untouched. | no upstream PR |
| F5 | `src/node/node.rs` (`hit_inner`) | no — the same hunk; `hit_inner` shifts 1392 → 1415 | no upstream PR |
| F6 | `src/events/pointer.rs` | no — untouched | no upstream PR |
| F7 | `src/node/element.rs` · `src/events/pointer.rs` · `src/events/keyboard.rs` (`activates_on_enter`, `activates_on_space`) | no. In `keyboard.rs` the Tab arm of `handle_key_or_input_event` loses four lines, `let old_focus = doc.focus_node_id;` and the three-line `if doc.focus_node_id != old_focus { doc.shell_provider.request_redraw(); }`. Neither activation function is touched; they shift −4: `activates_on_enter` 162 → 158, `activates_on_space` 179 → 175. `element.rs` and `pointer.rs` are untouched. | no upstream PR |
| F8 | `src/document.rs` (`get_client_bounding_rect`, `node_client_rects`, `transformed_bounds`, `is_transformed_chain`, `transformed_fragment_rects`) | same file, other functions. The hunks are in `clear_interaction_state_for_removed_node`, `clear_focus` and `set_focus_to`, none of the five. The five shift +49: `get_client_bounding_rect` 2392 → 2441, `transformed_bounds` 2429 → 2478, `is_transformed_chain` 2474 → 2523, `transformed_fragment_rects` 2484 → 2533, `node_client_rects` 2583 → 2632. | no upstream PR |

No upstream pull request was opened (the founder's standing hold; scope.md §Out of scope). The engine-debts fixes stay
as 2026-10-18-engine-debts left them; this chunk neither ratifies nor changes them.

## CARRY 4 — the document clock's merge surface
- Of the clock's files (`packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs`, `events/pointer.rs`),
  `38ca4e777` reaches `document.rs` alone, in the three focus functions above. `config.rs`, `clock.rs`, `resolve.rs`,
  `iframe.rs`, `scrolling.rs` and `events/pointer.rs` read `git diff --quiet HEAD` exit 0 after the take.
- No `Instant::now()` or `SystemTime::now()` is added (the fixed-sha census), and blitz-dom holds none outside
  `clock.rs` on the taken tree:
  ```
    5 probe       green · exit 1 · 0.01s · 2 B → 5.log · git show --format= 38ca4e777513607c7dcb659748380e7a650b018… (267 chars)
    6 probe       green · exit 1 · 0.0s · 2 B → 6.log · grep -rlE 'Instant::now\(\)|SystemTime::now\(\)' packages/… (96 chars)
  ```
  Both logs read `0`.
- Reading: the clock's surface is untouched; the invariant holds. Nothing is sent upstream.

## Engine behaviour — three changes in every document, as upstream wrote them
Each is witnessed by upstream's own tests, run as their own targets (`evidence/counts.md`):

1. **`:focus-within` matches the focused element and its DOM ancestors** (`stylo.rs:477-479` reads
   `ElementState::FOCUS_WITHIN`, where it read `false`; the stub probe, entry 9, reads `0`). Witnessed by
   `focus_within.rs`: `focus_within_matches_the_ancestors_and_clears_on_blur`,
   `focus_within_matches_a_display_contents_ancestor`,
   `focus_within_matches_when_focus_is_set_before_the_first_layout_pass`,
   `removing_the_subtree_containing_the_focus_clears_the_ancestors`, `moving_the_focused_subtree_takes_the_state_along`,
   `moving_the_focused_subtree_out_of_the_document_clears_the_state` — each `ok`.
2. **A node outside the document is not focused** (`set_focus_to` returns `false` before the tracing line,
   `document.rs:1800-1803`). Witnessed by `focus_within.rs`: `a_node_outside_the_document_cannot_be_focused ... ok`.
3. **A focus set or cleared asks the shell for a redraw** (`set_focus_to` and `clear_focus` call
   `request_redraw()`; the Tab arm's own redraw is removed). Witnessed by `focus_redraw.rs` on its counting
   `RecordingShell`: `focusing_a_node_requests_a_redraw`, `refocusing_the_same_node_requests_nothing`,
   `blurring_requests_a_redraw`, `tabbing_to_the_next_node_requests_a_redraw` — each `ok`, every exact count held on
   our engine (research.md's open question answered: no other redraw site of the fork moves them).

Headlessly, no harness, driver or stand document sets a shell provider, so the added redraws are the default no-op; the
windowed half is witnessed by `focus_redraw.rs` alone — the windowed smoke (entry 30, green, `service.name=seven_guis`
logged, up to the 10 s bound) cannot see a focus ring or a `:focus-within` style. A focus move still marks exactly
the two nodes (`changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing ... ok`, entry 12): `FOCUS_WITHIN`
writes go through `snapshot_node_and`, which marks only `FOCUS | CHECKED`.

## The take's line map, for the wrap's re-pointing
From `git diff --cached -U0 HEAD` after the take (old → new line numbers).
- `document.rs` 3604 → 3653: hunks `@@ -958,0 +959,15 @@` · `@@ -1764,0 +1780,5 @@` · `@@ -1765,0 +1786 @@` ·
  `@@ -1778,0 +1800,4 @@` · `@@ -1798,0 +1824,19 @@` · `@@ -1800,0 +1845,5 @@`. Old 1–958 unchanged; 959–1764 +15;
  1765 +20; 1766–1778 +21; 1779–1798 +25; 1799–1800 +44; 1801–3604 +49. `snapshot_node_and` 1635 → 1650,
  `clear_focus` 1759 → 1774, `set_focus_to` 1772 → 1793.
- `events/keyboard.rs` 300 → 296: `@@ -29 +28,0 @@` · `@@ -41,3 +39,0 @@`. Old 1–28 unchanged; 29 and 41–43 removed; 30–40 −1; 44–300 −4.
- `mutator.rs` 1882 → 1921: `@@ -17,0 +18 @@` · `@@ -732,0 +734,6 @@` · `@@ -739,0 +747,5 @@` · `@@ -797,0 +810,27 @@`.
  Old 1–17 unchanged; 18–732 +1; 733–739 +7; 740–797 +12; 798–1882 +39 (git's diff aligns the blank-line run after
  our `if`, so it places upstream's six lines one line later than the resolution wrote them). Our changed-set `if`
  reads `HEAD:mutator.rs:730-732` — research and the plan cite 731–733, one line off — and now 731–733, upstream's
  record 734–739.
- `node/node.rs` 2117 → 2140: `@@ -735,0 +736,23 @@`. Old 1–735 unchanged; 736–2117 +23.
- `stylo.rs` 1518 → 1520: `@@ -477 +477,3 @@`. Old 1–476 unchanged; 477 → 477–479; 478–1518 +2.
