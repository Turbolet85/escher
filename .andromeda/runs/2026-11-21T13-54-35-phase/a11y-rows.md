# a11y-plan · the rows that bind 2026-11-21-single-select-sized-as-a-list-box — picked 101 of 240 (64 by every draw) · rule any · pin 2894

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/role-attribute-then-element-then-unknown · Role from role attribute, element, else Unknown — A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown. · why: snapshot role readings and the stand checks controls_carry_role_and_name and aria_checked_reads_its_three_values_on_a_checkable_role_alone read roles set this way
contract/accessibility-tree-states/select-option · Accessibility tree states · select option — A select's option reads its chosen state into the tree.
contract/aria-property-reads/aria-expanded · ARIA name and property reads · aria-expanded — aria-expanded is read as true or false and nothing else on every element but a single select, which writes its list's open state instead and reads no author attribute.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/aria-states-on-every-element · ARIA states written on every element — The accessibility builder writes the selected, toggled and invalid states from ARIA state attributes on every element, and an element whose role does not support an attribute reads it as one whose role does. · why: Ratified. · guard: accessibility_states · except: A select's option, whose selected state is its chosen state, with no aria-selected read.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too, and its highlight moving rows; node creation, hover, active, other scrolls and layout alone do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/role-mapping-html-aam · Role mapping by HTML-AAM — Element roles in the accessibility tree are mapped by the HTML-AAM specification. · guard: accessibility_roles
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
decision/wcag-contrast-thresholds · WCAG 2.x text contrast thresholds — Text contrast is measured by the WCAG 2.x formula, from computed styles or, on an open list's highlighted row, from its painted pair, and must reach 4.5, or 3.0 for large text. · guard: stand_contrast

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/html-aam-role-mapping · Native element roles follow HTML-AAM — Native HTML elements map to accessibility roles as HTML-AAM states: landmarks, headings, lists, tables, interactive and inline semantics.
contract/accessibility-tree-hidden-content/display-none · Accessibility tree hidden content · display: none — display: none excludes the node and its children from the tree, except an option under a select.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse exclude the node and its children from the accessibility tree. · why: Every option of a single select but its chosen one computes display none. · guard: accessibility_hidden · except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
decision/single-select-expanded-from-list · Single select expanded from its list — A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected. · guard: select_popup

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
contract/select-closed-list/arrowdown-arrowup · single-choice select, list closed · ArrowDown / ArrowUp — A focused single-choice select that is no disabled form control, its list closed, moves to the next / previous enabled option on ArrowDown / ArrowUp.
contract/select-closed-list/enter-space · single-choice select, list closed · Enter / Space — Enter, and a Space with no typed text standing, open a focused single-choice select's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing.
contract/select-closed-list/home-end · single-choice select, list closed · Home / End — A focused single-choice select that is no disabled form control, its list closed, moves to the first / last enabled option on Home / End.
contract/select-closed-list/typeahead · single-choice select, list closed · typeahead — A character typed with no Ctrl, Alt or Meta on a focused closed single-choice select is typeahead, kept for 1 s of the document's clock.
contract/single-select/change-events · single-choice select · input and change events — Each change of a single-choice select's value dispatches one input then one change; a key that changes nothing dispatches none.
contract/single-select/optgroup-options · single-choice select · optgroup options — An optgroup's options are the select's own options, in tree order.
decision/active-descendant-unread · aria-activedescendant kept unread — The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant, so an assistive technology learns the current row only from its selected reading. · why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified · guard: accessibility_active_descendant_unread
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
contract/select-closed-list/other-keys · single-choice select, list closed · other keys — On a focused closed single-choice select every key other than the arrows, Home, End, typeahead, Enter and Space does nothing.
contract/select-open-list/navigation-keys · single-choice select, list open · navigation keys — With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.
contract/select-open-list/tab · single-choice select, list open · Tab — With a select's list open, Tab in either direction chooses the highlighted row and closes the list before focus moves.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/focus-stays-on-select-with-list-open · Focus stays on a select while its list is open — Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close. · why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
decision/focusable-elements · What is focusable — An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary. · why: The narrowing of disabled to form controls and the inert read were ratified. · guard: stand_keyboard
decision/label-click-focuses-select-only · Label click focuses its select and opens nothing — A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it. · why: pinned by a_labels_click_focuses_the_select_and_opens_nothing in the select_popup tests
decision/select-key-change-fires-input-change · Select key change fires one input then one change — Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none. · why: pinned by the_arrows_home_and_end_choose_an_enabled_option, which counts input and change per key, none where nothing changes
decision/select-list-choice-one-write-path · List choices share one write path — Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing. · why: pinned by enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change and a_choice_of_the_chosen_option_fires_nothing
decision/tab-and-shift-tab-move-focus · Tab and Shift+Tab move focus with focus events — Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events, in every document, skipping a node that does not show: out of the document, under display: none, or not visibility: visible. · why: pinned by tab_and_shift_tab_walk_the_stated_sequence_and_wrap and tab_and_shift_tab_skip_an_inert_subtree; the driver's press tab relies on it
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/tab-chooses-and-closes-select-list · Tab chooses the row and closes the list first — With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction. · why: pinned by tab_chooses_and_closes_before_focus_moves_either_way in the select_popup tests

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/lint-contrast-single-formula · Lint contrast through one formula — The driver's visual lint reads contrast through the harness's readers, the highlight reader first and the text contrast where it answers nothing, with the stand's exempt rule, never a second formula or threshold.
decision/select-list-colours-engine-constants · Open select list colours are engine constants — An open select list's highlight, disabled text and border colours are engine constants, in no sheet and not tokens, the highlight pair equal to the default sheet's chosen-row pair, black on #c8c8c8, and the harness asserts the highlight and disabled rows as painted against them. · why: the engine holds no tokens · guard: select_popup_painted
decision/select-list-no-motion · Select list without motion — A select list opens and closes with no motion.
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/contrast-exempt-disabled-controls · Disabled controls exempt from contrast — A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/lint-contrast-shares-one-reader · Lint contrast shares the harness reader — The visual lint's contrast reading uses the harness's readers, the highlight reader for an open list's highlighted row and the text contrast elsewhere, through the one contrast formula, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well. · why: no second formula or threshold
decision/multiple-select-chosen-row-pair · Multiple select chosen-row colour pair — A multiple select's chosen rows read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet. · why: hardcoded by design, as the engine holds no tokens · guard: select_interaction
decision/select-list-rows-one-height · Open select list rows take one height — An open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole.
decision/text-contrast-required-ratio · Text contrast required ratio per SC 1.4.3 — Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
contract/lint-contrast-finding/bounds · the lint's contrast finding · bounds — A contrast finding carries the node's bounds as numbers.
contract/lint-contrast-finding/defect · the lint's contrast finding · defect — A contrast finding carries its defect as a closed word.
contract/lint-contrast-finding/id · the lint's contrast finding · id — A contrast finding carries the id of the node read below its required ratio.
contract/lint-contrast-finding/ratio · the lint's contrast finding · ratio — A contrast finding carries the measured contrast ratio as a number.
contract/lint-contrast-finding/required · the lint's contrast finding · required — A contrast finding carries the required contrast ratio as a number.
decision/contrast-unread-past-non-colour-paint · Contrast unread past non-colour paint — The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal. · guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
decision/driver-click-in-shown-scrollbar-drags · Driver click in a shown scrollbar band drags — Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/open-select-list-not-open-state · Open select list sets no :open state — An open select's list is no `:open` state, so no author style keys on a list standing open. · why: pinned by an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match in the select_popup tests
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed, invalid and highlighted from the accessibility tree, never from the element.
contract/accessibility-tree/optgroup · accessibility tree · optgroup — An optgroup reads Role::Group, named by its non-empty label attribute.
contract/accessibility-tree/single-select · accessibility tree · single select — A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
contract/select-open-list/optgroup-header · single-choice select, list open · optgroup header — An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
contract/snapshot/multiple-select-rows-and-value · snapshot · multiple select rows and value — A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
contract/snapshot/select-list-rows · snapshot · select list rows — The snapshot holds no expanded key and reads an open list's highlighted row alone as highlighted; an open list's row reads its own bounds in the list.
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/adapter-tree-equals-harness-tree · Adapter tree equals harness tree — On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
contract/accessibility-tree/multiple-select · accessibility tree · multiple select — A multiple select carries set_multiselectable in the tree.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.
contract/accessibility-tree/focus · the accessibility tree · focus — Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
contract/accessibility-tree/select-expanded · the accessibility tree · a single select's expanded — A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
contract/accessibility-tree/select-role · the accessibility tree · select — A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/actions-answer/fields · the actions answer · fields — The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/driver-screenshot/row-shot · the driver's screenshot · a shot of an open list's row — A shot of a row crops to it and leaves the list open.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
contract/snapshot-text/state-tokens · the snapshot text form · state tokens — Each node is one line; selected or unselected, pressed or unpressed and invalid are written after value= and before focused.
contract/snapshot/multiple-select-row-bounds · the snapshot · a multiple select's rows and groups — A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
contract/snapshot/select-value · the snapshot · a select's value — A select's value is its chosen option's value; for a multiple select its first chosen option's in tree order; nothing when none is chosen.
decision/actions-verb-changes-nothing · The actions verb changes nothing — Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
decision/driver-schema-has-no-role-set · Driver schema defines no role or name set — The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/interactive-nodes-named · Interactive nodes carry a name — Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes. · guard: stand_accessibility_ids
decision/option-labels-leave-only-as-pixels · Option labels leave an open list only as pixels — An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site. · why: ratified
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
decision/screenshot-changes-nothing · A screenshot changes nothing — Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
decision/text-runs-label-their-parent · Text runs label their parent element — Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name. · why: accessible names of buttons and controls come from it; pinned by every_interactive_role_element_reads_a_name and role_and_name_match_the_accessibility_tree
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/snapshot-text/clickable · the snapshot text form · clickable — The token clickable is written after focused only when true.
contract/snapshot/closed-select-option-bounds · the snapshot · a closed select's options — A closed select's options read under it, each with the select's client rect as its bounds.
contract/snapshot/open-list-highlighted-row · the snapshot and the accessibility tree · an open list's highlighted row — While a single select's list is open, the tree names the highlighted option as the select's active descendant and the snapshot reads that row alone highlighted, the tree's focus staying on the select.
contract/snapshot/open-list-row-bounds · the snapshot · an open list's rows — A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids · except: A Tab order reached only after a state change, such as the order page with its failure shown, is stated as a local table of the keyboard check, not in the shared table.

# a11y-plan · its absences, names and overviews, whole

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
> NOT YET MEASURED — must-be-accessible critical paths and a11y triggers: no slice recorded them, and s03 recorded accessibility scope as out of slice.
absence/assistive-technology-reach — A platform accessibility adapter in any stand binary, so no active assistive technology is given a tree

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
> NOT YET MEASURED — a stated accessibility strategy (POUR coverage depth, agent-runnable invariants, naming conventions): no slice states one, and s01, s02, s03, s06, s07, s08, s10 and s13 recorded accessibility strategy as out of slice.

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
> NOT YET MEASURED — the structured violation JSON schema / log format, the WCAG criteria mapping of the existing tests and a screen-reader test pattern: no slice recorded them (s01, s02, s06, s07 and s13 recorded the a11y assertion harness as out of slice). The obs log format is now measured for escher's own sink (obs plan §3, §6 Log Coverage, §8): non-JSON text lines on stderr in two record classes — one line per printed event, `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…`, and, since 2026-10-07-driver-command-spans, one line per closed span of an admitted target, the same prefix then `span={the span's name}`, the span's recorded fields and the layer's close fields `message`, `time.busy` and `time.idle`, with no line when a span is created, recorded to, entered or exited (the second class was ratified by the founder, 2026-10-09, as obs plan §6 states) — both behind a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level — a closed span's every pair judged by the same rule, an outside target's span writing nothing; an a11y violation schema is not yet defined against it.

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
> NOT YET MEASURED — skip links, modal focus traps and route-change focus restoration: no slice recorded them; s01 recorded keyboard focus handling code as out of slice and s13 recorded keyboard interaction as out of slice.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
> NOT YET MEASURED — focus-ring, target-size and typography-readability tokens: no slice recorded them, and s01 and s07 recorded contrast and visual checks as out of slice. The stand controls' contrast pairs are measured since 2026-10-13-stand-contrast-harness (Color contrast above), and the CRUD list rows, the text outside the controls and the colour states reached by input, hover and active included, since 2026-10-14-stand-a11y-assertions.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
absence/select-list-motion — Motion on an open select's list

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
> NOT YET MEASURED — live regions, heading-hierarchy handling and per-surface screen-reader test specs: no slice recorded them; s01, s07, s10 and s13 recorded screen-reader handling as out of slice, and s08 recorded the screen-reader tree output as out of slice.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
absence/snapshot-list-open-state — A snapshot reading of whether a list is open (no expanded node key)

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
> NOT YET MEASURED — timeout extensions, plain-language targets, redundant-entry avoidance and accessible authentication: every slice except s04 recorded cognitive accessibility as out of slice.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
> NO RECORDED INTENT

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-a11y-decisions-log · ## 12. A11y Decisions Log
> NO RECORDED INTENT
