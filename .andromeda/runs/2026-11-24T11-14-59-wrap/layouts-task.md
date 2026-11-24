@task_prose part=session
The session merged upstream blitz main at 29a439e05 (six commits) into the workspace as a two-parent merge, with one import conflict resolved and one caller adapted; the next sync's merge base is now 29a439e05.
It added OS drag-and-drop to the desktop engine: a winit drop source in the shell, a DataTransfer model, dragenter, dragover, dragleave and drop events, a drag target in the document, and a Dioxus dropped-file reader with no path restriction.
It brought upstream layout and paint fixes: a br after text joins the text's anonymous flex item, one-axis overflow clip clips only the clipped axis in paint, and preventDefault is ignored on a non-cancelable event.
It moved Taffy from a git pin 0.15.0 to 0.16.0 from crates.io, retiring the workspace's one git dependency; Dioxus stays held at 0.7.3.
It added two windowed root examples, dnd and file_drop, and two engine test targets, drag_and_drop and flex_br; no stand control, stand id, CLI verb or gate leg changed.
dioxus-native-dom line coverage fell 91.14 to 88.59 percent, floor 83 unchanged; CI green on three OSes, Circle Drawer's canvas digest green on macOS and Windows.
@task_file n=1 path=examples/dnd.rs mark=merge
what: a new windowed root example of drag-and-drop between areas of a desktop window, printing drag events to stdout
@task_file n=2 path=examples/file_drop.rs mark=merge
what: a new windowed root example, a drop zone listing dropped files and data with previews; its image and HTML-file previews by file URL are refused
@task_file n=3 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the desktop shell's OS drop source, turning winit drag, drop and leave into the engine's drag events; accepts only file URIs as paths
@task_file n=4 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the engine's DataTransfer model, the dragged strings and files with their allowed and chosen drop effect
@task_file n=5 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: the engine integration test of drag events following the node under the drag, a widget accepting a drop, and a sub-document's drag
@task_file n=6 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: the engine layout test that a br between text breaks the line in a flex or grid container
@task_file n=7 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the converter from the engine's events to Dioxus events, the stand app's input path; gains the drag data and the dropped-file reader
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason reader behind the stand's layout readings, calling the calc resolver through Taffy's CalcHandle
@task_file n=9 path=Cargo.toml mark=merge
what: the workspace manifest; Taffy 0.16.0 from crates.io in place of the git pin, root examples' dev features
@task_file n=10 path=Cargo.lock mark=merge
what: the workspace lock; Taffy 0.16.0, no git source left
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine DOM document, now holding the current drag target node
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine event driver dispatching pointer, keyboard and now drag events to nodes, with the default-action and preventDefault rule
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine event handling entry, forwarding a drag into a sub-document over its host
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the layout tree construction, where a br after text now joins the text's anonymous flex item
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine layout module over Taffy, adapted to Taffy 0.16
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the layout of replaced elements such as images and canvases, adapted to Taffy 0.16
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: the table layout, adapted to Taffy 0.16
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget hook, such as the canvas widget; a widget accepts a drop only by setting dragover's dropEffect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer, its clipping boxes; one-axis overflow clip now clips only that axis
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: the desktop shell's manifest, gaining the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: the desktop shell application event loop, wired to the drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: the desktop shell crate root, declaring the drop source module
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: the desktop shell window, the native window behind the stand and its window chrome, now handling drag position and drop
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: the engine's UI event and DOM event types, gaining the drag variants and the drag event
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus native document bridging the stand app to the engine, gaining the drag arm of its event converter
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy converter, its calc resolver taking Taffy's CalcHandle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: the engine integration test aggregate, gaining the two new test modules
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the engine test of pointerleave, extended with preventDefault not cancelling a non-cancelable pointerleave
@task_item n=1 part=touches
decision/visible-region-through-clipping-boxes: the paint renderer's clipping boxes changed; one-axis overflow clip now clips only the clipped axis, and the one-axis clip readings are kept as they were.
@task_item n=2 part=touches
The ShellProvider window chrome contracts: the desktop shell window and application were merged with the OS drop source wired in; drag_window is a window move and none of the chrome controls changed, but the window they rule over did.
@task_item n=3 part=touches
decision/scroll-into-view-moves-every-scroller and decision/open-list-rows-own-bounds: layout construction, table and replaced-element layout moved to Taffy 0.16, and a br after text now joins the text's flex item; the stand's readings stayed green.
@task_item n=4 part=touches
decision/headless-stand-pinned-viewport and decision/held-instance-reads-as-fresh-boot: the stand's boot and readings run on the merged layout and paint engine; stand-repeat green three of three rounds.
@task_item n=5 part=touches
decision/circle-drawer-circles-painted-on-canvas: the paint renderer and the custom widget hook changed; the canvas widget now may accept a drop only by setting dropEffect, and the canvas digest read green on macOS and Windows, one of three watch runs.
@task_item n=6 part=touches
decision/fixed-document-skeleton-main-mount and absence/arbitrary-index-html-templates: the engine document changed, now holding a drag target; no template or mount changed.
@task_item n=7 part=touches
decision/crud-list-keyboard-region, decision/every-stand-control-keyed and decision/order-page-built-to-cause: the event driver and the Dioxus event converter changed; preventDefault is ignored on a non-cancelable event, and keyboard types stay Dioxus's own.
@task_item n=8 part=touches
contract/escher-session-actions/answer: actions dispatch through the event driver, which now ignores preventDefault on a non-cancelable event.
@task_item n=9 part=touches
contract/escher-session-layout/answer, contract/escher-session-snapshot/answer and decision/stand-lints-clean-at-boot: their readings come through the layout reason reader and the layout engine, both moved to Taffy 0.16.
@task_item n=10 part=touches
contract/escher-session-screenshot/answer and contract/escher-session-mcp/screenshot-result: screenshots are painted by the renderer whose one-axis overflow clip changed.
@task_item n=11 part=touches
contract/paint-bench-cli/arguments and contract/paint-bench-cli/backend: the paint renderer the bench drives was merged; the command line itself is unchanged.
@task_item n=12 part=touches
Surface cli overview: two new root examples, dnd and file_drop, are windowed desktop examples, not command-line surfaces; they take no arguments and own no exit-code contract.
@task_item n=13 part=touches
Names taken: root examples dnd and file_drop; engine test targets drag_and_drop and flex_br; engine modules datatransfer and dnd; DOM event names dragenter, dragover, dragleave, drop.
@task_fact n=1 slug=surface-desktop-native
fact: The desktop window accepts OS drag-and-drop of text and files as dragenter, dragover, dragleave and drop events on the node under the drag; a custom widget accepts a drop only by setting dragover's dropEffect.
@task_fact n=2 slug=surface-desktop-native
fact: The root examples dnd and file_drop are windowed desktop examples, not command-line surfaces; file_drop refuses its image and HTML-file previews by file URL, while its text and srcdoc previews stand.
@task_item n=1 part=untouched
Stand stable element ids: no stand id was added, renamed or removed.
@task_item n=2 part=untouched
The cold-agent and agent-run pipes, their usage, exit codes, record directory and Windows pass-through: no script or CLI changed.
@task_item n=3 part=untouched
escher-session command form, exit codes, refusals, recording, coverage role, help, graph, served tasks and host log: no verb, refusal, task or log name changed.
@task_item n=4 part=untouched
The MCP server's stdio transport, protocol revisions, tools list, result bound and end: no port, socket, listener or env var was added.
@task_item n=5 part=untouched
decision/load-page-only-network, decision/no-caller-input-echoed and decision/mcp-server-local-stdio: no network provider or echo path was added.
@task_item n=6 part=untouched
multi_doc command line: its arguments, env vars and stdout are unchanged.
@task_item n=7 part=untouched
decision/stand-accent-colour, decision/stand-a11y-by-attributes-alone, decision/stand-ids-as-attributes-alone and decision/flight-type-select-boot-reading: no stand page, colour, attribute or option changed.
@task_item n=8 part=untouched
Surfaces mobile-native and web-spa and the Decisions Log: nothing measured or ruled there.
