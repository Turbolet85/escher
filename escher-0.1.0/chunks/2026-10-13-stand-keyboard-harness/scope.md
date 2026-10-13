# Scope — 2026-10-13-stand-keyboard-harness

**Working entry** (escher-0.1.0/working-route.md:92, Epoch 6 — Polish & ship): "Stand keyboard harness — Tab,
Shift+Tab and activation keys dispatched headlessly on the stand, focused node read back per step (per a11y-plan §5)",
with seven CARRY blocks folded below. P3's premise closure has run: each bullet the entry did not state is verified
(tag dropped) or corrected (`[premise-corrected: …]`), evidence in `research.md`.

## What the chunk builds
- A headless keyboard harness on the stand: Tab, Shift+Tab and activation keys dispatched through the real event
  pipeline with no window or GPU, and the focused node read back after every step (per a11y-plan §5). (stated)
- "Read back per step" means three readings that must agree at each step: the stable element id of the node that
  holds focus, exactly that node's snapshot `focused` reading, and the accessibility tree's focus carrying its author
  id — the shape `stand_snapshot_state`'s `focused()` helper asserts (`stand_snapshot_state.rs:61-77`).
- The stand's surface is the four lean tasks `LeanTask::ALL` boots (counter, flight booker, timer, CRUD): each task's
  whole forward sequence by Tab and its reverse by Shift+Tab, wrap-around included — the engine's walk wraps through
  the root both ways (`traversal.rs:195-285`) — in both layout modes. The expected sequences are the stated
  `tab_order` table (`stand_accessibility_ids.rs:18-38`), today read by `focus_next_node` and never by a key press.
- "Activation keys" are Enter and Space on a focused control. [premise-corrected: at HEAD the engine activates nothing
  on Enter or Space — `keyboard.rs:71-118` acts only for a text input's editor and a range input's arrows and
  `Home` / `End`, no lean task handles a key, and the bridge's `synthetic_click_event` has no caller; measured through
  the driver on `counter`: Tab, Tab focuses `counter-increment`, `press enter` and `press space` each return an empty
  diff with the value still `0`, while a click reads `1` (`.andromeda/runs/2026-10-13T22-07-35-phase/probe-counter.txt`).
  No stand button is operable from the keyboard: SC 2.1.1 is not met on the stand at HEAD.] Whether the engine gains
  keyboard activation is fork F1 (P4), an engine change for every document.
- The harness serves the a11y target WCAG SC 2.1.1 (Keyboard) and 2.4.3 (Focus order) — CLAUDE.md Critical Warnings.
- It lands as a blitz-tests integration file (one file, one target per behaviour) with the shared Tab sequence and
  focus read-back helper stated once in `tests/blitz-tests/tests/common/mod.rs`. [premise-corrected: the target pin
  `test_blitz_tests_targets.py:18-30` asserts the manifest's shape and names no file, so a new file moves nothing in
  it; the counts that move are test-plan's file, target, reader and `run stand` links.]

## Folded freight (the seven CARRY blocks at working-route.md:92)
1. **Focus defects from 2026-10-06-snapshot-state-fidelity.** (1) A control whose `disabled` attribute is bare or
   `"false"` reads not enabled in the snapshot yet stays focusable (measured: `is_focussable()` true beside
   `enabled: Some(false)`, in the dioxus-native-dom unit pin and `stand_snapshot_state`'s parsed-markup pin; the
   bridge itself writes neither form). A Tab and Shift+Tab key-press focus check already exists in
   `stand_snapshot_state`.
   - Whether this chunk fixes defect (1) is fork F3: `flush_is_focussable` reads `disabled` by `attr_parsed`
     (`element.rs:703`) for every document, read again by the Tab walk, autofocus and `unkeyed_actionable`, and pinned
     by three checks (`research.md` §Graph impact) — an engine change for every document (playbook :64, inputs#I1).
2. The CARRY's own hypothesis, kept verbatim: "(2) hypothesis: a click on a plain `button` clears focus instead of
   focusing it (read from the code at that chunk's phase — pointer.rs `handle_click` has no button arm; measured there
   only for a click on a heading)" — verified at HEAD: no arm matches a plain `button`, so the walk ends unmatched and
   focus is cleared (`pointer.rs:853-856`); measured on `counter-increment` (P3 probe above).
3. **A reading for the plain-button focus hypothesis (from 2026-10-07-act-by-id).** A driver click on `crud-create`
   while `crud-surname` held focus returned `crud-surname` among `changed` — it lost focus — and named no node as
   gaining it (measured at `tests/blitz-tests/tests/stand_act_diff.rs`, the Create step, both layout modes);
   `stand_act_keys` adds a driver-run Tab and Shift+Tab focus check on CRUD.
4. **The plain-button focus reading, measured (from 2026-10-10-scrolling-box-bounds-and-hit).** After an accepted driver
   `click` on a plain `button` no node reads `focused` and the accessibility tree's focus carries no author id, in both
   layout modes and under both builds (the restated `stand_act_obstructed`; a11y-plan §5 → Focus restoration).
   "Whether a click should focus a button is decided at this entry's promotion."
   - That decision is fork F2: a click that focuses a `button` changes `handle_click` (`pointer.rs:640-866`) for every
     document — the founder's (playbook :64; inputs#I1) — and 24 standing check files click a stand button
     (`research.md` §Sweeps), each a candidate pin on that branch.
5. **The delete binding has one witness (from 2026-10-07-act-by-id).** `Harness::apple_keybinding` dispatches a macOS
   standard key binding to the focused element; its one standing check is the driver's `press backspace` in
   `stand_act_keys`, which reaches it on the macOS CI leg only (CI#37633611745, `Test (macos)`); on Linux and Windows
   no standing check calls it (read once on Linux through a deleted probe,
   escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/controls.md).
   - This chunk adds a standing check that calls `Harness::apple_keybinding` on every OS — feasible: the editor's
     `apply_apple_standard_keybinding` carries no platform gate (`text.rs:372`), and a direct call is not the
     platform-bound `press` testing.md:52 forbids.
6. **The second position reader reads a scrolled box shifted (from 2026-10-10-scrolling-box-bounds-and-hit).**
   `Node::absolute_position` answers, for a box that is itself scrolled, the box's position less its own scroll offset
   ((40, 50) scrolled by (70, 120) reads (−30, −70)), both layout modes and both builds, pinned by
   `scrolled_box_absolute_position` (red by design when the reader is fixed — rewritten then, not deleted);
   `get_client_bounding_rect` reads where the box stands, so the two disagree. Built on the unfixed reader (re-derived
   by the graph at P3): `handle_dom_event`, `synthetic_click_event_data`, `Node::focus`'s IME area, `scroll_into_view`
   and the harness's `layout_rect_of` and `write_node`. The fix is an engine change for every document in
   upstream-owned merge surface (`packages/blitz-dom/src/node/node.rs`), upstreamable.
   - Whether this chunk fixes the reader is fork F4 — an engine change for every document (playbook :64; inputs#I1).
7. The CARRY's own hypothesis, kept verbatim: "hypothesis: an activation key on a focused box that is itself scrolled
   clicks off its centre by that offset (read from the code, not measured — no stand control that takes a key is
   itself a scrolled box)". [premise-corrected: at HEAD no key synthesizes a click (`keyboard.rs:71-118`), so the
   hypothesis has no path; it gains one only if F1 activates through `synthetic_click_event_data`, whose point reads
   `absolute_position` (`node.rs:1929`) — the click's target is the node either way, only the event's coordinates
   would be off.]
8. **A listbox option no keyboard reaches (from 2026-10-10-snapshot-states).** Each CRUD row states `role="option"` in
   a `listbox` with no `tabindex`, so Tab never lands on a row and no key selects one (`crud.rs:56-82`: an `onclick`
   only); the CRUD's forward focus sequence is held by `stand_accessibility_ids`; a `tabindex` on the rows moves the
   Tab order that plan pins. "How an option is reached by keyboard — and the pinned Tab order restated with it — is
   decided at this entry's promotion."
   - That decision is fork F5; a stand-markup answer (a `tabindex` and a key handler in the CRUD's own markup) changes
     the app only and is the operator's, while an engine listbox keyboard model would change every document.
9. **A falsy Dioxus `inert` (from 2026-10-13-upstream-sync-polish-and-ship).** The bridge clears a framework-written
   `inert: false` by upstream's 28-name `is_bool_attr` (`mutation_writer.rs:458`), so the content stays focusable; no
   check of ours reads it (`dioxus_falsy_boolean_attrs` names the 27 of its own list) — not measured.
   - This chunk adds the check. [premise-corrected: the engine implements no `inert` (`grep -rn inert` over
     `packages/blitz-dom/src`: one doc comment), so content is reached by Tab under a truthy `inert` too and a Tab-reach
     check cannot fail; the check that can is the attribute — absent under `inert: false`, present under `inert: true`
     — with Tab reaching both recorded as the engine's measured limit, a spec claim (a11y-plan §5 "so it no longer
     makes content inert") amended at the wrap and the engine's `inert` given a route owner.]

## Boundaries
- Not this chunk: a windowed witness at an assistive technology and the stand's contrast re-colouring ("Stand a11y
  assertions"); keys for the three non-lean tasks and the stand requirement sweep ("Stand requirement sweep").
- No mutation runs — the founder's word, relayed by the operator at take-up (inputs#I1).
- Every engine change for every document a fork here would make (F1–F4) waits on the founder's ruling, raised as a
  question dialog at P4 (playbook :64; inputs#I1); without it the chunk measures and records, and the defect keeps a
  route owner. F1 and F2 change keyboard and pointer default actions for remote documents too, as the range model
  did: a security-plan §Input Validation row each, the Boundary widening pattern (playbook :36) named on the question.
- No new carrier, verb, field, log field or span: the harness is test code over the existing `Harness`, driver and
  snapshot readers — verified: the driver's twelve key names already hold `enter` and `space` (`schema.rs:14-27`).

## CI read at take-up
- `e40d9437` (the last wrap's flip, HEAD): CI#38473569865 push completed/success — green, 16/16 checks, wall 896 s.
  Nothing red to disposition.

## Gates and watches
- No `BLOCKED-ON`, no `WATCH`, no `PREREQ` on the entry.
