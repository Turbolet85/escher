# P5 review — 2026-11-25-virtual-nodes-in-every-read «Virtual nodes in every read» (v040-11)

**The plan:** `doc:2026-11-25-virtual-nodes-in-every-read/plan` rev 2. To open it in an editor:
`.andromeda/cache/docs/2026-11-25-virtual-nodes-in-every-read/plan.md`. No plan stands in the chunk's folder, and
writing in that file changes no record until a `put`. For a diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-virtual-nodes-in-every-read`.

**Answers it rests on** (`inputs#I2`, snapped from the relay):
- F13 A is the founder's own word, relayed verbatim.
- F14 A to F22 A are the escher overseer's delegate answers, so they are PROVISIONAL.
- F19 A carries its check.

## Goal
A canvas's virtual id, and no declared name, now crosses in five reads:
- the UI graph;
- `actions`;
- the coverage line;
- the `layout` answer;
- an explored path's kept check.

The canvas answer and the lint still hold none. How each read takes a virtual node:
- **Graph** (F14 A). One node per virtual node, owned by its canvas, with no handlers or reads.
- **`actions`** (F15 A, F18 A). It lists all six verbs for a virtual node, each with the cause its call returns.
  `keyed` reads false.
- **`layout`** (F16 A, F17 A). It takes a virtual id. The answer is placed by `app` in its canvas, at the region's
  offset, and the size cause is `layout`. `type`, `scroll`, `screenshot` and `canvas` keep `not-found`.
- **Coverage** (F19 A). A virtual node carries no handler. One check shows that a replayed virtual click reaches the
  canvas's own `onclick`, counted under the canvas's id.
- **Lint** (F20 A). It still reads no virtual node. Its guard now has an input it can read red on.
- **`clickable`** (F21 A) stays false.
- **Exploration** (F22 A) expands a virtual node like any listed node.

## Top acceptance criteria
- The graph holds every snapshot node, each virtual node owned by `vn-canvas`.
- `actions`' 5 × 6 cause table equals what each call returns.
- `layout` on a virtual id:
  - reads `by: app`, with `at` (7, 7) for `a` and (47, 27) for `e`;
  - matches the snapshot bounds and the canvas's shifts;
  - changes nothing, and reads `stale` or `not-found` as any id does.
- The graph, actions and layout answers carry the id and none of the declared names.
- Coverage: a replayed virtual click leaves `(vn-canvas, click)` exercised, and a check without that click lists it.
- Kept and explored checks of the new reads replay green.
- The stand's pins read unmoved: the no-`///` assertions, `ACTIONABLE_AT_BOOT`, `CLICKABLE`, the cross-OS digests and
  the committed checks.
- Controls W1–W7 and L1 each read red with their edit.
- The final HEAD's CI read is green.

## Touchpoints
- **New:** `tests/blitz-tests/tests/virtual_nodes_in_every_read.rs` (7 cases, two of them `cfg(unix)`).
- **Modify:**
  - `graph.rs` and `actionable.rs` (docs);
  - `execute.rs`, `actions.rs`, `layout.rs`, `refusal.rs`, `schema.rs` and `coverage.rs` (docs);
  - `fixtures/mod.rs` (`declares_five` is shared, plus a listened canvas page at (40, 30));
  - `virtual_nodes_as_elements.rs` and `virtual_nodes_declared.rs` (guards restated);
  - `docs/capabilities.md`.
- **Not touched:** `snapshot.rs`, `reached.rs`, the `Widget` trait and `lint.rs`'s code.
- **Counts:** none moves — 16 verbs, 8 causes, a11y leg 20, stand-repeat 17 named targets.

## Test commands (14 entries)
- **Baselines on the untouched tree:**
  - Entry 1, the manifest/lock probe from `b0e2d1c31`: green. It was controlled both ways in a scratch clone:
    `control-probes.md`.
  - Entry 5, the new target: red, an absent input.
  - Entry 6, the two virtual-node targets: green, 12 passed.
  - Entry 7, the fifteen stand companions: green, 96 passed, 2 ignored.
- **Seen green at the last wrap:** the two crates' unit tests, the seven_guis process checks, and the `build`, `a11y`,
  `fast` and `doc` legs.
- **Operator pass:** hygiene, push, then the CI read (`--wait 2700`; the base's CI wall was 2067 s). The CI read names
  the two `cfg(unix)` sites: no windows leg witnesses them.
- **Controls:** implement's one-shot `gate.py control` runs, `--timeout 3600`, in the tool's own clone. No step makes or
  deletes a worktree, `target/` or a build directory.

## Research
The forks were answered before the plan, so research was synced to them at P4 (rev 2). `snapshot.rs` and `reached.rs`
left its list, and `virtual_nodes_declared.rs` joined it. Its stale fork numbering is corrected.

## Capabilities
None claimed. v040-11's acceptance names Circle Drawer's circles on the stand and keyboard reach. Two later entries
carry them: «Circle Drawer's circles as virtual nodes» and «Canvas keyboard path».

After the word, P5 writes v040-11 one dated ledger note of what this chunk proves. No concretization preview applies.

## Expected amendments (the wrap's)
- **security-plan `decision/virtual-node-crossing`**, with the rows riding it. This is F13 A, `rul-izni57qv`, the
  founder's own word relayed verbatim, so the wrap halts once, as the answer card.
- **The other rows**, in architecture, a11y-plan, design-system, test-plan and obs-plan. These rest on delegate answers
  and land PROVISIONAL.
- **F9 A, F10 B and F12 A.** Their marks stay listed for the epoch boundary's batch (`rul-3ndrhilc`).

## Beyond the plan
1. **Owner of F19 A's stated limit** — a virtual node carries no handler of its own, so coverage counts only its
   canvas's listener. Proposed: a CARRY on «Circle Drawer's circles as virtual nodes» (its canvas carries `onclick`).
2. **Owner of F20 A's stated limit** — the lint reads no virtual node: no overlap, clip or off-screen finding on a
   region. Proposed: a CARRY on «Circle Drawer's circles as virtual nodes» (its circles overlap).
3. **Decided limits, no owner** — these stay stated limits in `docs/capabilities.md`, decided and not owed:
   - `type`, `scroll`, `screenshot` with an id and `canvas` read a virtual id `not-found` (F16 A);
   - a virtual node's `keyed` reads false (F15 A);
   - its `clickable` reads false (F21 A).

Apply? (yes / review / cancel)
