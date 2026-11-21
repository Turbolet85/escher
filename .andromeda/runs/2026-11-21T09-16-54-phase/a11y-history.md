@history_item n=1 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — `~ decision/focus-fixup-reads-inert-alone` gained "while the tree's focus names the window, a named SC 2.4.3 gap" (pinned in both modes), and `~ decision/removed-focused-node-drops-focus` corrected to "drops focus to the root element, the tree's focus naming the window". Item 1's fork changes exactly this rule. A fix retires the named gap. Keeping the limit leaves it standing. Either way, dropping focus from a control that no longer shows should match the removal path's reading (the root element, the tree naming the window).

@history_item n=2 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves) — the founder's F1 A «Убрать падение, фокус остаётся» ended the panic and left `DocumentMutator::flush` unedited. A focused control hidden by `hidden`, `style="display:none"` or CSS alone keeps focus and takes keys, and a select's choice still moves past its dropped anonymous block. The limit was owned by residual `res-6k56erh6`, by `escher-0.4.0-incubator/deferred-from-0.3.0.md` and by a CARRY on «A11y re-read». This is the origin of item 1. F1 A is "focus stays" only in the operator's reading of the fork text, not a new word of the founder's. Option B (focus given up after the next layout) was never built.

@history_item n=3 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a key press past a dropped anonymous block) — `BaseDocument::deallocate_anonymous_block` clears `layout_parent` on children still naming the block. `key_press_after_container_lost_its_box` has 3 tests in both modes: a button, a select's choice, and a control in no anonymous block. It is in neither the `a11y` nor the `stand-repeat` leg. This check is the guard of `decision/focus-fixup-reads-inert-alone`, and its rows would flip if item 1's fixup is built.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup built) — the founder's F3: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` reads `is_inert()`, with no `inert` arm joining the attribute writes. No DOM `blur`/`focusout` fires, and `disabled` and `hidden` take no part. `inert_subtree` went from 4 to 6 tests. This is the precedent and the exact site item 1 would extend. Widening the fixup to `display:none`/`hidden` changes a founder-chosen rule for every document, so it needs his ratification, and the no-blur choice must be re-decided for the new trigger.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — the founder's F4: `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`. `inert_subtree` asserts it. This is `contract/accessibility-tree/focus`. It is why a hidden focused control today reads window focus in the tree while the engine still holds focus: the split item 1 would close.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the capability statement's named limits) — `docs/capabilities.md` `## Accessibility reach` names "the focus fixup firing no `blur`" among its limits. Item 1's decision, fix or keep, must update that published limit by hand.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — the named limits were re-measured to the focus fixup firing no `blur`, a single select's `size` unread, and `aria-activedescendant` unread. The statement's pin reads none of these claims, so any change to the focus-fixup limit is a hand edit there.

@history_item n=8 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes tier 0 (SC 2.1.1 · 1.4.3 · 2.4.3), the `a11y` leg's files and the named limits, and "a change to the reach is made in that document by hand". Item 1 touches SC 2.4.3, the reach that document states.

@history_item n=9 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand`: the pin now reads the a11y leg's file count, and nothing else in the section. If item 1 adds a file to the `a11y` leg, that pin moves. A changed limit sentence is still edited by hand.

@history_item n=10 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — focusability reads `Node::is_inert` (presence, self or ancestor) and `ElementData::is_disabled` on form controls alone. The focus-fixup limit was first stated here, owned by a residual. A truthy `inert` takes focusability, hits and the tree away. These are `decision/focusable-elements`, `decision/inert-subtree-excluded` and `decision/inert-element-takes-no-hit`, all guarded by `inert_subtree`/`stand_keyboard`. A hidden-control fixup must not disturb them.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — the founder ratified the 12 engine-debts marks, including `disabled` narrowed to form controls, the `inert` read in focus order, `inert` taking focusability, hits and the tree, and the activation keys' disabled gate. This is the ratified ground item 1 rests on. Changing any of these needs his word again.

@history_item n=12 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed to residuals) — the `inert` gap was re-homed to a residuals line on the founder's F1–F3 «перенести». This is the precedent for keeping an engine a11y gap as a residual on his word, one of item 1's options.

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — disabled-ness is keyed by presence on the founder's «По наличию». `inert` is the 28th falsy name the check reads, and "the engine honours no `inert`" was recorded as a gap. `stand_keyboard` was added. This is the harness item 1's SC 2.4.3 re-assertion runs on (both layout modes, the shared `tab_order`).

@history_item n=14 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control, and a click focuses a button) — Enter on press and Space on release dispatch a Click on enabled buttons, summaries, submit/reset/button inputs and links. A click focuses a button. These are engine changes for every document, ratified by the founder. Under item 1's limit, a hidden focused control still takes these keys: they are the "takes keys" half of the debt (`contract/keyboard-activation/enter`, `/space`, `decision/keyboard-activation-enter-space`, `decision/click-focuses-buttons`).

@history_item n=15 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy value of the bridge's boolean attributes is removed on both attribute paths, so `hidden: false` stays laid out and in the tree. The first Tab/Shift+Tab focus check was added. A fixup keyed on `hidden` must read the attribute's presence after this clear (`decision/falsy-boolean-attribute-removed`).

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the bridge's falsy predicate became upstream's private 28-name `is_bool_attr`, and `mutation_writer.rs` is byte-identical to upstream's. A framework-written `inert: false` is removed. Trap: a fix in `mutator.rs` or the bridge diverges from upstream, so a later sync must carry it.

@history_item n=17 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files, `inert_subtree` and `stand_keyboard` among them. Membership is the founder's «Плюс проверки дерева». Item 1's checks beside `inert_subtree` land in that leg's tally, which the capability pin and `ci-integration.md` record.

@history_item n=18 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `inert_subtree` and three other non-`stand_` witnesses run three rounds in `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`'s leg check. A new focus-fixup check outside the `stand_` prefix joins neither leg unless named there, as `key_press_after_container_lost_its_box` did not.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg was 14 files and 112 tests, and `inert_subtree` asserts the focus fixup and the tree's window fallback through `&dyn Document`. This is the counting precedent an item-1 fix updates.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — a disabled, inert, hidden or empty select opens nothing, and "a select made inert, disabled, hidden or removed while open closes its list". This is `contract/select-closed-list/enter-space`. It is an existing engine reaction to a control becoming hidden, and a precedent for item 1's "no longer renders" trigger.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — focus stays on the select while its list is open, and the Tab walk holds open and after each close (`decision/focus-stays-on-select-with-list-open`). A focus fixup for a control that no longer renders must not take focus from a select whose options compute `display:none`.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the changed set, and a closing scroll marks its select through `reconcile_select_list`. This extends `decision/changed-set-marks`. A new focus drop must say whether it marks the set (a focus change does).

@history_item n=23 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's option is excepted from the `display: none` tree exclusion, and a select's changed choice marks the changed set. Trap for item 1: any "display:none means not rendered" test must except a select's options, which compute `display:none` yet stay in the tree.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree) — every option of a single-choice select but its chosen one computes `display: none`, and a `multiple` select's rows do not. This is the why of `decision/hidden-content-excluded`, and the same trap for a display-based focus fixup.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`) — the `display: none` exception widened to an option under a select's `optgroup` (`is_select_option`), while `hidden`, `inert` and `visibility: hidden` still exclude it (`contract/accessibility-tree-hidden-content/display-none`). This is the reader a hidden-control predicate could share.

@history_item n=26 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (4. ARIA Patterns & Roles; 6. Visual Design Verification) — `~ decision/hidden-content-excluded`, `~ contract/accessibility-tree/hidden-and-inert-nodes` and `~ decision/hidden-content-excluded-from-a11y-tree` added `visibility: collapse` to the exclusions, on W9. The set of "no longer shows" conditions item 1 must cover now includes collapse.

@history_item n=27 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed in a redraw after style resolve only when the changed set is non-empty, and a poll pushes no tree. A focus drop decided after layout (option B) must still mark the set before that redraw, or the tree's focus goes stale.

@history_item n=28 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — this made the changed set truthful: a mutation, a focus or checked change and a text input mark it, and node creation, hover, active, scroll and layout do not. The tree refresh reads it. This is the origin of `decision/changed-set-marks`. A layout-driven focus fixup would be the first layout-caused mark.

@history_item n=29 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract as one item. Widening what marks the set, as an item-1 fix might, changes a ratified contract.

@history_item n=30 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (`decision/harness-settle-leaves-changed-set`). Item 1's checks that read the set must drain it first.

@history_item n=31 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks` took the select-list qualifier, "a scroll that closes a list included". This is the row history of the contract an item-1 fix may extend.

@history_item n=32 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (2. A11y Strategy) — `~ decision/changed-set-marks` now also marks a highlight moving to another row. This is the latest wording of the row. Any new mark from a focus fixup is amended against this text.

@history_item n=33 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (7. Screen Reader Support) — `~ decision/snapshot-reads-accessibility-tree` and `~ decision/snapshot-reads-the-tree` added `highlighted`, and `~ contract/snapshot/open-list-highlighted-row` reads the tree's active descendant. The snapshot's `focused` comes from the tree, so under item 1's limit a hidden focused control reads no `focused` while the engine still routes keys to it.

@history_item n=34 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Keyboard Navigation) — `+ decision/hidden-checkable-label-toggle`: a native checkbox or radio keeps its checked state with no box, so a label click toggles a `display:none` control. This is ratified upstream behaviour. A focus fixup for unrendered controls must leave this label path alone.

@history_item n=35 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-shared-tab-order-table` gained the exception that a Tab order reached only after a state change is a local table of the keyboard check. Item 1's stand walk after hiding a focused control is such a state-reached order.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (CSS properties as `CSSStyleDeclaration` accessors) — a script's `element.style` write goes through the inline style, so `display` and `visibility` reach the tree's hidden-content exclusion (`decision/script-style-through-inline-attribute`). This is a script path that hides a focused control. It also lives on the `blitz-vibey-script` crossing that item 3 touches.

@history_item n=37 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (IME enabled at a focused input's editor construction) — a text input already focused when its editor is built has IME on (`decision/ime-enabled-for-prefocused-input`). Focus and blur side effects (IME) run on focus moves. An item-1 drop must say whether it runs them, as removal does (`decision/removed-focused-node-drops-focus`).

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` hazard in a spawned task) — a `MountedData` call in a task spawned from a key handler panicked with `RefCell already borrowed`, and Cells calls its handles synchronously. This is the start of item 2's lineage.

@history_item n=39 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured, its fix routed) — `DioxusDocument::poll` held `inner.borrow_mut()` across `render_immediate`, where dioxus-core first polls the task. It was pinned by `dioxus_runtime_probe`'s two pins, and the fix was routed to «MountedData buffered render» on the founder's answer.

@history_item n=40 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the spawned-task MountedData hazard record, closed by the buffered render) — `initial_build` and `poll` render into a `MutationBuffer` unborrowed, then replay it once through one `MutationWriter`. The pins `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}` assert the engine's, the snapshot's and the tree's focus on `focus-target`. This is `decision/buffered-dioxus-render` and the guard item 2 must keep green.

@history_item n=41 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's same-render limit) — a task polled during a render sees the document as the previous render left it, so `set_focus_to` on a node created in that render returns `false`. The probe's pins act only on earlier-mounted nodes. The limit is owned by an open residual (target next), which is item 2's `res-433ttjb2`. This is the exact debt item 2 forks on. A deferred or replayed act must keep the three focus readings agreeing.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — Cells' arrows call `MountedData::scroll_to` synchronously, and Enter focuses the editor. Cells' synchronous handle calls were kept after the buffered render. This is the stand app whose `MountedData` use item 2 could let go async. Its `stand_keyboard` rows (`CELLS_TAB_ORDER`) must hold.

@history_item n=43 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to residual rows, among them `res-6k56erh6` (item 1's residual). A fix or keep of item 1 must re-say these owner clauses to the row's new status, as the 2026-11-07 wrap did for its residuals.

@history_item n=44 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the shell builds and refreshes the tree through `Document::accessibility_tree`, so the Dioxus wrapper's override (stable ids) reaches it (`decision/a11y-tree-through-document-trait`). An item-1 or item-2 fix in the engine must keep focus readings equal through the wrapper (`decision/adapter-tree-is-document-tree`, guard `stand_accessibility_ids`).
