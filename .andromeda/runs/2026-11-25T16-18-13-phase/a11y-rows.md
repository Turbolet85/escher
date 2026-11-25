# a11y-plan · the rows that bind 2026-11-25-circle-drawer-virtual-nodes — picked 58 of 243 (41 by every draw) · rule any · pin 3938

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/capability-statement-reach-by-hand · Accessibility reach edited by hand — A change to the accessibility reach is written by hand into the capability statement's Accessibility reach section. · why: The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.
decision/accessibility-feature-default-on · Accessibility as a default-on feature — The accessibility tree is built only under the accessibility cargo feature, which is on by default in blitz-dom, blitz-shell and blitz and forwarded down that chain; custom widgets require it.
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — AccessKit is the technology the accessibility tree is built in, taken as a workspace dependency.
decision/stand-builds-no-platform-adapter · No platform adapter on the stand binaries — No stand binary builds a platform adapter: neither accesskit_xplat nor accesskit_winit is in the stand's dependency graph, so no assistive technology is given a tree by the stand. · why: Ratified as deferred: the adapter and its windowed witness at an active assistive technology are carried to a later version.

## 2-a11y-strategy · ## 2. A11y Strategy · twin a11y-strategy
decision/accessibility-tree-through-document-method · Accessibility tree built through the document's own method — The accessibility tree is built from the document on the initial tree request and rebuilt in a redraw after style resolve when the changed set is non-empty, both paths going through the document trait's accessibility-tree method. · why: so a wrapper's override, such as DioxusDocument's stable ids, reaches the tree
contract/aria-property-reads/aria-disabled · ARIA name and property reads · aria-disabled — Only the value true of aria-disabled sets the node disabled.

## a11y-strategy · ## 2. A11y Strategy · twin 2-a11y-strategy
decision/a11y-tree-refresh-on-change · Accessibility tree refreshed on change — The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree. · why: Ratified, one item with the engine's changed-set contract. · guard: stand_accessibility_ids
decision/a11y-tree-through-document-trait · Accessibility tree through the document trait — Both the initial build and the refresh of the platform accessibility tree go through the document trait's accessibility tree method. · why: So a wrapper's override, such as DioxusDocument's stable ids, reaches the platform tree.
decision/changed-set-marks · What the engine's changed set marks — The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too, and its highlight moving rows; node creation, hover, active, other scrolls and layout alone do not. · why: Ratified, one item with the accessibility tree's refresh on change.
decision/harness-settle-leaves-changed-set · Harness settle leaves the changed set — A harness settle neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads the set as what one step changed drains it first. · why: A settled step leaves the refresh and a later diff exactly what the step marked. · guard: stand_settle
decision/agent-readers-behind-accessibility · Snapshot readers behind the accessibility feature — The snapshot, its text and diff, the unkeyed-actionable check, the UI graph reader and the actionable-node reader stand behind dioxus-native-dom's accessibility feature; the signal registry and the reached-handler record are not gated.

## keyboard-navigation · ## 5. Keyboard Navigation · twin 5-keyboard-navigation
decision/inert-subtree-excluded · Inert subtree excluded — A truthy inert takes focusability, hits and the accessibility tree away from its subtree, and a framework-written inert false is removed. · why: ratified · guard: inert_subtree
decision/synthetic-click-at-node-centre · Synthetic click is a primary click at the centre — A synthetic click is a primary mouse click at the node's center. · why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
contract/dioxus-document/reached-handlers · Dioxus document reached-handler record · reached_handlers — The id and event of each node whose listeners held the event, resolved before the handlers run, first-reached, deduplicated; a dispatch its target or an ancestor stops, or a handler's panic unwinds, counts the innermost listener alone, a non-bubbling event its target alone; empty unless switched on.
contract/dioxus-document/record-reached · Dioxus document reached-handler record · record_reached — Switches on one reached record for the document, a second call keeping the one record; every other document pays one context lookup per dispatch.
contract/ui-graph-state/no-registered-state · UI graph state registration · no registered state — A page with no registered state reaches no registry, and its events dispatch with no registry recording.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
contract/accessibility-tree/hidden-and-inert-nodes · accessibility tree · hidden and inert nodes — Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
contract/hit-testing/inert · hit-testing · inert — A hit passes through an element carrying inert, and removing inert restores the tree and the hit.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/hidden-content-excluded-from-a11y-tree · Hidden content excluded from the accessibility tree — Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot. · except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
decision/inert-element-takes-no-hit · An inert element takes no hit — A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit. · why: The `inert` read was ratified. · guard: inert_subtree
decision/stand-lint-clean-at-boot · Stand reads no lint finding at boot — The stand at boot reads no visual lint finding of any kind on the four lean tasks, in both layout modes. · guard: the_stand_at_boot_reads_its_measured_findings
contract/driver-click-refusals/covered · Driver click refusals · covered — A driver click on an element under an open select list is refused `covered`; a virtual node reads `covered` only when an element lies over its region's centre.
decision/viewport-scheme-and-zoom-level · Viewport carries scheme and zoom, 1.0 unzoomed — The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed. · why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both

## 7-screen-reader-support · ## 7. Screen Reader Support · twin screen-reader-support
decision/adapter-tree-equals-harness-tree · Adapter tree equals harness tree — On the stand the tree the shell hands the adapter equals the harness's tree node for node, and every interactive-role node of the four lean tasks reads a non-empty name.
decision/snapshot-model-single-source · Snapshot model as single source — The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else; the canvas read, which reads one canvas element's draw commands, and the virtual-node read, which reads one canvas's declared nodes, are no readers of the snapshot model.
decision/snapshot-reads-accessibility-tree · Snapshot reads the accessibility tree — The snapshot takes each node's role, name, focus, selected, pressed, invalid and highlighted from the accessibility tree, never from the element.
decision/ui-graph-one-node-per-snapshot-node · UI graph mirrors the snapshot — The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too; names and closed words only.
contract/driver-actions/disabled · driver actions · disabled — The driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched.
contract/snapshot/enabled · snapshot · enabled — The snapshot's enabled field reads the presence of the element's disabled attribute, and on a virtual node the declaration's disabled flag, enabled false where declared disabled.
decision/actionable-key-rule · Actionable-key rule — Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it. · why: ratified · guard: unkeyed_actionable
decision/one-actionable-rule · One actionable rule — The unkeyed-actionable check and the actionable-nodes read share one crate-private actionable rule, focusable or an interactive role or the vdom walk's exact current listeners holding an event other than mounted, with no second role mapping; the data-dioxus-id mark serves event dispatch alone.
decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
contract/accessibility-tree/author-id · accessibility tree · author_id — In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
contract/snapshot/clickable · snapshot · clickable — The snapshot's clickable field says an element that is no disabled form control reacts to a click, read from the vdom walk's current click listeners and not the listener mark.

## screen-reader-support · ## 7. Screen Reader Support · twin 7-screen-reader-support
decision/actions-verb-changes-nothing · The actions verb changes nothing — Answering the actions verb moves no focus, snapshot or diff, and an open list stays open.
decision/adapter-tree-is-document-tree · The adapter reads the document's own tree — The tree the shell hands the platform adapter equals the document's accessibility tree node for node, with the same focus and root. · guard: stand_accessibility_ids
decision/circle-drawer-canvas-pixels-only · Circle Drawer's drawing area as one named canvas node — Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable; its circles and diameter labels reach no DOM, snapshot, accessibility tree or UI graph, leaving only as shot pixels, canvas verb draw commands and lint truncated boxes, no label's text among them. · guard: stand_circle_drawer_canvas
decision/interactive-nodes-named · Interactive nodes carry a name — Every interactive-role tree node of the lean stand tasks reads a non-empty accessible name and an HTML-AAM role, in both layout modes. · guard: stand_accessibility_ids
decision/keyed-wire-form · The keyed field on the carriers — The keyed reading on the actions answer is the actionable-key check's wire form, true where the id holds no slash.
decision/listeners-from-vdom-walk · Listeners read from the vdom walk — An element's current listeners are read from the vdom walk's dynamic attributes, never from the bridge's listener mark, so a listener removed on a re-render is not counted. · why: the bridge's listener attributes over-report: a removed listener leaves its event attribute and data-dioxus-id mark standing · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/readers-of-one-snapshot-model · One snapshot model, many readers — The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping; the canvas read and the declared-node read, each reading one canvas element, are no readers of the snapshot model.
decision/snapshot-reads-the-tree · Snapshot takes its readings from the tree — The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.
decision/ui-graph-carries-no-values · UI graph is read-only and carries no values — A UI graph read changes nothing and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text.
contract/actionable-nodes/listing · actionable nodes · listing — The list holds, in snapshot pre-order, the UI graph's nodes the actionable rule holds for or whose owner is set, each with id, owner and keyed.
contract/actions-answer/causes · the actions answer · causes — For every actionable node each verb taking an id is answered with the cause a check would refuse it with; a node resolving to no element reads stale.
contract/actions-answer/fields · the actions answer · fields — The answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
contract/driver-refusal/disabled · driver refusal causes · disabled — A click or a type is refused as disabled when the snapshot node reads not enabled, the first screen-level check, with nothing dispatched and focus as it was.
contract/handler-coverage/answer · handler coverage · answer — The answer lists the graph's handlers no check reached and holds ids, owner ids, event names and two counts, checks and held, only, no role or name.
contract/mcp-result/snapshot · MCP tool results · snapshot — An accepted snapshot result is the text form itself for a whole screen, and for a screen in parts the JSON line {text, part, parts}; an acting call's result is its JSON line with the diff.
contract/node-id/reserved-separator · served node ids · the separator /// — The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions answer and the layout answer, these three never carrying the node's declared name; no element's id holds ///.
contract/snapshot-diff/differs · the snapshot diff · differs — A changed node names the readings that differ from before the step, in node-key order.
contract/snapshot-diff/entries · the snapshot diff · entries — Two snapshots are compared by stable element id, entries carrying the later role and name; a text change is reported on the element whose name it changes, never as a text-run entry.
contract/snapshot-node/clickable · the snapshot node · clickable — clickable is true where the element's current listeners hold click and it is no disabled form control; the JSON node always carries it beside focused.
contract/snapshot-text/clickable · the snapshot text form · clickable — The token clickable is written after focused only when true.
contract/snapshot-text/state-tokens · the snapshot text form · state tokens — Each node is one line; selected or unselected, pressed or unpressed and invalid are written after value= and before focused.
decision/accessibility-feature-gate · Screen-reader integration behind the accessibility feature — Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
contract/snapshot-text/parts · the snapshot text form · parts — A screen over the text budget is read in parts of consecutive pre-order lines, never a split line, concatenating to the whole text with every node id on exactly one line.

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
