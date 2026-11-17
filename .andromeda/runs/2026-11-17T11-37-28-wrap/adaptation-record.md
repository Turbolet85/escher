# Adaptation record — 2026-11-17T11-37-28-wrap (0-pending wrap)

Operator-requested route adaptation at the Epoch 5 boundary of escher-0.3.0. The rulings are the founder's,
given by dialog in the overseer session on 2026-11-17 and relayed verbatim by the overseer, snapped as inputs#I1
(`inputs/I1-epoch6-opening-founder-2026-11-17.md.txt`, source
`~/dev/projects/additional/escher-overseer/relays/epoch6-opening-founder-2026-11-17.md`). His words:
«Только json.rs (Recommended)», «Сразу после Upstream sync (Recommended)», «Открываем сейчас (Recommended)».
Anchor: «Upstream sync ahead of polish and ship» (`chk-alpfcbcr`) stays first. The boundary inputs: the Epoch 5
diagnosis (`.andromeda/runs/2026-11-17T08-58-13-evolve-diagnose/proposals.md`) and the code audit
(`.andromeda/runs/2026-11-17T09-07-31-code-audit/proposals.md`, ledger tx 1751).

## Items

1. A new entry, the corrective chunk: `packages/escher-driver/src/json.rs` split by answer family, placed directly
   after the upstream sync, no behaviour change, `document.rs` left whole (the founder's answers 1 and 2).
2. The audit's proposal M1 beyond `json.rs`: the split direction it offered for `packages/blitz-dom/src/document.rs`
   (3341 lines, +2 this epoch).
3. The diagnosis's items that are escher's own for this epoch's route: (a) the cross-OS half of the CI reds (P6 /
   X2: Test (macos) span check at 2026-11-16-errors-with-the-action, Test (windows) `stand_act_errors` at
   2026-11-16-error-text-in-the-answer); (b) its three single lessons (P3, «the single-occurrence causes»): raw probe
   logs with home paths refused at hygiene, a dotted field name in `tracing::warn!`, and the `VERBS.len()` pin a
   test search missed.

## Dispositions

1. **Applied** — tx 1753: `chk-br6huuk6` «Answer writer by family — the driver's answer writer split by answer family
   (caused, layout, snapshot, diff), every carrier's answers unchanged; Blitz's `document.rs` left whole», after
   `chk-alpfcbcr`, the second entry of «Epoch 6 — Polish & ship»; its `CARRY:` `cry-gvojtiwf` quotes the founder's
   two answers verbatim and cites the snapshot by its path. No requirement line added (no new function). Authority:
   the founder, relayed verbatim by the overseer, inputs#I1; the entry text confirmed by the operator, 2026-11-17
   («Insert as written (Recommended)»). Read back: `pins` one row, introducer `CARRY:`; `abstained` 0 rows.
2. **Recorded, no route edit** — `document.rs` is not split this epoch: it is upstream's file, and splitting it turns
   every later sync into a conflict (the founder's «Только json.rs (Recommended)», inputs#I1). The operator
   confirmed, 2026-11-17 («Record only, no route edit (Recommended)»).
3. (a) **Not placed** — each red had its cause found and fixed by its chunk's operator pass (151261549 for Windows;
   the errors-with-the-action pass for macOS), so no `WATCH:` (which is for a red with no cause found) and no
   `CARRY:` (no work left); the reds are not reproducible on this host, and the operator pass already reads every OS
   leg of CI; the diagnosis's direction for the narrower local block (P6 / X2) is the skills' side, not escher's
   route. The operator confirmed, 2026-11-17 («Not placed, state why (Recommended)»).
   (b) **Not placed** — each stands already as a written rule (the Tier 3 hygiene entry, `observability.md`
   2026-10-06, `testing.md` 2026-10-10), and the diagnosis keeps them prose-level; the route carries owned work,
   not reminders. The operator confirmed, 2026-11-17 («None placed, state why (Recommended)»).

Epoch 6 opens now (the founder's «Открываем сейчас (Recommended)»): the next step is `/andromeda-phase` for
«Upstream sync ahead of polish and ship». No `gated` record stands, so no premise to re-verify. Curation: the
conversation carried no correction, so P3 is not run.
