# a11y-plan · the rows that bind 2026-11-23-circle-drawer-on-a-canvas — picked 88 of 242 (60 by every draw) · rule any · pin 3410

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/stand-builds-no-platform-adapter · No platform adapter on the stand binaries — No stand binary builds a platform adapter: neither accesskit_xplat nor accesskit_winit is in the stand's dependency graph, so no assistive technology is given a tree by the stand. · why: Ratified as deferred: the adapter and its windowed witness at an active assistive technology are carried to a later version.
decision/accessibility-feature-default-on · Accessibility as a default-on feature — The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
decision/capability-statement-reach-by-hand · Accessibility reach edited by hand — A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section. · why: The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.
decision/dioxus-accessibility-feature-by-name · Dioxus accessibility feature taken only by name — The workspace takes the Dioxus crates without default features, so a crate gets accessibility only by naming it or by depending on a crate that names it.
decision/driver-requires-accessibility · Session library always enables accessibility — The escher-driver session library enables accessibility on dioxus-native-dom unconditionally, with no feature table of its own, so the feature reaches every binary that depends on it. · why: Its executor reads the snapshot, which stands behind that feature.

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt in a redraw after style resolve when the changed set is non-empty, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
decision/role-attribute-then-element-then-unknown · Role from role attribute, element, else Unknown — A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown. · why: snapshot role readings and the stand checks controls_carry_role_and_name and aria_checked_reads_its_three_values_on_a_checkable_role_alone read roles set this way
contract/aria-property-reads/aria-disabled · ARIA name and property reads · aria-disabled — Only the value true of aria-disabled sets the node disabled.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-through-document-trait · Accessibility tree through the document trait — Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method. · why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too, and its highlight moving rows; node creation, hover, active, other scrolls and layout alone do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/one-read-per-aria-attribute · One read per ARIA attribute — The accessibility builder reads each ARIA attribute once, keeping the trimmed non-empty aria-label read and the closed-set state reads. · why: Differs from upstream: upstream's boolean-parse aria-selected and unfiltered aria-label reads were dropped in the merge.
decision/role-mapping-html-aam · Role mapping by HTML-AAM — Element roles in the accessibility tree are mapped by the HTML-AAM specification. · guard: accessibility_roles

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract
decision/wcag-contrast-thresholds · WCAG 2.x text contrast thresholds — Text contrast is measured by the WCAG 2.x formula, from computed styles or, on an open list's highlighted row, from its painted pair, and must reach 4.5, or 3.0 for large text. · guard: stand_contrast

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/html-aam-role-mapping · Native element roles follow HTML-AAM — Native HTML elements map to accessibility roles as HTML-AAM states: landmarks, headings, lists, tables, interactive and inline semantics.
contract/accessibility-tree-hidden-content/display-none · Accessibility tree hidden content · display: none — display: none excludes the node and its children from the tree, except an option under a select.
contract/accessibility-tree-hidden-content/hidden · Accessibility tree hidden content · hidden — The hidden attribute excludes the node and its children from the tree.
contract/accessibility-tree-hidden-content/visibility-hidden · Accessibility tree hidden content · visibility: hidden — visibility: hidden excludes the node and its children from the tree.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse exclude the node and its children from the accessibility tree. · why: Every option of a drop-down single select but its chosen one computes display none. · guard: accessibility_hidden · except: A select's option is excepted from display none and stays in the tree; hidden, inert and visibility hidden still exclude it.

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/double-click-threshold · Double click threshold — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, each still focusing its target as one click does.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy Dioxus value of one of the bridge's boolean attributes (upstream's is_bool_attr set) removes the attribute on both attribute paths; a truthy value is written, and an attribute outside the list holding false is written as "false". · why: So an enabled control does not match :disabled and hidden: false keeps an element laid out and in the tree.
contract/range-input/arrowleft-arrowdown · range input · ArrowLeft / ArrowDown — A focused range input with no disabled attribute steps down on ArrowLeft and ArrowDown.
contract/range-input/arrowright-arrowup · range input · ArrowRight / ArrowUp — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp.
contract/range-input/click · range input · click — A click on a range input focuses it and changes no value.
contract/range-input/end · range input · End — A focused range input with no disabled attribute steps to its maximum on End.
contract/range-input/home · range input · Home — A focused range input with no disabled attribute steps to its minimum on Home.
contract/range-input/input-event · range input · input event — Each change of a range input's value fires one input event; a move that changes nothing fires none.
contract/range-input/main-button-pointerdown · range input · main-button pointerdown — A main-button pointerdown on a range input that is not disabled sets the value from the hit's fraction of the content box, clamped to [0, 1].
contract/range-input/other-keys · range input · other keys — On a focused range input every key other than the arrows, Home and End does nothing.
contract/range-input/value · range input · value — A range input's value is held apart from the value attribute and sanitized to [min, max] on the step grid.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/click-focuses-buttons · A click focuses a button — A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus. · why: Ratified. · guard: stand_keyboard; stand_act_obstructed
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/double-click-on-document-clock · Double click read on the document's clock — Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, a harness-built document using the harness's virtual clock, while each click still focuses its target as one click does. · why: Ratified. · guard: stand_virtual_clock
decision/falsy-boolean-attributes-removed · Falsy boolean attributes are removed — A falsy Dioxus value of one of the bridge's 28 boolean attributes removes the attribute on both the dynamic and the static template path, while a truthy value is still written. · why: Uses upstream's own boolean-attribute predicate, the mutation writer kept identical to upstream's. · guard: dioxus_falsy_boolean_attrs, dioxus_falsy_disabled · except: An attribute outside the list holding false, such as aria-hidden or a data- attribute, is still written as "false".
decision/focusable-elements · What is focusable — An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary. · why: The narrowing of disabled to form controls and the inert read were ratified. · guard: stand_keyboard
decision/range-click-focuses-not-value · Range input click gives focus, not value — A click on a range input, including a label's synthetic click, focuses the input and changes no value. · why: ratified
decision/range-input-keys · Range input keys — A focused enabled range input steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and maximum on End, every other key doing nothing; each change fires one input event and a move that changes nothing fires none. · why: ratified
decision/range-pointerdown-sets-value · Range input pointerdown sets value — A main-button pointerdown on an enabled range input sets its value from the hit's fraction of the content box, clamped to [0, 1], a content box of no width reading 0. · why: ratified
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
decision/tab-and-shift-tab-move-focus · Tab and Shift+Tab move focus with focus events — Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events, in every document, skipping a node that does not show: out of the document, under display: none, or not visibility: visible. · why: pinned by tab_and_shift_tab_walk_the_stated_sequence_and_wrap and tab_and_shift_tab_skip_an_inert_subtree; the driver's press tab relies on it
contract/dioxus-document/reached-handlers · Dioxus document reached-handler record · reached_handlers — The id and event of each node whose listeners held the event, resolved before the handlers run, first-reached, deduplicated; a dispatch its target or an ancestor stops, or a handler's panic unwinds, counts the innermost listener alone, a non-bubbling event its target alone; empty unless switched on.
contract/dioxus-document/record-reached · Dioxus document reached-handler record · record_reached — Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
decision/focusability-recomputed-on-attribute-write · Focusability recomputed on tabindex, href, disabled — Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works. · why: pinned by a_tabindex_set_after_creation_takes_effect in the focusability_updates tests; roving-tabindex widgets rely on it
contract/ui-graph-state/no-registered-state · UI graph state registration · no registered state — A page with no registered state reaches no registry, and its events dispatch with no registry recording.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/lint-contrast-single-formula · Lint contrast through one formula — The driver's visual lint reads contrast through the harness's readers, the highlight reader first and the text contrast where it answers nothing, with the stand's exempt rule, never a second formula or threshold.
contract/visual-lint-finding/content · visual lint finding · content — A visual lint finding is written as ids, closed words and numbers only.
decision/stand-contrast-measured · Stand contrast measured from computed styles — SC 1.4.3 is measured on the stand from computed styles with the WCAG 2.x ratio, 3.0 required of large text and else 4.5, a row exempt exactly where the snapshot's enabled reads Some(false). · guard: stand_contrast, the_load_page_meets_its_ratios, cells_meets_its_ratios
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/contrast-exempt-disabled-controls · Disabled controls exempt from contrast — A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
decision/lint-contrast-shares-one-reader · Lint contrast shares the harness reader — The visual lint's contrast reading uses the harness's readers, the highlight reader for an open list's highlighted row and the text contrast elsewhere, through the one contrast formula, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well. · why: no second formula or threshold
decision/stand-lint-clean-at-boot · Stand reads no lint finding at boot — The stand at boot reads no visual lint finding of any kind on the four lean tasks, in both layout modes. · guard: the_stand_at_boot_reads_its_measured_findings
decision/text-contrast-required-ratio · Text contrast required ratio per SC 1.4.3 — Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
decision/contrast-unread-past-non-colour-paint · Contrast unread past non-colour paint — The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal. · guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/stand-accent-colour · Stand accent colour — The stand's accent is rest #3a5ce5, hover #2a4cd3, active #1f3fc1, stand-wide in every task CSS that declares it, a stand change and no engine change. · why: white on the earlier accent #4a6cf7 read 4.39 against 4.5, failing
decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/accessibility-tree/author-id · accessibility tree · author_id — In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.
contract/snapshot/value · snapshot · value — The snapshot's value is read from the element: the editor's text, else a range input's current value, else the value attribute.
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed, invalid and highlighted from the accessibility tree, never from the element.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/screenshot-changes-nothing · A screenshot changes nothing — Taking a screenshot moves no focus, leaves has_changes false and leaves the next diff empty.
decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/accessibility-tree/focus · the accessibility tree · focus — Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/handler-coverage/answer · handler coverage · answer — The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot-text/clickable · the snapshot text form · clickable — The token clickable is written after focused only when true.
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping.
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.

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
