## 2026-10-31-engine-debts — upstream's both-modes exception ended
**Section:** §Conventions → Tests (conventions.md: conv-tests); §Established Decisions → Layout (established-decisions.md: dec-layout); §Existing Scopes (existing-scopes.md: blitz-tests' `pseudo_element_flow_classification`)
**Change:**
- The both-modes rule holds no exception: was «except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it»; now scenarios run in both layout modes `for incremental in [false, true]`, no clause standing.
- The flow-classification decision: was its test file not asserting both layout modes; now asserting them, every test running `for incremental in [false, true]`.
- The target's scope record: was the file does not run both layout modes; now each of the three runs them.
**Why:** `pseudo_element_flow_classification`'s three tests now run in both layout modes, so no exception is left to state.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a multiple select's chosen set, keys and pointer
**Section:** §Cross-cutting Patterns → Invalidation and state integrity (cross-cutting-patterns.md); §Established Decisions → Scrolling and input, DOM semantics (established-decisions.md: dec-scrolling-and-input, dec-dom-semantics; architecture.md)
**Change:**
- The one path: every choice that changes the chosen option or a `multiple` select's chosen set goes through `apply_select_choice`, which takes the chosen set; a single select's choice of its chosen option fires nothing (was any select's choice).
- Was a `multiple` select not modelled; now modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
- New record, its keys through `press_multiple_select_key`: `ArrowDown`, `ArrowUp`, `Home` and `End` alone choose that row alone, with Shift span from the anchor, with Ctrl or Meta move the current row and change nothing; a Space flips the current row with no typed text standing and joins the text otherwise; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen; Ctrl+A is not modelled.
- New record, its pointer: `click_select_row`, behind an `option` arm of `handle_click`, stops at a disabled select, focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, Shift with no cursor yet spans from the last chosen row, a disabled row changes nothing.
- The click walk gains that arm: a click on a row focuses its select.
- Invalidation: `apply_select_choice` marks a select where a key, typeahead or, on a `multiple` select, a click on a row changed its chosen option or set (was a key or typeahead, its chosen option).
- A `selected` write sets that option chosen and, on a single-choice select only, its other options not, a `multiple` select's others left alone; the box holding its chosen option narrows to a single-choice select; a chosen set is held as its options' `CHECKED` state, HTML's selectedness algorithm settling a single select's.
**Why:** F6, to build it now: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The operator's answer at P2 Validate, 2026-10-31 13:13Z, reads his fork text as naming ctrl and shift, form entries, a value beyond one and `type`'s meaning: the operator's reading, not a new word of his.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — citation digits re-pointed
**Section:** every file of the pass: architecture.md; architecture-detail/conventions.md, cross-cutting-patterns.md, design-philosophy.md, established-decisions.md, existing-scopes.md, inherited-defaults.md, occupied-resources.md, stack-and-technologies.md, standard-contracts.md
**Change:** `file:line` citations re-pointed to the cited code's current lines, the claim beside each unmoved; design-philosophy.md and inherited-defaults.md took no other change.
**Why:** the chunk's added lines moved the code those citations read.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a native control's toggled in the tree
**Section:** §Established Decisions → Scrolling and input (established-decisions.md: dec-scrolling-and-input); §Standard Contracts → Mutation, query and CSSOM (standard-contracts.md: contract-mutation-query-and-cssom; architecture.md); §Standard Contracts → Dioxus DOM bridge (standard-contracts.md: the snapshot's reading of the builder's state writes)
**Change:**
- Was the builder writing no `toggled` from a native checked state, the tree stating no native checkbox or radio checked, a measured absence owned by a `.andromeda/residuals.md` line; now, a new record, `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never reads on it, and neither author attribute is read on one; an engine change for every document.
- The adapter crossing's states gain a native control's `toggled` and a `multiple` select's multiselectable.
- The snapshot's account of a native checkable control: it reads neither `aria-pressed` nor `aria-checked`, and the write above is stated there.
- architecture.md: the mutation line reads a native control's checked state on a checkable role; «Two measured limits» becomes the role limit and one measured limit.
**Why:** F5, to write it on checkable roles: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Kept:** the platform-tree refresh's windowed witness stays owed, a measured limit, the refresh held by the document-level tests and both feature builds of blitz-shell.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the eight engine changes of this chunk
**Section:** §Established Decisions → Scrolling and input (established-decisions.md: dec-scrolling-and-input; architecture.md)
**Change:**
- New record: eight more ratified direct engine changes for every document stand beside the seven: `visible_region` through transforms, a non-invertible transform taking no hit, the focus fixup, the tree's focus falling back to the window, a native control's `toggled` on a checkable role, the `multiple` select, `optgroup`, and the engine's `change` on a checkbox and a radio.
- Each is flagged upstreamable and nothing is sent; upstream `0980cce5f` has no hunk on `visible_region`, and its `hit_inner` hunk is a textual conflict with the non-invertible answer; their merge surface is a line owned by Epoch 3's «Upstream sync ahead of the UI graph».
- architecture.md's tally of direct engine changes gains the eight and that line, `hit_inner`'s conflict named.
**Why:** each is ratified by the founder's own word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); the conflict and the merge surface are as this chunk measured them.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the select limits that stand
**Section:** §Established Decisions → Scrolling and input (established-decisions.md: dec-scrolling-and-input; architecture.md)
**Change:**
- No popup is painted, its owner now named: the route entry «Select popup»; `size` is unread; Enter and Space open nothing on a single select.
- The option-click limit narrows: was an option is not clicked; now an option of a closed single select, under a group too, takes no click and a driver `click` on one stays refused `covered`.
**Why:** F8, the popup to be drawn in a chunk of its own and only a `multiple` select's rows built here, and his P5 word sending `size` to the popup's chunk: the founder's own words, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9, inputs#I12).
**Kept:** «no popup is painted» stays true beside the moved limits.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the focus fixup under inert
**Section:** §Established Decisions → DOM semantics (established-decisions.md: dec-dom-semantics; architecture.md); §Standard Contracts → Events (standard-contracts.md: contract-events; architecture.md)
**Change:**
- Was «A known limit: HTML's focus fixup is not built», a node holding focus when it becomes inert keeping it until focus moves, pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`, owned by a `.andromeda/residuals.md` line (target: next); now, a new record, `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, firing no DOM `blur` or `focusout`, so a subtree moved under an inert box is covered too, `disabled` and `hidden` reading as before, held by `inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key`.
- The inert rule's «no mutator hook is added» narrows: no `inert` arm joins the attribute writes.
- The events contract carves it out: a focus change dispatches `blur`, `focusout`, `focus`, `focusin`, and the inert fixup none.
- architecture.md's limit line becomes the fixup's own, its residual owner gone.
**Why:** F3, to clear focus without `blur`: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — optgroup
**Section:** §Established Decisions → DOM semantics (established-decisions.md: dec-dom-semantics; architecture.md); §Standard Contracts → Mutation, query and CSSOM, Nodes and widgets (standard-contracts.md: contract-mutation-query-and-cssom; architecture.md)
**Change:**
- Was a select's options its `option` children, «an `optgroup`'s are not listed»; now its `option` children and its `optgroup` children's, in tree order.
- New record: `is_select_option` holds under a group; `option_disabled` is the one rule `readings` and `form.rs` share, an option's own attribute or its group's; `maybe_record_node`'s `optgroup` arm records its select; the tree gives a group `Role::Group`, named by its non-empty `label`, which stays unpainted.
- The accessibility builder keeps an option whose parent is a `select` or an `optgroup` under one.
- architecture.md: `disabled` read by its presence on a form control through one reading, an option's through `option_disabled`; the mutation line carries group labels; a select's options include its groups'.
**Why:** F7, to build it now: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The operator's answer at P2 Validate, 2026-10-31 13:13Z, reads his fork text as the list through a group; a group's `label` as a name stands on the P5 lean, not F7's text, as that answer records it.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the default sheet's rows and chosen-row pair
**Section:** §Established Decisions → DOM semantics (established-decisions.md: dec-dom-semantics)
**Change:** New record: the default sheet adds `select:not([multiple]) > optgroup:not([hidden]) { display: contents; }`, `select:not([multiple]) > optgroup > option:checked { display: inline; }`, `select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }` and `select[multiple] > optgroup { display: block; }`, and holds the engine-default chosen-row pair `select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }`, 12.56:1 as `select_interaction` measures it, Chromium's inactive selected pair, hardcoded since the engine holds no tokens.
**Why:** the sheet's half of F6 and F7, the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); the pair is new engine text, its ratio measured.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the open state's and the redraw route's own checks
**Section:** §Established Decisions → DOM semantics, Shell and windowing (established-decisions.md: dec-dom-semantics, dec-shell-and-windowing)
**Change:**
- `:open`: was an upstream engine change with no test of its own; now held by its own `open_state`: the attribute at construction, a write and a removal through the mutator, a `summary`'s click and its Enter.
- The redraw route: was held by the windowed `seven_guis_native` smoke boot and `focus_redraw`; now also by blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read.
**Why:** the chunk's carried gaps close: `open_state` 4 tests and blitz-shell's first unit test.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — change on a checkbox and a radio
**Section:** §Established Decisions → JavaScript (established-decisions.md: dec-javascript); §Standard Contracts → Events (standard-contracts.md: contract-events; architecture.md)
**Change:**
- Was `change` synthesised after `input` on checkbox/radio in blitz-vibey-script's runtime; now a checkbox's or radio's `change` comes from the engine, the runtime synthesising none, so a page's script hears one `change`.
- Was the engine dispatching `change` only on a select's choice; now also, a new record, in `handle_click`'s checkbox and radio arms: `DomEventData::Change` after `input` with the same value, a checkbox always, a radio only where it was not checked before the click; an engine change for every document.
- A text or range input firing no `change` is stated (was a text input).
**Why:** F10, the engine to send it: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The operator's answer at P2 Validate, 2026-10-31 13:13Z, reads his fork text as the engine's `change`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the MountedData hazard of a spawned task
**Section:** §Established Decisions → Dioxus integration (established-decisions.md: dec-dioxus-integration)
**Change:** New record, no engine change: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run.
**Why:** the async form for Cells' handles was tried, panicked and did not ship; the hazard is a measured fact no master held.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the module records of three crates
**Section:** §Existing Scopes (existing-scopes.md: blitz-dom's node module, blitz-shell, dioxus-native-dom)
**Change:**
- blitz-dom's node module list gains `select`, the crate-private rules of the select model, holding `SelectCursor`, `RowGesture` and `Gestured`.
- blitz-shell's `event` module holds the crate's one `#[cfg(test)]` module, one unit test, a redraw request reaching the receiver and waking the loop once.
- dioxus-native-dom's snapshot module also re-exports `closed_select_of` and `is_multiple_select`.
**Why:** the chunk added the rule types and that unit test, and re-exported the two readers for the driver.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the blitz-tests records' counts
**Section:** §Existing Scopes (existing-scopes.md: blitz-tests' stand and engine records, its shared modules)
**Change:**
- `accessibility_states`: was 5 tests; now 6, `toggled` written from the checked state under a checkable role and not under any other, a click and a Space moving it.
- `inert_subtree`: was 4 tests, its focus-fixup limit pinned; now 6, a node coming under `inert` giving focus up with no DOM `blur` or `focusout` and taking no key, the tree's focus reading the window when the focused node was not built.
- `transformed_client_rect`: a target in a translated scrolling box was read outside its visible region; now inside.
- `stand_keyboard`: was 15; now 21, the sixteenth to the twenty-first being the load page's walk with Enter and Space on its button, Cells' walk, Cells' arrows and Enter committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the arrows clamping and bringing a cell into view.
- `stand_act_transformed`: was 2; now 3, a control in a scrolling box under a zoomed world reached where it shows.
- `stand_act_select`: was 5; now 10, a `multiple` select's name, role and rows each with its own bounds, a click on a row, a press with Shift, `type` with no stray click, and an option under a closed select's group refused `covered`.
- `select_interaction`: was 7; now 18, Enter and Space opening nothing, a `multiple` select's rows, keys, clicks and removal, the chosen row's ratios, a seeded set, and `optgroup`.
- `stand_contrast`: was 5; now 7, the load page's and Cells' ratios.
- `stand_cells_served`: was 4; now 5, the keyboard committing a value. `stand_load_page`: was 2; now 3.
- The shared module's tables gain `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`. `HeldNet`'s readers: was 24; now 28, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event` joining.
**Why:** each is this chunk's measured count at its report and its census.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' grid, its keys and its count
**Section:** §Existing Scopes (existing-scopes.md: the per-task count); §Standard Contracts → Dioxus DOM bridge (standard-contracts.md: Cells' author keys, a new record)
**Change:**
- The count of elements that react and state no role: was 676, «react to a click»; now 677, «react to input», its 676 data cells and `cells-grid` with its `onkeydown`.
- Cells' author keys gain `cells-grid`, focusable and carrying a listener, beside the nine.
- New record: `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside an open editor returns early and keeps its typed text; `.cells-grid:focus .cell.current` draws the accent outline, `2px solid #3a5ce5`.
**Why:** F13, one Tab stop: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). 677 is measured and disproved the forecast of 676: the count is every element with the listener mark that reads `GenericContainer`, and `cells-grid` joins the data cells.
**Kept:** the early return inside an open editor, which the keyboard path needs: the operator, by dialog at P2 Validate, 2026-10-31 13:13Z, restated the clause and kept the behaviour; the criterion it failed was the plan's, not the founder's, and F13 A says nothing of it. The 676 data cells stand as a true count.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the five engine-fact targets
**Section:** §Existing Scopes (existing-scopes.md: a new record of blitz-tests' engine-fact targets)
**Change:** New record, each one file and one target, in both layout modes: `visible_region_transformed` 6 tests, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2, each with what its tests read.
**Why:** the chunk added these five targets for F1, F2, F10, F11 and the carried `:open` gap.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the free-disk steps and what they show
**Section:** §Occupied Resources → CI infrastructure (occupied-resources.md: res-ci-infrastructure); §Stack and Technologies → CI/CD (stack-and-technologies.md: the row's list of actions)
**Change:**
- New record: the `Stand checks repeated` job runs `jlumbroso/free-disk-space` at the pinned sha `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`; on CI#41192433140 it reads `completed · success`, `/dev/root` going from 86 G free to 109 G free.
- That record states it is not shown to be the cure of the job's out-of-disk failure (`Free space left: 5 MB`, CI#41190771843): the green runner had 86 GB free before the step, the failed run has no `df` line, and the red job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread.
- New record: a step of the same action stands in the `matrix_test` job, skipped on every matrix row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` while the android row runs `ubuntu-latest`, read `completed · skipped` on `Test (android)` in CI#41190771843; so the new step is this workflow's one measured run of the action.
- The CI/CD row's `uses:` list gains the action.
**Why:** a scope widening on the operator's word, 2026-10-31, after the job's first run went red out of disk: a pinned action in a CI job, no boundary of the product moved. The matrix step's skip corrects the operator pass's first report, which read it from the YAML as running.
**Kept:** architecture.md's §Infrastructure Patterns does not state the step; its fact is this record's.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — pulldown-cmark resolved at 0.13.5
**Section:** §Stack and Technologies → the Markdown app's row (stack-and-technologies.md)
**Change:** Was pulldown-cmark 0.13; now pulldown-cmark 0.13 resolved 0.13.5, past RUSTSEC-2026-0414, `rdme` never setting `ENABLE_WIKILINKS`, so no reader is exposed.
**Why:** the advisory's fix, a semver-compatible move in `Cargo.lock` alone with no coupled pin: the operator's call, not put to the founder (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the public select readers and the row rules
**Section:** §Standard Contracts → Document core (standard-contracts.md: contract-document-core; architecture.md); §Standard Contracts → Nodes and widgets (standard-contracts.md: contract-nodes-and-widgets)
**Change:**
- `select_value` reads a `multiple` select's first chosen option in tree order; `set_select_choice` takes the chosen set.
- New record: `BaseDocument::select_values(&self, select: NodeId) -> Vec<String>` and `BaseDocument::select_of(&self, node: NodeId) -> Option<NodeId>`, public and read-only; public `BaseDocument::clear_select_typeahead(&mut self)`, its one non-test caller `execute.rs` `focus_multiple_select`; `dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`, public and re-exported because escher-driver has no blitz-dom dependency to name `local_name!` with, `is_masked_input` the precedent.
- New record: a write or removal of `multiple` through the mutator records the select (`record_select_of_multiple_write`), so a removal settles its set to one option; `set_option_selectedness` leaves a `multiple` select's other options alone.
- New record: `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, none public; `stepped` takes the row it steps from; `document.rs` holds a `pub(crate)` field `select_cursor`.
- architecture.md's document-core line names the readers and the typeahead's clearing; the choice's writers stay crate-private.
**Why:** F6 and F7, each to build it now: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). No type, verb or field is added by the three public items the plan did not name.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the tree's focus falls back to the window
**Section:** §Standard Contracts → Mutation, query and CSSOM (standard-contracts.md: contract-mutation-query-and-cssom)
**Change:** New record: `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; an engine change for every document.
**Why:** F4, the fallback to the window: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — visible_region through transforms
**Section:** §Standard Contracts → Scrolling, selection, tree (standard-contracts.md: contract-scrolling-selection-tree; architecture.md); §Standard Contracts → Driver session (standard-contracts.md: contract-driver-session, `off-screen`'s meaning and the crate docs)
**Change:**
- New record: `BaseDocument::visible_region` keeps its signature and reads each clipping box through its transforms via the private `padding_box_in_view` over `transformed_bounds`: exact under a translate and a scale; under a rotation the rotated padding box's bounding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; as before with no transform on the chain. Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged; an engine change for every document.
- `transformed_bounds`: was private; now `pub(crate)`, `visible_region` leaving the not-edited list of the client-rect record.
- Was the measured limit that a scrolling box under a transform stands untransformed, a target inside a translated one reading outside its visible region, owned by a `.andromeda/residuals.md` line; now that target reads inside, and the limit that stands is the rotation's.
- `off-screen`'s meaning and the crate docs: was a scrolling box under a transform read where it stands untransformed, so an element inside it can read `off-screen` though it shows; now under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view.
- architecture.md's line reads the same, `a measured limit` now the rotation's.
**Why:** F1, to fix it: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Kept:** `off-screen`'s remedy, the eight causes and the names are unmoved.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a non-invertible transform takes no hit
**Section:** §Standard Contracts → Scrolling, selection, tree (standard-contracts.md: contract-scrolling-selection-tree; architecture.md)
**Change:** New record, a first statement in this master: `Node::hit_inner` answers `None` for the whole subtree under a transform whose determinant is 0 or not finite; an engine change for every document. architecture.md's scrolling line carries it (was the hit stopping at the padding box and skipping a dropped node, no such answer stated).
**Why:** F2, to fix it: the founder's word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — clear_attribute, the method's name
**Section:** §Standard Contracts → Nodes and widgets (standard-contracts.md: contract-nodes-and-widgets, `ElementData::classes`)
**Change:** Was `class` atoms filled by `DocumentMutator::set_attribute`'s `class` arm and by `remove_attribute`; now by `clear_attribute`.
**Why:** `mutator.rs` holds `clear_attribute`; the chunk carried this rename.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — type and press on a multiple select
**Section:** §Standard Contracts → Driver session (standard-contracts.md: contract-driver-session)
**Change:**
- `type`'s help gains that on a `multiple` select the text chooses its match alone.
- Was `type` clicking its target; now every target but a `multiple` select, which it focuses with no click.
- New record: `Target` carries `multiple`; a click would land on the row at the centre, so the crate-private `focus_multiple_select` focuses with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row; no verb, argument, key name, result field, cause, session error or exit of its own.
- New record: `press` carries no Ctrl, so a set of rows that are not adjacent cannot be built through the driver, and its help states it: "a set of rows that are not adjacent needs Ctrl, which this verb does not carry".
**Why:** `type`: F6, the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), its fork text naming `type`'s meaning on the operator's reading at P2 Validate. `press`: the founder's P5 word, a limit and a residual line, relayed verbatim (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the lint's overlap leaves out a closed select's options
**Section:** §Standard Contracts → Driver session (standard-contracts.md: contract-driver-session, the visual lint and the `overlap` reading of a select's options)
**Change:**
- New: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's; `overlaps` skips such an entry on either side; a `multiple` select's rows are linted as usual.
- Was the flight booker reading one `overlap` at boot (`flight-type-one-way` × `flight-type-return`), the other three lean tasks 0, `stand_lint` pinning that as measured, the defect open and its fix owned by a `.andromeda/residuals.md` line; now all four lean tasks read 0, `stand_lint` pinning `at_boot` as `[0, 0, 0, 0]` in both layout modes (was `[0, 1, 0, 0]`).
- A `multiple` select's rows read their own client rects, and the pair of overlapping rows `stand_lint` plants reads exactly `fx-row-a` × `fx-row-b`.
**Why:** F9, to exclude closed options, as restated: the founder's fork, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Kept:** a closed select's options keep their select's client rect in the snapshot.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a multiple select in the bridge and the snapshot
**Section:** §Standard Contracts → Dioxus DOM bridge (standard-contracts.md: contract-dioxus-dom-bridge; architecture.md)
**Change:**
- New record: `multiple_select_values` in `dioxus_document.rs` fills `values` on `Input` and `Change` with one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set, while `value` stays `select_value`.
- New record: a `multiple` select's rows and groups read their own client rects as their `bounds`; its `value` reads its first chosen option in tree order.
- Was an option whose parent is a `select` reading its select's client rect through the private `select_parent`; now an option of a closed select, under an `optgroup` too, through the public `closed_select_of` and `BaseDocument::select_of`; a group's options are its children in the snapshot.
- `differs`: a single select's choice names the new option with `selected`, and a `multiple` select's names each row whose chosen state moved.
- architecture.md's bridge line: `closed_select_of` and `is_multiple_select` as the select tests the driver reads, naming the element and never the role, and the set and rows above.
**Why:** F6 and F7, each to build it now: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); `select_parent` retired for `select_of`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — stand-repeat's named targets
**Section:** §Standard Contracts → CI contracts (standard-contracts.md: contract-ci-contracts, `stand-repeat`)
**Change:** Named blitz-tests targets: was ten; now fifteen. `stand_requirement_sweep`'s `STAND_REPEAT_NAMED` still holds the first ten, stated beside the leg check.
**Why:** `ci-leg.sh`'s `stand-repeat` names the chunk's five new targets, and `test_stand_repeat_leg_runs_three_rounds` names them too.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the legs that do not run locked
**Section:** §Standard Contracts → CI contracts (standard-contracts.md: contract-ci-contracts); §Infrastructure Patterns → CI/CD (architecture.md)
**Change:** Was every cargo leg `--locked` but `examples/wasm_hello`; now every cargo leg but `fmt`, `cargo fmt --all --check`, and `examples/wasm_hello`.
**Why:** an older reading this pass's sweep found false against `ci-leg.sh`, whose `fmt` leg runs with no `--locked`; not a change of this chunk.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
