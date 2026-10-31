# Task — tests · 2026-10-31-engine-debts

## The chunk
Six recorded engine debts (`.andromeda/residuals.md` lines 31, 32, 33, 34, 39 and 40), each taken to the founder as its own fork at P4 and ending as his word says: fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual; no provisional answer, so every fix's check is conditional on his word.
The debts: the transformed scrolling box in `visible_region`, the `inert` focus fixup, native checkedness of checkbox and radio, the select model's stated limits, the select-option overlap in `lint`, and the served pages' missing keyboard and contrast rows.
It adds checks that change no engine: Tab-order, keyboard-activation and contrast rows on Cells and the load page, and one check each for `:open` on `details` and `dialog`, the windowed shell's redraw route and `pseudo_element_flow_classification` in both layout modes.
It re-reads the standing witnesses `stand_keyboard`, `stand_contrast`, `inert_subtree` and `select_interaction` and the `incremental_oracle` identity; P3 also reads upstream's `0980cce5f` over the same code, without taking it.

## Reached files
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/layout/writing_mode.rs
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/events/
- packages/escher-driver/src/lint.rs · edits `overlaps`
- tests/blitz-tests/tests/transformed_client_rect.rs · edits the pin `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`
- tests/blitz-tests/tests/inert_subtree.rs · edits the pin `removing_inert_restores_focus_hit_and_tree`
- tests/blitz-tests/tests/stand_lint.rs · edits the pinned `[0, 1, 0, 0]`
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · edits its checks, to run `for incremental in [false, true]`
- tests/blitz-tests/tests/select_interaction.rs · re-reads `select_interaction`, a standing witness
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads `stand_keyboard`, a standing witness
- tests/blitz-tests/tests/stand_contrast.rs · re-reads `stand_contrast`, a standing witness
- tests/blitz-tests/tests/incremental_oracle.rs · re-reads `incremental_oracle`, the identity of the two layout modes
- .github/scripts/test_capability_statement.py · edits the capability statement's pin, where a fix moves the text of `off-screen` or `covered`
- docs/capabilities.md · edits its statement of `off-screen` and `covered`
- .andromeda/residuals.md · edits lines 31, 32, 33, 34, 39 and 40
- .andromeda/architecture-detail/standard-contracts.md · edits the `ElementData::classes` row and its sidecar entry, the wrap's amendment

## Needs
- `visible_region` reading a scrolling box under a transform where it stands untransformed, and the driver's `off-screen` refusal of a target that shows: the pin in `transformed_client_rect` and any check that reads this limit through the driver.
- If the transformed-box fix is taken, its proof on a pan/zoom container like the node graph's, a CSS `transform` on a container holding absolutely positioned nodes, beside the existing translated fixture; whether a scale is in reach beside a translate is P3's to measure.
- The scrolled-box bounds and hit readings a `visible_region` fix sits beside: `scrolled_box_absolute_position`, `get_client_bounding_rect`, the snapshot's `bounds` and the harness's rect readers, which read a scrolled box where it stands by the founder's ratification, and which upstream's `0980cce5f` would collide with.
- The engine's bounds reader through its build: `writing-mode` and `tracing` are on in the workspace build only, so a change in `layout/writing_mode.rs` or the bounds it feeds owes a check of each body, and a green `-p blitz-tests` run is not the workspace leg's green.
- The `inert` focus fixup: a node holding focus when it or an ancestor becomes inert, the focus readings of the snapshot and the tree, the `inert_subtree` pin and SC 2.4.3.
- Native checkedness: `toggled` written from an `<input type="checkbox">` or `type="radio"` checked state in the tree the shell hands the adapter (HTML-AAM), over a crossing that holds closed values only; a fix that widens that crossing escalates.
- The snapshot readings beside that fix: `checked`, read from the element and already right, and `Reading`'s `pressed`, written from the same node property; P3 reads whether either moves, and the boolean model's limits pinned at dioxus-native-dom and the engine are where it would show.
- The select model's stated limits, each priced as its own fork: a `multiple` select and `optgroup` options unmodelled, no painted popup, a driver `click` on an option refused `covered`; `node/select.rs`'s rule tables and `stand_act_select` read this model, and a popup proven as pixels falls under the per-OS shot rule.
- The engine `change` event a checkbox, radio or text input does not fire: the event route through `events/` and the document's event bridge, a fork of its own beside the select's.
- The select-option overlap: each `ListBoxOption` holding its select's client rect in the snapshot, `lint`'s `overlaps`, `stand_lint`'s measured `[0, 1, 0, 0]` from `flight-type-one-way` and `flight-type-return` at the flight booker's boot; a painted popup changes what an option's bounds should read, so this fork is asked with that one.
- The served pages' new rows: Cells and the load page booted through `stand::boot_cells` and `stand::boot_load`, a Tab-order and keyboard-activation row for each including the `load-fetch` Button, in tables of their own or beside `tab_order` and `ENABLED_TAB_ORDER` in `tests/common/mod.rs`, whose counts and reader lists record them.
- A keyboard-activation row on `load-fetch` acts over the stand's `LoadNet`: it waits out that load through the driver's load wait, as `stand_load_page` does, with no sleep or clock read of its own.
- The contrast rows through the harness's `text_contrast` and its required ratios; a row that reads red moves the stand's colour or the engine's key handling, which is a further fork and never a silent re-pin.
- SC 2.1.1, 2.4.3 and 1.4.3 on the stand and the served pages: the OPEN gap in a11y-plan §5 these rows close, and the ground of the route's A11y re-read in Epoch 6.
- A check of its own for `:open` matching a `details` or `dialog` carrying `open`; a wrong reading surfaces as a fork.
- A check of its own for the windowed shell's redraw route through its event loop, read today only by the boot smoke and `focus_redraw`; a wrong reading surfaces as a fork.
- `pseudo_element_flow_classification`, upstream's file, which asserts neither layout mode: running it in both is an edit to an upstream test, and the additivity note records it.
- The gap lines of test-plan §1 these checks answer or the forks leave standing: the select's limits asserted by no check, `:open` with no test of its own, and the redraw route and the pseudo-element file as recorded at the upstream take.
- `incremental_oracle`: any engine fix keeps incremental and non-incremental layout identical, by the scope's own boundary.
- `off-screen` and `covered` state their limits in their own schema text, so a fix moves that text and whatever pins it: the driver's unit tests of its refusal texts and verb schema, and the capability statement's pin.
- The closed counts `NODE_FIELDS` thirteen, `VERBS` twelve and eight causes hold, as `test_capability_statement.py` counts them; a fix that would add one is a boundary question brought as a fork.
- Every engine fix is an engine change for every document: proven on the stand or a fixture, recorded as ratified by the founder with his word at this chunk's P4, its merge surface carried to Epoch 3's sync.
- The stale name in CARRY 3: `DocumentMutator::remove_attribute` is `clear_attribute`, a spec text for the wrap and never a phase edit; any plan entity spelling the old name is the same amendment.

## Standing conventions
- S1 · a pipeline check runs `for incremental in [false, true]`, the same assertions in each layout mode · §5 Integration Test Strategy
- S2 · blitz-tests holds one target per behaviour file under `tests/`, with no new `[[test]]` table, as `BlitzTestsTargetsTest` pins · §2 Test Strategy
- S3 · a regression test is named for the bug it guards · .andromeda/test-plan-detail/test-strategy.md
- S4 · a unit test sits inline in `#[cfg(test)]` or in a `#[path]` child file with its test path unchanged · §4 Unit Test Strategy
- S5 · a check reads shared tables and helpers through `mod common;`, `mod session_common;` and `mod fixtures;`, stating no second role or name mapping · §3 Test Harness Contract
- S6 · a fixture is stated in code and seeded by nothing; a page with one reader stays in its check's file and a page with more moves to `tests/fixtures/mod.rs`, which holds no print, env read, socket or `blitz_net` · .andromeda/test-plan-detail/test-data-fixtures.md
- S7 · a stand check asserts under the bundled DejaVu Sans with system fonts off and no skip path · §2 Test Strategy
- S8 · a session check holds no sleep and no wall-clock read unless wall time is its subject · §5 Integration Test Strategy
- S9 · a `stand_` file is picked up by `agent-run.sh` with no script change, any other is read by name and by `run all`, and each change re-counts `run.end` in the contract's proof line · §3 Test Harness Contract
- S10 · a stand check holds the zero-failure budget of `stand-repeat`, three rounds with no retry, whose named targets a new non-prefixed check may join · §10 Quality Gates & Coverage Targets
- S11 · a change passes `bash .github/scripts/ci-leg.sh fast` before push, every cargo command `--locked` · §9 CI Integration
- S12 · the per-crate line floors hold for `packages/escher-driver`, `packages/dioxus-native-dom` and `packages/blitz-test-harness`, and a mutation run stays on the dev host · §10 Quality Gates & Coverage Targets

## Not needed
- WPT conformance, its runner, scoring, quarantine and workflows: the job does not run on the fork, and the scope names no WPT row.
- The tests of `apps/browser`, persistence and URL suggestions: no debt reaches the browser app.
- blitz-vibey-script's DOM, Selection and Preact checks and its script helpers: no debt touches the script surface.
- The cold-agent pipe, `scripts/cold-agent.sh`, its verdict and `test_cold_agent.py`: the scope does not fire or change it.
- The session host's lifecycle over its socket, its state directories and the `stand_session_lifecycle` and `stand_session_quiet` process checks: no debt changes how a session starts or stops.
- CI's package-install bound, `apt-install.sh`, its caches, action pinning and permissions: the scope adds no install and no workflow step.
- The dependency history of test-that and keyboard-types: this entry bumps no dependency.
- The windowed assistive-technology witness and the platform adapter owed under its residual line: stated out of this version.
- Cells' selected-cell state, residual line 24: out of this entry.
- The canvas renderer, the CPU renderer's custom-paint gap, virtual-node production and a driver `drag`: 0.4.0 capabilities; the harness's own `drag` helper is untouched too.
- The take of `0980cce5f` and the merge-surface and upstream-delta reads: Epoch 3's sync owns them.
- The password mask, its caret and their checks: no debt reaches a password field.
- Inline tests of font feature mapping, SVG parsing, list markers, `text-transform`, gradients and CSS box corners: no debt reaches them.
- `clock.rs`'s `VirtualClock` and `WallClock` pins and the Timer's `TimerTicks`: no debt moves time.
- `paint_tree_bench` and the human-read fixtures under `examples/assets`: no budget is set and no assertion lives there.
