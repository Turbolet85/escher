# P5 review — 2026-10-18-engine-debts «Engine debts» (v020-03)

Plan: `escher-0.2.0/chunks/2026-10-18-engine-debts/plan.md` · scope `scope.md` · research `research.md`
Fork answers: `.andromeda/runs/2026-10-18T02-38-20-phase/founder-forks.md` §Answers — `delegate operator`, PROVISIONAL
(inputs#I2)

## Goal
The nine engine debts go to the outcomes the fork answers chose.
- **Seven fixed** in the engine, for every document:
  - F1 `inert` honoured: no focus, no hit, out of the tree and the snapshot;
  - F2 `aria-checked` read as `checked` on the checkbox, switch, radio and menuitem roles;
  - F4 `Node::absolute_position` subtracts only the ancestors' scroll;
  - F5 the hit stops at a `contain: paint` box;
  - F6 the password click caret is placed by the bullets;
  - F7 `disabled` is read on form controls only;
  - F8 the client rect is read under transforms.
- **Two pinned:** F3 `aria-activedescendant` (a named check holds it); F9 `covered` on a box with no area (a sentence in
  `covered`'s meaning).
- **The CARRY:** a check of the merged `aria-expanded` · `aria-disabled` · `aria-level` reads.

Each fix rewrites its standing pin, runs in both layout modes, has a mutation control red with the fix reverted, and
has its merge surface named against `23b9b26a9` (no upstream PR). Each is PROVISIONAL until the founder's own word at
this chunk's wrap, the Epoch 2 boundary (`.andromeda/playbook.md:48`, `:64-66`).

## Top acceptance criteria
- **v020-03:** nine answers, nine matching outcomes. 8 control rows, 7 merge-surface rows; the F3 and F9 pins hold.
- **F1:** `inert_subtree` 4 passed; Tab skips the subtree, a hit passes through, no tree node.
- **F7:** `stand_keyboard` 14 passed; `<div disabled tabindex=0>` takes focus, a disabled button does not; the a11y leg
  passes.
- **F2:** `accessibility_states` 4 → 5; the snapshot's `checked` reads `aria-checked` on the five roles. A
  5,500-character value appears nowhere.
- **F6:** equal-length passwords, narrow and wide, place one click at one index; `password_caret_click` 1 → 2.
- **F8:**
  - a translated button's snapshot `bounds` reads where it shows, and a driver `click` on it is accepted;
  - `transformed_client_rect` 4 passed, `stand_act_transformed` 2 passed.
- **F5 + F9:**
  - `covered`'s meaning drops the `contain: paint` sentence and adds the no-area one;
  - the cause set stays nine; escher-driver 74 passed.
- **Census:** 0 print / panic / env / log sites in the added engine lines; the silence probe reads 0; no manifest,
  lockfile or `deny.toml` moves; the bridge stays byte-identical to upstream.
- **Legs:** stand-repeat 3 of 3 rounds green, coverage 3 of 3 crates at or above floor, `fast`, `doc`, and CI green on
  the pushed HEAD.

## Touchpoints
- **Engine (upstream merge surface):** blitz-dom's `node/node.rs`, `node/element.rs`, `scrolling.rs`,
  `accessibility.rs`, `document.rs`, `events/pointer.rs`, `events/keyboard.rs`.
- **escher's own:** dioxus-native-dom's `snapshot.rs`; escher-driver's `schema.rs`, `refusal.rs`, `json.rs`, `lib.rs`.
- **Checks:** five new (`accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`,
  `transformed_client_rect`, `stand_act_transformed`) and five rewritten (`stand_keyboard`, `accessibility_states`,
  `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`).
- **Unchanged:** no stand file, manifest, script or workflow.

## Test commands
30 `[[gate]]` entries: 14 cargo test targets, 7 probes, the a11y · stand-repeat · coverage · audit · fast · doc legs,
and the operator's hygiene · push · CI read. 25 entries were baseline-run on the untouched tree:
- every red is this chunk's subject: a new file absent, or the count before its step (4 / 69 / 1);
- each authored literal or empty-input guard (entries 15–21) got a must-pass and a must-fail reading in a scratch clone
  (`.andromeda/runs/2026-10-18T02-38-20-phase/p5-controls.md`);
- one fix at baseline: the silence probe dropped `session_common/mod.rs`, which no step edits and whose recorded
  re-exec child reads `std::env::current_exe` (`:276`).

## Leaned, no fork — check these
- **The `switch` role mapping rides F2 A.** `role_from_name` maps `role="switch"` to no role today
  (`accessibility.rs:298-330`), and F2 A names switch. The mapping is one more engine change for every document,
  PROVISIONAL with F2.
- **F8's `visible_region` limit is a measured branch.** The forecast: a target in a translated scrolling box misreads
  `off-screen`. If step 8 measures that, `off-screen`'s meaning gains one limit sentence; if not, no sentence is added.
  The check pins whichever reading it measures.
- **The F1 focus limit.** A node focused when it becomes inert keeps focus until focus moves (HTML's focus fixup is not
  built). `inert_subtree`'s fourth test states and pins it.
- **Size.** 534 lines, about 290 without the fence: inside the ~150–400 guide.
- **Legs at baseline:** a11y green (11.8 s), audit green, stand-repeat 3 of 3 rounds green (275 s), coverage 3 of 3
  crates at or above their floors (131 s).

## Concretization preview — v020-03 (the never-weaken checkpoint)
- **Now:** «Each of the nine recorded engine debts carries the founder's recorded answer, and its outcome matches that
  answer: a fixed debt is proven by a check on the stand or a fixture in both layout modes, with an owner named for its
  upstream merge surface; a pinned debt has its limit stated and a check that holds it; a kept debt stands as a residual
  line.»
- **Proposed** (`.andromeda/runs/2026-10-18T02-38-20-phase/acceptance-v020-03.md`): the same outcome, made concrete.
  - **The answer:** «the founder's recorded answer» becomes «its recorded fork answer … given by the operator in the
    founder's stead (`delegate operator`, inputs#I2), PROVISIONAL until the founder's own word at the Epoch 2 wrap».
  - **The fixes:** the seven fixed debts are named, each with its check, its control row (8) and its merge-surface row
    (7) against `23b9b26a9` with its owning route entry.
  - **The pins:** the two pinned debts are named with their statement and check, and none is kept.
- **The weakening to decide.** The answer clause moves from the founder's word to a delegate's. That is the directive's
  arrangement (inputs#I1), and implement can only verify an answer that exists now; the founder's own word is the
  wrap's to collect. A claim note records it, dated (`note-claim-v020-03.md`). If you want the founder's word kept as
  the condition, say so: the cap then stays unclaimed (pooled) and is verified only after his ruling.

## Beyond the plan
| # | Proposal the word approves that neither plan.md nor the matrix claim holds |
|---|---|
| 1 | The merge surface of the seven engine fixes is owned by a CARRY on the working-route entry «Upstream sync ahead of determinism and traces» (escher-0.2.0/working-route.md:23), written by this chunk's wrap (route-resolve). |
| 2 | The PROVISIONAL marks of the seven fixes and the `switch` mapping are listed for the founder at this chunk's wrap, the Epoch 2 boundary, under «Provisional discharge» (playbook:48): his own word ratifies or reworks each. |
| 3 | If step 8 measures the `visible_region` misreading, that pinned limit gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |
| 4 | The F1 focus-fixup limit (a node made inert keeps focus) gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |

Apply? (yes / review / cancel)
