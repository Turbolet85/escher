# Adaptation record — the 2026-10-07T02-41-58 0-pending wrap

Path: Setup step 6, the no-op path (master 18 records, 18 complete, 0 pending, 0 gated). The tree at entry held
bookkeeping only: the handoff, `friction-log.ndjson`, `code-metrics.ndjson`, the evolve tool's own trail under
the previous wrap's run dir, and the untracked run dirs of the Epoch 3 diagnosis and code audit. No chunk, no
report, no fan-out, no gate.

## The request

word: "adaptation: add one route entry FIRST in Epoch 4, ahead of the upstream sync: a corrective chunk from the
Epoch 2 and 3 code audits. Items for your disposition: (1) the 7 mutants that survived in dioxus-native-dom
(Epoch 2 audit: element_id.rs place_children; dioxus_document.rs create_head_element,
flush_queued_mounted_events, Document::id twice, handle_event twice), each killed by a test in a new test file
or shown unreachable; (2) walk in element_id.rs at cognitive 26; (3) clone pairs between stand_*.rs grew 1 to 13
(the per-task control tables); (4) three stand test functions over cognitive 15. Also apply the deferred ugrep
learning." — the operator, 2026-10-07

The direction names the entry and its slot, which satisfies the trajectory gate for both. The four items were
handed over for the wrap's disposition; each disposition below is the wrap's, open to the operator's edit
before the entry is promoted.

## Route edit

`escher-0.1.0/working-route.md`: one entry, "Audit corrections", inserted as the first line under
`### Epoch 4 — Driver core`, with its `↓` separator, ahead of "Upstream sync ahead of the driver core". Two
lines added, none removed, no frozen line in the diff. The previous first markerless entry carried no
annotation, so nothing was re-pinned. Epoch 4 now holds 8 entries, all markerless.

Read back after the write: `route.py cursor` names the entry as next (markerless 20 of 38); `route.py pins`
lists its four `CARRY:` blocks; `upgrade.py detect` U14 reads 20 markerless entries with 0 introducers out of
form.

## Items and their dispositions

| # | Item | Disposition | Basis |
|---|---|---|---|
| 1 | The seven surviving mutants in `dioxus-native-dom` | In the entry, as its first `CARRY:`; the outcome is the operator's own wording — each killed by a test in a new test file or shown unreachable | The Epoch 2 audit's Survivors table (`.andromeda/runs/2026-10-06T16-58-31-code-audit/proposals.md`). Its line:column sites are as at `42b80ad9`; the five functions were confirmed present at `b7e3a43b` by name |
| 2 | `walk` in `element_id.rs`, cognitive 26 | In the entry, as its second `CARRY:`; under the ceiling with every id unchanged. Ordering lean: after item 1, whose tests guard it | The Epoch 3 audit's proposal M1 (`.andromeda/runs/2026-10-07T02-19-56-code-audit/proposals.md`): the one escher-authored source function over the ceiling |
| 3 | Clone pairs between `stand_*.rs` checks, 1 to 13 | In the entry, as its third `CARRY:`; the per-task control tables stated once | The same audit's Informational: 1 pair of 11 lines at the Epoch 2 record, 13 pairs of 220 lines at the Epoch 3 record, and the driver entries below prove on the same stand tasks |
| 4 | Three stand test functions over cognitive 15 | In the entry, as its fourth `CARRY:`; under 15 with both layout modes still run in each | The same proposal M1: 21, 17 and 16. With `walk`, `complexity.over_ceiling` reads 78 to 74 |

All four stay in the one entry the direction asked for. Items 3 and 4 touch the same checks, and items 1 and 2
the same crate, so none was moved to a later entry.

Not decided here: the audit's second direction for item 4 — whether test functions belong in the
`over_ceiling` scalar at all — is a question about the audit's own measure, put to the founder by the audit.
The entry says so and does not answer it.

No requirement was added: the entry claims no capability, and `requirements.md` and the verification matrix
are untouched.

## Curation

One Tier-1 entry, on the operator's request — see `curation.md` in this run dir.

## Other duties of this path

- Gated records: none, so no premise was re-verified.
- Epoch-growth valve: Epoch 4 at 8 entries, under the valve's level — not surfaced.
- Sidecar consolidation, seed-rule supersession, registry migration: not requested.
- Amendments: none; this wrap produced and measured no fact a master states.
