# a11y-plan · the rows that bind 2026-11-18-a11y-re-read — picked 124 of 240 (86 by every draw) · rule any · pin 1905

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/a11y-tier-0 · Accessibility tier 0 — The accessibility tier is 0, covering WCAG success criteria 2.1.1, 1.4.3 and 2.4.3.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/accessibility-reach-tier-0 · Accessibility reach at tier 0 — The published accessibility reach is tier 0: WCAG SC 2.1.1, 1.4.3 and 2.4.3 and no other criterion, with what an assistive technology reads unmeasured.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt when a poll takes a non-empty changed set, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
contract/accessibility-tree-states/native-checkbox-or-radio · Accessibility tree states · native checkbox or radio — A native checkbox or radio under a checkable role writes toggled from its own checked state, reading neither aria-checked nor aria-pressed; under any other role it writes nothing.
contract/accessibility-tree-states/select-option · Accessibility tree states · select option — A select's option reads its chosen state into the tree.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and rebuilt on a poll whose drained changed set was non-empty. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle
decision/native-checkable-toggle · Native checkbox toggle from its checked state — A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing. · guard: a_click_and_a_space_move_a_native_control_in_the_tree
decision/toggle-read-by-role · Toggle state read per role — The aria-checked attribute sets toggled on the CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles alone, aria-pressed on every other role, and a native checkbox or radio input reads neither. · why: Ratified; follows HTML-AAM and ARIA in HTML. · guard: accessibility_states

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
decision/a11y-ci-leg · The a11y CI leg — The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check. · why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option. · guard: LegScriptTest's a11y tuple
decision/wcag-contrast-thresholds · WCAG 2.x text contrast thresholds — Text contrast is measured by the WCAG 2.x formula from computed styles and must reach 4.5, or 3.0 for large text. · guard: stand_contrast

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
contract/accessibility-tree-hidden-content/display-none · Accessibility tree hidden content · display: none — display: none excludes the node and its children from the tree, except an option under a select.
contract/accessibility-tree-hidden-content/hidden · Accessibility tree hidden content · hidden — The hidden attribute excludes the node and its children from the tree.
contract/accessibility-tree-hidden-content/visibility-hidden · Accessibility tree hidden content · visibility: hidden — visibility: hidden excludes the node and its children from the tree.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none and visibility hidden exclude the node and its children from the accessibility tree. · why: Every option of a single select but its chosen one computes display none. · guard: accessibility_hidden · except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
decision/single-select-expanded-from-list · Single select expanded from its list — A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected. · guard: select_popup

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
decision/stated-tab-order-walked · Stated Tab order walked — Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders. · guard: stand_keyboard
contract/cells-grid/arrows · seven_guis Cells grid · arrows — The arrows move the Cells grid's current cell, clamped, and bring the cell into view.
contract/cells-grid/editor-enter · seven_guis Cells grid · editor Enter — Enter in the Cells editor commits and returns focus to the grid.
contract/cells-grid/enter · seven_guis Cells grid · Enter — Enter on the Cells grid opens the editor on the current cell, focused.
contract/cells-grid/tab-from-editor · seven_guis Cells grid · Tab from editor — Tab leaving an open Cells editor commits it.
contract/cells-grid/tab-stop · seven_guis Cells grid · Tab stop — cells-grid, named Cells by aria_label with no role, is one Tab stop.
contract/crud-list/arrowdown-arrowup-home-end · seven_guis CRUD list · ArrowDown / ArrowUp / Home / End — With crud-list focused, ArrowDown, ArrowUp, Home and End move the selection as a row's click does.
contract/crud-list/tab-stop · seven_guis CRUD list · Tab stop — The list crud-list, a listbox named People, is the third CRUD Tab stop; its person rows are not focusable.
contract/keyboard-activation/enter · keyboard activation · Enter — A pressed Enter on the focused node dispatches a Click when the node is no disabled form control and is a button, a summary, an input of type submit, reset or button, or an a with href.
contract/keyboard-activation/handler-order · keyboard activation · handler order — On a keyboard-dispatched Click the app's handlers run first, then the engine's handle_click.
contract/keyboard-activation/space · keyboard activation · Space — A released Space dispatches a Click for a summary, or for a button or an input of type submit, reset, button, checkbox or radio that is no disabled form control, never for a link.
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
contract/range-input/arrowleft-arrowdown · range input · ArrowLeft / ArrowDown — A focused range input with no disabled attribute steps down on ArrowLeft and ArrowDown.
contract/range-input/arrowright-arrowup · range input · ArrowRight / ArrowUp — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp.
contract/range-input/end · range input · End — A focused range input with no disabled attribute steps to its maximum on End.
contract/range-input/home · range input · Home — A focused range input with no disabled attribute steps to its minimum on Home.
contract/range-input/other-keys · range input · other keys — On a focused range input every key other than the arrows, Home and End does nothing.
contract/range-input/value · range input · value — A range input's value is held apart from the value attribute and sanitized to [min, max] on the step grid.
contract/select-closed-list/arrowdown-arrowup · single-choice select, list closed · ArrowDown / ArrowUp — A focused single-choice select that is no disabled form control, its list closed, moves to the next / previous enabled option on ArrowDown / ArrowUp.
contract/select-closed-list/enter-space · single-choice select, list closed · Enter / Space — Enter, and a Space with no typed text standing, open a focused single-choice select's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing.
contract/select-closed-list/home-end · single-choice select, list closed · Home / End — A focused single-choice select that is no disabled form control, its list closed, moves to the first / last enabled option on Home / End.
contract/select-closed-list/other-keys · single-choice select, list closed · other keys — On a focused closed single-choice select every key other than the arrows, Home, End, typeahead, Enter and Space does nothing.
contract/select-closed-list/typeahead · single-choice select, list closed · typeahead — A character typed with no Ctrl, Alt or Meta on a focused closed single-choice select is typeahead, kept for 1 s of the document's clock.
contract/select-open-list/close · single-choice select, list open · close — Closing a select's list clears the typed typeahead text.
contract/select-open-list/enter-space · single-choice select, list open · Enter / Space — With a select's list open, Enter and Space choose the highlighted row and close the list.
contract/select-open-list/escape · single-choice select, list open · Escape — With a select's list open, Escape closes the list with the value unchanged and no event, as a second click, a press outside and a page scroll do.
contract/select-open-list/navigation-keys · single-choice select, list open · navigation keys — With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.
contract/select-open-list/tab · single-choice select, list open · Tab — With a select's list open, Tab in either direction chooses the highlighted row and closes the list before focus moves.
contract/single-select/optgroup-options · single-choice select · optgroup options — An optgroup's options are the select's own options, in tree order.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false". · why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
contract/range-input/click · range input · click — A click on a range input focuses it and changes no value.
contract/range-input/input-event · range input · input event — Each change of a range input's value fires one input event; a move that changes nothing fires none.
contract/single-select/change-events · single-choice select · input and change events — Each change of a single-choice select's value dispatches one input then one change; a key that changes nothing dispatches none.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/disabled-control-stops-click-walk · A disabled form control stops the click walk — A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it. · why: Ratified.
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/focus-fixup-reads-inert-alone · Focus fixup reads inertness alone — A focused node that becomes inert gives focus up at the flush, in every document, with no DOM blur or focusout firing; disabled and hidden take no part, so a focused control that no longer shows keeps focus and takes keys. · why: The limit of reading inertness alone was ratified. · guard: inert_subtree; key_press_after_container_lost_its_box
decision/focus-stays-on-select-with-list-open · Focus stays on a select while its list is open — Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close. · why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
decision/focusability-recomputed-on-attribute-write · Focusability recomputed on tabindex, href, disabled — Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works. · why: pinned by a_tabindex_set_after_creation_takes_effect in the focusability_updates tests; roving-tabindex widgets rely on it
decision/focusable-elements · What is focusable — An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary. · why: The narrowing of disabled to form controls and the inert read were ratified. · guard: stand_keyboard
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/keyboard-activation-enter-space · Enter and Space activate as a click — A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action. · why: ratified; a disabled form control takes neither key · guard: stand_keyboard, stand_act_keys · except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
decision/one-engine-change-per-checkable-click · One engine change per checkable click — A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none. · why: pinned by checkbox_click_fires_input_and_change_events in the vibey-script dom tests and by the checkable_change_event tests
decision/range-input-keys · Range input keys — A focused enabled range input steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and maximum on End, every other key doing nothing; each change fires one input event and a move that changes nothing fires none. · why: ratified
decision/removed-focused-node-drops-focus · Removing the focused node drops focus with blur steps — Removing the focused node resets focus to the body and runs blur side effects. · why: pinned by ime_is_disabled_when_focused_text_input_is_removed and removing_the_subtree_containing_the_focus_clears_the_ancestors
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
decision/tab-and-shift-tab-move-focus · Tab and Shift+Tab move focus with focus events — Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events. · why: pinned by tab_and_shift_tab_walk_the_stated_sequence_and_wrap and tab_and_shift_tab_skip_an_inert_subtree; the driver's press tab relies on it
decision/tab-chooses-and-closes-select-list · Tab chooses the row and closes the list first — With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction. · why: pinned by tab_chooses_and_closes_before_focus_moves_either_way in the select_popup tests
decision/falsy-boolean-attributes-removed · Falsy boolean attributes are removed — A falsy Dioxus value of one of the bridge's 28 boolean attributes removes the attribute on both the dynamic and the static template path, while a truthy value is still written. · why: Uses upstream's own boolean-attribute predicate, the mutation writer kept identical to upstream's. · guard: dioxus_falsy_boolean_attrs, dioxus_falsy_disabled · except: An attribute outside the list holding false, such as aria-hidden or a data- attribute, is still written as "false".
decision/hidden-checkable-label-toggle · Native checkable state kept with no box — A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none. · why: Ratified as upstream's behaviour. · guard: clicking_label_toggles_hidden_checkbox
decision/label-click-focuses-select-only · Label click focuses its select and opens nothing — A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it. · why: pinned by a_labels_click_focuses_the_select_and_opens_nothing in the select_popup tests
decision/range-click-focuses-not-value · Range input click gives focus, not value — A click on a range input, including a label's synthetic click, focuses the input and changes no value. · why: ratified
decision/select-key-change-fires-input-change · Select key change fires one input then one change — Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none. · why: pinned by the_arrows_home_and_end_choose_an_enabled_option, which counts input and change per key, none where nothing changes
decision/select-list-choice-one-write-path · List choices share one write path — Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing. · why: pinned by enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change and a_choice_of_the_chosen_option_fires_nothing

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/select-list-colours-engine-constants · Open select list colours are engine constants — An open select list's highlight text, highlight background, disabled text and border colours are engine constants, in no sheet and not tokens, and the harness asserts the list's highlight and disabled rows as painted against them. · why: the engine holds no tokens · guard: select_popup_painted
decision/stand-contrast-measured · Stand contrast measured from computed styles — SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false). · guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none or visibility: hidden or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.
decision/lint-contrast-single-formula · Lint contrast through one formula — The driver's visual lint reads contrast through the harness's text contrast with the stand's exempt rule, never a second formula or threshold.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/contrast-exempt-disabled-controls · Disabled controls exempt from contrast — A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
decision/contrast-unread-past-non-colour-paint · Contrast unread past non-colour paint — The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal. · guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/highlighted-row-contrast-unread · Highlighted list row contrast unread by the lint — The visual lint's contrast reads an open list row's plain pair even when the row is highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
decision/multiple-select-chosen-row-pair · Multiple select chosen-row colour pair — A multiple select's chosen rows read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet. · why: hardcoded by design, as the engine holds no tokens · guard: select_interaction
decision/stand-accent-colour · Stand accent colour — The stand's accent is rest #3a5ce5, hover #2a4cd3, active #1f3fc1, stand-wide in every task CSS that declares it, a stand change and no engine change. · why: white on the earlier accent #4a6cf7 read 4.39 against 4.5, failing
decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
decision/text-contrast-required-ratio · Text contrast required ratio per SC 1.4.3 — Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both
decision/wcag-contrast-ratio-helper · WCAG contrast ratio matching Chrome — Border bevels and scrollbar thumbs use a WCAG contrast ratio helper that matches Chrome's GetContrastRatio.
decision/lint-contrast-shares-one-reader · Lint contrast shares the harness reader — The visual lint's contrast reading uses the harness's one contrast reader, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well. · why: no second formula or threshold

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
contract/accessibility-tree/single-select · accessibility tree · single select — A single select reads ComboBox and writes expanded from its list's open state, reading no aria-expanded; an author's open or aria-expanded on it steers nothing; its chosen option reads selected and the tree's focus stays on the select.
contract/select-open-list/optgroup-header · single-choice select, list open · optgroup header — An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.
contract/snapshot/select-list-rows · snapshot · select list rows — The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/adapter-tree-equals-harness-tree · Adapter tree equals harness tree — On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
contract/accessibility-tree/author-id · accessibility tree · author_id — In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
contract/snapshot/checked · snapshot · checked — The snapshot's checked field reads a native checkbox's or radio's state, else a checkable role's toggled.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.
contract/snapshot/multiple-select-rows-and-value · snapshot · multiple select rows and value — A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
contract/snapshot/value · snapshot · value — The snapshot's value is read from the element: the editor's text, else a range input's current value, else the value attribute.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/active-descendant-kept-unread · Missing active descendant kept unread on the CRUD list — The CRUD list's rows are not focusable and the accessibility tree's focus stays on the list, never the option; the missing aria-activedescendant is kept unread. · why: ratified · guard: accessibility_active_descendant_unread
decision/checked-reading · Source of the checked reading — checked is a native checkbox's or radio's state, else on a checkable role its toggled reading, True true and False and Mixed false, nothing where the role states none. · why: ratified
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
contract/accessibility-tree/focus · the accessibility tree · focus — Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
contract/accessibility-tree/native-checked · the accessibility tree · a native checkbox's or radio's checked state — On a checkable role toggled reads True or False from the native checked state, no author attribute read on a native control; under any other role a native control writes nothing.
contract/accessibility-tree/select-expanded · the accessibility tree · a single select's expanded — A single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open.
contract/snapshot/open-list-highlighted-row · the snapshot and the accessibility tree · an open list's highlighted row — An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.
decision/adapter-tree-is-document-tree · The adapter reads the document's own tree — The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root. · guard: stand_accessibility_ids
contract/accessibility-tree/root · the accessibility tree · root — The tree root is a Window node with id u64::MAX.
contract/accessibility-tree/select-role · the accessibility tree · select — A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot/multiple-select-row-bounds · the snapshot · a multiple select's rows and groups — A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
contract/snapshot/open-list-row-bounds · the snapshot · an open list's rows — A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
contract/snapshot/select-value · the snapshot · a select's value — A select's value is its chosen option's value; for a multiple select its first chosen option's in tree order; nothing when none is chosen.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/invalid-state-not-colour-alone · Invalid state not by colour alone — Invalid dates carry an invalid class with red styling and disable the Book button, and the two date inputs also state aria-invalid, so the invalid state is in the tree and the snapshot and not carried by the class alone.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids
decision/invalid-state-not-by-class-alone · Invalid date state stated beyond its class — The flight booker's invalid date inputs state aria-invalid beside the invalid class and its red styling, and the Book button is disabled, so the error cue does not rest on colour. · why: So the invalid state is in the accessibility tree and the snapshot and not carried by the class alone. · guard: stand_flight_booker; stand_snapshot_state

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/a11y-test-leg · Accessibility test leg in the slow tier — The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget. · guard: ci.yml a11y job

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y

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
