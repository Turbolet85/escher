# Task — tests · 2026-10-31-select-popup

## The chunk
Builds a painted list that a closed single `select` opens, in every document and renderer, a new engine overlay across paint, hit testing and keys.
A person or an agent chooses an option from the open list by pointer and by key, through the one write path `apply_select_choice`; an option can be acted on through the driver; the list closes without a choice.
Restates the lint's `overlaps` exclusion and the `covered` refusal with its schema text, since an open list's options read boxes of their own; paints or reads a group's label and reads or leaves `size`.
The focus and key contract, reduced motion and the engine's colours are the founder's P4 forks; the tests follow what he picks.
Adds `escher-0.4.0-incubator/` to `.gitignore`, and keeps the watch on the free-disk step of `Stand checks repeated`, read from CI runs and not re-built.

## Reached files
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-paint/src/render.rs
- packages/dioxus-native-dom/src/snapshot.rs · edits the `bounds` an option reads
- packages/escher-driver/src/execute.rs · edits the `covered` refusal of a closed select's option
- packages/escher-driver/src/lint.rs · edits the `overlaps` exclusion
- packages/escher-driver/src/schema.rs · edits the `covered` schema text and help
- tests/blitz-tests/tests/stand_contrast.rs · edits its table reading `select#flight-type`
- tests/blitz-tests/tests/stand_keyboard.rs · edits its table reading `select#flight-type`
- tests/blitz-tests/tests/stand_act_select.rs · edits its table reading `select#flight-type`
- tests/blitz-tests/tests/stand_lint.rs · edits its table reading `select#flight-type`
- .gitignore · edits the incubator lines
- .github/workflows/ci.yml · re-reads the free-disk step in `Stand checks repeated`

## Needs
- The select model in blitz-dom and its in-file tests over stated tables: a new open or closed state beside single and `multiple`, with `optgroup`, and the limits `select_interaction` and `stand_act_select` assert.
- `apply_select_choice` as the one write path: a choice from the open list goes through it, and the select then reads that choice, in the snapshot and in what a form's `construct_entry_list` submits for a named select, a single select's choice through a group included.
- The list as an overlay in paint: pixel checks on a harness frame of the open list and the closed select, through `screenshot` and `screenshot_rect`, and how a raster reads the list's options and a group's label.
- Hit testing over the open list: harness `hit`, `hit_node`, `click`, `click_at`, `mouse_down_at`, `mouse_up_at` and `move_mouse_to` reaching an option, and a click outside closing the list with no choice.
- Keys on a single select, today no opening on Enter or Space, arrows and typeahead changing the choice with no list open: harness `press`, `press_with` and `NamedKey`, `stand_keyboard`, and whether a closed select's arrows keep moving the choice follows his fork.
- Tab order with a list open or closed, as `stand_keyboard` reads `tab_order` and `ENABLED_TAB_ORDER` on the flight booker, whose return flight is chosen on `flight-type`.
- The snapshot: a closed select's options read `bounds` equal to its client rect, an open list's read real boxes; the `snapshot` unit tests that call `resolve(0.0)`, its text form and its diff, and the parsed-markup pins in `stand_snapshot_state`.
- The driver's action on an option of the open list, through `Session::run`: returning after settle with the snapshots' diff, a refused call leaving the instance unchanged; `stand_act_select` and the `session_common` builders that read it.
- The `covered` refusal: its closed-option reading restated, with `stand_act_obstructed` and `stand_act_refused` as its readers, the schema's unit tests and the error and refusal texts.
- The lint's `overlaps` exclusion over options under an `optgroup` included, restated for open options; `stand_lint` and `Session::lint`, read by integration checks and not by unit tests.
- A group's label, named in the tree and unpainted today since `content: attr()` is unbuilt, and `size`, unread today: each painted, read or left per this chunk's lean or fork, with a check for whichever is built.
- A `multiple` select's own rows, keys and clicks stay as 2026-10-31-engine-debts built them, its existing checks staying green unless the `size` lean or fork moves them.
- The accessibility tree says whether the select is open, `aria-expanded`'s `expanded` read by upstream's builder: `accessibility_states`, `stand_accessibility_ids` and the `controls` table's `flight-type` `Role::ComboBox`.
- The list's painted colours, a new engine default since the engine holds no tokens, under SC 1.4.3: `text_contrast` with `required_ratio` 4.5 and 3.0 over its option text, through `stand_contrast`, and the list's focus order under SC 2.1.1 and 2.4.3.
- Reduced motion as a P4 fork: any check it needs reads the harness's virtual time, whose `tick` and `advance_time` move it and `settle` and `screenshot` never do.
- The stand's `select#flight-type` as the stand's popup: the shared tables in `tests/blitz-tests/tests/common/mod.rs` and the stand checks that read it move with any new row.
- `dioxus-native`'s windowed path shows the same list from the engine, no OS window assumed: no test launches a window and `dioxus-native` holds none, so the claim is read at the engine and through the harness.
- The closed sets stay as they are: no verb, node key, cause, session error or exit status is added, so `test_capability_statement.py`'s counts over `docs/capabilities.md` and the schema tables hold unless a fork names a change.
- No carrier, listener, port, env var or workspace crate is added, as the scope states; anything a fork would need is named on that fork.
- The Boundary widening the wrap halts on: a validated surface admits new input and pixels, so the tests read this chunk's new input and pixels over the stand and every document.
- `.gitignore` gains `escher-0.4.0-incubator/` beside its two incubator lines before the wrap's P1.
- The free-disk watch: CI#41190771843's out-of-disk red, 0/3 since 2026-10-31-engine-debts, retiring at a recurrence or 3 green runs of `Stand checks repeated`.

## Standing conventions
- S1 · A new blitz-tests check is one target per behaviour under `tests/blitz-tests/tests/`, with no new `[[test]]` table, as `BlitzTestsTargetsTest` pins; regression tests are named for the bug they guard · §2 Test Strategy and §4 Unit Test Strategy
- S2 · A new unit test sits in a `#[cfg(test)] mod tests` block in its source file, or in a `#[path]` file declared `#[cfg(test)]` at its parent · §4 Unit Test Strategy
- S3 · A new stand or session check runs in both layout modes, boots through the stand's pinned options with the bundled DejaVu Sans and system fonts off, and holds no skip path · §3 Test Harness Contract and test-data-fixtures.md
- S4 · A fixture page more than one check reads is stated in `tests/blitz-tests/tests/fixtures/mod.rs`; a page with one reader stays in its check's file, and the module holds no print, `tracing` call, env read, file, socket or listener · §3 Test Harness Contract and §7 Test Data & Fixtures
- S5 · A shared stand table in `common/mod.rs` is stated expectation, never a second role or name mapping; a check declares `mod common;` or `mod session_common;` after its `use` block and its reader counts are kept · §3 Test Harness Contract
- S6 · A session check holds no sleep and no wall-clock read unless wall time is its subject · §5 Integration Test Strategy
- S7 · A frame compared byte for byte stays on one host; a check compared across OSes holds no shot, and a shot of a document with no resolved font is refused with no skip path · §2 Test Strategy
- S8 · A stand check holds the zero-failure budget of the `stand-repeat` leg, three rounds with no retry, and a target with no `stand_` prefix that must repeat joins that leg's named list · §9 CI Integration and §10 Quality Gates & Coverage Targets
- S9 · New code stays above the per-crate line floors of `coverage_floor.py`, `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % · §10 Quality Gates & Coverage Targets
- S10 · A check holds under both the workspace and the `-p blitz-tests` engine features, every cargo leg but `fmt` runs `--locked`, and a green per-package run is not the workspace leg's green · §9 CI Integration
- S11 · A chunk that adds stand checks states the new `run stand` passed, failed, ignored and file counts, a new `stand_` file being picked up by its prefix with no script change · §3 Test Harness Contract
- S12 · A check or a log path writes no stable id, accessible name or typed text to stdout, stderr or the state directory, and no third-party test or mocking framework is added · §5 Integration Test Strategy and §8 Mocking & Stubbing Discipline

## Not needed
- The apps/browser unit tests of about pages, history, favicon, URL suggestions, persistence and tabs: the chunk changes nothing under `apps/browser`.
- blitz-vibey-script's DOM, Selection and Preact tests: the chunk adds no script API.
- The WPT runner, its scoring, quarantine and diff scripts and its workflows: repository-guarded to upstream, and no entity of them is edited.
- The style bridge's font feature mapping, the SVG parsing tests, list markers and `text-transform`: the list does not reach those paths.
- blitz-paint's gradient and `css_box` corner unit tests: the overlay does not change corner or gradient geometry.
- `@font-face` and resource loading, `HeldNet`, `LateNet` and `LoadNet`: the list loads nothing and the flight booker boots offline.
- The session host's socket lifecycle, expiry watches and `Host` guard: no carrier, listener or session error is added.
- The cold-agent pipe's shim contract tests and verdict fields: `scripts/cold-agent.sh` is not edited.
- The stand Timer's ticks and `timer_step`: the Timer holds no select.
- Upstream merge records and the test-that bump: upstream sync is the next epoch's work.
- Non-adjacent `multiple` sets through the driver: out of scope, a residual since `press` holds no Ctrl.
- The human-read example fixtures, `paint_bench` and the `screenshot` example: they assert nothing and the chunk gives them no duty.
