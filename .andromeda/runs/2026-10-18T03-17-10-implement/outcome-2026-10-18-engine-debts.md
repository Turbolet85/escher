# Implement outcome — 2026-10-18-engine-debts

**Outcome: green.** Smoke skipped: no boot path changed, and every proof is a headless blitz-tests check.

The full gate block, trail `.andromeda/runs/2026-10-18T03-17-10-implement/gate-2026-10-18-engine-debts.json`:

    entries 31 · green 27 · red 0 · recorded 0 · timeout 0 · not-run 4

The four not run:
- **Entry 26** is v020-03's gated criterion (`not run — defer (key): gated: …`). It is fired only by step 13, in a
  gated `/andromeda-implement`, once the founder's own word discharges the PROVISIONAL batch at this chunk's wrap.
- **Entries 29–31** are `leg operator`: the hygiene read, the fast-gate-then-push, and the CI read.

**Matrix.** v020-03 is claimed gated. No ref was written: the tool refuses a gated cap's ref without `--cleared-file`,
and the block has not cleared. It stays `planned` until step 13.

**Next.** The operator pass first: entry 29 (`gate.py hygiene`) before `chore(2026-10-18-engine-debts): operator
pre-CI commit…`, then entry 30 (the fast leg, the clean-tree guard, the push), then entry 31 (`ci.py conclusion`).
Then `/andromeda-wrap-session`. At the wrap, the Epoch 2 boundary, the founder's word on the PROVISIONAL batch
decides v020-03. If he has not ruled, the wrap's gated arm holds the cap.

## Files changed (22, all on research's lists; `gate.py scope`: clean, 0 recorded)

New checks (5):
- `tests/blitz-tests/tests/accessibility_merged_states.rs` (3)
- `tests/blitz-tests/tests/inert_subtree.rs` (4)
- `tests/blitz-tests/tests/accessibility_active_descendant_unread.rs` (1)
- `tests/blitz-tests/tests/transformed_client_rect.rs` (4)
- `tests/blitz-tests/tests/stand_act_transformed.rs` (2)

Engine (blitz-dom, 7 files):
- `node/node.rs`: F1 `is_focussable` + `is_inert`, `hit_inner`'s inert stop; F4 `absolute_position` +
  `position_in_containing_block`; F5 the `contain: paint` stop, matching paint's applicability rule.
- `node/element.rs`: F7 `ElementData::is_disabled`, used by `flush_is_focussable`.
- `scrolling.rs`: F4, the add-back dropped.
- `accessibility.rs`: F1 inert excluded; F2 `aria-checked` on the five checkable roles, `switch` → `Role::Switch`.
- `document.rs`: F8 `get_client_bounding_rect` / `node_client_rects` through `transformed_bounds`,
  `is_transformed_chain` and `transformed_fragment_rects`.
- `events/pointer.rs`: F6 `click_password_input`; F7 the three gates.
- `events/keyboard.rs`: F7 the two activation gates.

Snapshot and driver (5 files):
- `dioxus-native-dom/src/snapshot.rs`: F2 `checked` / `pressed`, plus three unit tests (1 CARRY, 2 F2).
- `escher-driver/src/schema.rs`: F2, the `checked` and `pressed` node-key help.
- `escher-driver/src/refusal.rs`: F5 / F9 `covered`; F8 `off-screen`.
- `escher-driver/src/json.rs`: F5 / F9.
- `escher-driver/src/lib.rs`: F5 / F9 / F8.

Rewritten checks (5):
- `stand_keyboard.rs`: two pins renamed and rewritten, still 14 tests.
- `accessibility_states.rs`: 4 → 5 tests.
- `scrolled_box_absolute_position.rs`.
- `hit_clipped_at_scrolling_box.rs`: two tests rewritten, still 13.
- `password_caret_click.rs`: 1 → 2 tests.

Evidence: `evidence/controls.md` (8 rows), `evidence/merge-surface.md` (7 counted rows, F3 listed),
`evidence/readings.md`.

## Deviations from the plan, with justifications

No scope-record lines: every edit is on research's lists.

1. **F6, the click point (step 9).** The plan finds the bullet under `(tx, ty)`, but `tx` carries the editor's scroll
   of the *real* text. `click_password_input` instead scrolls the bullets' layout the way paint does (`caret.x1 −
   content width`, blitz-paint `draw_masked_input_text`). That way no width of the real characters reaches the
   caret, even in a value that overflows the field. A double click calls `select_all`, as planned; a triple click
   does the same.
2. **F2, the checkable-role set.** The snapshot needs the builder's five roles. Exporting blitz-dom's
   `is_checkable` would move blitz-dom's `lib.rs`, which is outside entry 21's twelve-file gate. So `snapshot.rs`
   carries its own private copy of the five-role match.
3. **F2, `checked` with no stated value.** A checkable role that states no `aria-checked` reads `checked` `None`
   (`toggled.map(== True)`), not `Some(false)`, consistent with how `pressed` reads `None` where nothing is stated.
4. **F2, the `pressed` help.** The help says "the five roles whose toggle reads as `checked`" and does not spell
   `aria-checked`. The standing `schema_tests.rs:316` asserts that `pressed`'s help does not contain `aria-checked`,
   and that file is not on the lists.
5. **F2, native checkbox and radio (corrected on the operator's word, 2026-10-18).** In the first pass, a native
   `<input type="checkbox">` (`Role::CheckBox`) wrote its `aria-checked` as `toggled`.
   - **The operator's correction inside F2 A:** «a native <input type=checkbox> or <input type=radio> must NOT take
     aria-checked (HTML-AAM / ARIA in HTML: the native checkedness wins, the author attribute is ignored there)».
   - **The fix:** the read is gated `is_checkable(role) && !is_native_checkable(element_data)` (`accessibility.rs`;
     `is_native_checkable` is an `input` whose `type` is exactly `checkbox` or `radio`, as `role_from_element_data`
     reads it).
   - **The check:** `accessibility_states`' fourth test now asserts that an unchecked native checkbox, an unchecked
     native radio and an unchecked native checkbox with `role="switch"`, each carrying `aria-checked="true"`, read no
     `toggled`. The count stays 5.
   - **Controls:** the F2 row of `controls.md` was re-run on the new shape. The native gate's own control read red,
     sha256 equal, green (`readings.md`).
   - **Re-fired green:** entries 3, 4, 5, 15, 17, 18, 21, 22, 23 (stand-repeat), 24 (coverage), 27 (fast) and 28 (doc).
   - **The operator's second correction, the same day:** «a native checkbox or radio takes neither aria-checked nor
     aria-pressed (HTML-AAM)». The read became: `is_native_checkable` → none, else `is_checkable(role)` →
     `aria-checked`, else `aria-pressed`. The fourth test adds `aria-pressed` beside `aria-checked` and an unchecked
     native radio with `role="button"`; still 5 tests.
   - **Controls on the final shape:** F2 row 3.4 s; native gate 3.2 s; both red, sha256 equal, green.
   - **Re-fired green after the second correction:** entries 4, 15, 17, 18, 21, 22 (a11y), 24 (coverage) and 27
     (fast).
6. **F8, inline fragments.** The first edit mapped transforms inside `inline_fragment_rects`. Entry 11 then read red:
   `offset_rect` reads that function, and the plan keeps `offset_rect` untransformed. The transform now lives in a
   private `transformed_fragment_rects` used only by `get_client_bounding_rect` and `node_client_rects`;
   `inline_fragment_rects` is byte-identical to the base again.
7. **F8, the forecast measured (step 8).** `visible_region` misreads a target inside a translated scrolling box. The
   test pins that, and `off-screen`'s meaning gains the planned sentence in `refusal.rs` (doc comment, `meaning()`,
   unit table) and `lib.rs`. `lib.rs` now opens the paragraph with "Three readings are known to be off".
8. **F1, where the focus limit is pinned.** Step 2 says the focus-kept limit is "pinned by its fourth test", which
   is ambiguous. It is `inert_subtree`'s fourth test, `removing_inert_restores_focus_hit_and_tree`, and the limit is
   stated in both `inert_subtree`'s and `stand_keyboard`'s `//!` docs.
9. **F7, a consequence for links and summaries.** `disabled` is now read only on `button`, `input`, `select` and
   `textarea`. So a `<summary disabled>` or an `<a href disabled>` now takes Enter / Space activation and a click,
   as HTML does.
10. **The CARRY's tables** carry a few rows beyond the plan's minimum: `aria-expanded` `""`, `TRUE`, `yes` and
    `" true "` read none; `aria-disabled` `""`, `TRUE` and an unknown value set nothing; `aria-level` absent reads
    none. Each table's row count is asserted.
11. **F6, the caret index.** The index is read as measured and asserted equal across the two fillings, inside
    1–7. The check prints no index, so `readings.md` states equality only.

## Surfaced / stuck

None. No spec↔reality gap needed surfacing. Step 8's forecast held, and the plan had prescribed the response.

## For the wrap — a new open residuals line (target: next)

Measured from the builder's code (`readings.md`): native checkedness does not reach the accessibility tree.
`build_accessibility_node` reads no `checkbox_input_checked()` and no `CHECKED` state, so a native checkbox or radio,
checked or not, carries no `toggled` on its tree node. What the platform adapter hands an assistive technology
therefore never says whether a native checkbox is checked. HTML-AAM maps it to the checked state. The snapshot's
`checked` is unaffected, because it reads the element.

Proposed line, for the wrap's route-resolve to append to `.andromeda/residuals.md` as `open`, target next:
«native checkedness unread by the accessibility builder: an `<input type="checkbox">` or `type="radio"` writes no
`toggled` from its own checked state, so the tree the shell hands the adapter never states it checked (HTML-AAM);
the snapshot's `checked` reads the element and is right — found at 2026-10-18-engine-debts, after the operator's
F2 A correction that a native input takes no `aria-checked`».

Owner: the wrap's route-resolve. Its decision is the founder's or the operator's, because writing native
checkedness to the tree is an engine change for every document.

## Process hygiene

| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries (the gate entries, the 8 mutation controls) | this run | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --args blitz-tests`: `census: none running` |
| escher-session hosts (stand-repeat and fast-leg session checks) | a leg | terminated — same census, none running |

## Decisions & corrections

- **Guard refusal, a recurring standing learning.** The PreToolUse bash guard refused a bare `cd` into the chunk's
  `inputs/` dir ("moves the session cwd"); re-issued with absolute paths. This recurs host-linux.md «Paths & the
  working directory», which the last handoff's deferred learnings already list as recurring.
- **Guard refusal.** A `cat` heredoc with a file target (the covered-text script) was refused; the edits were
  re-issued through the Edit tool.
- **Guard refusal.** `xargs … command grep` was refused (xargs cannot run the builtin); re-issued as a shell `for`
  loop.
- **Operator correction (2026-10-18), with a direction.** «One fix before the operator pass, inside F2 A: a native
  <input type=checkbox> or <input type=radio> must NOT take aria-checked … Stop after that; no operator pass yet.»
  The fix and its check landed and were re-fired green. No operator entry was fired.
- **Operator correction (2026-10-18), with a direction.** «Yes, add that condition: a native checkbox or radio takes
  neither aria-checked nor aria-pressed (HTML-AAM) … Then run the operator pass, each entry by hand in plan order …
  Stop and tell me if any entry is red; start no wrap.» The operator pass is recorded in `evidence/operator-pass.md`.
- **Sweep hazard (the census entry).** The census probe (entry 17) counts `.expect(`, `.unwrap()` and `panic!` on
  every added line of `snapshot.rs`, its unit tests included. The new snapshot unit test was written without
  them.
