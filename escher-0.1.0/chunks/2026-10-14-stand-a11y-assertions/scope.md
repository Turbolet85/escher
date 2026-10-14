# Scope — 2026-10-14-stand-a11y-assertions

**Working entry** (escher-0.1.0/working-route.md:94, Epoch 6 — Polish & ship): "Stand a11y assertions — SC 2.1.1
keyboard reach, SC 2.4.3 focus order, SC 1.4.3 contrast on every stand control; a11y CI leg gating merges", with twelve
CARRY blocks folded below. P3's premise closure has run: each bullet the entry did not state is verified (tag dropped,
evidence beside it) or corrected (`[premise-corrected: …]`), evidence in `research.md`.

## What the chunk builds
- Assertions, not records, of the a11y target on the stand: SC 2.1.1 (Keyboard) — every stand control reachable and
  operable from the keyboard; SC 2.4.3 (Focus order) — each task's focus sequence asserted; SC 1.4.3 (Contrast
  minimum) — every stand control's text/background pair asserted to pass. (stated)
- An a11y CI leg that gates merges. (stated)
- The three readings already exist as harnesses: `stand_keyboard` (11 tests: the Tab and Shift+Tab sequence per lean
  task through `common::tab_order`, Enter and Space on each stand button), `stand_contrast` (18 rows over the four lean
  tasks at boot: pass 9 · fail 5 · exempt-inactive 3 · no-text 1, a record that "records and gates nothing") and
  `stand_accessibility_ids` (the named-controls table). This chunk turns them into pass/fail assertions where they
  record, and closes the gaps the CARRYs name; a11y-plan §6/§9 hold the readings. Verified: `stand_contrast` asserts the
  stated record, so a `Fail` row passes it (`stand_contrast.rs:194-225`); `stand_keyboard`'s Tab walk runs from boot
  only (`stand_keyboard.rs:118-146`), where `flight-return-date`, `crud-update` and `crud-delete` are disabled and
  outside `tab_order` (`common/mod.rs:50-70`) — no check reaches or orders them in their enabled state.
- "Every stand control" means the four lean tasks `LeanTask::ALL` boots (counter, flight booker, timer, CRUD), the
  surface the three harnesses read; the three non-lean tasks (Temperature Converter, Circle Drawer, Cells) and Home are
  the "Stand requirement sweep" entry's. Verified: layout-templates §Surface: desktop-native → Primary screens mounts one
  lean task in TaskShell for every stand assertion, and a11y-plan §6 records the four lean tasks.
- [premise-corrected: no branch protection, no ruleset and no pull request on the fork — `gh api …/branches/build%2Fescher-0.1.0/protection` 404, `…/rulesets` `[]`, `gh pr list` none, read 2026-10-14 (research.md §Measurements)]
  The a11y leg today is `ci-leg.sh a11y` = `cargo test --workspace --locked --test accessibility_hidden --test
  accessibility_roles --test focusability_updates` (`.github/scripts/ci-leg.sh:34`), its argument list pinned by
  `test_ci_workflows.py:289`, run by the CI job "Accessibility (a11y) tests" (`.github/workflows/ci.yml:246-266`);
  `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` run in the workspace `test` leg, not in it. Nothing
  blocks a merge on the fork: work lands by push to the build branch and a red job reads as the run's verdict, which
  phase Setup 5a and the wrap read. "Gating merges" is therefore one of two things — the stand assertions join the
  existing `a11y` leg, whose red is the run's verdict as every leg's is; or a required status check on the build branch
  that refuses a push or a merge, a repository setting that blocks where nothing blocks today. The second is a new CI
  gate — the founder's fork (inputs#I1); whether the first is one too is asked with it.

## Folded freight (the twelve CARRY blocks at working-route.md:94)
1. **`author_id` on the platform tree (from 2026-10-06-accessibility-tree-identity).** The operator ratified the stable
   element id leaving the process as AccessKit `author_id` through the platform accessibility adapter; assert that no
   `author_id` on the tree the shell hands the adapter carries a `NodeId`, `ElementId`, `ScopeId` or pointer form.
   Today's proof is headless: `stand_accessibility_ids` reads `DioxusDocument::accessibility_tree`, not the platform
   adapter (security-plan §Input Validation, `id` row).
2. **The stand binary builds no AccessKit (from 2026-10-06-accessibility-tree-identity).** seven_guis takes
   dioxus-native with `default-features = false` and names no `accessibility` feature, so `just seven_guis` compiles no
   platform adapter and no AT can read the windowed stand (measured at that chunk with `cargo tree -p seven_guis -e
   features -i dioxus-native`: prelude · system-fonts · vello-hybrid · woff; re-read at take-up: seven_guis'
   manifest names `prelude`, `system-fonts`, `woff`, and no `accessibility`). "Whether the stand enables it is decided
   at this entry's promotion." Enabling the platform adapter on the stand is a boundary widening (playbook :36) — the
   founder's fork, raised as a dialog (inputs#I1).
3. **Windowed witness of the refresh on change (from 2026-10-07-change-tracking-and-diff).** The shell's `View::poll`
   rebuilds the platform accessibility tree when its changed set was non-empty (ratified by the founder, 2026-10-07,
   the witness owed); no windowed run witnesses it — the windowed binary builds blitz-shell without `accessibility`,
   and on the dev host AccessKit's `update_if_active` never runs the tree build (`.claude/rules/a11y.md:46`, the AT-SPI
   bus reads `IsEnabled` false); proven at the document level only (the `changed_set_` unit tests, both feature builds
   of blitz-shell). Witness here that a focus move and a rewritten accessible name reach the platform tree (a11y-plan §2).
   - [premise-corrected: `AccessibilityState` is built only from a live winit window and hands the adapter `doc.accessibility_tree()` inside `update_if_active` (`packages/blitz-shell/src/accessibility.rs`; `window.rs:150-151`, `:377-381`) — no in-process path reaches the adapter]
     A windowed witness on the dev host is blocked by the bus state, which the rule says to leave alone (the operator's
     desktop), and no path reaches the adapter without a window. What is reachable headlessly is the two halves the
     refresh composes: the tree the shell hands the adapter is exactly `Document::accessibility_tree` called through
     `&dyn Document` (the override for a `DioxusDocument`), and a focus move and a rewritten name each leave the changed
     set non-empty, which is the poll's rebuild condition. A witness at the platform tree itself keeps an owner.
4. **The tree override is compiled in, the adapter still is not (from 2026-10-07-act-by-id).** `escher-driver` names
   `accessibility` on dioxus-native-dom, so both seven_guis binaries compile the `accessibility_tree` override and the
   snapshot modules; `accesskit_xplat` and `accesskit_winit` are in neither graph (measured at
   escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/feature-reach.md); enabling the adapter is still this entry's
   decision (CARRY 2's fork).
5. **The stand's listbox reads no accessible name (from 2026-10-10-snapshot-states).** `crud-list` states
   `role="listbox"` (`examples/seven_guis/src/tasks/crud.rs:57-58`) and its snapshot line reads the name `""`: an
   interactive-role element outside the 15 named controls `stand_accessibility_ids` asserts. "How the list is named is
   decided at this entry's promotion, with that check's table of named controls restated."
   - Naming it in the stand's markup (an `aria-label`) is a stand change, the operator's — no engine change. Verified:
     the builder takes a trimmed non-empty `aria-label` as the name (`.claude/rules/a11y.md`, Accessibility tree); the
     flight dates are named so (layout-templates §Surface: desktop-native → Primary screens); no check pins the list's
     name `""` (research.md §Sweeps); `host_log` and `mcp_log` take the rows' text as name needles, so the list's name
     joins that census.
6. **Windowed witness of the three states (from 2026-10-10-snapshot-states; ratified by the founder, 2026-10-10,
   «Ратифицировать»).** In a windowed build with `accessibility` on, the platform adapter is handed `selected`,
   `toggled` and `invalid` on each element node whose markup states them; what an AT reads of them is not measured; they
   are proven headlessly only (`accessibility_states`; `stand_snapshot_state`). The witness rides the one owed for the
   refresh on change (item 3).
7. **Re-colour every pair the stand contrast check records `fail` (from 2026-10-13-stand-contrast-harness; the
   operator's word at that chunk's P5 review).** `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and
   `crud-create` read `#ffffff` on `#4a6cf7` at 4.39 against 4.5; SC 1.4.3 cannot be asserted green until they pass,
   and a re-colouring moves the record `stand_contrast` asserts row for row.
   - The accent `#4a6cf7` is a stand token used beyond the five buttons. Verified with the reach widened (research.md
     §Sweeps: 25 accent-family hits in 7 task files): white text sits on it also on `crud-update` and `crud-delete` once
     a row is selected (`crud.rs:139`, `:154`, `:261-270`), on the selected CRUD row (`crud.rs:230-233`), on the active
     flight toggle (`flight_booker.rs:170-174` — `flight-return` once chosen), and on the circle drawer's toolbar and
     dialog buttons (`circle_drawer.rs:150-163`, `:218-231`); it is a non-text border, outline or `accent-color` in
     timer, cells, temp converter and the flight dates. Its hover `#3a5ce5` and active `#2a4cd3` shades are declared
     beside it in counter, flight booker, timer, CRUD and circle drawer. White reads 4.39 on the rest shade, 5.47 on the
     hover and 6.83 on the active (research.md §Measurements). Which colour replaces it is the operator's (a stand
     change, no engine change).
8. **The colour-changing states not measured (same chunk, same word).** Hover and active on the primary buttons and
   `back-btn`, the flight booker's toggle in its other position, an invalid date (`#c53030` on `#fff5f5`), and the date
   and the buttons in their non-boot enabled state; each is SC 1.4.3 text, and `stand_contrast` reads each task at boot
   only.
   - `Harness::text_contrast` reads computed styles; hover and active are reachable headlessly by a pointer move and a
     pressed button through the harness's input synthesis, the other states by real input. The mechanism is verified:
     a `PointerDown` sets the hover node then `active_node()` (`driver.rs:182-190`), and `mouse_down_at` /
     `move_mouse_to` dispatch and pump (`input.rs:112-136`). That `text_contrast` then reads the `:hover` / `:active`
     background is not measured at P3 — implement measures it first (research.md §Open questions).
9. **The CRUD list rows (same chunk, same word).** `crud-person-{id}` (role `option`) lie outside `controls()` and are
   not measured; the CARRY's own hypothesis, kept verbatim: "hypothesis: a selected row reads `#ffffff` on `#4a6cf7` =
   4.39, a fail (predicted at that chunk's phase, not measured)". Re-derived at HEAD from the declaration:
   `.list-item.selected` is `color: #ffffff` on `background-color: #4a6cf7` (`crud.rs:230-233`), 4.39 over the declared
   hex; the harness reading is implement's.
10. **Text outside the controls (same chunk, same word).** Task titles, labels and value displays are SC 1.4.3 text
    too, and this entry names controls only.
    - Measuring them widens the record past "every stand control"; whether they are asserted here or recorded is a
      scope lean for P4 (the CARRY placed them here, so they are in scope at least as a record). Verified as a lean
      basis: every declared text pair off the accent passes 4.5 — `#task-title` 18.88, `.timer-label` 9.74, CRUD labels
      11.59, the inactive toggle 9.33, the invalid date 5.11, the success message 6.15 (research.md §Measurements, over
      the declared hex; the harness reading is implement's).
11. **The CRUD list's tree focus (from 2026-10-13-stand-keyboard-harness; the operator's condition on fork F5).**
    `crud-list` takes focus and its ArrowDown, ArrowUp, Home and End move the selection, but the accessibility tree's
    focus is the list, never the option: the builder reads no `aria-activedescendant`, so an AT learns the current row
    only from its `selected` reading; a click on a row or on the list leaves nothing focused (measured at that chunk:
    `stand_keyboard`). "How the current option reaches the tree is decided at this entry's promotion."
    - Two shapes: the builder reads `aria-activedescendant` and the tree's focus follows it — a new attribute read in
      every document, an engine change for every document and a new attribute crossing (playbook :64, :36; the
      founder's, inputs#I1); or the stand moves real focus onto the option (a roving `tabindex` in the CRUD markup, the
      operator's) — which moves the Tab sequence `common::tab_order` and `stand_keyboard` assert. Verified: the tree's
      `focus` is the engine's focused node or the `Window` (`packages/blitz-dom/src/accessibility.rs:62`), and no
      `aria-activedescendant` is read in `packages/` or the stand (0 hits, research.md §Sweeps).
12. **The engine honours no `inert` (from 2026-10-13-stand-keyboard-harness).** A truthy `inert` takes no focusability
    away: Tab reaches a button under `inert: true` as under `inert: false`, both layout modes (pinned by
    `stand_keyboard::the_engine_honours_no_inert`, `tests/blitz-tests/tests/stand_keyboard.rs:598`); whether an inert
    subtree still takes a hit is not measured; the bridge clears a framework-written `inert: false`
    (`dioxus_falsy_boolean_attrs`). "A fix is an engine change for every document, a founder's fork."
    - No stand task renders `inert` — verified: 0 hits for `inert` in `examples/seven_guis/src` (research.md §Sweeps).
      The stand's SC 2.1.1 / 2.4.3 do not turn on it; the fork is whether this chunk asks it at all.

## Boundaries
- No mutation runs — the founder's word, relayed by the operator at take-up (inputs#I1): no mutation control is a
  witness in this chunk; a red-first witness is taken another way.
- Every founder fork is raised as a question dialog at P4 for the operator to take to the founder (inputs#I1): an
  engine change for every document (playbook :64 — items 11 and 12), a boundary widening (playbook :36 — item 2's
  platform adapter, item 11's attribute crossing), a new CI gate (the leg gating merges). Without his ruling the chunk
  asserts what the stand can meet by its own markup and records the rest with a route owner.
- Not this chunk: the three non-lean tasks' keys and requirement proof ("Stand requirement sweep"); the cold-agent test
  ("Cold-agent test"); coverage floors and flakiness budgets ("Quality gates").
- No new carrier, verb, field, log field or span: the work is stand markup and styles, test code over the existing
  `Harness`, snapshot and accessibility readers, and a CI leg's test list. Verified for the non-fork work: research.md
  §Files to modify names stand task files, blitz-tests files and two CI scripts, none an engine, driver or telemetry
  file; the list's name rides the already ratified name crossing (security-plan §Input Validation, `aria-label` row).

## CI read at take-up
- `ccfcda48` (the last wrap's flip, HEAD): CI#38521790203 push completed/success — green, 16/16 checks, wall 851 s.
  Nothing red to disposition.

## Gates and watches
- No `BLOCKED-ON`, no `WATCH`, no `PREREQ` on the entry.
