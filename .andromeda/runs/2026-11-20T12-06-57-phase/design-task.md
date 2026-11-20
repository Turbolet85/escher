@task_prose part=chunk
Ends the four surviving browser mutants that replace the scene render's returned size with the constants (0, 0), (0, 1), (1, 0) and (1, 1): each is either caught by a browser test or kept by the founder's word.
Re-derived coordinate at the current head: the render function's body starts at the audited span, so the coordinate stands; its returned size is read by no caller, so only a direct test of that function can catch the four.
The planned change is a direct unit test in the capture module's own test module that asserts the returned width and height for two documents whose viewports differ in size, each wider and taller than 1 px.
Choosing among that test, removing the unused return value, or keeping the four on the founder's word is a fork put to the founder; any mutation run of the four starts only on his word.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser's capture module — the screenshot capture and the scene render that paints a document's viewport into a paint scene and returns its size — edits: the scene render function's own test module, or its return value as the founder's fork rules
@task_item n=1 part=needs
The scene render's returned size: the width and height in pixels it returns for a document's viewport, asserted directly by a new unit test for two inputs of different sizes, each larger than 1 px in both dimensions, so that all four constant replacements fail it.
@task_item n=2 part=needs
Indirect: the expected size the test asserts is the viewport size derived from the document's viewport and its scale; the logical viewport and its scale rule decide what value is correct, and the test must assert that value, not a hard-coded guess that ignores the scale.
@task_item n=3 part=needs
The new test lives inside the browser crate's own test module and builds what the render takes, a document with a set viewport and a paint scene, under the crate's default features; it reaches the render function directly, not through the screenshot path, since both callers discard the size.
@task_item n=4 part=needs
Invariant: what the browser paints and saves stays as it is; the scene render's painting and both callers (the screenshot capture, which resolves its own size before rendering, and the anyrender scene capture) behave as before, held by the existing browser tests; if the fork removes the return value, only the unused size goes.
@task_item n=5 part=needs
The mutation tier: a cargo-mutants run of the browser crate, even of these four alone, is planned as an operator entry or a fork for the founder and starts only on his word; it is never a gate that fires by itself, and it runs on the dev host only, never in CI.
@task_item n=6 part=needs
No new carrier, driver verb, log field, span, environment read or listener; no new screenshot refusal or outcome; the change stays inside the capture module and the browser crate's test run.
@task_standing n=1
convention: a new test that renders a document is built with the system fonts the tests use and never paints with text skipped
home: typography
@task_item n=1 part=not_needed
Spacing, margins and cell spacing hints: the chunk lays out nothing new and asserts no box geometry.
@task_item n=2 part=not_needed
Depth strategy, stacking, paint order and hit-testing: the chunk changes no paint order and tests no hit.
@task_item n=3 part=not_needed
Motion and the virtual clock, driver advance, smooth scroll and scrollbar fades: the test advances no clock and scrolls nothing.
@task_item n=4 part=not_needed
Iconography, cursors and favicons: nothing in the chunk sets a cursor or reads an icon link.
@task_item n=5 part=not_needed
The select list: its overlay, rows, events, gestures and accessibility tree are untouched.
@task_item n=6 part=not_needed
escher-session commands, answers, exit statuses and codes, stdout and stderr lines, the mcp role and its tasks: the chunk adds and changes no command line or session output.
@task_item n=7 part=not_needed
The actions, coverage and UI graph lines and the coverage check files: the chunk reads and writes none of them.
@task_item n=8 part=not_needed
WPT runner output and its result colours: the chunk does not touch the WPT runner.
@task_item n=9 part=not_needed
The shell provider's window controls (minimize, maximize, drag, decorations) and the JS window object's sizes: the chunk exposes no window size to script and moves no window.
@task_item n=10 part=not_needed
The cli and web-spa surfaces: no argument parsing, WASM page or canvas sizing changes.
