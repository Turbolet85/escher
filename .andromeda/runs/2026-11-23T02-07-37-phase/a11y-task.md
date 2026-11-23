@task_prose part=chunk
Makes what an app paints from Rust through a custom widget appear in a headless harness shot and in the driver's screenshot verb, in every build selection the project runs (harness, driver and the stand session binary alike).
First measures on the stand: boots a canvas, takes and reads a shot in both layout modes under each build selection, before any fix.
Each check it adds is run red first (before its fix or with the fix reverted) and the red reading is kept beside the green.
Collects every operator fork (which element hosts a canvas, app-painted pixels leaving through the screenshot answer, whether the stand gains a canvas page or task) into one forks file, answering none by a default.
Re-reads the screenshot's contract and the stand's served tasks' readings, which stand unless a ruling moves one.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait an app paints through, recording a scene from a render context, styles, size and scale
@task_file n=2 path=packages/blitz-paint/src/lib.rs
what: the paint crate's custom widget scene building, behind its custom-widget feature
@task_file n=3 path=packages/blitz-paint/src/render.rs
what: the paint pass that appends a custom widget's recorded scene
@task_file n=4 path=packages/blitz-paint/Cargo.toml
what: the paint crate's feature list, among them the custom-widget feature that decides whether a widget is painted per build selection
@task_file n=5 path=packages/blitz-test-harness/src/screenshot.rs
what: the harness screenshot and element-rect screenshot over the CPU renderer, which the driver's screenshot verb rides
@task_file n=6 path=packages/dioxus-native-dom/src/mutation_writer.rs
what: the Dioxus mutation writer that sets a custom widget from an object element's data attribute, and the canvas element data nothing reads
@task_file n=7 path=.andromeda/cache/chunks/2026-11-23-canvas-in-a-headless-shot/evidence/founder-forks.md mark=edits
what: the chunk's one file of operator forks, each with what crosses, the rule it meets, options with measured cost and dependences
@task_item n=1 part=needs
The screenshot changes nothing: taking a shot of a painted canvas runs no step and changes no document, focus or tree state.
@task_item n=2 part=needs
The screenshot's contract stands: an 800 by 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable before anything is painted, with a canvas now painted into it.
@task_item n=3 part=needs
A password input is still painted masked in every shot; the password text is never revealed by any new paint path.
@task_item n=4 part=needs
Pixels as a content class: option labels of an open list leave only as pixels by a ratified widening; a canvas's app-painted content leaving through the screenshot answer is a widening of its own, a fork for the operator, not a default.
@task_item n=5 part=needs
The driver's screenshot verb, its element shot and its open-list row shot keep their answers; no new verb, argument, carrier, cause, refusal or session error unless a fork's ruling adds one.
@task_item n=6 part=needs
The build feature change that turns the custom-widget paint on in every selection must not pull a platform accessibility adapter into any stand binary nor change which accessibility features are on by default.
@task_item n=7 part=needs
The session library keeps accessibility enabled and the snapshot readers stay behind the accessibility feature in every build selection after the feature change.
@task_item n=8 part=needs
The stand's served tasks keep their snapshot, lint and actions readings; the stand reads no lint finding at boot and its contrast assertions hold, re-recorded only if a ruling moves one, never loosened.
@task_item n=9 part=needs
If the stand gains a canvas page or task (an operator fork), its readings join the stand's expectation tables, contrast, focus order and lint checks like every served task.
@task_item n=10 part=needs
Contrast is left unread past non-colour paint: a canvas's painted scene is not read as a text colour pair by the lint or the harness contrast reader.
@task_item n=11 part=needs
The adapter tree equals the harness tree and the snapshot reads the accessibility tree: a canvas host element enters the tree by the same role and name rules as any element, with no canvas-specific tree reading added.
@task_item n=12 part=needs
Every new check runs in both layout modes (incremental and non-incremental) and is shown red first with its red reading kept in evidence.
@task_item n=13 part=needs
Every build, test and CI cargo leg passes the locked flag; no new crate or dependency beyond a taken fork; the accessibility CI leg and the gate it holds still pass.
@task_item n=14 part=needs
No content in a log field and no new log field or span: a canvas's painted content never reaches a log line.
@task_standing n=1
convention: each new accessibility or stand assertion is a Rust test over the harness and its AccessKit tree, run in both layout modes
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand check is repeated three rounds and runs in the accessibility test leg that gates CI
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Keyboard navigation contracts and decisions (Tab order, activation keys, range input, select open and closed list keys, Cells grid and CRUD list keys): the chunk adds no key handling.
@task_item n=2 part=not_needed
ARIA property reads, ARIA states and role mapping rows (aria-checked, aria-pressed, aria-expanded, switch role, HTML-AAM mapping): no ARIA attribute is read or written anew.
@task_item n=3 part=not_needed
Accessible name rows (aria-label, aria-labelledby, label element, precedence): no naming rule changes.
@task_item n=4 part=not_needed
Hidden and inert content rows (aria-hidden, display none, visibility hidden, inert hit-testing): no hiding or inertness rule changes.
@task_item n=5 part=not_needed
Select and multiple-select tree, snapshot bounds, value and highlighted-row rows, and form event values: no select behaviour changes.
@task_item n=6 part=not_needed
Snapshot diff, snapshot text form, actionable nodes, handler coverage and UI graph state registration: no new snapshot field or graph state.
@task_item n=7 part=not_needed
Cognitive accessibility: invalid state beyond colour or class, shared tab order table.
@task_item n=8 part=not_needed
Focus rules: focus-visible, focus fixup, removing the focused node, click focusing buttons and labels, IME enablement.
@task_item n=9 part=not_needed
Scrollbar thumb contrast stroke, near-white thumb state change, hidpi or zoom reshaping, colour scheme as device change: no scrollbar, zoom or scheme change.
