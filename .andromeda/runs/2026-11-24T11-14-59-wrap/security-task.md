@task_prose part=session
The session merged upstream DioxusLabs/blitz main at 29a439e05 (six commits) as a two-parent merge; the next sync's merge base moves from 165d698c3 to 29a439e05, and the 14 left-out upstream commits stay out (the inherited count of 15 was a miscount).
It brought an OS drag-and-drop input: a winit drop source feeding DragOver, DragLeave and Drop events into the event driver, a DataTransfer model, and a Dioxus dropped-file reader that reads a dropped file through std::fs with no path restriction, ratified by the founder (D1 A).
It moved Taffy from a git pin to taffy 0.16.0 from crates.io, retiring the workspace's last git dependency and the Taffy exception to the no-git-dependency rule; the lock holds no git source; blitz-shell gains the workspace url crate; no other lock package moved; dioxus held at 0.7.3.
It added two root examples (dnd, file_drop; file_drop refuses its image and HTML-file previews by file: URL, while its srcdoc and text previews stand), two blitz-tests targets (drag_and_drop, flex_br), preventDefault ignored on a non-cancelable event, one-axis overflow clip painting only the clipped axis, and a br after text joining the text's flex item.
Measured: dioxus-native-dom line coverage 91.14 % to 88.59 % against an unchanged 83 % floor (the dropped-file reader's lines uncovered); no port, socket, listener or env var added; audit, licences and every CI leg green.
@task_file n=1 path=Cargo.toml mark=merge
what: merged — the root manifest: the workspace dependency set and its pins, Taffy now 0.16.0 from crates.io with no git source, the root package's dioxus-native dev features gaining net and html
@task_file n=2 path=Cargo.lock mark=merge
what: merged — the lockfile the locked builds and the audit leg read: taffy 0.16.0 from crates.io with its checksum, no git-source package left
@task_file n=3 path=examples/dnd.rs mark=merge
what: added by the merge — a root example window showing drag-and-drop, printing the dragged data to its stdout
@task_file n=4 path=examples/file_drop.rs mark=merge
what: added by the merge — a root example window listing dropped files and data with previews; its image and HTML-file previews by file: URL refused, its srcdoc and text previews standing
@task_file n=5 path=packages/blitz-shell/src/dnd.rs mark=merge
what: added by the merge — the shell's OS drop source: winit drag data turned into the engine's drag events, a dropped URI accepted only as a file:// URI and turned into a path
@task_file n=6 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: added by the merge — the DataTransfer model: dragged strings and files as one item list, the allowed and chosen drop effect, the data store mode
@task_file n=7 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: added by the merge — the drag-and-drop integration test target over the document's UI event handling
@task_file n=8 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: added by the merge — the integration test target for a br inside a flex or grid container
@task_file n=9 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: resolved a conflict in, and edited — the Dioxus native bridge's event data converters: the drag data handed to a Dioxus handler and the dropped-file reader reading a dropped file through std::fs with no path restriction
@task_file n=10 path=packages/blitz-dom/src/layout_reason.rs
what: edited — the engine's layout-reason reading, its one call of the calc resolver adapted to Taffy's CalcHandle
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: merged — the engine document, gaining the node a drag is over
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: merged — the engine's event driver: dispatch of pointer, keyboard and now drag events to the node under them, and the default-prevented check that now ignores preventDefault on a non-cancelable event
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: merged — the engine's UI event handling, forwarding a drag to a sub-document over its host
@task_file n=14 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: merged — layout tree construction and its anonymous boxes, a br after text now joining the text's anonymous flex item
@task_file n=15 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: merged — the engine's layout entry, adapted to Taffy 0.16
@task_file n=16 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: merged — replaced-element sizing, adapted to Taffy 0.16
@task_file n=17 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: merged — table layout, adapted to Taffy 0.16
@task_file n=18 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: merged — the custom widget contract: a widget accepts a drop only by setting the DragOver event's drop effect
@task_file n=19 path=packages/blitz-paint/src/render.rs mark=merge
what: merged — the paint pass, a one-axis overflow clip now clipping only the clipped axis
@task_file n=20 path=packages/blitz-shell/Cargo.toml mark=merge
what: merged — the shell crate's manifest, gaining the workspace url dependency
@task_file n=21 path=packages/blitz-shell/src/application.rs mark=merge
what: merged — the shell application's event loop, wiring the OS drop source
@task_file n=22 path=packages/blitz-shell/src/lib.rs mark=merge
what: merged — the shell crate's module list, gaining the drop source module
@task_file n=23 path=packages/blitz-shell/src/window.rs mark=merge
what: merged — the shell window, routing OS drag position, leave and drop to the document
@task_file n=24 path=packages/blitz-traits/src/events.rs mark=merge
what: merged — the UI event and DOM event type model, gaining the dragenter, dragover, dragleave and drop events and the drag event
@task_file n=25 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: merged — the Dioxus native document's event converter, gaining its drag arm
@task_file n=26 path=packages/stylo_taffy/src/convert.rs mark=merge
what: merged — the style-to-Taffy converter, its calc resolver taking Taffy's CalcHandle
@task_file n=27 path=tests/blitz-tests/tests/all.rs mark=merge
what: merged — the blitz-tests aggregate test file, gaining two module lines
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: merged — the pointerleave integration test target, extended with preventDefault not cancelling pointerleave
@task_item n=1 part=touches
decision/engine-dependency-pins: the Taffy pin moved from a git rev of DioxusLabs/taffy to taffy 0.16.0 from crates.io; a row naming the git rev or Taffy 0.15 is stale.
@task_item n=2 part=touches
decision/no-git-dependencies: its Taffy exception is retired; the root manifest and the lock now hold no git dependency at all (founder's T2 A).
@task_item n=3 part=touches
decision/no-git-dependency: the same retirement; any except or warning naming Taffy's git pin is stale.
@task_item n=4 part=touches
decision/upstream-sync-dependency-set: this sync took the dependency set whole (taffy bumped, url added to the shell crate, no cargo update, no other lock package moved, dioxus held at 0.7.3); the next sync's merge base is now 29a439e05 where a row holds 165d698c3.
@task_item n=5 part=touches
decision/locked-builds and decision/workspace-inherited-dependency-versions: the lock moved by the merge alone, and the shell crate's new url dependency is the inherited workspace one.
@task_item n=6 part=touches
decision/per-crate-coverage-floors: dioxus-native-dom line coverage fell from 91.14 % to 88.59 %, floor 83 % unchanged; a row holding the measured figure is stale.
@task_item n=7 part=touches
decision/dependency-audit-ci-gate, decision/licence-gate, decision/cargo-deny-audit-tool: the audit leg read the moved lock with taffy 0.16.0 from crates.io and passed advisories and licences; no ignore added.
@task_item n=8 part=touches
threat-model-summary: a new untrusted input reaches the engine, the OS drag-and-drop source, and a Dioxus handler can read a dropped file from disk with no path restriction.
@task_item n=9 part=touches
decision/file-url-only-for-file-documents, decision/file-urls-only-from-file-documents, contract/blitz-net-provider-error/file-forbidden: a dropped file:// URI is turned into a local path by the drop source and read outside the fetch path; the file_drop example refuses its file: URL previews.
@task_item n=10 part=touches
decision/dioxus-scheme-local-files and decision/file-dialog-via-rfd: the OS drop is a second way a user-chosen local file path reaches the app, beside the native file dialog and the local-file scheme.
@task_item n=11 part=touches
decision/event-is-trusted-by-source: drag events now come from the OS drop source through the event driver.
@task_item n=12 part=touches
decision/custom-widget-click-synthesis and the other custom-widget rules: a custom widget now accepts a drop only by setting the DragOver event's drop effect, and the drop effect must be allowed by the drag source.
@task_item n=13 part=touches
decision/cancelled-pointerup-still-clicks and rules on default actions and preventDefault: preventDefault is now ignored on a non-cancelable event (P1 A); pointerleave is not cancelled by it.
@task_item n=14 part=touches
decision/clipboard-via-winit: drag data now also crosses from winit, as a DataTransfer of strings and files, beside the clipboard.
@task_item n=15 part=touches
decision/untrusted-bytes-decoded-without-panic: the drop source takes OS-originated data with no parse of the engine's own beyond the file URI.
@task_item n=16 part=touches
decision/stale-node-ids-never-panic: the document now keeps the node a drag is over between drag events.
@task_item n=17 part=touches
decision/dropped-anonymous-block-clears-parent: anonymous box construction changed, a br after text joining the text's anonymous flex item.
@task_item n=18 part=touches
decision/visible-region-through-transforms and contract/refusal-causes/covered, off-screen: a one-axis overflow clip now paints only the clipped axis; the readings stay as they were (C1 A).
@task_item n=19 part=touches
decision/no-screen-content-in-logs and decision/escher-telemetry-allowlist-sink: the dnd example prints drag data to its own stdout outside the telemetry sink; the drop input has no log site.
@task_item n=20 part=touches
absence/served-network-api and decision/session-socket-sole-listener: the merge added no port, socket, listener or env var; they stand true.
@task_fact n=1 slug=input-validation
fact: The OS drag-and-drop input enters the engine from the shell's winit drop source as DragOver, DragLeave and Drop events dispatched to the node under the drag; the data is OS-originated and passes no parse of ours, a dropped URI is accepted only as a file:// URI and turned into a path; ratified on the founder's D1 A.
@task_fact n=2 slug=input-validation
fact: The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction, upstream's design, ratified on the founder's D1 A; no harness input reaches it and its lines stand untested, a deferral to 0.5.0.
@task_fact n=3 slug=input-validation
fact: The root examples dnd and file_drop are registered (D2 A); file_drop refuses its image and HTML-file previews by file: URL in img src and iframe src, while the srcdoc preview of dropped HTML data and the text preview read no URL and stand.
@task_fact n=4 slug=input-validation
fact: A custom widget accepts a drop only by setting the DragOver event's drop effect, and a drop effect is taken only where the drag source allows it.
@task_fact n=5 slug=input-validation
fact: preventDefault is ignored on a non-cancelable event (P1 A).
@task_fact n=6 slug=threat-model-summary
fact: The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.
@task_fact n=7 slug=logging-monitoring
fact: The drag-and-drop input has no log site; the dnd root example's println of drag data is example stdout, outside the telemetry sink.
@task_item n=1 part=untouched
The MCP server, its stdio transport, tool results, request bounds and protocol errors: no MCP code moved.
@task_item n=2 part=untouched
The driver session socket, its closed grammar, bounds, replies, lifecycle and idle expiry, and the driver verbs and their refusal causes other than the clip readings named above.
@task_item n=3 part=untouched
Recording, recorded check files, kept checks and cold-agent runs and their owner-only directories.
@task_item n=4 part=untouched
The stable element id, snapshot text, the actions, graph and layout answers and their carriers.
@task_item n=5 part=untouched
Select list keys, choice and typeahead, range input actions, and password and file input value masking on the carriers.
@task_item n=6 part=untouched
Secret management: CI secrets, signing material, publishing guards and the environment variable contract; no env var added.
@task_item n=7 part=untouched
Outbound HTTP, the script fetcher, sub-resource requests and the CORS, same-origin, timeout and response-size absences: no network code changed.
@task_item n=8 part=untouched
The WPT runner, its quarantine and its post-results workflow.
@task_item n=9 part=untouched
Authentication and authorization rows and the CI token and action-pinning rules: no CI step or workflow edited.
@task_item n=10 part=untouched
The accessibility adapter rows: the adapter probe on the moved lock still finds no accessibility adapter crate.
@task_item n=11 part=untouched
The left-out upstream commits: they stay out as the last sync ruled, and no security row states their count.
