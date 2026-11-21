# P5 review — 2026-11-21-single-select-sized-as-a-list-box · «Single select sized as a list box»

Plan rev 4 (`doc:2026-11-21-single-select-sized-as-a-list-box/plan`). Its render to open in an editor is
`.andromeda/cache/docs/2026-11-21-single-select-sized-as-a-list-box/plan.md`. No plan stands in the chunk's folder,
and writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-single-select-sized-as-a-list-box` is a diff
of the chunk's records.

## Goal
A single select whose HTML display size is above 1 is rendered as an in-flow list box `size` rows tall. It opens no
popup, its keys and clicks choose a row at once, and it may stand with nothing chosen. Residual `res-ysz67imk` closes.
This is the founder's F1 A, «Падает только последняя часть (Recommended)»: only F8's last sentence falls, and a
`multiple` select stays as built. He raised no objection to the five things taken from HTML: «Всё по HTML, возражений
нет (Recommended)» (inputs#I3). His F1 answer ratifies the engine reading a single select's `size` in every document,
a boundary widening under the playbook's *Boundary widening* pattern (`rul-izni57qv`). His own word was relayed by the
overseer.

## How
- A third reading, **list box** (`multiple`, or a display size above 1), answered at every arm research's graph query
  found keyed on single versus `multiple`: the popup gate, `click_select`, `click_option`, the key dispatch, the
  tree's role and `expanded`, `layout_reason`, `closed_select_of`, and the selectedness algorithm.
- The look comes from presentational hints on the select and its rows (the `cellpadding` precedent), since no selector
  of the default sheet can read a parsed `size`. `default.css` is not edited.
- A `size` write restyles the rows and re-settles the choice.
- The driver gains nothing. Its `click` and `press` help are restated, and so is the capability statement, whose held
  counts are unmoved.

## Top acceptance criteria
- A single select sized above 1:
  - lays out as a list box `size` × 1.2 em tall, its rows boxes of their own;
  - opens no list on a click, Enter or a Space;
  - reads `ListBox` with no `expanded`;
  - may hold nothing chosen.

  A display size of 1 (`"1"`, `"0"`, `"many"`, `"-3"`, empty, absent) keeps the drop-down and its popup.
- Its keys choose and scroll through the one write path, and Enter and Space do nothing. A row click chooses that row
  alone whatever modifier is held.
- A `size` write moves a select both ways, closing an open list. A drop-down left with nothing chosen settles to its
  first enabled option.
- Through the driver: rows with their own bounds, a row `click` accepted, `actions` with no `covered`, `press` choosing
  with no list, `type` by the text's start. No verb, field, cause or span is added.
- The stand does not move: the flight booker's `flight-type` keeps its popup, and the cross-OS digests and boot lint
  hold.

## Touchpoints
16 files modified, none created:
- blitz-dom: `node/select.rs`, `node/select_list.rs`, `stylo/presentational_hints.rs`, `mutator.rs`, `document.rs`,
  `events/keyboard.rs`, `events/pointer.rs`, `accessibility.rs`, `layout_reason.rs`;
- dioxus-native-dom: `snapshot.rs`;
- escher-driver: `schema.rs`, `schema_tests.rs`;
- `docs/capabilities.md`;
- checks: `select_interaction.rs`, `select_popup.rs`, `stand_act_select.rs`. The existing check files are extended on
  your answer (inputs#I2), so no leg, pin or count moves.

## Test commands
11 entries:
- unit tests of the three crates — baseline green, 433 passed;
- the three select targets — baseline green, 56 passed;
- thirteen companions — baseline green, 130 passed;
- the statement test, `a11y`, `fast`, `doc` and `stand-repeat` — each seen green at the last wrap;
- the operator's hygiene, push and CI read.

Four one-shot `gate.py control` runs at implement: the popup gate, the selectedness rule, the rows' hints and the
bridge's closed select.

## Capabilities
None claimed. The pool's unclaimed caps belong to other entries, and v040-04, which owned the residual, is already
verified. The closure rides `matrix#v040-04 notes` as the wrap's ledger note. There is no concretization preview.

## Sizing
One implement window. The diff covers 16 files, the engine changes are spread over 9 blitz-dom files with no new
module, and the work is about 17 new checks in three existing files. No split is proposed.

## Notes
- **Plan length:** about 406 prose lines, at the template's ~400 bound.
- **Mechanical checks:** all pass; planlint reads 0 hits.
- **Validation-1:** aligned.

## Beyond the plan
none
