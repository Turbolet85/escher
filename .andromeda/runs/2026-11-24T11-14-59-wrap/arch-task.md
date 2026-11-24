@task_prose part=session
The session synced upstream Blitz main 29a439e05 into the fork by a two-parent merge, taking six upstream commits as upstream wrote them, with one conflict resolved and two adaptations; the next sync's merge base moves from 165d698c3 to 29a439e05.
It brought OS drag-and-drop: a winit drop source in blitz-shell, a DataTransfer model and drag events in blitz-traits, drag dispatch and sub-document drag forwarding in blitz-dom, and a Dioxus drag data bridge whose dropped-file reader reads files through std::fs with no path restriction (the founder's answer D1 A).
It brought three engine behaviour changes on the founder's answers C1, B1, P1: one-axis overflow clip clips only the clipped axis in paint, a br after text joins the text's anonymous flex item, and preventDefault is ignored on a non-cancelable event.
Taffy moved from the git pin 0.15.0 to 0.16.0 from crates.io, retiring the workspace's one git dependency and the Taffy exception to the no-git-dependencies rule; stylo_taffy's calc resolver now takes taffy::CalcHandle.
Measured: dioxus-native-dom line coverage 91.14 to 88.59 percent against the unchanged 83 percent floor; the standing left-out set is 14 upstream commits, not the 15 stated; two new blitz-tests targets drag_and_drop and flex_br; two new windowed root examples dnd and file_drop.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event data bridge converting Blitz events to Dioxus event data, now with the drag data and the dropped-file reader
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason reader explaining why a node is placed and sized, its one call of the stylo_taffy calc resolver adapted to CalcHandle
@task_file n=3 path=examples/dnd.rs mark=merge
what: a new windowed root example showing drag-and-drop between areas
@task_file n=4 path=examples/file_drop.rs mark=merge
what: a new windowed root example listing dropped files and data with previews
@task_file n=5 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's winit drop source turning OS drags into drag UI events, accepting only file URIs for dropped files
@task_file n=6 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the shared DataTransfer model of drag data, items, files, effectAllowed and dropEffect
@task_file n=7 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: the integration test target for drag event dispatch, drop effects, widget drops and sub-document drags
@task_file n=8 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: the integration test target for a br between text in flex and grid containers
@task_file n=9 path=Cargo.toml mark=merge
what: the workspace manifest and its shared dependency list, Taffy now 0.16.0 from crates.io
@task_file n=10 path=Cargo.lock mark=merge
what: the workspace lock file, now holding no git source
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the BaseDocument core, gaining the drag target node
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the event driver dispatching UI events to DOM targets and default actions, gaining drag dispatch and the non-cancelable preventDefault rule
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the document's UI event handling and sub-document event forwarding, gaining drag forwarding
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout box construction including anonymous blocks and anonymous flex items for text
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the Taffy layout traversal and layout traits over the DOM
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: layout of replaced elements
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: HTML table layout over Taffy grid
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget trait and its event handling, a widget accepting a drop by setting dropEffect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer emitting a paint scene, including overflow clip shapes per axis
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: the blitz-shell crate manifest, gaining the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: the shell's winit application handler, wiring the drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: the blitz-shell crate root and its modules
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell's window holding a document, handling drag position
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: the shared UiEvent and DOM event types, gaining drag variants and BlitzDragEvent
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document wrapping BaseDocument and its event converter, gaining a drag arm
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the stylo_taffy conversion of Stylo styles to Taffy styles, its calc resolver taking CalcHandle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate module list run all, not a test target of its own
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the integration test target for pointerleave dispatch, extended with preventDefault not cancelling pointerleave
@task_item n=1 part=touches
decision/taffy-layout: Taffy is now 0.16.0 from crates.io, no longer the git pin 0.15.0 rev a3997d55; its exception wording is stale.
@task_item n=2 part=touches
decision/no-git-dependencies: its Taffy exception is retired; the lock now holds no git source, so the rule stands with no except.
@task_item n=3 part=touches
decision/upstream-sync-merge-base: the next sync's merge base moves from 165d698c3 to 29a439e05.
@task_item n=4 part=touches
decision/upstream-sync-by-two-parent-merge, decision/upstream-take-engine-changes and decision/upstream-behaviour-changes-ratified-per-take: this sync took six commits by a two-parent merge, each behaviour change ratified on the founder's answers T1, T2, D1, D2, C1, B1, P1; the standing left-out set counts 14 commits, each named with its reason.
@task_item n=5 part=touches
decision/one-axis-clip-readings-kept: re-stated on C1 A; paint now clips only the clipped axis for one-axis overflow clip.
@task_item n=6 part=touches
decision/default-action-event-order and decision/handlers-before-default-action: preventDefault is now ignored on a non-cancelable event (P1 A), so a handler cannot cancel pointerleave; any contract naming JS preventDefault reads against this.
@task_item n=7 part=touches
contract/layout/stylo-taffy-exports and contract/layout/taffy-traits: stylo_taffy resolve_calc_value takes taffy::CalcHandle in place of a raw pointer; the layout reason reader's one call adapted.
@task_item n=8 part=touches
contract/document-trait/handle-ui-event, contract/dioxus-document/handle-ui-event and contract/ui-event/pointer-leave: UiEvent gains DragOver, DragLeave and Drop.
@task_item n=9 part=touches
contract/dom-event-kind/from-str and the DOM event dispatch contracts: the DOM event type gains dragenter, dragover, dragleave and drop, dispatched to the node under the drag.
@task_item n=10 part=touches
contract/nodes-widgets/widget-trait and decision/custom-widget-object: a custom widget accepts a drop only by setting the DragOver event's dropEffect in handle_event.
@task_item n=11 part=touches
decision/sub-document-event-forwarding and decision/forwarded-event-coordinates: drag events are now forwarded to a sub-document over its host, which can accept and leave a drag.
@task_item n=12 part=touches
decision/dioxus-skips-unhandled-event-kinds and contract/dioxus-bridge/registry-dispatch: the Dioxus converter gains a drag arm; drag events now reach Dioxus handlers as NativeDragData.
@task_item n=13 part=touches
decision/dioxus-0-8-move-held, decision/keyboard-types-named-by-absolute-path and decision/workspace-keyboard-types-path: Dioxus held at 0.7.3; upstream's BeforeInputData and keyboard_types imports left out of the Dioxus event bridge, its keyboard types staying dioxus_html's.
@task_item n=14 part=touches
decision/per-crate-coverage-floors, decision/per-crate-coverage-floor and decision/coverage-floors: dioxus-native-dom line coverage fell from 91.14 to 88.59 percent, the dropped-file reader's lines uncovered; the 83 percent floor holds unchanged.
@task_item n=15 part=touches
decision/workspace-dependencies and decision/workspace-shared-dependencies: blitz-shell gains the existing workspace url dependency; the root package's dioxus-native dev features gain net and html.
@task_item n=16 part=touches
decision/blitz-tests-one-target-per-file, decision/blitz-tests-target-per-file and decision/blitz-tests-single-test-target: two targets join by autodiscovery with no manifest edit; the aggregate module list gains two mod lines and stays test false.
@task_item n=17 part=touches
decision/anonymous-blocks-in-layout-children-only and decision/text-nodes-not-laid-out: layout construction now folds a br after text into the text's anonymous flex item (B1 A).
@task_item n=18 part=touches
decision/file-url-load-rule and decision/file-script-load-refused: the file_drop example refuses image and HTML-file previews by file URL; its srcdoc and text previews read no URL.
@task_item n=19 part=touches
contract/example-clis and decision/dioxus-example-shape: two new windowed root examples dnd and file_drop, neither a command-line surface; dnd prints to stdout.
@task_item n=20 part=touches
decision/fork-additions-in-own-functions and decision/direct-engine-changes-flagged-upstreamable: the fork's own layout reason reader took a one-line adaptation to the upstream calc API.
@task_fact n=1 slug=established-decisions
fact: OS drag-and-drop input: blitz-shell's winit drop source turns OS drags into UiEvent DragOver, DragLeave and Drop; a dropped file arrives only through a file URI; the founder's own answer D1 A, not provisional.
@task_fact n=2 slug=established-decisions
fact: The Dioxus dropped-file reader reads a dropped file through std::fs with no path restriction, upstream's design ratified on the founder's answer D1 A; its lines are untested by any harness input.
@task_fact n=3 slug=standard-contracts
fact: blitz-traits DataTransfer model: BlitzDataTransfer holds string and file items with types, files, effectAllowed and dropEffect, a store mode, and BlitzDragOperation and BlitzDragOperations; BlitzDragEvent carries pointer coordinates and the shared data transfer.
@task_fact n=4 slug=standard-contracts
fact: Drag events: UiEvent DragOver, DragLeave and Drop map to DOM dragenter, dragover, dragleave and drop dispatched to the node under the drag; dragleave fires when the drag leaves the document; a drop effect must be allowed by the drag source.
@task_fact n=5 slug=standard-contracts
fact: NativeDragData implements Dioxus's NativeDataTransfer, HasFileData, HasDataTransferData, HasDragData and ModifiersInteraction, modifiers through convert_modifiers.
@task_fact n=6 slug=established-decisions
fact: A br between text in a flex or grid container breaks the line inside the text's anonymous item, also when hoisted from display contents; a trailing br and a br between flex items add no line (B1 A).
@task_fact n=7 slug=established-decisions
fact: Root examples dnd and file_drop are windowed examples, compiled by the build leg and run by no test; file_drop refuses image and HTML-file previews by file URL, its srcdoc and text previews standing (D2 A).
@task_item n=1 part=untouched
The driver, session, MCP role, command line and kept-check contracts and verbs: no verb, argument, carrier or refusal cause was added or changed.
@task_item n=2 part=untouched
Occupied resources: no port, socket, listener or environment variable was added; the name rows of binaries, crates and environment variables stand.
@task_item n=3 part=untouched
Telemetry, logging and scrub rows: no log site was added; the drop input logs nothing.
@task_item n=4 part=untouched
CI leg, gate and agent-run contracts: no CI step, leg list, exit code or status shape was edited; the new targets join no named leg list.
@task_item n=5 part=untouched
Accessibility tree, AccessKit adapter, snapshot, stable element id and UI graph contracts: none of them moved.
@task_item n=6 part=untouched
Select list, form control, scroll, hit testing and geometry reader rows: no select, scroll or hit behaviour changed.
@task_item n=7 part=untouched
WPT runner, browser app, rdme, Boa script runtime and browser history rows: the sync reached none of them.
@task_item n=8 part=untouched
Rows that say left out of a contract or check: no row states the 15 count, and no left-out commit's ruling changed.
