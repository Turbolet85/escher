@task_prose part=chunk
A run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off, on every path a run boots through: the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document that names no font context of its own.
The generic families a run's styles name (sans-serif, serif, monospace, an unknown family) reach the bundled set and not the host.
The bundled set is held in the document's font context, where a later canvas painter can reach it, and the acceptance names canvas text as a future reader of it; the canvas renderer itself is not built.
fonts-unavailable is kept and re-verified: an app whose font context holds no family it can paint text with is still refused before anything is painted or read.
The proof: every CI OS of the test leg (Linux, macOS, Windows) reads one state of a run the same way, compared by its readings (snapshot text, bounds, text extents, lint findings), never by a shot's bytes.
@task_file n=1 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand's boot, which builds the stand's single bundled-font context from DejaVu Sans (stand typography) — edits: the stand's font context construction
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the blitz-test-harness Harness and its options, whose font context defaults to none — edits: HarnessOptions::font_ctx default
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the blitz-dom document core, where BaseDocument::new turns an absent font context into the host's system font collection under the system-fonts feature
@task_file n=4 path=examples/seven_guis/assets/DejaVuSans.woff2
what: the bundled font asset, the stand's DejaVu Sans, already in the tree
@task_file n=5 path=packages/escher-driver/src/refusal.rs mark=re-reads
what: the fonts-unavailable refusal cause, which refuses an app with no family it can paint text with before anything is read
@task_file n=6 path=packages/escher-driver/src/screenshot.rs mark=re-reads
what: the screenshot's FontsUnavailable error, which refuses before anything is painted
@task_item n=1 part=needs
One bundled font set for a run: the stand's DejaVu Sans is the only font every run path shapes and measures text with, the host's system fonts off — the stand typography and the stand sharing the bundled font extended from the stand to every run path.
@task_item n=2 part=needs
The harness-built document's font context: a harness document that names no font context of its own must reach the bundled set, never the host's system collection the document core builds from an absent font context.
@task_item n=3 part=needs
The driver session's served tasks and the shared fixture pages boot their documents with the bundled font context, the same as the stand's boot.
@task_item n=4 part=needs
Where the bundled set is held: in the document's font context (the document configuration a document is built from), reachable by a later canvas painter, not a stand-local constant alone; canvas text is named as a future reader of it.
@task_item n=5 part=needs
Generic family resolution: sans-serif, serif, monospace and an unknown family resolve to the bundled set, under the default-font fallback when no listed family is available and the rule that no generic family means sans-serif.
@task_item n=6 part=needs
fonts-unavailable kept: the refusal cause, the lint verb's fonts-unavailable finding, the screenshot error FontsUnavailable and the document's can_paint_text reader still refuse an app with no paintable family before anything is painted or read; the cause's text is unchanged.
@task_item n=7 part=needs
Cross-OS readings: one state of a run reads the same on the test leg's Linux, macOS and Windows runners, compared by its readings — snapshot text, bounds, text_extent, lint findings — held in CI as a committed reading each OS replays or as a kept check replayed through the one run path.
@task_item n=8 part=needs
No shot compared by bytes across OSes: a shot's PNG bytes differ between hosts, so no screenshot step enters a cross-OS byte check.
@task_item n=9 part=needs
No change to which font a remote page or the browser app reads; any change there is an engine behaviour change that goes to the founder as a fork, flagged as a direct engine change.
@task_item n=10 part=needs
The driver's boundary is not widened: no new carrier, verb, argument, field, refusal cause, log field or span; refusal causes stay closed at eight.
@task_item n=11 part=needs
No new dependency crate: the font is read from the woff2 asset already in the tree through the font formats and font crates the workspace already builds (woff support, the skrifa, parley and vello version alignment).
@task_item n=12 part=needs
The stand stays offline and pinned: the bundled font is read with no network and no host environment, and the stand's telemetry and environment rules hold.
@task_item n=13 part=needs
Readings carry no process-local values: the snapshot and its text form read the same across hosts, with text extents and bounds the only geometry compared.
@task_item n=14 part=needs
The document defaults a run's documents are built with (colour scheme, media type, document mode, Stylo prefs) are unchanged; only the font context is fixed to the bundled set.
@task_standing n=1
convention: each new test has a descriptive name and asserts its fixture's condition first
home: conventions
@task_standing n=2
convention: each new blitz-tests integration test file holds one behaviour, is one test target, and uses the shared fixtures module stated once
home: conventions
@task_standing n=3
convention: each new or changed public item and module is documented, rustdoc warnings are errors, and a constant states its rationale
home: conventions
@task_standing n=4
convention: a change to a CI job or leg keeps the CI workflow contract pinned by its tests and the leg run through the one CI leg runner
home: standard-contracts
@task_standing n=5
convention: each new unit test sits at the end of its source file, and expect or unwrap carries a justification
home: conventions
@task_item n=1 part=not_needed
The WPT runner, its suites, reftests, outputs, result statuses and PR-body diff reporting: the chunk changes no WPT behaviour.
@task_item n=2 part=not_needed
The browser app's chrome, about pages, history store and persistence: which font the browser app reads is left as it is.
@task_item n=3 part=not_needed
Scripting in Boa, script globals, script fetchers and the JS runtime's embedder API: no script path changes.
@task_item n=4 part=not_needed
The open select list, select readers and select default actions: their machinery is not touched.
@task_item n=5 part=not_needed
Scroll and hit testing, programmatic and user scrolling, and the event dispatch order: no input path changes.
@task_item n=6 part=not_needed
The accessibility tree, its ARIA states and the platform adapter: font loading changes no accessibility reading.
@task_item n=7 part=not_needed
The UI graph, stable element ids, the actions verb and handler coverage: the chunk adds no node, id or handler reading.
@task_item n=8 part=not_needed
The cold-agent and agent-run scripts, their events, exit codes and verdicts: no cold-agent run changes.
@task_item n=9 part=not_needed
Networking (blitz-net, the HTTP cache, per-host request caps, the net provider): the font is a file in the tree, never fetched.
@task_item n=10 part=not_needed
The run seed and seeded randomness: the sibling host dependence the previous entry settled, not reached here.
@task_item n=11 part=not_needed
Paint-tree structure, paint layers, gradients, borders and the window renderer choice: painting is not restructured.
@task_item n=12 part=not_needed
The canvas renderer, GlyphRun painting, the CPU renderer's custom-paint gap and virtual nodes: all 0.4.0, not built.
