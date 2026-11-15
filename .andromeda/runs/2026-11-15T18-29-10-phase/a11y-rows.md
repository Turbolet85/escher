# a11y-plan · the rows that bind 2026-11-15-taffy-from-upstream-git — picked 90 of 240 (44 by every draw) · rule any · pin 1162

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/accessibility-feature-default-on · Accessibility as a default-on feature — The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
decision/driver-requires-accessibility · Session library always enables accessibility — The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it. · why: Its executor reads the snapshot, which stands behind that feature.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — AccessKit is the technology the accessibility tree is built in, taken as a workspace dependency.
decision/accesskit-xplat-platform-adapter · accesskit_xplat as the platform adapter — The in-repo accesskit_xplat package, a workspace member, is the AccessKit platform adapter crate.
decision/dioxus-accessibility-feature-by-name · Dioxus accessibility feature taken only by name — The workspace takes the Dioxus crates without default features, so a crate gets accessibility only by naming it or by depending on a crate that names it.

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt when a poll takes a non-empty changed set, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/native-checkable-toggle · Native checkbox toggle from its checked state — A native checkbox or radio on a checkable role writes toggled true or false from its own checked state, and under any other role writes nothing. · guard: a_click_and_a_space_move_a_native_control_in_the_tree
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and rebuilt on a poll whose drained changed set was non-empty. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/a11y-tree-through-document-trait · Accessibility tree through the document trait — Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method. · why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not. · why: Ratified, one item with the accessibility tree's refresh on change.

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
decision/a11y-ci-leg · The a11y CI leg — The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check. · why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option. · guard: LegScriptTest's a11y tuple
decision/wcag-contrast-thresholds · WCAG 2.x text contrast thresholds — Text contrast is measured by the WCAG 2.x formula from computed styles and must reach 4.5, or 3.0 for large text. · guard: stand_contrast

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
contract/accessibility-tree-hidden-content/display-none · Accessibility tree hidden content · display: none — display: none excludes the node and its children from the tree, except an option under a select.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none and visibility hidden exclude the node and its children from the accessibility tree. · why: Every option of a single select but its chosen one computes display none. · guard: accessibility_hidden · except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.
decision/single-select-expanded-from-list · Single select expanded from its list — A single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected. · guard: select_popup

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/stated-tab-order-walked · Stated Tab order walked — Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders. · guard: stand_keyboard
decision/active-descendant-unread · aria-activedescendant kept unread — The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant, so an assistive technology learns the current row only from its selected reading. · why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified · guard: accessibility_active_descendant_unread
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false". · why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
contract/multiple-select/keys-and-row-clicks · multiple select · keys and row clicks — A multiple select takes its own keys and row clicks, apart from the single-choice select's handling.
contract/range-input/main-button-pointerdown · range input · main-button pointerdown — A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
contract/select-open-list/navigation-keys · single-choice select, list open · navigation keys — With a select's list open, the engine paints the list over the page, focus stays on the select, and the select's keys move the highlight over enabled rows, choosing nothing.
contract/single-select/optgroup-options · single-choice select · optgroup options — An optgroup's options are the select's own options, in tree order.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/focus-fixup-reads-inert-alone · Focus fixup reads inertness alone — A focused node that becomes inert gives focus up at the flush, in every document, with no DOM blur or focusout firing; disabled and hidden take no part, so a focused control that no longer shows keeps focus and takes keys. · why: The limit of reading inertness alone was ratified. · guard: inert_subtree; key_press_after_container_lost_its_box
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
decision/checkable-click-toggles-and-focuses · Checkbox or radio click toggles, fires input, focuses — Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control. · why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/disabled-optgroup-one-rule · Disabled optgroup rows read disabled by one rule — A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share. · why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
decision/focus-stays-on-select-with-list-open · Focus stays on a select while its list is open — Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close. · why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
decision/focusable-elements · What is focusable — An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary. · why: The narrowing of disabled to form controls and the inert read were ratified. · guard: stand_keyboard
decision/hidden-checkable-label-toggle · Native checkable state kept with no box — A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none. · why: Ratified as upstream's behaviour. · guard: clicking_label_toggles_hidden_checkbox
decision/keyboard-activation-enter-space · Enter and Space activate as a click — A pressed Enter on a focused enabled button, summary, submit/reset/button input or link with href, and a released Space on a focused enabled button, summary, or submit/reset/button/checkbox/radio input, dispatch a synthetic Click that runs the app's handlers and then the default click action. · why: ratified; a disabled form control takes neither key · guard: stand_keyboard, stand_act_keys · except: A disabled attribute on a summary or a link gates neither key, as HTML does; a link takes no Space.
decision/range-pointerdown-sets-value · Range input pointerdown sets value — A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0. · why: ratified
decision/script-style-through-inline-attribute · Script element.style goes through the style attribute — A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion. · why: pinned by style_bindings in the vibey-script dom tests; hidden-content exclusion reads the computed display and visibility written there
decision/select-list-choice-one-write-path · List choices share one write path — Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing. · why: pinned by enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change and a_choice_of_the_chosen_option_fires_nothing

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/select-list-colours-engine-constants · Open select list colours are engine constants — An open select list's highlight text, highlight background, disabled text and border colours are engine constants, in no sheet and not tokens, and the harness asserts the list's highlight and disabled rows as painted against them. · why: the engine holds no tokens · guard: select_popup_painted
decision/stand-contrast-measured · Stand contrast measured from computed styles — SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false). · guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none or visibility: hidden or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.
decision/lint-contrast-single-formula · Lint contrast through one formula — The driver's visual lint reads contrast through the harness's text contrast with the stand's exempt rule, never a second formula or threshold.
contract/visual-lint-finding/content · visual lint finding · content — A visual lint finding is written as ids, closed words and numbers only.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/contrast-exempt-disabled-controls · Disabled controls exempt from contrast — A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
decision/driver-click-in-shown-scrollbar-drags · Driver click in a shown scrollbar band drags — Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/hidpi-or-zoom-change-reshapes-text · Hidpi or zoom change reshapes text — A total scale change from hidpi or zoom invalidates text shaping. · why: the shell's scale-factor change and the browser zoom keys write hidpi scale and zoom, and rely on text being reshaped at the new scale
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
decision/select-list-rows-one-height · Open select list rows take one height — An open select list's rows take one height, the select's font on one line, so a row with a larger authored font clips, a stated limit.
decision/stand-accent-colour · Stand accent colour — The stand's accent is rest #3a5ce5, hover #2a4cd3, active #1f3fc1, stand-wide in every task CSS that declares it, a stand change and no engine change. · why: white on the earlier accent #4a6cf7 read 4.39 against 4.5, failing
decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
decision/stand-lint-clean-at-boot · Stand reads no lint finding at boot — The stand at boot reads no visual lint finding of any kind on the four lean tasks, in both layout modes. · guard: the_stand_at_boot_reads_its_measured_findings
decision/text-contrast-required-ratio · Text contrast required ratio per SC 1.4.3 — Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.
contract/lint-contrast-finding/bounds · the lint's contrast finding · bounds — A contrast finding carries the node's bounds as numbers.
decision/author-thumb-visible-state-change · Near-white author thumb visibly changes state — A near-white author scrollbar thumb must still visibly change on hover and on drag.
decision/contrast-unread-past-non-colour-paint · Contrast unread past non-colour paint — The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal. · guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
decision/highlighted-row-contrast-unread · Highlighted list row contrast unread by the lint — The visual lint's contrast reads an open list row's plain pair even when the row is highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.
decision/lint-contrast-shares-one-reader · Lint contrast shares the harness reader — The visual lint's contrast reading uses the harness's one contrast reader, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well. · why: no second formula or threshold
decision/multiple-select-chosen-row-pair · Multiple select chosen-row colour pair — A multiple select's chosen rows read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet. · why: hardcoded by design, as the engine holds no tokens · guard: select_interaction
decision/scheme-change-is-device-change · Scheme change counts as a device change — A color-scheme change on the viewport is a device change that rebuilds the stylist device. · why: pinned by color_scheme_change_recascades_styles in the device_coalescing tests
decision/wcag-contrast-ratio-helper · WCAG contrast ratio matching Chrome — Border bevels and scrollbar thumbs use a WCAG contrast ratio helper that matches Chrome's GetContrastRatio.
contract/lint-contrast-finding/defect · the lint's contrast finding · defect — A contrast finding carries its defect as a closed word.
contract/lint-contrast-finding/id · the lint's contrast finding · id — A contrast finding carries the id of the node read below its required ratio.
contract/lint-contrast-finding/ratio · the lint's contrast finding · ratio — A contrast finding carries the measured contrast ratio as a number.
contract/lint-contrast-finding/required · the lint's contrast finding · required — A contrast finding carries the required contrast ratio as a number.

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
contract/snapshot/multiple-select-rows-and-value · snapshot · multiple select rows and value — A multiple select's rows read their own client rects in the snapshot, and its value reads its first chosen option in tree order.
contract/snapshot/select-list-rows · snapshot · select list rows — The snapshot holds no expanded key and the highlighted row is in no snapshot and no tree; an open list's row reads its own bounds in the list.
decision/adapter-tree-equals-harness-tree · Adapter tree equals harness tree — On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
contract/select-open-list/optgroup-header · single-choice select, list open · optgroup header — An optgroup is painted as a header of an open list, heavier than a row by weight on every build, and a header chooses nothing.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
contract/accessibility-tree/select-role · the accessibility tree · select — A select reads ComboBox, a multiple select ListBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/driver-screenshot/row-shot · the driver's screenshot · a shot of an open list's row — A shot of a row crops to it and leaves the list open.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot/closed-select-option-bounds · the snapshot · a closed select's options — A closed select's options read under it, each with the select's client rect as its bounds.
contract/snapshot/multiple-select-row-bounds · the snapshot · a multiple select's rows and groups — A multiple select's rows and groups read their own client rects as their bounds, unlike a closed select's options.
contract/snapshot/open-list-row-bounds · the snapshot · an open list's rows — A select whose list is open is no closed select; an open row's bounds is its painted row's in the snapshot and the diff, the diff's differs naming bounds.
decision/accessibility-feature-gate · Screen-reader integration behind the accessibility feature — Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
decision/adapter-tree-is-document-tree · The adapter reads the document's own tree — The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root. · guard: stand_accessibility_ids
decision/interactive-nodes-named · Interactive nodes carry a name — Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes. · guard: stand_accessibility_ids
decision/option-labels-leave-only-as-pixels · Option labels leave an open list only as pixels — An option's label and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site. · why: ratified
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/snapshot/open-list-highlighted-row · the snapshot and the accessibility tree · an open list's highlighted row — An open list's highlighted row is in no snapshot and no tree, a stated limit: the tree says the list is open, the row shows only in a screenshot.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids

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
