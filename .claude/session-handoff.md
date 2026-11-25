# Session Handoff

**Last Updated:** 2026-11-25T14:19:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `808f9611f`)
**Status:** clean
**Last Commit:** 2026-11-25-virtual-nodes-in-every-read — virtual nodes in every read: a canvas's virtual nodes in the UI graph, actions, layout, coverage reach and exploration (see `git log -1`)

## Position
- Done: 2026-11-25-virtual-nodes-in-every-read.
  - The UI graph, `actions`, `layout` and exploration read a canvas's virtual node by its `{owner}///{key}` id, never
    its declared name. `click` and `layout` take a virtual id; the lint and the canvas answer hold none.
  - A replayed virtual click counts its canvas's own listener under the canvas's id.
  - CI#45619756069 is green 17/17 on `808f9611f`.
- Next: «Circle Drawer's circles as virtual nodes», Epoch 4's first markerless entry. It now carries seven CARRYs,
  three from this wrap.

## Work done
- Wrap `.andromeda/runs/2026-11-25T13-41-29-wrap/`: 18 master operations landed in 10 TXs (3929–3938), over five
  masters (architecture, security-plan, design-system, test-plan, a11y-plan).
- One answer card, Q1: 3 security-plan operations on F13 A. The founder confirmed it in his own words,
  «Подтвердить все 3 (Recommended)» (inputs#I5).
- 26 leaf findings applied across 10 leaves, none by hand and none declined. No curation entry written.
- Route: 3 CARRYs pinned, 3 residuals opened.

## Drift resolved
- These land ratified on the founder's own word (F13 A, inputs#I2; the card's confirm, inputs#I5):
  - security-plan `decision/virtual-node-crossing` re-stated (rule and guard), `contract/stable-element-id/virtual-node-form`;
  - the id's served reads in architecture (`ui-graph/graph`, `dioxus-bridge/virtual-node-id`,
    `stable-element-id/virtual-node-id`), design-system `actions-line/nodes`, test-plan `element-id/virtual-form` and
    `decision/reserved-graph-id-form` (narrowed to the lint and the stand), a11y-plan `ui-graph-one-node-per-snapshot-node`
    and `node-id/reserved-separator`.
- These land PROVISIONAL, resting on the escher overseer's delegate word (F14 A–F22 A): the `not-found` rows of
  architecture and test-plan, `ui-graph/owner`, `driver-session/canvas`, `handler-coverage/canvas`, the new
  `driver-session/layout-virtual-node` and `actions-verb/virtual-node`, security-plan `actions-answer/verbs`, the
  reserved-form guard. `decision/virtual-id-verb-table` lands provisional on implement's own choice.
- Rejected: 20 adds or sets by check 2 (one fact twice across masters), 4 by check 4 (the a11y graph row claimed
  coverage holds virtual nodes; two canvas-coverage rows dropped a still-true clause; a still-true decision retired),
  2 by check 5, 2 by fit `not_master` (obs-plan's no-span add; the `keyed-wire-form` except).

## Notes
- v040-11 stays pooled and unclaimed. Its ledger note is written at P7.3.
- Carried, with a line each in `escher-0.5.0-incubator/deferred-from-0.4.0.md`, all to «Circle Drawer's circles as
  virtual nodes»:
  - `cry-dvektvzh`: the windows unused import at `virtual_nodes_in_every_read.rs:24`, its fix the import behind
    `#[cfg(unix)]` with the windows `cargo check --tests`;
  - `cry-a2jh4k7x`: coverage lists no virtual node (F19 A);
  - `cry-5zhvg5ed`: the lint reads no virtual node (F20 A).
- Residuals, open, each with an incubator line: `res-qytby7yb` (four verbs read a virtual id `not-found`),
  `res-7ohmiyz4` (`keyed` false), `res-77aoengs` (`clickable` false).
- Carried from before: `cry-tjzqz3hz`, the two drag-and-drop gaps, on «Canvas keyboard path». Also security-plan
  `dec-kxrtstbq`'s `verified_was`, and `host-linux.md` 9 template lines behind.
- Left in place on the operator's word: every probe branch, worktree, scratch clone and build directory.
- Last failed command: none.

## Deferred learnings
2 learnings analyzed but not applied:
- recurrence-despite-learning: `testing.md` 2026-10-10 (search `tests/` for the call builders) — `stand_act_spans`
  pinned the lifted refused `layout` and read red at implement's first full firing.
- recurrence-despite-learning: `testing.md` 2026-10-19 (a `cfg(unix)` change checked by a windows `cargo check
  --tests`) — not run before the push; the unused import reached CI as a warning.
Review with `/andromeda-wrap-session --review` if any should be applied.
