# Session Handoff

**Last Updated:** 2026-11-25T10:25:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `c34877ccd`)
**Status:** clean
**Last Commit:** 2026-11-25-virtual-nodes-as-elements — virtual nodes as elements: a canvas's virtual nodes in the accessibility tree, snapshot and diff, taking `click` and refused like elements (see `git log -1`)

## Position
- Done: 2026-11-25-virtual-nodes-as-elements.
  - A canvas's taken virtual nodes are children of its accessibility node, and the snapshot and its diff read them, with their `{owner}///{key}` id and declared name.
  - `click` alone takes a virtual id. It acts at the node's region and is refused in the one order.
  - The graph, actions, lint and exploration hold none.
  - CI#45580739068 is green 17/17 on `c34877ccd`.
- Next: «Virtual nodes in every read» (v040-11), Epoch 4's first markerless entry. It carries one CARRY from this wrap.

## Work done
- Wrap `.andromeda/runs/2026-11-25T09-46-09-wrap/`: 35 master operations landed in 9 TXs (3858–3866), over six masters (all but layout-templates).
- One answer card, Q1: F3 A, 10 operations. The founder confirmed it in his own words, «Подтвердить все 10 (Recommended)» (inputs#I7).
- 31 leaf findings were applied across 16 leaves, none by hand and none declined. One Tier 2 learning was added (`testing.md`).
- Route: 4 CARRYs pinned, 0 residuals.

## Drift resolved
- These land ratified on the founder's own word (inputs#I2, inputs#I7):
  - F3 A — the crossing: security-plan `decision/virtual-node-crossing` (new) and the retire of `absence/virtual-nodes-on-carriers`, plus the reserved-form rows of four masters;
  - F11 A — architecture `decision/declaration-change-marks-nothing`, which replaces the retired provisional `…-declaration-marks-nothing`;
  - F8 A with F2 A — architecture `contract/nodes-widgets/virtual-nodes` and a11y-plan `contract/snapshot/enabled`.
- These land with a PROVISIONAL mark in their sidecar entries, resting on a delegate's word:
  - F9 A, F10 B, F12 A: the tree, snapshot, `click`, `covered` and `not-found` rows;
  - the V2 ruling: test-plan `decision/inert-guard-dropped`, state provisional.
- Rejected:
  - by check 2: 34 repeats of one fact in another master;
  - by fit `not_master`: the tree-id add, and test-plan's lengthened `reserved-graph-id-form` set. Main re-set that row shorter, and it fits.
- Directive point 4: this chunk wrap's letter has no door to end another chunk's sidecar-entry marks. The F2 A marks of «Virtual nodes declared» (entries `amd-cbkr7gme`, `amd-v6evyr7o`) stand for a 0-pending wrap's `rec.py amend` arm.

## Notes
- v040-11 stays pooled and unclaimed. Its ledger note is written at P7.3.
- Carried, with a line each in `escher-0.5.0-incubator/deferred-from-0.4.0.md`:
  - `cry-q5re2cbp` → «Virtual nodes in every read»: only `click` takes a virtual id; `clickable` reads false.
  - `cry-llqauv26` → «Circle Drawer's circles as virtual nodes»: `covered` by another virtual node.
  - `cry-kou37dij` → the same entry: the carriers are not read on the binary (P5 card row 1).
  - `cry-2gya4rcg` → «Pan and zoom»: a rotated or scaled canvas read unrotated and unscaled.
- Carried from before: `cry-tjzqz3hz`, the two drag-and-drop gaps, on «Canvas keyboard path». Also security-plan `dec-kxrtstbq`'s `verified_was`, and `host-linux.md` 9 template lines behind.
- Left in place on the operator's word: every probe branch, worktree, scratch clone and build directory.
- Last failed command: none.

## Deferred learnings
1 learning analyzed but not applied:
- recurrence: `testing.md:22` already rules that a change of a CI leg's file count sweeps the literal pins of that count. The `fast` leg's second firing still read `test_the_leg_counts_are_stated_in_words` `20 != 19`.
Review with `/andromeda-wrap-session --review` if any should be applied.

## Session End Status
Completed normally at 2026-11-25 12:57:19
