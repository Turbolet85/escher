# P5 review — 2026-11-21-engine-debts · Engine debts (v040-06)

**Plan:** `doc:2026-11-21-engine-debts/plan` rev 3. To read it in an editor, open the render at
`.andromeda/cache/docs/2026-11-21-engine-debts/plan.md`.
- No plan file stands in the chunk's folder.
- Writing in that render changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-engine-debts` diffs the
  chunk's records.

## Goal
Close the four engine debts as the founder ruled them: F1 A · F2 A · F3 A · F4 A · F5 A, his labels verbatim in
`inputs#I3`.
1. **A focused control that stops showing gives focus up after the next layout, with no `blur`.**
   - Which controls: `display: none` on it or a container (`hidden` included), `visibility: hidden` or `collapse` —
     the set the tree already drops.
   - This is an engine change for every document; his answer ratifies it.
2. **The buffered render's limit is re-stated and pinned by two probe checks** (tests only). If the first reads red,
   the fork goes back to him.
3. **`ScriptDocument::with_seed(u64)`, off by default,** seeds `Math.random` with a splitmix64 copy.
4. **The error-boundary registry drops a boundary once its component is gone.** `DioxusDocument` prunes before each
   render, and reads never prune.

## Top acceptance criteria
- `focus_fixup_unshown`, both modes:
  - 6 hide cases each read the engine, snapshot and tree focus as none / 0 / window;
  - the changed set marks the control;
  - `onblur` and `onfocusout` counters read 0, and Enter clicks nothing;
  - `display: contents` and a hidden sibling keep focus;
  - a `:focus-within` hider shows again after the next pump.
- `key_press_after_container_lost_its_box`: focus given up; a key on a re-focused boxless control neither panics nor
  fails to act.
- `dioxus_runtime_probe`:
  - (a) a task awaiting a just-shown node's handle focuses it, the three readings agreeing;
  - (b) a task polled in the render that removes its target leaves focus off it, its reading a measured literal.
- `seeded_random`: one seed reads one sequence, two seeds read two, an unseeded document draws from the host.
- `boundary.rs` units: 20 remounts keep the registry bounded (measured literal, predicted 2); a new error still reads
  new; a boundary dropped in a step reads until the next render.
- The `a11y` leg runs nineteen files green. `fast`, `doc` and `stand-repeat` are green. CI is green on the pre-CI
  commit.

## Touchpoints
- **New:**
  - `tests/blitz-tests/tests/focus_fixup_unshown.rs`
  - `packages/blitz-vibey-script/tests/seeded_random.rs`
  - `packages/blitz-vibey-script/src/seeded_random.rs`
- **Modify:**
  - blitz-dom: `node/node.rs`, `layout_reason.rs`, `document.rs`, `resolve.rs`
  - blitz-tests: `key_press_after_container_lost_its_box.rs`, `dioxus_runtime_probe.rs`
  - blitz-vibey-script: `lib.rs`, `state.rs`, `runtime.rs`, `document.rs`
  - dioxus-native-dom: `boundary.rs`, `dioxus_document.rs`
  - CI and docs: `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`, `docs/capabilities.md`

## Test commands and baselines (13 entries; fence parses; planlint 0 hits)
| # | Gate | Baseline |
|---|---|---|
| 1 | lib units of blitz-dom, dioxus-native-dom and blitz-vibey-script | green, 304 passed |
| 2 | vibey `seeded_random` + `dom` + `url_search_params` | **red**, predicted: no target `seeded_random`, the file step 7 creates |
| 3 | blitz-tests `focus_fixup_unshown` + `key_press_after_container_lost_its_box` + `dioxus_runtime_probe` + `inert_subtree` | **red**, predicted: no target `focus_fixup_unshown`, step 3's file |
| 4 | 14 companion targets | green, 128 passed, 3 ignored |
| 5 | `test_ci_workflows.py` | green |
| 6 | `test_capability_statement.py` | green |
| 7 | `ci-leg.sh a11y` | green, 18 · 173 · 0 · 0 |
| 8 | `ci-leg.sh fast` | seen green at the last wrap |
| 9 | `ci-leg.sh doc` | green, cached |
| 10 | `ci-leg.sh stand-repeat` | green, 3 of 3 rounds, 912.66 s |
| 11–13 | the operator's hygiene, push and `ci.py` entries | — |

Four one-shot `gate.py control` runs at implement: the fixup call removed; the visibility arm removed; the seed
returning `0.5`; the prune call removed. Each names its case and runs under `--timeout 1200`.

**One reading to note.** The capability statement records the `a11y` leg as `18 · 163 · 0 · 0`; today it reads
`18 · 173 · 0 · 0`. That line was already stale before this chunk. Step 9 re-measures it from gate 7's own log, so
no separate owner is needed.

## Research: what the plan rests on
- **Premises corrected at P3.** These went to the founder in `founder-forks.md`:
  - F3: the recorded same-render case is unreachable through `MountedData`.
  - F5: the reason "a dropped boundary cannot be told" was wrong.
- **Corrected again at P4.** Pruning "at each registration" cannot be built: the factory reaches no `VirtualDom`, and
  `Runtime::has_context` unwraps on a dropped scope. You chose "before each render" (`inputs#I4`).

## Concretization preview — the claim this chunk makes
**matrix#v040-06**
- **Today's acceptance:**
  > Each of the four debts reaches the founder as a fork; each one fixed is proven headless on the stand or a fixture
  > in both layout modes, and each one kept stands as a residual with his reason, a wait on another project's release
  > named as such.
- **Proposed concrete form:**
  > The four debts reached the founder as five forks in one file (`founder-forks.md`). He fixed all four by his own word
  > (`inputs#I3`), so none is kept. Each fix is proven headless, both layout modes, by:
  > - (res-6k56erh6) `focus_fixup_unshown` — after a layout, a focused control hidden by any of the six cases gives
  >   focus up. The engine, snapshot and tree read no focus, the changed set marks it, and no `blur` or `focusout`
  >   runs. `display: contents` and a hidden sibling keep focus.
  > - (res-433ttjb2) `dioxus_runtime_probe` — a task awaiting a just-shown node's handle focuses it. A task polled in
  >   the render that removes its target leaves focus off it, its reading pinned.
  > - (res-jqbadec2) `seeded_random` — `with_seed` gives one sequence per seed, and an unseeded document draws from the
  >   host.
  > - (res-k2zggoqx) `boundary.rs` units — the registry stays bounded over 20 remounts, and a new error still reads
  >   new.
  >
  > Each is controlled red-before-green. The four residuals close at the wrap.
- **Never weakened.** Every arm of today's acceptance stays: a fork each, a proof each fix, both layout modes. The
  "kept as a residual" arm has no subject, because the founder kept none.
- **Method:** integration, as recorded. Not an affordance cap.

## Expected amendments (wrap), in the plan's notes
- a11y-plan ×3, architecture ×12, security-plan ×3, design-system ×3, test-plan ×2. All are row edits by handle.
- No escalate pattern applies: no recommended option widens a boundary (`rul-izni57qv` read, no hit).
- The engine change of F1 is ratified by the founder's own answer (`inputs#I3`, delegate: none).

## Beyond the plan
none

Apply? (yes / review / cancel)
