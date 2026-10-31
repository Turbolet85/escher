## 2026-10-31-engine-debts — the citations the chunk's edits moved
**Section:** §API Security (api-security.md); §Error Handling (error-handling.md, security-plan.md); §Input Validation (input-validation.md); §Secret Management (secret-management.md, security-plan.md); §Threat Model Summary (threat-model-summary.md); §Data Protection, §Dependency Security, §Logging & Monitoring (security-plan.md)
**Change:**
- Citations re-anchored to the line ranges they now stand at, in rows whose claims read unchanged and beside the rewritten rows the entries below name.
- The master, security-plan.md, moved by citations alone: no sentence of it changed.
**Why:** the chunk's edits to blitz-dom, dioxus-native-dom, escher-driver, blitz-vibey-script's tests, blitz-tests, the Cells task, `ci.yml`, `ci-leg.sh` and `test_ci_workflows.py` moved the cited lines.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — RUSTSEC-2026-0414, the pulldown-cmark lock update
**Section:** §Dependency Security (dependency-security.md: the audit tool's record)
**Change:**
- A new record: RUSTSEC-2026-0414 is fixed by the lockfile update of pulldown-cmark (was 0.13.4; now 0.13.5), in `Cargo.lock` alone, semver-compatible with no coupled pin and `deny.toml` unmoved.
- It was reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed.
- The audit leg reads `advisories ok` and `licenses ok`.
**Why:** the report's dependency fact, no standing text naming the advisory; the report records the update as the operator's call, not put to the founder.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the free-disk action at its two CI sites
**Section:** §Dependency Security › Supply chain integrity (dependency-security.md: two new CI action rows)
**Change:**
- Job `Stand checks repeated` runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false` and `swap-storage: false`: a CI action site and not a crate.
- It reads `completed · success` on the chunk's green run, taking `/dev/root` from 86 G free to 109 G free on that runner.
- It is not shown to be the cure for the runner out of disk (`Free space left: 5 MB`) on the red run: the green runner had 86 GB free before the step, the failed run has no `df` line, and the failed job ran on an older image than every other `ubuntu-24.04` job, that image's free disk unread.
- The same action's step on the `matrix_test` job is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest`.
**Why:** the first CI run went red on `Stand checks repeated`; the step is a scope widening on the operator's word (the operator, 2026-10-31), and the answer card the operator confirmed at P2 Validate reads that no boundary of the product moves. The report reads the skipped step from the job's state, correcting a claim read from the YAML alone.
**Kept:** the step stays written though not shown to be the remedy; the report lists it among its insufficient fixes.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `visible_region` through transforms, and `off-screen`'s meaning
**Section:** §Error Handling (error-handling.md: `off-screen`'s text and its as-measured list); §Input Validation (input-validation.md: a new Layout values row)
**Change:**
- `off-screen`'s meaning: was a scrolling box under a transform read where it stands untransformed, so an element inside it can read `off-screen` though it shows; now under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view.
- The as-measured list: was the transformed-box sentence at the 0.2.0 engine-debts report; now the rotation sentence at this chunk's report, `covered`'s no-area sentence staying at 0.2.0.
- A new row: on every document, remote pages included, `visible_region` reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads the padding box in the client rect's frame; a mapped box with a non-finite edge answers `None`; exact under a translate and a scale, under a rotation it holds corners the box does not show.
- Its callers are `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`; the added lines hold no log site, print, env read or panic path; pinned by `visible_region_transformed` 6, `stand_act_transformed` 3 and `transformed_client_rect`.
**Why:** fork F1, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator); the answer card the operator confirmed at P2 Validate reads it as an engine change on his word, nothing new crossing.
**Kept:** `off-screen`'s remedy, the cause set (eight) and the cause names, unchanged.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `type` on a `multiple` select, and the public select readers
**Section:** §Error Handling (error-handling.md: `type`'s help); §Input Validation (input-validation.md: two new rows)
**Change:**
- `type`'s help also covers a `multiple` select: its match alone, a text opening on a space typing nothing; still fixed text.
- A new row: `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing, since no label can match and the Space would flip a row; `Target` holds `multiple`, and no verb, argument, key name, result field, cause, session error or exit is added.
- A new row, the readers `BaseDocument::select_of` · `select_values` · `clear_select_typeahead` and `dioxus_native_dom::closed_select_of` · `is_multiple_select`: each reader taking a node goes through `get_node`, a stale id reading none, and `clear_select_typeahead` takes no node; their callers are named there, and no type, verb or field is added.
**Why:** fork F6, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator). The `type` row as a boundary widening is the operator's confirm at P2 Validate (by dialog, 2026-10-31), recorded as his reading of the founder's fork text and not a new word of the founder's. The report states the readers beyond the plan's letter: without `clear_select_typeahead` a second `type` joins the first's text.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `MountedData` panic from a spawned task
**Section:** §Error Handling (error-handling.md: a new record)
**Change:**
- A new record: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed`, any app able to meet it.
- Measured headlessly through the harness's `poll(None)`; the windowed shell runs the same lines, read and not run.
- Cells calls its handles synchronously, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call.
**Why:** the report measures the hazard, outside the chunk's fixes, after the planned async form panicked and was not used; no standing text stated it.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `open` row's test gap closed
**Section:** §Input Validation (input-validation.md: the `open` row)
**Change:**
- Was no test asserts `:open`, while `details_element` and the a11y leg read green; now `open_state`'s 4 tests assert `:open` in both layout modes: read from the attribute at construction, moved by a write and a removal through the mutator, and flipped by a summary's click and its Enter.
**Why:** the chunk's new `open_state` target, the report's reading of that gap as closed.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a native checkable's checked state in the tree
**Section:** §Input Validation (input-validation.md: the `id` row's control-states class, the control-states row, a new row)
**Change:**
- The control-states row: was a native checkbox's or radio's checked state reaches no tree node, a measured absence with its fix owned by a `.andromeda/residuals.md` line; now it reaches the tree node under a checkable role and under no other.
- The same row: was nothing else writes `toggled`; now nothing else but a native checkable's own state, under a checkable role.
- The `id` row: was on each element node whose markup states them; now whose markup or native checked state states them, `toggled` on a native checkable coming from `checkbox_input_checked()` on a checkable role and nothing under any other.
- A new row: in every document, remote pages included, `build_accessibility_node` writes `Toggled::True` or `False` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control; a closed value carrying no attribute text; pinned by `accessibility_states` 6, what an assistive technology reads unmeasured.
**Why:** fork F5, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator); the answer card the operator confirmed at P2 Validate reads the write as a closed value inside the ratified adapter crossing.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the focus an inert subtree loses, and the tree's `focus`
**Section:** §Input Validation (input-validation.md: the `inert` row, a new row)
**Change:**
- The `inert` row: was a known limit, HTML's focus fixup not built, so a node holding focus when it becomes inert keeps it until focus moves, the fix owned by a `.andromeda/residuals.md` line; now a node that holds focus when it comes under `inert` gives it up: `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, calls `BaseDocument::clear_focus`.
- No `inert` arm joins the attribute writes, so a subtree moved under an inert box is covered too; `disabled` and `hidden` unmoved.
- A new row: in every document, remote pages included, `build_accessibility_tree` writes the tree's `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; the inert focus clear fires no DOM `blur` or `focusout`; closed values, the added lines holding no log site; pinned by `inert_subtree` 6, what an assistive technology reads unmeasured.
**Why:** forks F3 and F4, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator); the answer card the operator confirmed at P2 Validate reads the focus fixup as an engine change on his word, nothing new crossing, and the window fallback as closed values inside the ratified adapter crossing.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the engine's `change` on a checkbox and a radio
**Section:** §Input Validation (input-validation.md: the keyboard-activation row, a new row)
**Change:**
- A new row: in every document, remote pages included, `handle_click`'s checkbox and radio arms send, after `input`, a `DomEventData::Change` with the same value, the value class `input` carries: a checkbox always, a radio only where it was not checked before the click.
- `blitz-vibey-script`'s runtime synthesizes no `change`, so a page's script hears one `change`, from the engine.
- No carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added; pinned by `checkable_change_event` 3, `text_range_no_change_event` 2 and `radio_click_fires_change_only_where_it_was_not_checked`.
- The keyboard-activation row names these arms beside `handle_click`'s other clicks.
**Why:** fork F10, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator). Its reading as a boundary widening is the operator's confirm at P2 Validate (by dialog, 2026-10-31), recorded as his reading of the founder's fork text and not a new word of the founder's.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `select` row's rules, limits and pins
**Section:** §Input Validation (input-validation.md: the `select` row)
**Change:**
- Settling: a `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone.
- The option list: was the `option` children, an `optgroup`'s not listed; now the select's `option` children and those of its `optgroup` children, in tree order.
- The driver's `type`: was the same typeahead on a select; now on a single select, a `multiple` select's in its own row.
- Stated limits: was a `multiple` select and an `optgroup`'s options not modelled, no popup painted, an option not clicked (refused `covered`); now no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read.
- Pins, as measured at this chunk's report: the rules module's unit tests were 4, now 8; `select_interaction` were 7, now 18; `stand_act_select` were 5, now 10; both layout modes.
**Why:** forks F6 and F7 end the two limits, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator); the answer card the operator confirmed at P2 Validate groups these restatements with the rows of the entries below.
**Kept:** no popup and the closed option's `covered` refusal, on fork F8, the same dialog; `size` left to the popup's chunk, on the founder's word at the P5 review (2026-10-31, relayed verbatim by the operator).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a `multiple` select's keys, clicks and chosen set
**Section:** §Input Validation (input-validation.md: two new rows, the keyboard-activation row)
**Change:**
- A new row, keys, in every document, remote pages included: a closed key match, `press_multiple_select_key`: ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift span from the anchor (with no cursor yet, from the last chosen row), with Ctrl or Meta move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone.
- The same row, clicks: `click_select_row` behind `handle_click`'s `option` arm: a disabled select stops it, it focuses the select and clears the typeahead and text selection, plain chooses the row alone, Ctrl or Meta flips it, Shift spans, a disabled row changes nothing.
- Disabled rows are never chosen and every choice passes `apply_select_choice`, the one write path, taking a set; no log site, print, env read or panic path, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added.
- A new row, the set's exits: form entries; `input` and `change` values (`multiple_select_values` fills `values` with one `(name, FormValue::Text(value))` per chosen option, `value` staying `select_value`); the snapshot's `value`, `select_value`'s first chosen option in tree order; the options' tree nodes, the select taking `set_multiselectable`.
- The keyboard-activation row names the `option` arm.
**Why:** fork F6, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator). Its reading as an engine change for every document and a boundary widening is the operator's confirm at P2 Validate (by dialog, 2026-10-31), recorded as his reading of the founder's fork text and not a new word of the founder's.
**Kept:** Ctrl+A is not modelled.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `optgroup` and its `label`
**Section:** §Input Validation (input-validation.md: a new row)
**Change:**
- A new row, in every document, remote pages included: read by a closed element-name match; `options` walks a select's `option` children and its `optgroup` children's, in tree order, `is_select_option` holding under a group.
- `owner_select` and `option_disabled`, the one rule `readings` and form submission share; `maybe_record_node`'s `optgroup` arm records its select.
- A group takes `Role::Group` and its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing; pinned by `select_interaction`, `stand_act_select` and `form`'s tests.
**Why:** the list through a group: fork F7, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator), read as a boundary widening by the operator's confirm at P2 Validate (by dialog, 2026-10-31), recorded as his reading of the founder's fork text and not a new word of the founder's. A group's `label` as a name stands on the operator's lean at P5, not on F7's text, as the operator's note at that confirm records.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a non-invertible transform takes no hit
**Section:** §Input Validation (input-validation.md: a new Paint values row)
**Change:**
- A new row: on every document, remote pages included, `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, so it takes no hit; the added lines hold no log site, print, env read or panic path; pinned by `hit_non_invertible_transform` 2.
**Why:** fork F2, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator); the answer card the operator confirmed at P2 Validate reads it as an engine change on his word, nothing new crossing.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' `cells-grid` keys
**Section:** §Input Validation (input-validation.md: a new Example apps row)
**Change:**
- A new row: `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match and no role: arrows move `current`, clamped, Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid.
- A click inside a cell's own open editor returns early.
**Why:** the grid: fork F13, on the founder's word (the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator). The early return: the report's deviation, needed for a keyboard-opened editor to keep its return to the grid, stands on the operator's answer at P2 Validate (by dialog, 2026-10-31), which restates the plan's criterion that a pointer-opened editor reads as it did: that criterion was the plan's, and F13's text says nothing of it.
**Kept:** the early return, though one pointer path moves.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
