@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job ("Accessibility (a11y) tests", `bash .github/scripts/ci-leg.sh a11y`) in the slow tier with no required status check; this is the leg the chunk's exit names ("a11y leg green") and the origin of decision/a11y-ci-leg and decision/a11y-ci-gate.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream merge's precedent: 16 `file:line` citations into merged upstream files re-pointed by the merge's line map with no claim text changed, and the a11y leg's files left unnarrowed; this take's moved upstream lines owe the same citation sweep at the wrap.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the shell to build and refresh the tree through `Document::accessibility_tree` so DioxusDocument's override (stable ids as `author_id`) reaches the platform tree, plus the `aria-label` / `<label>` name sources and the Dioxus feature wiring (`default-features = false`); upstream's AccessKit refresh (`be60597f6`) and the Dioxus 0.8 shape must keep decision/accessibility-tree-through-document-method, decision/a11y-tree-through-document-trait and contract/accessibility-tree/author-id.

@history_item n=4 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot take role, name and focus from the tree `accessibility_tree` returns, with no second mapping (decision/snapshot-reads-accessibility-tree); a take that edits the tree builder moves every snapshot reading, so stand snapshot checks decide.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy Dioxus boolean attribute removed on both attribute paths so `hidden: false` stays in the tree; snapshot reads `enabled` (presence of `disabled`), `checked`, `value` and the password mask from the element; precursor of decision/falsy-boolean-attributes-removed and contract/snapshot/enabled, which a bridge-touching take must keep.

@history_item n=6 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful (mutation, focus, checked, text input; not creation, hover, active, scroll, layout) and the shell rebuild the tree on a poll that took a non-empty set; this is the changed-set contract the scope says every resolution keeps, and the AccessKit refresh or mutation changes of the take must be read against it (decision/a11y-tree-refresh-on-change, decision/changed-set-marks).

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed set as one item, and the bridge's falsy clear; a take that moves either reopens a ratified contract, so a change is a fork, not a silent resolution.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set; trap for any upstream harness/settle change the take brings (decision/harness-settle-leaves-changed-set, guard `stand_settle`).

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the obs sink described in §3 prints only safe fields for `blitz*` and `accesskit_xplat` targets and drops other targets whole; the scope's sink-allowlist re-check of new `tracing`/`log` call sites in the taken delta (the AccessKit refresh included) must keep that note true.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, reaching both seven_guis binaries, while neither builds a platform adapter (`accesskit_xplat`/`accesskit_winit` absent); origin of decision/driver-requires-accessibility and decision/stand-builds-no-platform-adapter, which the take's lock/manifest moves must be re-read against.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — sync precedent: upstream's `autofocus` reading change recorded as a body change, and the stand's adapter absence re-read on the merged lock where the accesskit family moved (accesskit 0.25.1, accesskit_unix 0.24.0); a take moving accesskit (e.g. `be60597f6`) owes the same re-read.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the builder's lower-case closed-set reads of `aria-selected`/`aria-pressed`/`aria-invalid` on every element, ratified, one parser per attribute; "the accessibility state reads" the scope lists as a surface every resolution keeps (decision/aria-state-lower-case-closed-values, decision/aria-states-on-every-element).

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the password mask as an engine change for every document (bullets in blitz-paint, `MASKED_VALUE` in the snapshot), ratified; the scope lists the password mask among engine surfaces escher changed, so a paint/text-input take must keep decision/password-never-revealed.

@history_item n=14 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — sync precedent: the bridge took upstream's private 28-name `is_bool_attr`, `mutation_writer.rs` byte-identical to upstream's, a falsy `inert` removed; a Dioxus 0.8 or bridge take must keep decision/falsy-boolean-attributes-removed and that byte-identity or record why not.

@history_item n=15 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x from computed styles, 4.5 / 3.0 large) and `stand_contrast`; a Stylo/Parley take can move computed colours or font metrics, and a moved number is re-measured, never loosened (decision/stand-contrast-measured, decision/text-contrast-required-ratio).

@history_item n=16 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input's keys, click focus and pointerdown value, ratified for every document; the scope names the range model as an engine surface a take must keep (contract/range-input/*, decision/range-input-keys).

@history_item n=17 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control, and a click focuses a button) — keyboard activation and click focus as engine changes for every document; the scope lists keyboard activation among surfaces a take keeps (decision/keyboard-activation-enter-space, decision/click-focuses-buttons).

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — `disabled` keyed one way by presence on the founder's word, CRUD list keys, `stand_keyboard`; upstream code reading `disabled` by value would conflict with decision/disabled-by-presence.

@history_item n=19 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand, three contrast tables) — the accent darkened to `#3a5ce5` and the three contrast tables pinned, the a11y leg widened to six files on the founder's F4; tables are re-measured if the take moves styles or text metrics (decision/stand-accent-colour, decision/stand-contrast-asserted).

@history_item n=20 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed) — the stand's platform adapter and its windowed AT witness re-homed to a residual; the background of decision/stand-builds-no-platform-adapter that a take bringing adapter changes must not silently alter.

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the sync precedent for the tree builder: upstream's six reads (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`) taken as written while escher's closed-set state reads and trimmed `aria-label` were kept, one read per attribute; the template for resolving `be60597f6` and the empty-alt read (decision/one-read-per-aria-attribute, contract/aria-property-reads/*).

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role and the snapshot's sources) — `aria-checked` on the five checkable roles, `aria-pressed` elsewhere, `role="switch"` → `Switch`; a builder conflict must keep decision/toggle-read-by-role and contract/aria-role-mapping/role-switch.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — `inert` takes focusability, hits and the tree away, `disabled` read by presence on form controls alone; the scope names `inert` and hit stops as engine surfaces a take must keep (decision/inert-subtree-excluded, decision/focusable-elements).

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click read on the document's clock; the "clock invariant" the scope lists among conflict resolutions to keep (decision/double-click-on-document-clock).

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — the toggle per role, `inert`, `disabled` narrowing, activation gate and double-click clock are ratified; changing them in a take is a fork for the founder.

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — sync precedent: upstream's `:focus-within` taken for every document, `:focus-visible` still never matching; a take touching focus-state selectors must keep decision/focus-visible-never-matches or record the change.

@history_item n=27 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model: changed choice marks the changed set, an option's `selected` its chosen state with no `aria-selected`, options kept under `display: none`; "the select model" is a surface the scope says every resolution keeps (contract/accessibility-tree-states/select-option, decision/hidden-content-excluded).

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — sync precedent: upstream's dropped-node hit skip taken on the founder's «Десять коммитов», and the stand graph re-read at the take; the cherry-pick precedent the scope cites and the per-take adapter re-read.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — named a11y witnesses outside the `stand_` prefix run three rounds in `stand-repeat`, held by `stand_requirement_sweep`; upstream's new test files must fit this fork's blitz-tests target rule without breaking those lists (decision/stand-checks-repeated-three-rounds).

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files on the founder's «Плюс проверки дерева»; the leg's membership that must stay green after the take (decision/a11y-test-leg).

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the tier-0 reach, edited by hand since its pin reads none of its claims; if the take moves a named limit or the reach, the section is edited by hand (decision/capability-statement-reach-by-hand, decision/accessibility-reach-tier-0).

@history_item n=32 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat list's membership held jointly by the sweep check and `test_stand_repeat_leg_runs_three_rounds`; trap when the take edits `ci-leg.sh` or adds test targets.

@history_item n=33 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read on the take at `ebacd68bd`) — manifests, lock and `deny.toml` unchanged by that take and the adapter probe `^accesskit_(xplat|winit) ` 0 rows; the re-read form this take repeats on the new lock.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into code the take moved, re-pointed) — citations into blitz-shell `window.rs`, blitz-dom `node/element.rs` / `mutator.rs`, blitz-vibey-script re-pointed after upstream split modules; the same sweep is owed after this take.

@history_item n=35 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`:open` matches an open `details` or `dialog`) — upstream engine behaviour recorded as a new body bullet on the founder's word on the take; precedent for recording upstream behaviour changes the take brings.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (CSS properties as `CSSStyleDeclaration` accessors) — upstream script change recorded so `element.style` reaches the hidden-content exclusion (decision/script-style-through-inline-attribute); a script-engine take is read the same way.

@history_item n=37 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (IME enabled at a focused input's editor construction) — upstream engine change recorded on the founder's word (decision/ime-enabled-for-prefocused-input); precedent for upstream-sourced rows.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move) — a lock-only move still owes the adapter-absence re-read; this take's lock moves owe it too.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkbox's or radio's checked state in the tree) — native toggled written from `checkbox_input_checked()` on checkable roles, neither author attribute read; a builder conflict must keep decision/native-checkable-toggle and contract/accessibility-tree-states/native-checkbox-or-radio.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values) — `set_multiselectable`, the multiple select's value and `evt.values()`; part of the select model a take must keep (contract/accessibility-tree/multiple-select, contract/form-event-values/multiple-select).

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — `optgroup` → `Role::Group`, `is_select_option` for the `display: none` exception, `option_disabled` one rule; select-model rows a take must keep (contract/accessibility-tree/optgroup, decision/disabled-optgroup-one-rule).

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg's per-file tally method (14 result lines, passed/failed/ignored) re-read on CI; the scope's per-OS tallies and a11y leg re-read follow it.

@history_item n=43 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup built) — `DocumentMutator::flush` clears focus on an inert node with no `blur`; a mutator take must keep decision/focus-fixup-reads-inert-alone.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio) — the engine dispatches one `change` per checkable click, the script runtime none; a script or click-handling take must keep decision/one-engine-change-per-checkable-click.

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks) — `click_select_row` behind `handle_click`'s `option` arm and the multiple select's keys (contract/multiple-select/keys-and-row-clicks); conflict surface in `handle_click`.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` hazard in a spawned task) — the panic that led to the buffered render; background for the buffered render's merge surface (`cry-246ouqff`) and Dioxus 0.8's `mutation_buffer.rs`.

@history_item n=47 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes `focus` only where the focused node was built, else `u64::MAX`; the AccessKit refresh must keep contract/accessibility-tree/focus and contract/accessibility-tree/root.

@history_item n=48 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `Node::hit_inner` answers `None` for a non-invertible transform; the carried textual conflict with `0980cce5f`'s `hit_inner` hunk and `563d3db73`'s transform resolution meet this hit stop.

@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and chosen-row pair) — `#000000` on `#c8c8c8` hardcoded in `assets/default.css`; a take editing the default sheet must keep decision/multiple-select-chosen-row-pair.

@history_item n=50 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — the engine overlay `select_list` answered first by `hit_with_scrollbar` and painted after the root on every document; a hit or paint take must keep decision/single-select-engine-overlay-list.

@history_item n=51 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a scroll or move that closes a list marks its select; the qualifier `cry-k5oclmzx` lands at this chunk's wrap as an amendment, and decision/changed-set-marks already states the list opening or closing.

@history_item n=52 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — `aria-expanded` read on every element but a single select, which writes `expanded` from engine state; upstream's builder reads (`be60597f6`) must keep contract/aria-property-reads/aria-expanded and decision/single-select-expanded-from-list.

@history_item n=53 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — Enter / Space / click open the list; keyboard-activation conflicts must keep contract/select-closed-list/enter-space.

@history_item n=54 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — `press_open_list_key`, `apply_select_choice` one write path, Tab chooses then moves focus, page scroll closes; rows decision/select-list-choice-one-write-path, decision/tab-chooses-and-closes-select-list.

@history_item n=55 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — focus stays on the select, a label's click opens nothing; decision/focus-stays-on-select-with-list-open and decision/label-click-focuses-select-only, in `handle_click` / pointer code a take may touch.

@history_item n=56 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — the published limits are edited by hand at a chunk's report; any limit the take moves is restated there.

@history_item n=57 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the standing tally 14 files · 114 tests on CI#41235503227, stand-repeat seventeen words; the baseline the take's a11y leg is re-read against.

@history_item n=58 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup`, `select_popup_painted`, `stand_act_popup` outside the a11y leg, list coverage headless (decision/select-list-coverage-headless).

@history_item n=59 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — no `expanded` node key, highlighted row in no tree, open row bounds from `select_list_bounds`; contract/snapshot/select-list-rows and absence/snapshot-list-open-state.

@history_item n=60 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's colours and its measured pair) — engine constants on `SelectListView`, measured by `select_popup_painted`; a paint take (vello 0.12, AnyRender) can move painted pixels, re-measured not loosened (decision/select-list-colours-engine-constants).

@history_item n=61 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — `Entry.in_select_box` and the highlighted-row contrast limit (decision/highlighted-row-contrast-unread).

@history_item n=62 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — the list is no `:open` state and paints whole past clips and `opacity`; a paint/selector take must keep decision/open-select-list-not-open-state.

@history_item n=63 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed after the take) — the last take's citation sweep across seven detail files and thirteen cited files; the same is owed after this take.

@history_item n=64 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — the last take's adapter re-read at `e42ee1f25`; this take reads it again at `563d3db73`, beside, not replacing, the earlier re-reads (decision/stand-builds-no-platform-adapter).

@history_item n=65 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden) — the `embolden` leg the scope lists among the proofs that still pass after the take.

@history_item n=66 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves) — the fixup reads `is_inert()` alone, a focused control that no longer shows keeps focus, owned by a CARRY on «A11y re-read»; decision/focus-fixup-reads-inert-alone, tied to the `deallocate_anonymous_block` merge surface.

@history_item n=67 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a key press past a dropped anonymous block) — escher's `deallocate_anonymous_block` fix and `key_press_after_container_lost_its_box`; `563d3db73` ("Resolve transforms through anonymous block boxes") is read against it per `cry-lurql7hp`.

@history_item n=68 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the contrast reader reads `None` past `mix-blend-mode`; upstream's mask paint (`a946a0a7b`) or text-shadow may need the same question of decision/contrast-unread-past-non-colour-paint.

@history_item n=69 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending paints with no frame test) — names the blend frame-test absence's owner as a CARRY on «Upstream sync ahead of determinism and exploration»: this chunk's freight `cry-jbkftesv`, two headless frame tests in both layout modes; the open list paints last past blend layers.

@history_item n=70 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — lock-only move re-read with Dioxus at `"0.7.3"`, and the adapter owner moved to residual `res-llkgqygv`; a Dioxus 0.8 shape changes that requirement and owes the re-read.

@history_item n=71 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — the bridge's listener attributes over-report; the vdom walk's `dynamic_attrs` (dioxus-core 0.7.10 public API) reads exactly; a Dioxus 0.8 move must keep decision/listeners-from-vdom-walk.

@history_item n=72 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the spawned-task MountedData hazard record, closed by the buffered render) — `initial_build` and `poll` render into `buffered_mutations` then replay once; the merge surface `cry-246ouqff` the take re-reads, which upstream's 0.8 `mutation_buffer.rs` replaces whole (decision/buffered-dioxus-render).

@history_item n=73 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's same-render limit) — a task polled during render acts on the previous render's document, owned by an open residual; re-read if the Dioxus 0.8 shape is taken.

@history_item n=74 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said to their rows' status) — the stand's platform adapter and windowed AT witness owned by `res-llkgqygv`, re-carried at 0.3.0 (target: next); a take bringing adapter or AccessKit changes must not move decision/stand-builds-no-platform-adapter without the operator.

@history_item n=75 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` from the vdom walk; part of dioxus-native-dom, escher's most-modified crate, which `stand_graph` and the snapshot checks decide after the take (contract/snapshot-node/clickable).

@history_item n=76 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph) — the graph behind `accessibility`, `signal_registry` ungated, no served id holding `///`; rows decision/agent-readers-behind-accessibility and contract/node-id/reserved-separator a bridge take must keep.

@history_item n=77 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the bridge dispatch's write record) — the registry's dispatch set around `runtime.handle_event` in `DioxusEventHandler::handle_event`; a merge surface for any Dioxus bridge take (contract/ui-graph-state/no-registered-state).

@history_item n=78 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and the check's listener leg) — `Readings` the one crate-private actionable rule, `data-dioxus-id` mark for dispatch alone (decision/one-actionable-rule).

@history_item n=79 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` behind `accessibility`; part of the gated set in decision/agent-readers-behind-accessibility that a feature-wiring change in the take must keep.

@history_item n=80 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record on handle_event's path) — the reached record written in `handle_event` before `runtime.handle_event`, ungated because the wasm build compiles dioxus-native-dom without `accessibility`; contract/dioxus-document/reached-handlers and record-reached, a bridge merge surface.

@history_item n=81 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into the rows this chunk is bound by, each verified against the code at `edd0cfaea7f0`; any row the take's resolutions move is amended at the wrap against that baseline.
