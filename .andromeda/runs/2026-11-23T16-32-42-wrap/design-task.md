@task_prose part=session
The session measured whether a screenshot of a canvas alone comes out byte-identical on CI's Linux x86_64, Windows x86_64 and macOS aarch64 hosts. It did: all twelve rows were equal on all three hosts in CI#45209188300.
It added one test target. The target paints three canvas fixtures from Rust (a solid fill, an antialiased edge, and glyphs drawn with the document's font context). For each fixture it takes the driver screenshot in both forms, whole and element by id, and in both layout modes, full and incremental. It then checks each answer digest against a committed table of twelve rows. No PNG is committed.
The founder ruled that canvas-only shots must be byte-equal across hosts (F1, option A). So the no-shot cross-OS rule now has a canvas-only exception. A shot that holds document text still differs between linux x86_64 and macOS aarch64.
The capability statement now carries the canvas exception and the measured difference with its witness. It also lists the canvas conditions that were not measured: another SIMD level of the CPU renderer, a scale other than 1.0, and gradient or image content.
The session changed no product symbol, driver verb, argument, answer field, exit status, dependency, config, CI script or committed check, and retired nothing.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new stand check (test target) that requires the driver screenshot of a canvas-only page to read the same bytes on every host. It covers three canvas fixtures painted from Rust (solid fill, antialiased edge, and glyphs drawn with the document's font context), both screenshot forms (whole and element by id) and both layout modes. Each answer digest is held against one table of twelve rows.
@task_file n=2 path=docs/capabilities.md
what: the capability statement, which says what Escher can and cannot do and its limits. Its limit on a shot's PNG bytes now says: a shot repeats byte for byte on one host; with document text it differs between linux x86_64 and macOS aarch64; a canvas-only shot is held equal on linux x86_64, windows x86_64 and macOS aarch64. It also lists the canvas conditions that were not measured.
@task_item n=1 part=touches
Any decision or convention saying that a screenshot's bytes differ by OS, or that no screenshot can be compared byte for byte across hosts. Such a rule now has a canvas-only exception: a shot that holds only canvas content is byte-equal on linux x86_64, windows x86_64 and macOS aarch64. The rule still holds for shots of document text.
@task_item n=2 part=touches
The why of any rule that rests on a shot being byte-identical on one host only. That reason now holds only for shots that contain document or page text. Canvas-only shots have been measured as equal across the three CI hosts.
@task_item n=3 part=touches
contract/escher-session-commands/screenshot and contract/escher-session-mcp/screenshot-result. The screenshot verb, in its whole form and its element form (--id), is now relied on to give the same answer digest on every host for a canvas-only page. The contract itself did not move: no argument, field or answer shape changed.
@task_item n=4 part=touches
decision/screenshot-never-skips-text and the typography rows on shots that hold text: text in a page is what keeps a shot's bytes host-specific (linux x86_64 against macOS aarch64), and that limit is now stated as scoped to shots holding document text.
@task_item n=5 part=touches
decision/tests-built-with-system-fonts and decision/test-engine-with-system-fonts. The new check draws glyphs on a canvas with the document's font context, which tests build from system fonts. Those glyphs read equal on all three hosts. This is indirect: the decisions stand, but a canvas-glyph shot is now known to be host-independent under them.
@task_item n=6 part=touches
contract/native-config/font-context. The canvas widget's fonts hook keeps the document's font context, and the check uses it to paint canvas text. This is indirect: the font context is used, not changed.
@task_item n=7 part=touches
The surface-web-spa overview and any row on canvas rendering (a canvas painted from Rust, sized and placed in the page). A canvas's own pixels now have a stated property across hosts: byte-equal at scale 1.0 under the default CPU renderer. Other SIMD levels, other scales, and gradient or image content were not measured.
@task_item n=8 part=touches
The capability statement's limits, as any design-system row mirrors them. The limit on a shot's PNG bytes changed its scope, and new lines say which canvas conditions are unmeasured. No capability was claimed: the working-route entry Circle Drawer on a canvas (v040-09) stays pooled and unclaimed.
@task_fact n=1 slug=surface
fact: A screenshot of a canvas-only page (canvas content painted from Rust, including glyphs drawn with the document's font context) reads the same bytes on linux x86_64, windows x86_64 and macOS aarch64. This holds in both the whole and element forms and in both layout modes, at scale 1.0 under the default CPU renderer. A shot that holds document text still differs between linux x86_64 and macOS aarch64. This was ratified on the founder's F1 ruling: require byte equality.
@task_fact n=2 slug=surface
fact: For canvas shots, byte equality across hosts is not measured, so not claimed, in three cases: under another SIMD level of the CPU renderer, at a scale other than 1.0, or when the canvas holds gradient or image content.
@task_item n=1 part=untouched
Colour palette rows (engine colours, select list colours, color-scheme recascade, color tokens): the fixtures use fixed fills and no colour rule changed.
@task_item n=2 part=untouched
Spacing, border radius and depth-strategy rows (margins, cell spacing, z-index paint levels, hit-testing at paint clips, the select list's paint order): no layout or paint-order rule moved.
@task_item n=3 part=untouched
Motion rows (the virtual clock, driver advance, scroll behaviour, overlay scrollbar timings, select list motion): no clock or animation behaviour changed.
@task_item n=4 part=untouched
Iconography rows (cursor icons, cursor resolution, favicon): not reached.
@task_item n=5 part=untouched
The select list rows (its overlay, gestures, events, accessibility, snapshot bounds, its row in a screenshot): no select behaviour or contract moved.
@task_item n=6 part=untouched
escher-session exit statuses and exit codes, stderr and session-error objects, the MCP role's stdout and Err ending, help, and the other commands (snapshot, layout, graph, actions, coverage, tasks). No verb, argument, carrier, field, cause, exit, log field or span changed.
@task_item n=7 part=untouched
WPT runner output and result colours, the shell provider (window drag, maximize, minimize, decorations), the JS window object, dioxus-native config items (base colour, alpha mode), and the mobile-targets decision: not reached.
