@task_prose part=session
The session merged upstream DioxusLabs/blitz main at 29a439e05 (six commits) as a two-parent merge, resolved one conflict, and made two adaptations by hand.
It added drag and drop: an OS drop source in the shell, a DataTransfer model and drag event types (dragenter, dragover, dragleave, drop) in the event traits, drag dispatch in the engine's event driver, and a Dioxus drag-data and dropped-file reader.
It also took a br after text joining the text's anonymous flex item, one-axis overflow clip clipping only its axis, and preventDefault ignored on a non-cancelable event.
It bumped Taffy 0.15 from a git pin to 0.16.0 from crates.io, retiring the workspace's last git dependency; Dioxus stays held at 0.7.3; the lock moved by that one package alone.
Measured: every CI leg green, the a11y leg green, the adapter probe on the moved lock reads 0 accesskit_xplat or accesskit_winit in the stand binary's tree, stand checks green 3 of 3 rounds, CI green on three OSes.
Two root examples, dnd and file_drop, and two engine test targets, drag_and_drop and flex_br, are new; the next sync's merge base is 29a439e05.
@task_file n=1 path=examples/dnd.rs mark=merge
what: a windowed root example showing drag and drop between areas of a Dioxus app
@task_file n=2 path=examples/file_drop.rs mark=merge
what: a windowed root example listing dropped files and data with text and srcdoc previews
@task_file n=3 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's OS drop source, turning winit drag and drop into the engine's drag events
@task_file n=4 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of dragged strings and files, drop effects and allowed effects
@task_file n=5 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: the engine's drag and drop integration tests over the event handling
@task_file n=6 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: the engine's layout tests for a br between text in flex and grid containers
@task_file n=7 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event converter's data types, carrying engine events to Dioxus handlers; now its drag data and dropped-file reader
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs
what: the engine's layout reason reading, its calc value call adapted to the new Taffy handle
@task_file n=9 path=Cargo.toml mark=merge
what: the workspace manifest: dependency versions, Taffy and the held Dioxus version, root examples' dev features
@task_file n=10 path=Cargo.lock mark=merge
what: the workspace lock, the resolved dependency set the adapter probe reads
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document, now holding the current drag target node
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine's event driver: dispatch order, default actions and preventDefault, now drag dispatch
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine's event handling entry, now forwarding a drag into a sub-document
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout tree construction, anonymous flex items around text and br
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout entry over Taffy
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's layout of replaced elements
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: the engine's table layout
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget contract, a widget accepting a drop by setting the dragover drop effect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: the engine's paint, its overflow clip now per axis
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: the shell crate's manifest, now taking the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: the shell application's window event loop, wiring the drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: the shell crate's module list
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell window, carrying OS drag position and drop into the document
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: the engine's event types: UiEvent and the DOM event data, now with drag variants
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document, its converter from engine events to Dioxus events, now a drag arm
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, its calc value resolver now taking a Taffy calc handle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: the engine tests' all-in-one target, its module list
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the engine's pointer leave tests, now a non-cancelable pointerleave whose preventDefault is ignored
@task_item n=1 part=touches
The a11y CI leg and the adapter check: the stand binary's tree re-read on the moved lock still holds no accesskit_xplat or accesskit_winit (0); the rows that state no platform adapter on the stand binaries and the platform adapter choice are re-confirmed, not changed.
@task_item n=2 part=touches
The dependency set behind accessibility: Taffy bumped to 0.16.0 from crates.io; no AccessKit or Dioxus package moved and Dioxus stays held at 0.7.3, so a row that names the Dioxus accessibility feature or the AccessKit version is re-confirmed.
@task_item n=3 part=touches
Accessibility tier 0 and the assistive technology reach absence: drag and drop is a new input capability with no keyboard or assistive-technology route and no a11y check (a11y n/a); a row stating what reach the engine gives may need it named.
@task_item n=4 part=touches
The engine's event types and the event driver gained drag events (dragenter, dragover, dragleave, drop) and preventDefault is now ignored on a non-cancelable event; rows on keyboard activation handler order, default actions, the click walk and AccessKit requests reaching one event handler rule over that driver.
@task_item n=5 part=touches
The Dioxus event converter and its data types (the one conflict was here) gained drag data; rows on the Dioxus document reached-handler record, listeners read from the vdom walk, handler coverage and the actionable-node rule rule over the handlers a node carries, and a drop listener is a new kind of handler.
@task_item n=6 part=touches
Layout: a br after text now joins the text's anonymous flex item (one line break inside flex and grid containers); rows on snapshot bounds of rows and options, element shots and stated Tab order read positions the layout produces.
@task_item n=7 part=touches
Paint: one-axis overflow clip clips only the clipped axis; rows on screenshots, contrast unread past non-colour paint, and Circle Drawer's canvas digest read painted pixels (stand canvas and screenshot checks stayed green).
@task_item n=8 part=touches
Stand checks repeated three rounds: stand-repeat read 3 of 3 rounds green on the merged tree; the three-round value is unchanged.
@task_item n=9 part=touches
The custom widget contract now states a widget accepts a drop only by setting the dragover drop effect; a row on Circle Drawer's drawing area as one named canvas node rests on the custom widget surface.
@task_item n=10 part=touches
The upstream sync merge base moved 165d698c3 to 29a439e05; a row of a11y-plan holding the sync base would move with it.
@task_fact n=1 slug=1-a11y-scope-summary
fact: Drag and drop reaches a document only as OS pointer drop input through the shell; no keyboard or assistive-technology route drives a drag or a drop, and no accessibility check covers it.
@task_fact n=2 slug=1-a11y-scope-summary
fact: The root examples dnd and file_drop are windowed and their accessibility is unchecked (unrunnable here); the build leg only compiles them.
@task_fact n=3 slug=9-ci-integration
fact: The adapter probe is re-read on every lock move of an upstream sync: the stand binary's tree must still hold no accesskit_xplat or accesskit_winit after the take.
@task_item n=1 part=untouched
ARIA role mapping, ARIA property and state reads, and the toggle and checked readings: no role, attribute read or state rule moved.
@task_item n=2 part=untouched
Accessible name precedence, label binding and text runs naming their parent: no naming rule moved.
@task_item n=3 part=untouched
Password masking and the never-revealed password rule: nothing reads or writes a password value.
@task_item n=4 part=untouched
WCAG contrast thresholds, the contrast ratio helper and the lint's contrast finding fields: no threshold, formula or finding field moved.
@task_item n=5 part=untouched
Cognitive accessibility: the invalid state beyond colour and class and the shared tab order table are unchanged.
@task_item n=6 part=untouched
UI graph state registration, the snapshot text form, the snapshot diff, the MCP result shape and served node id separators: no driver or harness wire shape changed.
@task_item n=7 part=untouched
Hidden and inert content exclusion from the accessibility tree: no hidden, inert or display rule moved.
