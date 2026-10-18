# P5 review, card 2 — 2026-10-18-engine-debts «Engine debts» (v020-03)

This card folds the operator's review of `.andromeda/runs/2026-10-18T02-38-20-phase/p5-card-1.md` (relay-3.md, snapped as
inputs#I3) and his dialog answer «Gated claim (Recommended)». Everything on card 1 stands except what is listed here.
The plan was re-checked in full after the fold:
- the gate fence parses, 31 entries;
- planlint reads 0 hits;
- research's lists parse, 22 items;
- inputs: 3 entries, 0 unparsed;
- `new` 26 = `baseline` 26.

## What changed
- **v020-03 keeps the founder's own word as its condition** (requirements.md:13, «by his word»; never weakened).
  - The criterion reads «carries the founder's recorded answer». The delegate's answers (inputs#I2) stand first,
    PROVISIONAL.
  - The cap is claimed GATED: `matrix.py claim --gated-file`, its block
    `.andromeda/runs/2026-10-18T02-38-20-phase/gated-v020-03.md`: «the founder's own word discharging this chunk's
    PROVISIONAL batch at its wrap, the Epoch 2 boundary: the nine fork answers the operator gave as delegate
    (inputs#I2), the seven engine fixes and the `switch` role mapping among them (inputs#I3)».
  - The ledger then refuses v020-03's ref until a clearance is recorded (`implement --cleared-file`), so green checks
    alone never verify it.
- **The gated criterion:**
  - **Step 13** writes `evidence/founder-word.md` from his recorded word: one row per fork he ratifies, the `switch`
    mapping under F2. It runs only in a gated `/andromeda-implement`.
  - **Gate entry 26** counts 9 such rows. It is deferred (`defer = "gated: …"`) and fired only through `--void` once
    the block reads cleared.
  - **Its controls** ran in the scratch clone only: absent red, 9 rows green, 8 red (`p5-controls.md`).
- **At the wrap:**
  - If the founder rules at the discharge card, the cap verifies through the gated path.
  - If he has not ruled by the coverage gate, the gated arm holds v020-03 unverified and the master record goes
    `gated`. His word then runs step 13, and a later wrap flips it. The cap is never un-claimed or deferred meanwhile.
- **The `switch` role mapping** is named in the PROVISIONAL batch itself (Goal; the block).

## Concretization preview — v020-03 (folded)
- **Now:** «Each of the nine recorded engine debts carries the founder's recorded answer, and its outcome matches that
  answer: a fixed debt is proven …; a pinned debt has its limit stated and a check that holds it; a kept debt stands as
  a residual line.»
- **Proposed** (`.andromeda/runs/2026-10-18T02-38-20-phase/acceptance-v020-03.md`): the founder's recorded answer stays
  the condition, gated on his own word. The fixes are named, each with its check, its control row (8) and its
  merge-surface row (7); the two pins are named; none is kept. One clause is added: a fork he reworks is not this
  acceptance's, and the cap stays claimed and unverified until the reworked outcome is built.
- Nothing is weakened: the outcome clause and the evidence are as card 1 proposed, and the condition is the founder's
  word.

## Beyond the plan (rows 1–4 accepted on card 1, inputs#I3; unchanged)
| # | Proposal the word approves that neither plan.md nor the matrix claim holds |
|---|---|
| 1 | The merge surface of the seven engine fixes is owned by a CARRY on the working-route entry «Upstream sync ahead of determinism and traces» (escher-0.2.0/working-route.md:23), written by this chunk's wrap (route-resolve). |
| 2 | The PROVISIONAL marks of the seven fixes and the `switch` mapping are listed for the founder at this chunk's wrap, the Epoch 2 boundary, under «Provisional discharge» (playbook:48): his own word ratifies or reworks each, and that word is v020-03's gated clearance. |
| 3 | If step 8 measures the `visible_region` misreading, that pinned limit gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |
| 4 | The F1 focus-fixup limit (a node made inert keeps focus) gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |

Apply? (yes / review / cancel)
