@task_prose part=chunk
Bundles a run's fonts: a run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off, on every path a run boots through.
The paths are the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document that names no font context of its own; the generic families a run's styles name reach the bundled set, not the host.
Keeps the fonts-unavailable refusal as it is, re-asserting that an app whose font context holds no paintable family is refused before anything is painted or read.
Proves that every CI OS (Linux, macOS, Windows) reads one state of a run the same way, compared by its readings and never by a shot's bytes, and holds the bundled set where a later canvas text painter can reach it.
@task_file n=1 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the test harness that builds a document for a test, and its options — edits: HarnessOptions font_ctx, the font context a harness-built document gets when it names none
@task_file n=2 path=examples/seven_guis/src/stand.rs
what: the seven-GUIs stand's boot, which builds a single-font context from the bundled DejaVu Sans for the stand and its fixture pages
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the base document's construction, which turns an absent font context into the host's system font collection under the system-fonts feature
@task_file n=4 path=examples/seven_guis/assets/DejaVuSans.woff2
what: the bundled DejaVu Sans font asset, the one font set a run's text is shaped and measured from
@task_file n=5 path=packages/escher-driver/src/refusal.rs mark=re-reads
what: the driver's refusals, the fonts-unavailable refusal of an app whose font context holds no family it can paint text with
@task_file n=6 path=packages/escher-driver/src/screenshot.rs mark=re-reads
what: the screenshot verb's fonts-unavailable refusal, raised before anything is painted
@task_item n=1 part=needs
A run's text is shaped and measured from the one bundled font set (DejaVu Sans) with the host's system fonts off, on every path a run boots through: the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document.
@task_item n=2 part=needs
A harness-built document that names no font context of its own no longer gets the host's system font collection: it gets the bundled set. This changes how tests that build a document through the harness read fonts.
@task_item n=3 part=needs
The generic families a run's styles name (sans-serif, serif, monospace, and an unknown or unavailable family) resolve to the bundled set and never to a host font, including the fallback to a default font and the default sans-serif layout when no generic family is named.
@task_item n=4 part=needs
The fonts-unavailable refusal is kept: an app whose font context holds no family it can paint text with is still refused, by the driver's refusals and the screenshot verb, before anything is painted or read, and its text is unchanged.
@task_item n=5 part=needs
The font context the stand and the driver session pass into the document's configuration carries the bundled set, built as a single-font context from the asset already in the tree.
@task_item n=6 part=needs
Cross-OS proof: one state of a run reads the same on the Linux, macOS and Windows runners of the CI test leg, compared by its readings — snapshot text, bounds, text extents and lint findings — and never by a shot's PNG bytes.
@task_item n=7 part=needs
The readings the comparison uses are those the driver's existing verbs already answer (the snapshot and its bounds and text extents, the lint findings); the comparison reads them, it adds nothing to their answers.
@task_item n=8 part=needs
A screenshot still never paints with text skipped: the bundled set is what it paints text with, and a shot is never the cross-OS comparison.
@task_item n=9 part=needs
The bundled set is held where a later canvas text painter can reach it — the document's font context, not a stand-local constant alone — and the acceptance names canvas text as a future reader of the bundled set.
@task_item n=10 part=needs
No change to which font a remote page or the browser app reads; such a change goes to the founder as an engine-change fork.
@task_item n=11 part=needs
No new carrier, verb, argument, field, cause, log field or span is added to the driver or the session's surfaces by this chunk.
@task_item n=12 part=needs
No new dependency crate: the font asset is a file already in the tree, loaded through the font stack the engine already uses.
@task_standing n=1
convention: each new test or fixture page that builds a document is built on the bundled font context, never on the host's system fonts
home: typography
@task_standing n=2
convention: each new check of a run's state reads it through the driver's existing command line output, read by an agent, never by comparing image bytes
home: surface-cli
@task_item n=1 part=not_needed
Colour rows — engine colours, colour schemes, select list colours, colour tokens: the chunk changes no colour.
@task_item n=2 part=not_needed
Spacing and depth rows — margins, cell spacing, z-index paint levels, stacking contexts, hit-testing at paint clips: fonts do not reach them.
@task_item n=3 part=not_needed
Motion rows — the virtual clock, advance, scroll behaviour, scrollbar fades, select list motion: the chunk moves no clock and animates nothing.
@task_item n=4 part=not_needed
Iconography rows — cursor icons, cursor resolution, favicons: untouched.
@task_item n=5 part=not_needed
The open select list's rows, events, highlight and accessibility tree, and its overlay placement: the chunk changes no select behaviour.
@task_item n=6 part=not_needed
The WPT runner's output and result colours: the runner is not a run path this chunk bundles fonts for.
@task_item n=7 part=not_needed
The shell provider's window controls (maximize, minimize, drag, decorations) and the JS window object's sizes and device pixel ratio: the chunk changes no window behaviour.
@task_item n=8 part=not_needed
The session's exit statuses, stderr errors and mcp role results, and the actions, coverage and graph lines' content: the chunk changes no answer and no status.
@task_item n=9 part=not_needed
Brand identity and border radius: the chunk adds no tokens and no radius.
@task_item n=10 part=not_needed
Mobile targets and the alpha compositing mode of the native configuration: untouched by font bundling.
