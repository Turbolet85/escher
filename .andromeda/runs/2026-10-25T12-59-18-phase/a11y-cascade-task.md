# Task — a11y · 2026-10-25-cold-agent-bar

## The chunk
Extends the 7GUIs stand so the driver's `start` serves every 0.2.0 check: Cells served as an over-budget screen read in parts, the flight booker's one-way / return choice turned from two `aria-pressed` buttons into a real single-choice `select`, and a load in flight carried on the stand (its shape a P4 fork).
Re-pins the stand's own a11y tables the flight booker change moves: the Tab order and Enter/Space tables, the contrast rows, the element ids and actionable keys, the snapshot, diff and act checks and the flight flow.
Gives the cold-agent pipe goals over v020-04 to v020-09 (recording and replay, lint, `advance`, the load wait, the select, the screen in parts) and its contract tests.
Runs one live cold-agent run on the operator's login over `escher-session mcp`, judged by the founder's wrong-call bar (his fork at P4), its written check replayed on fresh children.
No engine change for every document is intended; one the stand would need surfaces as a P4 fork.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the tasks `start` boots
- examples/seven_guis/src/stand.rs · edits the headless boot and `stand::options`
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return mode control
- examples/seven_guis/src/tasks/cells.rs
- examples/seven_guis/src/tasks/mod.rs
- examples/seven_guis/tests/flows/flight.sh
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`, the shared role and name tables
- tests/blitz-tests/tests/stand_keyboard.rs · edits the Tab order and Enter/Space tables
- tests/blitz-tests/tests/stand_contrast.rs · edits the control rows, `TEXT_ROWS`, `STATE_ROWS`
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/stand_flight_booker.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_load_wait.rs
- tests/blitz-tests/tests/stand_non_lean_tasks.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- scripts/cold-agent.sh · edits `TASKS` and the `stand` task's prompt and goals
- .github/scripts/test_cold_agent.py

## Needs
- The single-choice `select`'s keyboard model on the flight booker: ArrowDown / ArrowUp / Home / End and typeahead on the document's clock, one `input` then one `change` per change, SC 2.1.1 met on the new control
- Focusability and focus order: a `select` focusable with no tabindex, the flight booker's stated Tab order and enabled-state order moving as two buttons become one select (SC 2.4.3), `stand_keyboard`'s walk agreeing engine focus, snapshot `focused` and tree focus
- Enter and Space activation: the flight booker's mode buttons leaving the Enter/Space tables, the Book button's activation kept
- A click on a `select` focusing it (focus restoration on click)
- Disabled-ness keyed by `disabled`'s presence on a form control: the return-date input and the Book button, their focusability and the driver's `disabled` refusal where the snapshot's `enabled` reads false
- The Dioxus bridge's falsy boolean attributes removing `disabled`, reached through the flight booker's `disabled` bindings
- A select's option kept in the tree despite `display: none`, reading its chosen state as `selected` with no `aria-selected` read
- The stand's state-by-attribute controls: the flight booker's mode buttons' `aria-pressed` going away, its date inputs' `aria-invalid` and the CRUD list's `aria-selected` kept
- The snapshot over the tree: a select's `value` its chosen option's value, its options as children named by their labels, a `<label>` naming a `select`, no second role or name mapping
- Stable element ids carried as AccessKit `author_id` and the actionable-key rule: the select, its options and Cells' controls keyed, `unkeyed_actionable` empty on every served task
- The headless tree the shell hands the adapter equal to the harness's tree and every interactive-role node named, re-read on the changed flight booker
- The changed set: a select's changed choice marks it and drives the tree refresh
- SC 1.4.3 on the stand: `Harness::text_contrast` and `stand_contrast`'s three tables with their exempt rule, rows added or moved for the select and its states
- The visual lint's `contrast` reading through the same exempt rule, reached by the cold agent's lint goal on the served tasks
- The flight booker's invalid-date cue stated by `aria-invalid` and asserted without colour (error recovery), kept across the mode-control change
- The `a11y` CI leg's fourteen files and the `stand-repeat` leg's a11y targets (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, `select_interaction`) that the re-pinned stand files run in
- The snapshot's masked values (`MASKED_VALUE`) on every carrier the cold agent's run reads

## Not needed
- The platform adapter (`Adapter` creation before the window shows) and its windowed assistive-technology witness: no stand binary builds an adapter and the chunk runs no windowed check
- Role derivation's strong-native-semantics TODO and the HTML-AAM landmark inventory: no engine change is made
- The upstream description reads (`aria-describedby`, `aria-description`) and the merged `aria-expanded` · `aria-disabled` · `aria-level` reads: the snapshot reads none of them and the stand states none
- The range input's key and pointer model: the Timer's slider is unchanged
- `inert` and HTML's missing focus fixup: no stand change adds `inert`
- Language and direction, text alternatives in examples/assets fixtures, and IME: not on any surface the chunk touches
- Focus-ring, motion, colour-scheme and user-preference rules: recorded absent, none added
- Cognitive orientation cues, timeout extensions and the other NOT YET MEASURED cognitive items: no recorded intent the chunk reaches
- The structured a11y violation schema and screen-reader test patterns: not yet defined, not built here
