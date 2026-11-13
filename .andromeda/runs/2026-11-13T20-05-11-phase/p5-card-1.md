# P5 review — 2026-11-13-cognitive-load-corrective · «Cognitive load corrective»

The plan to open in an editor: `.andromeda/cache/docs/2026-11-13-cognitive-load-corrective/plan.md`, rendered from
`doc:2026-11-13-cognitive-load-corrective/plan` rev 5. No plan stands in the chunk's folder, and writing in that file
changes no record until a `put`. To diff the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-cognitive-load-corrective`.

## Goal
Bring three functions under the cognitive-complexity ceiling of 15, read on the dev host by rust-code-analysis-cli,
with behaviour unchanged for every document and every driver call:
- `handle_click` (97), `packages/blitz-dom/src/events/pointer.rs`;
- `Session::execute` (32), `packages/escher-driver/src/execute.rs`;
- `DioxusDocument::graph` (50), `packages/dioxus-native-dom/src/graph.rs`.

Each body moves into private sibling functions in the same file. A closure or a nested `fn` would not lower the
reading: the analyser adds a nested space into its parent's sum. There is no file split, and `mutator.rs` and
`document.rs` stay as they are.

`handle_click` is split arm by arm, upstream's arms included. That is your P4 answer (`inputs#I1`), and the price
you accepted is a hand re-apply when a later upstream sync edits one of those arms.

## Steps
1. Keep P5's before readings in the evidence: 6 functions over 15 across the three files, and the census `equal`.
2. Add `tests/blitz-tests/tests/click_default_actions.rs` before any engine edit. It holds the five click default
   actions no check reaches today, in both layout modes:
   - a link leaving the document hands `navigate_to` its URL;
   - an `<a>` with no href walks on to its label;
   - an unmatched click clears focus;
   - a disabled submit stops the walk;
   - a `multiple` select's label click focuses it and chooses nothing;
   - an app hears a double click.

   Each case is controlled red, then green, by a one-shot `gate.py control` on the untouched engine.
3. `handle_click` becomes a walk (the three gates in today's order), a dispatch match, and one private function per
   arm. The file-input arm moves byte for byte under its feature, and the label arm's recursion stays.
4. `execute`'s arms become private functions. The `type` closure's body becomes a named function, and the `command`
   span stays in `run`.
5. `graph`'s state readings, `ran`, per-node handlers, reads and the states' assembly become private functions under
   the `accessibility` gate.
6. Read the after state, then run the remaining gates.

## Top acceptance criteria
- The complexity entry reads `last line 3`: only handle_pointermove, handle_pointerdown and touch_action_pan_axes stay
  over 15, all of them outside this chunk. The prediction is 6 → 3; record what it measures.
- `click_default_actions` is green in both modes, and each case read red under its control.
- The 15 standing click-arm suites stay green under the workspace build.
- escher-driver's and dioxus-native-dom's unit tests stay green.
- `fast` is green.
- The census reads `last line 0`: no log site, print, env read or panic path added.
- `mutator.rs` and `document.rs` are unchanged.
- The operator's CI read gives `verdict: green`, including "Coverage report" (floors held, readings forecast to move)
  and "Stand checks repeated".
- No capability is claimed. The pool's eight (v030-08 … v030-15) each assert a surface this chunk doesn't build.

## Touchpoints
- New: `tests/blitz-tests/tests/click_default_actions.rs`.
- Modify: `pointer.rs`, `execute.rs`, `graph.rs`.

## Test commands
| # | entry | baseline |
|---|---|---|
| 1 | complexity probe | red 6, the subject; controlled green (3) on stubbed bodies |
| 2 | silence and panic census | green, every file equal; controlled red on an added `println!` |
| 3 | mutator.rs / document.rs guard | green; controlled red on an edit |
| 4 | `cargo test -p blitz-tests --test click_default_actions` | red, file absent |
| 5 | the 15 standing click-arm suites, `--workspace` | green, 114 passed |
| 6 | `cargo test -p escher-driver --locked` | green, 104 |
| 7 | `cargo test -p dioxus-native-dom --locked --lib` | green, 109 |
| 8 | `ci-leg.sh fast` | seen green at the last wrap |
| 9–11 | operator: hygiene → push → `ci.py conclusion --wait 1800` | — |

The controls are in `.andromeda/runs/2026-11-13T20-05-11-phase/p5-controls.md`. stand-repeat and coverage run as
CI's jobs, inside the operator's green read, not as local entries.

## Concretization preview
None: no capability is claimed.

## Flags
- Aggregate B passes: 0 masters read `picked 0`.
- Expected amendments (wrap):
  - test-plan's `decision/complexity-ceiling-host-only` clause "engine functions over it are left as they are" names
    this chunk's exception;
  - obs-plan §6 records `click_default_actions` as silent;
  - rows citing lines of the three files get a re-point.

## Beyond the plan
none
