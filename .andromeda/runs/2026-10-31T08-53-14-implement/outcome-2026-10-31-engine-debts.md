# Outcome — 2026-10-31-engine-debts: green + smoke

Every entry `/implement` may fire reads green on the final tree: `entries 37 · green 32 · red 0 · recorded 1 · timeout 0 · not-run 4`. Entries 34 to 37 are the operator's. Nothing is committed and nothing pushed.

The thirteen forks end as the founder's word says: F1 to F7, F9, F10 and F13 fixed, F8 not built, F11 pinned as a stated limit, F12's rows added. CARRY 2's three checks are written, CARRY 1's reading is recorded, and CARRY 3 stays the wrap's amendment. `v030-04` is `implemented`.

**Next → the operator pass, entries 34, 35, 36 and 37, then `/andromeda-wrap-session`.** Entry 34 is the hygiene read, 35 the pre-CI commit's gate and the push, 36 the CI read, 37 the CI tallies. The pause on escher (inputs#I9) is the founder's to lift, and whether it holds this chunk's push is not settled here: I9 names it about the popup's chunk.

## Files changed

46 against the base `2109f1c2e`, +2898 −331: 43 listed in research.md, 3 recorded. `scope: clean — changed 46 · listed 43 · recorded 3 (companion 3 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 615`.

- **Engine:** `blitz-dom` (`scrolling`, `document`, `node/node`, `node/select`, `mutator`, `accessibility`, `events/keyboard`, `events/pointer`, `form`, `assets/default.css`), `blitz-vibey-script` (`runtime/events`, `tests/dom`), `blitz-shell/event`, `dioxus-native-dom` (`snapshot`, `dioxus_document`).
- **Driver:** `escher-driver` (`execute`, `lint`, `refusal`, `schema`, `lib`).
- **Stand and records:** `examples/seven_guis/src/tasks/cells.rs`, `docs/capabilities.md`, `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`, `Cargo.lock`.
- **Checks:** five new targets, twelve edited, `common/mod.rs`.
- **Evidence:** `chunks/2026-10-31-engine-debts/evidence/` holds `counts.md`, `readings.md`, `census.md`, `sink.md`, `merge-surface.md`.

## Deviations from the plan

Scope record lines:

- `packages/escher-driver/src/schema_tests.rs` · companion: it pins `type`'s help.
- `packages/dioxus-native-dom/src/lib.rs` · companion: it re-exports the two readers below.
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` · companion: the moved pin, deviation 1.

Each deviation, with its justification:

1. **A forecast falsified: `stand_non_lean_tasks`' Cells reads 677, not 676.** The count is every element with the listener mark that reads `GenericContainer`, and step 15 gives `cells-grid` an `onkeydown` and no role. The pin moved with its reason, and its doc now says "react to input". `.andromeda/residuals.md:23` and `a11y-plan-detail/screen-reader-support.md` still say 676, the wrap's to move. Nothing is wrong in the code; the plan's "read unmoved" was.
2. **Step 15's async form is not used.** A task spawned from a key handler that writes a signal panicked with `RefCell already borrowed` (`events.rs:162`): it is polled inside `render_immediate`, which holds the document. An async `onmounted` would also focus one poll late. Cells calls the handles synchronously with `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call. This leans on that behaviour of the handle, and a comment states it. Measured headlessly; the windowed path was read from the same lines and not run (`evidence/readings.md`).
3. **Step 16's "`press` alone" cannot reach a value.** The driver's twelve key names hold no character. `press` walks and opens the editor, `type` supplies the characters, `press` commits.
4. **Three public items the plan does not name.** `BaseDocument::clear_select_typeahead`: without it a second `type` on a `multiple` select joins the first's text, since focusing with no click leaves the typeahead standing. `dioxus_native_dom::closed_select_of` and `is_multiple_select`: escher-driver has no blitz-dom dependency to name `local_name!` with, and `is_masked_input` is the precedent. No type, verb or field is added.
5. **`visible_region` takes a helper, `padding_box_in_view`, in place of "the existing lines run unchanged".** My first form kept them and read cognitive complexity 25. The helper reads 11 with the same arithmetic in the same order for a page with no transform, and `a_page_with_no_transform_…` and every scrolled-box check stay green.
6. **Three small behaviours beyond the plan's letter**, for the review to read:
   - A click inside a cell's own open editor now returns early. A pointer path moves here: before, that click reset the editor to the cell's stored text, and now typed text stays. Step 15 says a pointer-opened editor reads as today, so this is the one place it does not. It is needed so a click in a keyboard-opened editor keeps its return to the grid.
   - Shift with no cursor yet spans from the last chosen row, so a set the markup seeds is spanned from it (pinned by `a_set_the_markup_seeds_…`).
   - `type` on a `multiple` select with a text opening on a space types nothing: no label can match, and the Space would flip a row.
7. **`form.rs` already held a tests module.** Step 11 and research call these its first tests; they joined it.
8. **Step 12's planted case is two overlapping rows** of the `multiple` select, reading exactly `fx-row-a` × `fx-row-b`. A box laid over the select reads once, at the select, and would not show that rows are linted.
9. **Extra rows the plan did not list:** `cells_a_click_on_another_cell_commits_an_open_editor_to_its_own_cell`, which pins the mixed-path defect step 15 closes, and `a_click_and_a_space_move_a_native_control_in_the_tree` as a sixth test in `accessibility_states`, not inside the restated fourth.

No plan-listed rejection was crossed: no DOM `blur`, no `ctrl` on `press`, no `size` read, no `grid` role, `blitz-paint` unmoved.

## Guard entries

None: the plan lists no `gate.py guard` entry. Entry 1, its preservation guard, is a plain `git diff --quiet`, and it reads green.

## Surfaced for the wrap and the operator

- **A hazard outside this chunk's fixes:** any app spawning a `MountedData` call beside a signal write meets the panic in deviation 2. No engine line moved. It holds no owner yet; the wrap pins one.
- **Unmeasured until CI:** MSRV 1.91, macOS and Windows, the coverage floor (escher-driver stands under one point above its floor) and the `stand-repeat` leg, now fifteen named targets. I ran none of them here. This chunk adds no `cfg` site.
- **The wrap's records:** the expected amendments in `plan.md`, the popup's route entry and requirement (`matrix.py add` mints its id), the residual line for `press` holding no Ctrl, and `CLAUDE.md`'s and the rules' statements of the old limits, none of which `/implement` touches. `.claude/rules/a11y.md` still states the fixup unbuilt and 85 tests.
- **The windowed smoke reads the boot alone.** Cells' keyboard path is proven headlessly and through the binary, never by a person at a window. What an assistive technology reads stays unmeasured.

## Process hygiene

`gate.py census --comm escher-session --comm seven_guis_native --comm seven_guis_nati --args agent-run`: `census: none running`.

| Process | Started by | Final state |
|---|---|---|
| `seven_guis_native` (entry 33, twice) | this run, through the gate tool | terminated by `timeout 10` |
| `escher-session` hosts (the sink readings, the Cells smoke) | this run, by hand | stopped by `stop`, state directories removed |
| `escher-session` children of `host_log`, `mcp_log`, `recording` and kin | this run, through the checks | reaped by their guards |

## Decisions & corrections

- A standing learning recurred: `harness.center_of` on an inline element reads no box, so a click on a default `label` missed it (`checkable_change_event`, fixed by `display: block` in the fixture).
- A guard refusal: a `cat` heredoc with a file target was refused once by the Bash guard and re-issued through the Write tool.
- Entry 4's census forbids `.unwrap()` and `.expect(` on added lines under `packages/`, tests included, so the new unit tests there use neither, unlike their neighbours.
- No operator correction or ruling arrived during this run.
