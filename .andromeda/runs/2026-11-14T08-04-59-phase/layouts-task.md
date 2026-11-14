@task_prose part=chunk
Bundles a run's fonts: a run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off, on every path a run boots through.
The paths are the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document that names no font context of its own; generic families resolve to the bundled set, not the host.
The bundled set is held in the document's font context, where a later canvas painter can reach it, and the acceptance names canvas text as a future reader of it.
It re-asserts the fonts-unavailable refusal as kept, and proves that every CI OS reads one state of a run the same way, compared by its readings and never by a shot's bytes.
@task_file n=1 path=examples/seven_guis/src/stand.rs
what: the stand's boot, which builds the stand's single bundled font context from DejaVu Sans for the stand and its fixtures
@task_file n=2 path=packages/blitz-dom/src/document.rs
what: the document's construction, which turns an absent font context into the host's system font collection under the system-fonts feature
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the harness options a harness-built document boots from, whose font context defaults to none
@task_file n=4 path=packages/escher-driver/src/refusal.rs mark=re-reads
what: the fonts-unavailable refusal: an app whose font context holds no family it can paint text with is refused before anything is painted or read
@task_file n=5 path=packages/escher-driver/src/screenshot.rs mark=re-reads
what: the fonts-unavailable refusal before a screenshot is painted
@task_item n=1 part=needs
A run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off — at the stand's boot.
@task_item n=2 part=needs
The driver session's served tasks (counter, crud, flight-booker, cells, timer, load) boot with the bundled font set and read no host font.
@task_item n=3 part=needs
The shared fixture pages and a harness-built document that names no font context of its own read the bundled font set, not the host's system collection.
@task_item n=4 part=needs
The generic families a run's styles name — sans-serif, serif, monospace and an unknown family — resolve to the bundled set and never to a host font.
@task_item n=5 part=needs
The fonts-unavailable refusal is kept: an app whose font context holds no family it can paint text with is still refused before anything is painted or read, and the refusal's text is unchanged.
@task_item n=6 part=needs
Every CI OS of the test leg (Linux, macOS, Windows) reads one state of a run the same way, compared by its readings — the snapshot text, bounds, text extents and lint findings.
@task_item n=7 part=needs
No shot is compared by its bytes across operating systems: a shot's image bytes already differ between hosts and are never the cross-OS proof.
@task_item n=8 part=needs
The snapshot answer's text, bounds and text extents of a run's state come from the bundled font set, so they read the same on every host.
@task_item n=9 part=needs
The stand's lint findings at boot read the same on every host; the stand still lints clean at boot under the bundled font set.
@task_item n=10 part=needs
The session roles share the stand boot, so the bundled font set reached at the stand's boot is the one every session role reads.
@task_item n=11 part=needs
The bundled set is held in the document's font context, not as a stand-local constant alone, so a later canvas painter's text can read the same set; the acceptance names canvas text as a future reader of it.
@task_item n=12 part=needs
Indirect: no change to which font a remote page or the browser app reads, unless it is brought to the founder as an engine-change fork.
@task_item n=13 part=needs
Indirect: the chunk adds no new carrier, verb, argument, field, cause, log field or span to the driver session's surface.
@task_item n=14 part=needs
Indirect: no new dependency crate; the font asset is a file already in the tree.
@task_item n=15 part=needs
Indirect: a change that widens a boundary or changes a contract goes to the founder as a fork, never decided in place; the canvas renderer and glyph-run painting stay out of this chunk.
@task_standing n=1
convention: each new check of a run's state compares its readings — snapshot text, bounds, text extents, lint findings — and never a shot's bytes
home: surface-cli
@task_standing n=2
convention: each new stand test or fixture page boots through the stand's own boot and its bundled font context, never a hand-built one
home: surface-desktop-native
@task_item n=1 part=not_needed
Window chrome controls of the shell provider (drag, maximize, minimize, close, decorations): fonts do not reach window chrome.
@task_item n=2 part=not_needed
The stand's stable element ids and the rule that ids, names, roles and states are attributes alone: the chunk adds and renames no element.
@task_item n=3 part=not_needed
The agent-run and cold-agent test-pipe command lines (their usage, exit codes, run logs, record directories, Windows pass-through): the chunk changes neither pipe.
@task_item n=4 part=not_needed
The multi-document and paint-bench command lines and their arguments, environment knobs and backend choice: the chunk adds no benchmark argument.
@task_item n=5 part=not_needed
The MCP server's transport, protocol revisions and tool list: the chunk adds no tool and changes no transport.
@task_item n=6 part=not_needed
Keyboard, list-region, scroll-into-view and clipping-visibility layout decisions: the chunk changes no interaction or scrolling.
@task_item n=7 part=not_needed
The stand's accent colour and the fixed document skeleton with its main mount: the chunk changes no colour or page skeleton.
@task_item n=8 part=not_needed
The absence of arbitrary index.html templates for a document: the chunk adds no template.
@task_item n=9 part=not_needed
The mobile-native and web-spa surfaces: they hold no rows and the chunk builds nothing there.
