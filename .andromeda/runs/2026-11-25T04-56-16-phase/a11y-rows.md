# a11y-plan · the rows that bind 2026-11-25-virtual-nodes-as-elements — picked 51 of 243 (28 by every draw) · rule any · pin 3791

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/capability-statement-reach-by-hand · Accessibility reach edited by hand — A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section. · why: The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.
decision/accessibility-feature-default-on · Accessibility as a default-on feature — The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
decision/accessibility-reach-tier-0 · Accessibility reach at tier 0 — The published accessibility reach is tier 0: WCAG SC 2.1.1, 1.4.3 and 2.4.3 and no other criterion, with what an assistive technology reads unmeasured.
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — AccessKit is the technology the accessibility tree is built in, taken as a workspace dependency.
decision/single-select-engine-overlay-list · Single select opens an engine overlay list — A closed drop-down single select (display size 1) opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root; a single list box opens none. · why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt in a redraw after style resolve when the changed set is non-empty, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/a11y-tree-through-document-trait · Accessibility tree through the document trait — Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method. · why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too, and its highlight moving rows; node creation, hover, active, other scrolls and layout alone do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle

## 5-keyboard-navigation · ## 5. Keyboard Navigation · twin keyboard-navigation
decision/active-descendant-unread · aria-activedescendant kept unread — The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant, so an assistive technology learns the current row only from its selected reading. · why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified · guard: accessibility_active_descendant_unread
decision/disabled-by-attribute-presence · Disabled-ness keyed one way by attribute presence — On a button, input, select or textarea, disabled-ness is the disabled attribute's presence alone, read alike by :disabled, the pointer gates, Enter and Space activation and focusability, so disabled="false" still disables.
decision/stated-tab-order-walked · Stated Tab order walked — Each lean task, the load page and Cells state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step, and every lean task's control lying in one of the two lean orders. · guard: stand_keyboard

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/removed-focused-node-drops-focus · Removing the focused node drops focus with blur steps — Removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects. · why: pinned by ime_is_disabled_when_focused_text_input_is_removed and removing_the_subtree_containing_the_focus_clears_the_ancestors
decision/disabled-by-presence · Disabled read by attribute presence — The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count. · why: The presence reading and its narrowing to form controls were ratified. · guard: focusability_updates; stand_keyboard
decision/driver-type-focuses-select-no-click · Driver type focuses a select with no click — The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. · why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`.
contract/lint-contrast-finding/id · the lint's contrast finding · id — A contrast finding carries the id of the node read below its required ratio.

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else; the canvas read, which reads one canvas element's draw commands, and the virtual-node read, which reads one canvas's declared nodes, are no readers of the snapshot model.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed, invalid and highlighted from the accessibility tree, never from the element.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, names and closed words only.
contract/accessibility-tree/author-id · accessibility tree · author_id — In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/accessibility-feature-gate · Screen-reader integration behind the accessibility feature — Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
decision/enabled-by-disabled-presence · Enabled follows the disabled attribute's presence — A present disabled attribute, whatever its value, reads not enabled and not focusable on an element that can be disabled; aria-disabled moves no snapshot reading. · guard: a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping; the canvas read and the declared-node read, each reading one canvas element, are no readers of the snapshot model.
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/accessibility-tree/focus · the accessibility tree · focus — Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
contract/accessibility-tree/root · the accessibility tree · root — The tree root is a Window node with id u64::MAX.
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/driver-screenshot/element-shot · the driver's screenshot · an element shot — An element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen.
contract/node-id/reserved-separator · served node ids · the separator /// — No served id holds ///; the form {owner}///{key} is reserved.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot-text/clickable · the snapshot text form · clickable — The token clickable is written after focused only when true.
decision/adapter-tree-is-document-tree · The adapter reads the document's own tree — The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root. · guard: stand_accessibility_ids
contract/accessibility-tree/select-expanded · the accessibility tree · a single select's expanded — A drop-down single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open; a single list box writes no expanded.
contract/mcp-result/snapshot · MCP tool results · snapshot — An accepted snapshot result is the text form itself for a whole screen, and for a screen in parts the JSON line {text, part, parts}; an acting call's result is its JSON line with the diff.
contract/snapshot-text/parts · the snapshot text form · parts — A screen over the text budget is read in parts of consecutive pre-order lines, never a split line, concatenating to the whole text with every node id on exactly one line.
contract/snapshot-text/state-tokens · the snapshot text form · state tokens — Each node is one line; selected or unselected, pressed or unpressed and invalid are written after value= and before focused.

# a11y-plan · its absences, names and overviews, whole

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
> NOT YET MEASURED — must-be-accessible critical paths and a11y triggers: no slice recorded them, and s03 recorded accessibility scope as out of slice.
absence/assistive-technology-reach — A platform accessibility adapter in any stand binary, so no active assistive technology is given a tree
absence/keyboard-drag-and-drop — A keyboard or assistive-technology route to drag and drop; a drag reaches a document only as OS pointer drop input

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
