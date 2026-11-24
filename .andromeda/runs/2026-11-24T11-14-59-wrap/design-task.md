@task_prose part=session
The session merged upstream Blitz main at 29a439e05 as a two-parent merge: six upstream commits taken as upstream wrote them, with two adaptations by hand, and the 14 left-out commits named with their reasons.
It added OS drag-and-drop to the engine: the dragenter, dragover, dragleave and drop events, a DataTransfer model, a winit drop source in the shell, a Dioxus drag-data converter with a dropped-file reader, and two windowed root examples, dnd and file_drop.
It changed layout and paint: Taffy went from the 0.15 git pin to 0.16.0 from crates.io (calc values now pass as a CalcHandle). A br after text now joins the text's anonymous flex item. One-axis overflow clip now clips only the clipped axis. preventDefault is now ignored on an event that cannot be cancelled.
It retired the workspace's one git dependency and the Taffy exception. It added the drag_and_drop and flex_br integration test targets and extended pointer_leave.
dioxus-native-dom line coverage fell from 91.14 % to 88.59 % against an unchanged floor of 83 %. The next sync's merge base is now 29a439e05. CI was green on three OSes.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest: Taffy taken at 0.16.0 from crates.io and the git pin gone; the root package's dioxus-native dev features gain net and html
@task_file n=2 path=Cargo.lock mark=merge
what: the lock file: Taffy 0.16.0 from crates.io; the lock now holds no git source
@task_file n=3 path=examples/dnd.rs mark=merge
what: a new windowed root example showing drag areas that print drag events to stdout
@task_file n=4 path=examples/file_drop.rs mark=merge
what: a new windowed root example with a drop zone that lists dropped files and data with previews, styled by its own CSS
@task_file n=5 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's new OS drop source: it turns a winit drag into the engine's drag events and maps the drop operation; file URIs only
@task_file n=6 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the new DataTransfer model: drag operations, effectAllowed and dropEffect, items of strings and files
@task_file n=7 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: a new integration test target: drag events follow the node under the drag, a drop effect must be allowed by the source, a widget accepts a drop, a sub-document takes a drag
@task_file n=8 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: a new integration test target: a br between text breaks the line in flex and grid containers; a trailing br or a br between flex items adds no line
@task_file n=9 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event converter's data types: gains the drag data (DataTransfer, files, modifiers) and the dropped-file reader, which has no path restriction; the conflict was in its imports
@task_file n=10 path=packages/blitz-dom/src/layout_reason.rs
what: the layout-reason reader of the engine: its one calc-value call adapted to Taffy's CalcHandle
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document: gains the current drag target node
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the event driver: dispatches the drag events to the node under the drag; ignores preventDefault on an event that cannot be cancelled
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine's event handling: forwards a drag into a sub-document over its host
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout box construction: a br after text joins the text's anonymous flex item
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the layout engine glue to Taffy, moved to Taffy 0.16
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: layout of replaced elements, moved to Taffy 0.16
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout, moved to Taffy 0.16
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget boundary: a widget accepts a drop only by setting the dragover event's dropEffect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer: one-axis overflow clip now clips only the clipped axis
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: the shell crate's manifest: gains the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: the shell's application event loop: wires in the drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: the shell crate root: declares the drop source module
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell's window: passes OS drag positions, drops and leaves to the document
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: the engine's UI and DOM event types: gain the drag events and their names dragenter, dragover, dragleave, drop
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document's event converter: gains the drag arm
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy converter: calc values resolved through Taffy's CalcHandle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test list: gains the drag_and_drop and flex_br modules
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the pointer-leave integration test target: gains a test that preventDefault does not cancel pointerleave
@task_item n=1 part=touches
Depth: the hit-testing-stops-at-paint-clip rows (both twins). Paint now clips only the clipped axis for a one-axis overflow clip, so a row that says hit-testing stops where paint clips now meets a one-axis clip. Check that the row still holds and whether it needs the one-axis case.
@task_item n=2 part=touches
Depth: stacking-context and auto z-index rows over paint and hit order. The paint renderer changed its clip shape. Check that grouping and paint order are unaffected.
@task_item n=3 part=touches
Surface desktop-native: the shell provider contracts, window drag above all. The shell now also takes OS drag-and-drop of files and data, which is a different thing from dragging the window. Check that no row's word on drag clashes, and whether the window-drag row should mark the difference.
@task_item n=4 part=touches
Surface desktop-native: the select-list-events contracts (open, close, choose, close on change) and the single-select open gestures. The event driver now ignores preventDefault on an event that cannot be cancelled, so a row that relies on cancelling a change or a pointer event is touched.
@task_item n=5 part=touches
Motion: the driver verbs and virtual clock contracts (click, press, type, advance) and the actions-decided-by-the-verbs'-checks decision. They dispatch through the event driver, which gained drag dispatch and the non-cancelable rule. No verb drags, so check that none needs to.
@task_item n=6 part=touches
Motion: the shown-scrollbar-takes-thumb-drag decision. It is a pointer drag of a scrollbar thumb, not the new drag-and-drop events. Check that its wording does not now read as the DOM drag events.
@task_item n=7 part=touches
Typography and layout: text-line rows. A br after text now joins the text's anonymous flex item, so in flex and grid containers it breaks the line, and a trailing br adds no line. Check rows on how text lays out.
@task_item n=8 part=touches
Spacing: the table cellspacing, cellpadding and valign hints decision and the select-list-placed-in-viewport decision. Table and box layout moved to Taffy 0.16, so check that the hints and the placement still read true.
@task_item n=9 part=touches
Surface desktop-native: the logical-viewport-from-scale decision and the JS window size contracts. Layout now runs on Taffy 0.16. Check that no row names the old Taffy version or its git pin.
@task_item n=10 part=touches
Surface desktop-native: the dioxus-native configuration contracts (base color, alpha mode, font context). The root package's dioxus-native dev features gained net and html. Check whether a configuration row names the feature set.
@task_item n=11 part=touches
Surface web-spa: the WASM canvas overview. The shell gained a winit drop source and the url dependency. Check whether the drop source builds or applies on wasm32, or whether the overview needs to say that drop is native only.
@task_item n=12 part=touches
Iconography: the cursor resolution decision and the shell provider set_cursor contract. A drag over the window may set a drop cursor through the shell. Check that the cursor resolution order still covers it.
@task_item n=13 part=touches
Color palette: the engine-colour rows and the absence of colour tokens in the engine and integration crates. The new file_drop example brings its own CSS colours, which is example code and not engine code. Check that the absence still holds as worded.
@task_fact n=1 slug=depth-strategy
fact: For an overflow clip on one axis only, paint clips that axis and leaves the other unclipped (ratified on the founder's answer C1 A).
@task_fact n=2 slug=surface
fact: The desktop shell accepts OS drag-and-drop of files and data as dragenter, dragover, dragleave and drop events on the node under the drag. A drop lands only where the dropEffect is allowed by the source's effectAllowed. A custom widget accepts a drop only by setting the dragover event's dropEffect.
@task_fact n=3 slug=surface
fact: An event that cannot be cancelled ignores preventDefault. A handler's preventDefault does not cancel pointerleave.
@task_fact n=4 slug=typography
fact: A br after text joins the text's anonymous flex item, so it breaks the line inside flex and grid containers. A trailing br adds no line, and neither does a br between flex items.
@task_item n=1 part=untouched
Brand identity, border radius, anti-patterns, self-validation protocol and the design decisions log: none hold rows, and the session brings nothing to them.
@task_item n=2 part=untouched
escher-session's commands, exit statuses and exit codes, stdout and stderr shapes, mcp role results, the session error remedy and the tasks contracts: no verb, answer shape, status or task changed.
@task_item n=3 part=untouched
The actions line, the UI graph line and the coverage line with its check files: the line coverage that moved is a crate's test coverage, not the coverage verb.
@task_item n=4 part=untouched
The WPT runner's output and result colours: the runner is not reached.
@task_item n=5 part=untouched
Font and type rows (font element hints, lang locale, generic and unavailable family fallback, system and bundled fonts, document.fonts, screenshot fonts-unavailable): font selection and the font context did not change.
@task_item n=6 part=untouched
Motion's virtual clock, settle, load wait, smooth scroll, into-view scroll and overlay scrollbar fade timings: no timing, animation or scroll behaviour changed.
@task_item n=7 part=untouched
The select list as an engine overlay, its colours, its chosen-row pair and its snapshot and accessibility contracts: how the list is drawn and read did not change.
