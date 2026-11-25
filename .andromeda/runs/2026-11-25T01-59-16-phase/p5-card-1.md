# P5 review — 2026-11-25-virtual-nodes-declared «Virtual nodes declared»

**The plan to open in an editor:** `.andromeda/cache/docs/2026-11-25-virtual-nodes-declared/plan.md`, the render of
`doc:2026-11-25-virtual-nodes-declared/plan` rev 2.
- No plan stands in a chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-virtual-nodes-declared`
  is a diff of the chunk's records.

**Whose word this plan rests on.** Every fork was answered by a delegate (inputs#I2), and none by the founder:
- F2 A is overseer1's word.
- F1 B′, F3 A (defer), F4 A, F5 A, F6 A and the duplicate rule are the escher overseer's words.

Every row resting on one of them lands PROVISIONAL. No escalate pattern is met by an option taken (`rul-izni57qv`,
`rul-3ndrhilc`, `rul-72xhfjoz` read): nothing crosses, no mark is removed, and there is no upstream merge.

## Goal
An app declares virtual nodes inside a canvas widget.
- **The declaration** is a defaulted pull read on the engine's `Widget` trait, `virtual_nodes(&self)`. It returns
  `VirtualNodeDeclaration`s, each holding a key, an AccessKit `Role`, a name and a content-box region. Both items are
  in blitz-dom behind `custom-widget`.
- **The read.** `DioxusDocument::virtual_nodes(canvas)` reads them in process. It validates them in a fixed order:
  - each refused declaration reads its index and a closed reason: `EmptyKey` · `KeyHoldsSlash` · `DuplicateKey` ·
    `EmptyName` · `RegionOutside`;
  - each taken node gets `{owner}///{key}` through `virtual_node_id`;
  - before the first layout the read answers `NotLaidOut`, and a non-canvas or stale node answers `NoCanvas`.
- **Nothing crosses.** There is no carrier, verb, log site or span, no changed-set mark and no invalidation signal.

**F2 A's bounds are stated in the plan's Goal and held by three probes:**
- additive only, the type and the defaulted method plus two re-export line pairs;
- behind the feature;
- nothing sent upstream;
- STOP if it cannot stay additive.

## Top acceptance criteria
- **The bounds.** blitz-dom loses 0 lines from base `b7f61dd0a`. The 15 files holding a `Widget` implementation and
  every manifest and the lock are unchanged (probes 1 to 3).
- **The read.** Each refusal arm, `NotLaidOut` and `NoCanvas` are witnessed by a named unit case.
- **The frame.** On a padded, bordered canvas, in both layout modes, a region that fits the border box but not the
  content box is refused.
- **Nothing on the snapshot or graph.** With a declaring canvas, `snapshot` and `graph` hold no `///` and no planted
  name. The needles are live in the in-process read.
- **The controls.** K1 to K14 each read `holds` with their case `failed`, and their red readings are in evidence.
- **The gates.** `fast` and `doc` are green, and CI on the operator pass's final HEAD reads `verdict: green`.

## Touchpoints
- **New:**
  - `packages/dioxus-native-dom/src/virtual_node.rs`;
  - `tests/blitz-tests/tests/virtual_nodes_declared.rs`.
- **Modified:**
  - `packages/blitz-dom/src/node/custom_widget.rs`, `node/mod.rs` and `lib.rs`, added lines only;
  - `packages/dioxus-native-dom/src/lib.rs`;
  - `packages/dioxus-native-dom/src/element_id.rs`, doc comments only.

## Test commands (12 entries)
| # | entry | baseline |
|---|---|---|
| 1 | probe: manifests and lock unchanged | green; controlled both sides |
| 2 | probe: blitz-dom removed lines = 0 | green; controlled both sides |
| 3 | probe: the 15 implementor files unchanged | green; controlled both sides |
| 4 | `build` | seen green |
| 5 | `cargo test -p dioxus-native-dom --locked --lib` | green, 131 passed; vacuous for the new cases |
| 6 | `cargo test -p blitz-tests --locked --test virtual_nodes_declared` | red, no such target: the chunk's own |
| 7 | the seven companions | green |
| 8 | `fast` | seen green |
| 9 | `doc` | seen green |
| 10 to 12 | operator: hygiene, push, `ci.py conclusion` | — |

The probe controls are in `.andromeda/runs/2026-11-25T01-59-16-phase/control-probes.md`. They ran in a scratch clone
in the session scratchpad, which is left in place.

**Absent, with reasons in the plan:** smoke, live leg, self-verify, audit, stand-repeat, a11y leg, local coverage.

## Leans to confirm (decided by an artifact, shown here for your word)
- **The re-export lines.** The blitz-dom re-export of the type is two added line pairs, in `node/mod.rs` and
  `lib.rs`. Upstream's `pub use custom_widget::{…}` list is not edited. I read «additive only … no upstream line
  edited beyond the trait and the type» (inputs#I2) as allowing added re-export lines and forbidding edited ones.
- **No dioxus-native re-export.** The «Circle Drawer» entry adds what an app needs.
- **The module is ungated.** `virtual_node.rs` is ungated like `signal_registry`: blitz-dom's `custom-widget` is always
  on in dioxus-native-dom.
- **The fixed rule order.** The rules apply as key, then duplicate, then name, then region. A duplicate's first
  claimant keeps the key even when it is refused later, after the author-key precedent.
- **The absence check.** Its control is the live-needle precondition. Its mutation control is owed by a later entry
  (Beyond the plan, row 3).

## Concretization preview
None. No capability is claimed. v040-11's acceptance spans five entries (working-route lines 35-39), so it stays
pooled and this chunk links nothing in the ledger.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | **F3, the names' crossing** (whether a virtual node's name and key ride today's accessible-name and stable-id exits, the adapter alone, or neither) stays the founder's fork. It is deferred by the escher overseer's F3 A, which is no answer to the fork, and is asked of the founder before that entry starts. | «Virtual nodes as elements — in the accessibility tree and from it the snapshot and diff; taking `click`; refused covered, off-screen, disabled or stale (v040-11)» — a CARRY on that entry |
| 2 | **F1's invalidation half and F6's changed-set mark**: a declaration change signalling its reader and marking its canvas in the changed set. This amends arch `decision/changed-set-write-sites` and a11y `decision/changed-set-marks`, both ratified. It is left by F1 B′ and F6 A to the first reader that refreshes. | the same entry — a CARRY on it |
| 3 | **A mutation control of «a declaring canvas puts nothing on the snapshot or graph»**, owed once code exists that could carry a virtual node to a carrier. | the same entry — a CARRY on it |

Apply? (yes / review / cancel)
