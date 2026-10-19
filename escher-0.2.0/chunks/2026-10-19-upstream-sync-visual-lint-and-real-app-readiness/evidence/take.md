# The take — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Made by /andromeda-implement, run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19T19:17Z, onto the base
`67f17b9d632ceb66f6c01b347d9f9ba036dd29b6` (build/escher-0.2.0). Two no-commit cherry-picks, in this order; the merge
base with upstream stays `23b9b26a9` (inputs#I2: the operator's answer to F1, «Cherry-pick two (Recommended)», as
delegate, PROVISIONAL).

## The two commits
1. `17de049b11095808e0d093be2a8f0520e4dcbc0e` "Bump WPT pin to 9d8acee (#1228)" (Nico Burns, 2026-10-18) —
   `git cherry-pick --no-commit 17de049b1…`: exit 0, clean; `wpt/WPT_COMMIT` alone (1 file, +1 −1).
2. `38ca4e777513607c7dcb659748380e7a650b018f` "Support the :focus-within pseudo-class (#1226)" (Nico Burns,
   2026-10-18) — `git cherry-pick --no-commit 38ca4e777…`: exit 1. `document.rs`, `events/keyboard.rs`,
   `node/node.rs`, `stylo.rs`, `tests/blitz-tests/tests/all.rs` and the two new test files auto-merged; one conflict,
   `CONFLICT (content): Merge conflict in packages/blitz-dom/src/mutator.rs` (8 files, +428 −5 upstream).

## The conflict and its resolution — the take's one non-additive edit
In `add_children_to_parent` (`packages/blitz-dom/src/mutator.rs`; our `if` at `HEAD:mutator.rs:730-732`, which
research and the plan cite as 731–733), markers at lines 731–742 of the conflicted file:

```
<<<<<<< HEAD
        if new_parent_is_in_document && !child_ids.is_empty() {
            self.doc.changed_nodes.insert(parent_id);
        }
=======

        // If one of the moved subtrees contains the focused node, the ancestors it is
        // detached from stop matching :focus-within. Record them while the parent links
        // are still intact; the state is transferred after the children are reattached.
        let mut focus_within_old_chain: Option<Vec<NodeId>> = None;

>>>>>>> 38ca4e777 (Support the :focus-within pseudo-class (#1226))
```

Resolved by keeping both sides in that order, markers removed and no other line added or removed: our three-line
changed-set `if` first, then upstream's blank line, three comment lines, the `focus_within_old_chain` declaration and
its trailing blank line (now `mutator.rs:731-739`). Our changed-set insert for the new parent stays, so the changed
set's contract (marked by mutations of in-document nodes) is unchanged; upstream's chain record at the loop's head and
its transfer after reattachment auto-merged beside our per-child insert.

Then `git add packages/blitz-dom/src/mutator.rs` and `git cherry-pick --quit` (exit 0). `git status` after the quit
(code paths; the phase files unstaged and the run dirs untracked besides):

```
On branch build/escher-0.2.0
Your branch is up to date with 'origin/build/escher-0.2.0'.

Changes to be committed:
	modified:   packages/blitz-dom/src/document.rs
	modified:   packages/blitz-dom/src/events/keyboard.rs
	modified:   packages/blitz-dom/src/mutator.rs
	modified:   packages/blitz-dom/src/node/node.rs
	modified:   packages/blitz-dom/src/stylo.rs
	modified:   tests/blitz-tests/tests/all.rs
	new file:   tests/blitz-tests/tests/focus_redraw.rs
	new file:   tests/blitz-tests/tests/focus_within.rs
	modified:   wpt/WPT_COMMIT
```

No cherry-pick is in progress: `.git/CHERRY_PICK_HEAD` and `.git/sequencer` are absent.
`git diff --quiet 17de049b1 -- tests/blitz-tests/tests/all.rs tests/blitz-tests/tests/focus_within.rs
tests/blitz-tests/tests/focus_redraw.rs wpt/WPT_COMMIT` exits 0: those four files are byte-identical to upstream's.

## Gate-driven edits
None. Every gate read green on the first firing of the block (`entries 34 · green 29 · red 0 · recorded 1 · timeout 0
· not-run 4`, the four not-run the operator entries), so no edit beyond the resolution was made and the preservation
guard's exclusions are the plan's as written.

## The exact-take and preservation entries
From the trail `.andromeda/runs/2026-10-19T19-16-25-implement/gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`:

```
  1 probe       green · exit 0 · 0.01s · 0 B → 1.log · diff <(git diff --src-prefix=a/ --dst-prefix=b/ 67f17b9d63… (315 chars)
  2 probe       green · exit 0 · 0.01s · 0 B → 2.log · git diff --quiet 67f17b9d632ceb66f6c01b347d9f9ba036dd29b6 … (555 chars)
```

Entry 1 reads no output: the code paths differ from the base by upstream's added and removed lines of the two
commits and nothing else, our changed-set insert kept. Entry 2 reads exit 0: nothing outside the take's nine paths
moves.

## Commit trailers for the operator's pre-CI commit
The pre-CI commit's body carries both, the take being two cherry-picks folded into one commit:

```
(cherry picked from commit 38ca4e777513607c7dcb659748380e7a650b018f)
(cherry picked from commit 17de049b11095808e0d093be2a8f0520e4dcbc0e)
```
