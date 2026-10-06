# Adaptation record — the founder's four intent rulings, applied at the 2026-10-06-snapshot-model wrap

Authority: the founder's own word, given as this wrap's directive (2026-10-06). Two points the directive left open
were answered by the founder at this wrap, in one question card: the slot of the new route entry ("Next entry") and the
way v010-16 enters the ledger ("Add it now", with the read-back below).

## Ruling 1 — additivity is a preference, not a goal
- `escher-0.1.0/intent.md` §Principles, "Upstream stays in reach": adds that additivity is a preference, not a goal, and that the upstream code is changed directly where staying additive would force contorted logic.
- `escher-0.1.0/working-route.md`: the three markerless sync entries (Epochs 4, 5, 6) read "our changes kept mostly additive". The three frozen sync lines (Epochs 2 and 3, and none in 1) are untouched: a frozen line is never edited.
- Spec masters: no edit. grep `additive` over the seven masters and the registries reads 0 hits.

## Ruling 2 — the stand is the proof, plus minimal fixtures beside it
- `intent.md` §Principles, "The stand is the proof": adds the minimal fixture beside the stand for a case the tasks lack, an element covered by another named as one.
- `requirements.md` header: "on the 7GUIs stand, or on a minimal fixture beside it where the tasks lack the case".
- `working-route.md`: `CARRY:` on "Refusal detection" (the covered case is proven on such a fixture) and on "Stand requirement sweep" (the sweep counts the fixtures with the tasks).
- Ledger: a dated note on v010-11, whose acceptance still opens "On the stand" (written at P7.3 through `matrix.py note`).
- Leaves: CLAUDE.md `modules` (seven_guis) and `.claude/docs/services/seven_guis.md` name the fixture clause.
- Not decided here: where the fixtures live and what they are called. That is the owning chunk's plan.

## Ruling 3 — ids stay stable across edits of the app's code
- `intent.md` §Findings 1, EXPECT: adds "stable across edits of the app's code".
- `requirements.md`: new line `v010-16 · Id stability across code edits`.
- `working-route.md`: new markerless entry "Id stability across code edits … (v010-16)", placed as the next entry, ahead of "Snapshot state fidelity" (the founder's slot answer), with a `CARRY:` naming today's id rule and a labelled hypothesis about which edits move an id.
- Ledger: v010-16 appended to `escher-0.1.0/verification-matrix.json` as `planned`, unclaimed, by a scripted append (the ledger tool has no verb that creates an entry). The script first proved that re-dumping the parsed file reproduces it byte for byte, then appended; `git diff` reads 14 insertions and 0 deletions. Read back through the tool: `matrix-read-v010-16.txt` (`show --id v010-16` beside a sibling, `show --unclaimed`, `coverage`) — every field the sibling prints is printed for v010-16, it is listed in the unclaimed pool, and coverage reads `verified 2/16`.
- Not decided here: which edits must keep the id. The acceptance is outcome-level and is concretized at claim.

## Ruling 4 — the wrong-call bar starts in 0.2.0
- `intent.md` §Findings 9, EXPECT: 0.1.0 sets no bar on the wrong-call count; its run is the baseline; a bar starts in 0.2.0.
- `requirements.md` v010-15: the parenthetical names the baseline and the 0.2.0 bar; the requirement's own words are unchanged, so it still equals the ledger's `requirement` field.
- `working-route.md`: `CARRY:` on "Cold-agent test".
- `.andromeda/residuals.md`: one `open` entry, target 0.2.0.
- Ledger: a dated note on v010-15 (written at P7.3 through `matrix.py note`).
- Spec masters: no edit. test-plan §3 already records `wrong_calls` as "recorded, never deciding".

## Route edits from the chunk's own outcome (factual, no ruling needed)
- `CARRY:` on "Compact snapshot serialization": the first wire form of the snapshot is a crossing for ids, names and a text control's text, to be escalated then.
- `CARRY:` on "Act by id": v010-03 is unclaimed and named on no markerless entry; its driver leg lands there.
