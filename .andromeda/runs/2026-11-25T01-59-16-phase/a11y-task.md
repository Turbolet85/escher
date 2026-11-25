@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas widget (an element the app paints from Rust through the custom widget): each node an author key, a role from a closed set, a non-empty name and a region inside the canvas's content box.
A declaration breaking a rule is not taken and nothing panics; each taken node gets the reserved id form owner, the /// separator, then the key; two nodes of one canvas with one key are refused.
The declared nodes are readable in process, validated and with their ids; no accessibility tree node, snapshot, diff, click, UI graph, actions or keyboard reach is added here.
The declaration's form, the names' crossing onto a carrier, the role set, where the declaration lives and the region's frame are founder's forks written to evidence, with the phase stopping at P4 with no default taken.
Two carried items ride the wrap: the security summary's drag-and-drop gaps and the test plan's one-host screenshot clause; neither adds a code step.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine custom widget trait an app paints a canvas through, with its commented accessibility tree hook for multiple nodes per widget
@task_file n=2 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id and the reserved virtual node id form, its separator /// and the function that spells a virtual node's id
@task_item n=1 part=needs
The served node id's reserved separator /// and the virtual node id form owner///key: a taken node's id is spelled through it, and a key holding / is refused so the id stays one node's.
@task_item n=2 part=needs
A declared node's role drawn from a closed set; its relation to the existing role mapping (role attribute, element, else Unknown) and to the driver schema defining no role or name set is a fork the plan writes, not a choice it takes.
@task_item n=3 part=needs
A declared node's name is non-empty and is author-written text; interactive nodes carry a name.
@task_item n=4 part=needs
The names' crossing: author text on a ratified carrier is a widening, so a declared name or key reaches no snapshot, MCP result, UI graph or log field in this chunk; whether it ever does is the founder's fork.
@task_item n=5 part=needs
A declaration breaking a rule (empty key, key with /, role outside the set, empty name, region outside the content box, duplicate key in one canvas) is not taken, stands for no node, and panics on no path.
@task_item n=6 part=needs
The region is held in the canvas's content-box frame, the frame the custom widget paints in and hears pointer events in.
@task_item n=7 part=needs
The in-process read of what a canvas declares now, validated and with ids, adds no driver verb, cause, carrier, log field or span and no reading on the socket, command line, MCP or a kept check.
@task_item n=8 part=needs
The accessibility tree built through the document's own method and its refresh on change stay untouched: virtual nodes enter no AccessKit tree here, that belongs to the later entry.
@task_item n=9 part=needs
Circle Drawer's drawing area stays one named canvas node with pixels only; its circles as virtual nodes belong to a later entry.
@task_item n=10 part=needs
Accessibility assertions are Rust tests: each new test of a declaration's rules and refusals is such a test, a control entering the plan with its red reading.
@task_item n=11 part=needs
The test plan's no-screenshot-step row and its one-host clause, carried to this chunk's wrap report, against canvas-only shot digests held equal across three hosts.
@task_item n=12 part=needs
The drag-and-drop gaps (OS drag-and-drop as untrusted input with no keyboard route, the dropped-file reader with no path restriction) carried to this chunk's wrap leaf pass, never written by hand.
@task_standing n=1
convention: each new accessibility assertion is a Rust test over AccessKit or the document, run in the accessibility test leg
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: expected roles and names for stand checks are stated once in shared tables, never repeated per test
home: 7-screen-reader-support · screen-reader-support
@task_item n=1 part=not_needed
Keyboard navigation rows (Tab order, focus fixup, activation keys, range inputs, select lists, Cells grid, CRUD list): the canvas keyboard path is a later entry.
@task_item n=2 part=not_needed
Visual design verification rows (contrast thresholds, lint contrast findings, scrollbar thumbs, select list colours, scheme and zoom): the chunk paints nothing new and adds no lint finding.
@task_item n=3 part=not_needed
ARIA property and state reads (aria-checked, aria-pressed, aria-expanded, aria-labelledby and the rest) and hidden or inert content rules: virtual nodes carry no ARIA attributes and enter no tree here.
@task_item n=4 part=not_needed
Snapshot field contracts (checked, enabled, value, password value, select rows and bounds, file input value) and snapshot text tokens: a virtual node reaches no snapshot in this chunk.
@task_item n=5 part=not_needed
Driver click refusals, actions answer causes and handler coverage: click and the covered, off-screen, disabled and stale refusals belong to later entries.
@task_item n=6 part=not_needed
Select and optgroup behaviour, form event values and UI graph state registration: no form control or graph state changes.
@task_item n=7 part=not_needed
Platform adapter rows (accesskit_xplat, no adapter on the stand, adapter creation order, adapter tree equals harness tree): no adapter changes.
@task_item n=8 part=not_needed
Cognitive accessibility rows (invalid state stated beyond colour or class): no invalid state is touched.
