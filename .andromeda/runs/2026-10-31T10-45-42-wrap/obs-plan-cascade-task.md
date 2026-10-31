# Task — obs-plan · 2026-10-31-engine-debts

## The session
The engine changed for every document: `visible_region` reads clipping boxes through transforms, a non-invertible transform takes no hit, focus leaves a node under `inert` with no `blur`, the tree's focus falls back to the window, a native checkbox or radio writes `toggled` on checkable roles, a `multiple` select and `optgroup` work in keys, pointer, form, tree, sheet, snapshot and bridge, and the engine sends `change` on a checkbox and a radio.
The driver changed without a new verb, argument, key name, result field, cause, session error or exit: `type` reaches a `multiple` select, `lint` skips a closed select's options, and the fixed text of `off-screen`, `type` and `press` moved.
The stand and CI changed: Cells' `cells-grid` takes one Tab stop and arrow keys, `stand-repeat` names fifteen targets instead of ten, and `Stand checks repeated` gained a pinned free-disk step.
Five checks were added and thirteen edited; the shared stand module gained two tab-order constants; fixture readers went from 24 to 28 and the workspace tally to `214 · 1181 · 0 · 12`.
The measured instrumentation result is no log site, print, env read, listener, wall-clock read or panic path on the added lines, with 2 `info_span!` sites unmoved and `host_log` and `mcp_log` at 0 needles under both builds.
Retired: the script runtime's `change` synthesis and `target_is_checkbox_or_radio`, the snapshot's private `select_parent`, and upstream's both-modes exception in `pseudo_element_flow_classification`. The report changed 47 files; the list below holds 40, leaving out `packages/blitz-dom/assets/default.css`, `docs/capabilities.md`, `Cargo.lock`, `.github/scripts/test_ci_workflows.py`, `packages/escher-driver/src/schema_tests.rs`, `packages/dioxus-native-dom/src/lib.rs` and `packages/blitz-vibey-script/tests/dom.rs` for the item limit.

## Reached files
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/mutator.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/form.rs
- packages/blitz-vibey-script/src/runtime/events.rs
- packages/blitz-shell/src/event.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/dioxus_document.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/refusal.rs
- packages/escher-driver/src/schema.rs
- packages/escher-driver/src/lib.rs
- examples/seven_guis/src/tasks/cells.rs
- .github/scripts/ci-leg.sh
- .github/workflows/ci.yml
- tests/blitz-tests/tests/visible_region_transformed.rs
- tests/blitz-tests/tests/hit_non_invertible_transform.rs
- tests/blitz-tests/tests/open_state.rs
- tests/blitz-tests/tests/checkable_change_event.rs
- tests/blitz-tests/tests/text_range_no_change_event.rs
- tests/blitz-tests/tests/transformed_client_rect.rs
- tests/blitz-tests/tests/stand_act_transformed.rs
- tests/blitz-tests/tests/inert_subtree.rs
- tests/blitz-tests/tests/accessibility_states.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_load_page.rs
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs
- tests/blitz-tests/tests/stand_non_lean_tasks.rs
- tests/blitz-tests/tests/common/mod.rs

## Touches
- The reader count of `tests/blitz-tests/tests/fixtures/mod.rs` moved from 24 to 28 at `obs-plan.md:92`; `session_common`'s 31 is unmoved.
- The target name lists at `obs-plan.md:76` and `:87` name no count the session moved; only their citation digits can move.
- The select checks' silence statement at `obs-plan.md:95` now pins more: `multiple` keys and clicks, `optgroup`, `select_cursor`, `press_multiple_select_key` and `click_select_row`. The `DomEventKind::Change` mention there is unmoved.
- The silence claims over every edited check stand on the new census, which read 0: `transformed_client_rect`, `stand_act_transformed`, `inert_subtree`, `accessibility_states`, `select_interaction`, `stand_act_select`, `stand_lint`, `stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`, `pseudo_element_flow_classification` and `stand_non_lean_tasks`.
- The shared module `tests/blitz-tests/tests/common/mod.rs` gained `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` at `:93-98`. Its silence reading stands; the report names no move in its count of readers.
- The `Focussed node` INFO line was re-read with its needles at 0, under the new focus fixup that calls `clear_focus`. The claim is unmoved; its citation `document.rs:1901-1902` moved.
- `host_log` and `mcp_log` were re-run at 0 needles under both builds, and the `seven_guis_native` smoke at `RUST_LOG=info` read green with `service.name=seven_guis`. These claims are unmoved.
- The driver's span claims were re-read unmoved: 2 `info_span!` sites, and no verb, argument, result field, cause, session error or exit added, so the `command` and `tool_call` fields and their closed words stand.
- The silence claims over the private driver modules reach new code in `execute` (`focus_multiple_select`) and `lint` (`Entry.in_closed_select`). The added-lines census reads 0 over both.
- The fixed answer text moved in `off-screen`'s `meaning()`, `type`'s help, `press`'s help and the crate docs. It stays fixed `'static` text holding nothing a caller typed, and it is an answer, never a log line.
- The answer crossing reaches a `multiple` select: the snapshot's `value` reads `select_value`, the first chosen option in tree order, and its rows and groups read their own client rects.
- Blitz-dom's log-site citations can move through the added lines in `document.rs` (`:340-341` to `:780` and `:2574-2578`): `:1381`, `:1468-1487`, `:1471` and `:1472-1477`.
- `mutator.rs` citations can move through its added ranges up to `:1291`: `:1376`, `:1395` and `:1417`.
- `events/pointer.rs` citations can move through its added ranges at `:21-24`, `:793-836` and `:889-899`: `:929-935`, `:930` and `:935`.
- `runtime/events.rs` citations can move through the two added lines (`:4`, `:154`) and the retired synthesis: `:20-21`, `:34-36`, `:58-74`, `:81`, `:84-96`, `:172` and `:278`. The `report_js_error` site is not named as changed.
- The citation `packages/escher-driver/src/lint.rs:176-231` can move through the additions at `:7` and `:202-203`.
- Any citation in the detail into a reached file past its added lines can move, including `scrolling.rs`, `node/node.rs`, `node/select.rs`, `events/keyboard.rs`, `accessibility.rs`, `form.rs`, `snapshot.rs`, `dioxus_document.rs`, `execute.rs`, `refusal.rs`, `schema.rs`, `lib.rs`, `ci-leg.sh` past `:59` and `ci.yml` past `:310`, which moves by 6.
- The logged-events groups that list engine and test emitters gain no site through this session; only their citations into `blitz-dom`, `blitz-shell`, `dioxus-native-dom` and `blitz-vibey-script` can move.
- Any stated count moved if obs-plan holds it: `blitz-tests` `.rs` files 148 to 153, test targets 147 to 152, workspace tally `209 · 1125 · 0 · 12` to `214 · 1181 · 0 · 12`, `a11y` leg `14 · 85` to `14 · 112`, `agent-run.sh run stand` `205 · 0 · 7` to `222 · 0 · 7`. `stand_*` files at 50 and `Ran 108 tests` are unmoved.
- Any stated per-target count moved if obs-plan holds it: `inert_subtree` 4 to 6, `accessibility_states` 5 to 6, `select_interaction` 7 to 18, `stand_act_select` 5 to 10, `stand_act_transformed` 2 to 3, `stand_lint` 7 to 8, `stand_keyboard` 15 to 21, `stand_contrast` 5 to 7, `stand_load_page` 2 to 3, `stand_cells_served` 4 to 5.
- Any statement about tests inside a crate can go stale: blitz-dom unit tests 106 to 112, blitz-shell 0 to 1, blitz-vibey-script `dom` 30 to 31; escher-driver 100 and dioxus-native-dom 82 are unmoved.
- `stand-repeat`'s named targets moved from ten to fifteen in `ci-leg.sh`; any obs-plan CI text counting them is stale.
- `Stand checks repeated` gained `jlumbroso/free-disk-space` at a pinned sha (`ci.yml:311-316`). The report names no new artifact or upload, so the fork's two uploaded artifacts should still read true; a CI group listing that job's steps would need it.
- The statement that the upstream take's `pseudo_element_flow_classification` is silent stands, but its three tests now run in both layout modes, and any wording of the exception that kept it to one mode is retired.
- The upstream-take statement at `obs-plan.md:98` names `flush_open_state` and the shell's redraw route; `open_state` and `blitz-shell`'s first unit test now pin them, adding no log site.
- Any obs-plan text naming a retired item is stale: `select_parent`, `target_is_checkbox_or_radio`, the runtime's `change` synthesis, the pin name ending `reads_outside_its_visible_region`, and the word `untransformed` in driver text.
- Cells' reacting-element count moved from 676 to 677 through `cells-grid`; the 676 data cells are unmoved, so a statement of the data cells stands.

## New facts
- F1 · log-coverage · The five new checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` are silent, the silence census reading 0 over the chunk's checks.
- F2 · log-coverage · The transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines.
- F3 · log-coverage · `blitz-shell/src/event.rs` gained the crate's first unit test (`:128-172`), an inline test held out of the added-lines census, so that census's 0 does not speak for it.
- F4 · log-coverage · The new readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site, field or print.
- F5 · log-coverage · Cells' `cells-grid` Tab stop, key handler and `MountedData` handles, and its `.cell.current` outline, are instrumentation n/a and add no log site, print or env read.
- F6 · pii-scrubbing-compliance · A `multiple` select's chosen set leaves only as form entries on `input` and `change`, one per chosen option, as the snapshot's `value` and as the options' tree nodes, never as a log line or field.
- F7 · pii-scrubbing-compliance · An `optgroup`'s non-empty `label` becomes a `Role::Group` name in the accessibility tree, inside the ratified name crossing; it is not painted.
- F8 · pii-scrubbing-compliance · The engine's `change` on a checkbox or radio carries the value class its `input` already carries, and a page's script now hears one `change`, from the engine.

## Untouched
- escher-telemetry's sink, line format, target allowlist, scrub and chaining panic hook: `packages/escher-telemetry/` is not reached, and no scrub or redaction shape moved.
- The panic hook section does not gain the `RefCell already borrowed` hazard: the async form that panicked did not ship, and no panic path was added.
- The install sites of `escher_telemetry::init` in `examples/seven_guis/src/main.rs` and `session_host.rs`, and the stand's `stand.rs` boots and `LoadNet`: none of those files is reached.
- Metric coverage: `debug_timer`, the resolve phase timings and blitz-paint's counters are not reached, and `packages/blitz-paint/` shows no diff.
- The feature wiring and telemetry dependencies in the `Cargo.toml` files: no manifest was edited, and the only dependency move is a `pulldown-cmark` lock bump reached through `rdme`, not a telemetry crate.
- The agent-run and cold-agent log formats, state areas and verdict fields: `scripts/agent-run.sh` and `scripts/cold-agent.sh` are not reached; a moved `run stand` tally does not change an event's shape.
- `.github/scripts/apt-install.sh`'s diagnostic line and the upstream-only publish and WPT workflows: none is reached.
- The emitters and raw values of `apps/browser`, `examples/wasm_hello`, `wpt/runner`, `blitz-net` and `blitz-html`: none is reached.
- The script diagnostics drain, `take_js_errors`, `take_messages` and the 256-error bound in `blitz-vibey-script`'s document and state modules: neither module is reached.
- The body's NOT YET MEASURED notes and the NO RECORDED INTENT sections §10, §11 and §12: the session measured none of those subjects and records no intent for them.
