@task_prose part=chunk
Fixes upstream issue 1578 in the engine: a custom widget's paint is handed its content-box size and its scene lands at the content-box origin inside the element's own transform, with padding, border and a transform alike; the scene stays unclipped.
Moves a custom widget's pointer events into the same content-box frame (border-box origin plus left/top border and padding, CSS px, no transform applied); a sub-document's events are unmoved.
Gives canvas text the bundled fonts: the document's own font context is handed to each widget before its paint through a new defaulted method of the widget trait, and the document gains a public font context accessor, both behind custom-widget.
Adds stand checks for placement, pointer frame and canvas text, each shown red first, in both layout modes; re-reads the existing canvas shot and widget layout checks green and keeps every standing stand reading.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the scene painter's custom widget node pass, which sizes a widget's paint — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: the element painter that appends a custom widget's scene into the frame — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, forwarding pointer events to a custom widget — edits: the custom-widget arm of handle_dom_event
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget trait an app paints a canvas through, a public surface diverging from upstream — edits: the Widget trait, a new defaulted method handing the font context
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document holding the font context every glyph run is shaped from — edits: a public font_ctx accessor on BaseDocument
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the blitz-tests manifest, its difference from upstream — edits: the dev-dependency list, parley joining it
@task_file n=7 path=tests/blitz-tests/tests/stand_canvas_shot.rs mark=re-reads
what: the stand check that an app widget painted from Rust shows in a headless shot, re-read green under the fix
@task_file n=8 path=tests/blitz-tests/tests/custom_widget_layout.rs mark=re-reads
what: the check of a custom widget's layout sizes, re-read green under the fix
@task_item n=1 part=needs
A custom widget's paint receives its content-box size times the scale, not its border-box size; a widget with padding and a border reads its content box (100 by 60 where it read 130 by 90).
@task_item n=2 part=needs
A custom widget's scene is placed at its content-box origin inside the element's own transform, so under a scale transform its content offset is scaled with it, and nothing it paints falls outside its border box.
@task_item n=3 part=needs
A custom widget's scene stays unclipped, as today; no content-box clip is pushed for it, unlike a replaced image.
@task_item n=4 part=needs
Pointer move, down, up, cancel and leave events forwarded to a custom widget carry coordinates relative to its content-box origin in CSS px, the frame its paint draws in; no transform is applied to them, and a sub-document's events keep the border-box origin.
@task_item n=5 part=needs
Canvas text is shaped from the document's own font context, handed to each widget before its paint; a widget that draws text paints the same bundled font as the document's text, in both layout modes.
@task_item n=6 part=needs
Two new public engine items, the defaulted widget trait method and the document's font context accessor, each behind custom-widget, each with its doc comment, flagged upstreamable; no existing implementor changes, and the doc leg joins the local gate.
@task_item n=7 part=needs
A check's widget shapes text with parley, taken as a blitz-tests dev-dependency already in the lock; no package enters the lock and every cargo build, test and CI leg runs locked.
@task_item n=8 part=needs
Every new check is run red first, on the tree before its fix or with the fix reverted, and that red reading is kept in the chunk's evidence beside its green, every path in a kept log respelled with no temp or home path.
@task_item n=9 part=needs
Every new stand check runs in both layout modes, incremental and non-incremental, and reads the same.
@task_item n=10 part=needs
Canvas pixels are read from a headless shot painted on the CPU buffer under the stand's bundled font, no skip; a check's widget paints through the scene, never a texture or resource, which the CPU renderer leaves transparent.
@task_item n=11 part=needs
The screenshot's contract stands: an 800 by 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable before anything is painted, a masked password input, nothing run or changed by a shot.
@task_item n=12 part=needs
Canvas text leaves only as pixels in a shot, inside the ratified crossing of canvas content as shot pixels; no committed check holds a screenshot step and no cross-OS byte comparison holds a shot.
@task_item n=13 part=needs
The stand's standing readings hold: the seven served tasks' snapshot, lint and actions digests, the committed kept checks and the stand checks keep their readings, never loosened; the canvas shot and widget layout checks read green under the fix.
@task_item n=14 part=needs
No new verb, argument, carrier, cause, session error, exit, log field or span is added, and no content enters a log field.
@task_item n=15 part=needs
The engine change reaches upstream blitz-dom and blitz-paint, so the workspace clippy legs including upstream blitz-dom and the per-package and workspace test legs must read green over it.
@task_item n=16 part=needs
The last wrap's CI verdict, unread and in progress, is re-read before this chunk's push and never folded as green.
@task_standing n=1
convention: each new integration test stands as its own blitz-tests target, one target per file
home: test-strategy
@task_standing n=2
convention: a regression test is named for the bug it fixes and documents it, here upstream issue 1578
home: 2-test-strategy
@task_standing n=3
convention: each new test is controlled red before green, with its red reading kept
home: unit-test-strategy
@task_standing n=4
convention: each new stand check runs in both layout modes
home: test-scope-summary
@task_standing n=5
convention: each new stand check runs under the bundled real font and never skips for want of one
home: 2-test-strategy · test-strategy
@task_standing n=6
convention: each new pixel assertion reads a CPU buffer and compares frames whole or by pinned literal pixels
home: 2-test-strategy · test-strategy
@task_standing n=7
convention: each new test pins its expected values as literals and its failures print no argument value
home: unit-test-strategy
@task_standing n=8
convention: each new test states its data in code, shared fixtures and stand tables kept in one module that is no target
home: 3-test-harness-contract
@task_standing n=9
convention: each new stand check joins the three-round stand repeat under a zero-failure budget
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Session wire framing, session state directory, idle expiry and session error rows: the chunk adds nothing to the socket protocol or the session host.
@task_item n=2 part=not_needed
Cold-agent pipe and agent-run rows (events, verdicts, exit codes, selections): no agent run is changed or fired.
@task_item n=3 part=not_needed
WPT runner, status set, expectations and reftest rows: no conformance suite is run or changed.
@task_item n=4 part=not_needed
Lint defect, snapshot node, element id and contrast reader rows: the canvas is not read as data and lint is out of scope.
@task_item n=5 part=not_needed
Virtual clock, advance, timer tick and settle rows: no time or settle behaviour moves.
@task_item n=6 part=not_needed
Driver refusal cause set and order, argument kinds and verb table rows: no cause or verb is added.
@task_item n=7 part=not_needed
Select list painting, chosen select colours, scrollbar fade and password caret rows: other paint paths stay untouched.
@task_item n=8 part=not_needed
Dioxus bridge buffered render and component-alone harness construction rows: the chunk does not reach the bridge.
@task_item n=9 part=not_needed
CI package install, skia archive witness, action pinning, cache and secrets rows: no workflow or CI script is edited.
@task_item n=10 part=not_needed
Network stand-in, load counting and offline boot rows: no network or load path is touched.
@task_item n=11 part=not_needed
Cross-host canvas shot byte equality, Circle Drawer on a canvas, canvas lint, virtual nodes and drag rows: later entries.
