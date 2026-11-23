@task_prose part=chunk
Measures what a headless shot of a canvas (a widget painted from Rust through Widget::paint) reads on each CI host: linux x86_64, macOS aarch64 and windows x86_64, in both layout modes, as the PNG's bytes and as the decoded RGBA raster.
The measurement reaches macOS and Windows only through CI runs; it tells the encoder apart from the rasterizer as the cause of any difference.
Re-measures the stated basis of the no-shot cross-OS rule (a shot's PNG bytes differ between linux x86_64 and macOS aarch64) for a canvas shot at HEAD with bundled fonts.
Puts the readings to the founder as one fork: canvas-shot byte equality across hosts required (amending the no-shot cross-OS rule) or held within a tolerance he sets.
After his word, adds a stand check holding a canvas shot on every CI OS to that word, equal to one committed table or within the tolerance.
@task_file n=1 path=.github/workflows/ci.yml mark=re-reads
what: the CI matrix whose cargo test --all --tests legs on linux, macOS and windows and the linux test leg already run every blitz-tests target
@task_item n=1 part=needs
The screenshot verb and the driver screenshot result: a canvas shot's pixels cross only as they already do, with no new verb, argument, carrier, result field, refusal cause, session error, exit, log field or span.
@task_item n=2 part=needs
The headless shot of a canvas widget painted from Rust through Widget::paint, read on each host in both layout modes as PNG bytes and as decoded RGBA raster.
@task_item n=3 part=needs
The no-shot cross-OS rule (no screenshot in cross-OS byte checks; cross-OS checks hold no shot): the chunk measures its basis for a canvas shot and the founder's word may amend it or keep it with a tolerance.
@task_item n=4 part=needs
Screenshot never paints with text skipped: a canvas shot taken for the measurement or the stand check paints its text, so glyph coverage is part of what is compared.
@task_item n=5 part=needs
Fonts for a shot: the tests are built with system fonts and the shot refuses with fonts-unavailable where none can be found; the canvas measured runs with the bundled fonts the basis predates.
@task_item n=6 part=needs
Unavailable font families fall back to the default font and no generic family lays out as sans-serif: the canvas text measured on each host resolves its face through these.
@task_item n=7 part=needs
The canvas measured is a stand fixture booted alone, as the existing canvas shot, placement and text stands boot theirs, not Circle Drawer.
@task_item n=8 part=needs
The kept stand check keeps no pixels: it holds a canvas shot to one committed table or to a tolerance, never to a stored image.
@task_item n=9 part=needs
The committed Counter checks stay free of a screenshot step unless the founder's word amends that rule too.
@task_item n=10 part=needs
Engine colours are hardcoded constants and the base colour comes from the dioxus-native configuration: the solid fill and background the shot reads are these fixed values on every host.
@task_item n=11 part=needs
The logical viewport is physical size over total scale: the canvas shot's raster dimensions on each host follow from this.
@task_item n=12 part=needs
The founder's fork written into evidence with what crosses, the rule it meets, its options with measured cost (CI wall-clock per run, no billed minutes) and its dependences; technical forks go to the operator as dialogs.
@task_item n=13 part=needs
Each new control comes with its red reading; no raw log with a temp or home path goes into evidence.
@task_standing n=1
convention: each new test is a stand fixture booted alone in the blitz-tests targets, run by the existing test legs, with a red reading shown for it
home: surface
@task_standing n=2
convention: each new shot check compares no stored pixels and no PNG byte equality across OSes unless the founder's word amends the no-shot cross-OS rule
home: surface · typography
@task_item n=1 part=not_needed
Select list rows: their colours, placement, gestures, events, accessibility, snapshot bounds and screenshot row; the canvas fixture holds no select.
@task_item n=2 part=not_needed
The escher-session command answers for actions, coverage, graph, layout, snapshot and help, its tasks list and its MCP results other than the screenshot result; the chunk adds no verb, argument or field.
@task_item n=3 part=not_needed
The escher-session exit statuses and exit codes, stderr errors and session error remedy; no new exit or refusal cause is added.
@task_item n=4 part=not_needed
Motion and the virtual clock: driver advance, click, press, type, scroll behaviour, scrollbar fades and animation; the shot is of a static canvas.
@task_item n=5 part=not_needed
Depth and hit-testing: stacking contexts, z-index paint levels and hit-test clipping; nothing is clicked or hit-tested.
@task_item n=6 part=not_needed
Shell provider window controls, cursor icons, favicon and window compositing alpha mode; the shot is headless.
@task_item n=7 part=not_needed
The WPT runner's output, result colours and test-name hyperlinks.
@task_item n=8 part=not_needed
Web-spa WASM pages, the bundled font context on WASM and mobile targets; the measured hosts are the three desktop CI OSes.
@task_item n=9 part=not_needed
The JS window object, script runtime document.fonts and MountedData calls during a render.
@task_item n=10 part=not_needed
Presentational hints: font element face and size, table cellspacing, body margins, lang locale and root-relative units.
