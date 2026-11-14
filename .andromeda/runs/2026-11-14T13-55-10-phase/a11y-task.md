@task_prose part=chunk
Builds one auto-exploration command over a held app: from boot it lists the actions the screen offers and drives them breadth-first, each path re-reached from a fresh session and the frontier ordered by path length, until a step budget is spent, then returns a closed report.
The verbs it drives come from the driver's own actions answer and the schema's verb table, never a hand list, and every step is a validated call through the one run path, with no second executor.
A refusal met on a step is reported by its closed cause and a panic inside a step by a closed word, the path kept; neither stops the exploration.
Each explored path is kept as an owner-only recorded check with no screenshot step that replays green; the check's step format states a version and can carry a verb with more than one target.
Proven on the stand's served tasks in both layout modes and on new fixtures: one that panics in a handler, one that holds a refusing control.
@task_item n=1 part=needs
The actions answer: exploration takes the actions each node offers from its fields and the verbs it lists, and drives only a verb the answer offers on a node and the schema's verb table holds.
@task_item n=2 part=needs
The actionable rule and the actionable-node listing: what exploration enumerates as drivable is the same set the driver's one actionable rule and actionable-key rule mark, read from the snapshot.
@task_item n=3 part=needs
The actions verb changes nothing: enumerating the available actions at each step must leave the document unchanged, so the frontier is read without side effects.
@task_item n=4 part=needs
Driver refusal causes, among them disabled and covered: a refusal exploration meets is reported by its closed cause from the actions answer's causes, not by free text, and does not stop the run.
@task_item n=5 part=needs
No screenshot in an explored path: a recorded check holds no screenshot step and exploration takes no pixel at rest; a screenshot is not one of the verbs it drives.
@task_item n=6 part=needs
The snapshot as single source: exploration's reads of the screen go through the one snapshot model over the accessibility tree, so the session it drives has accessibility enabled as the session library always does.
@task_item n=7 part=needs
Password text is never revealed and the UI graph carries no values: a recorded path and the closed report name nodes and verbs, never a field's value or text content.
@task_item n=8 part=needs
The keyed wire form and served node ids: a recorded step names its target by the node id the carriers use, with the reserved separator kept, so a replay re-reaches the same node.
@task_item n=9 part=needs
Navigation is a boundary fork held most conservatively: an action that would navigate the document (a link, a form submission to another URL) is not driven, and nothing new is loaded.
@task_item n=10 part=needs
A disabled form control stops the click walk and disabled-ness reads by attribute presence: exploration treats such a control as offering no drivable action or as a refusal by cause, never as a panic.
@task_item n=11 part=needs
Open select list rows and open-list state: a step that opens a single select's list leaves the list's rows as reachable targets read from the snapshot, with no reading of whether a list is open beyond what the snapshot holds.
@task_item n=12 part=needs
Recorded check step format, versioned: a kept check states its format version, and a step with more than one target is an addition to that format; the canvas direction adds no two-target verb production in this chunk.
@task_item n=13 part=needs
No panic on untrusted input: the step budget and the check directory are refused as typed values, and the check directory is a launcher-named directory refused when open to group or other.
@task_item n=14 part=needs
Logs carry no content: a span, if any, holds fixed words and counts only, judged by the sink's target allowlist and redaction of text, value, html and attrs.
@task_standing n=1
convention: Each new fixture's interactive controls carry an accessible name, and the stand's expected roles and names are stated once in the shared expectation tables.
home: screen-reader-support · 7-screen-reader-support
@task_standing n=2
convention: Each new accessibility-reading test is a Rust test over the accessibility tree and runs in the accessibility test leg of the slow tier, repeated three rounds on the stand.
home: 3-a11y-assertion-harness-contract · 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Range input contracts (its keys, pointerdown, click, value and input event): exploration drives verbs through the run path and changes no range behaviour.
@task_item n=2 part=not_needed
The seven_guis Cells grid and CRUD list keyboard contracts: exploration reaches those tasks only as served tasks it drives, it changes no grid or list key handling.
@task_item n=3 part=not_needed
ARIA property reads, ARIA state rows and the ARIA role mapping rows: no role, name or state mapping is added or changed.
@task_item n=4 part=not_needed
The platform adapter and accessibility tier decisions (AccessKit platform adapter, no adapter on stand binaries, accessibility reach at tier 0, browser accessibility off by default): exploration runs in process over the driver session.
@task_item n=5 part=not_needed
Cognitive accessibility rows on invalid state stated beyond colour or class: no invalid-state presentation is touched.
@task_item n=6 part=not_needed
Focus and Tab-order decisions (Tab and Shift+Tab, focus fixup, focusability recomputed, focus-visible matching): exploration changes no focus behaviour.
@task_item n=7 part=not_needed
Hidpi, zoom, colour scheme and select list colour or motion decisions: exploration changes no viewport, scheme or engine paint constant.
