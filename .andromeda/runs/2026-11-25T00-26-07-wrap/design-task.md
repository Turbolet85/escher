@task_prose part=session
The session taught the driver's lint verb to read inside a canvas: a label a canvas widget paints now reads the existing truncated defect when its glyph ink box is not inside its region.
A label's region is the element's content box, cut by each pushed clip, layer or mask path's bounding box and by the element's clip region; the ink box is taken from each glyph's outline bounds in the run's own font, through skrifa.
The lint answer's written form, its keys and the six-defect set are unchanged; a canvas finding carries the label's box as numbers in text and visible, and no label text crosses.
Circle Drawer reads no canvas finding at boot, after a click and at 12 and 100 circles, in both layout modes; a planted page of six numbered cases proves the reading.
It measured «Wide label» at 20 px in DejaVu Sans as an ink box 102.02 by 15.48 CSS px, which moved one planted case from x 110 to x 90.
Four lint-like canvas checks were left out and unowned: labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region, and a shape with no area.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: new; the canvas label reader of the driver's lint — each glyph run a canvas paints, its ink box and the clip boxes in force over it
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: new; the unit tests of the canvas label reader — clips, nested pushes, transforms, layers and masks, an empty pop, an unreadable font
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: new; the stand test of the canvas lint — a planted page of numbered canvas label cases and Circle Drawer at four states, in both layout modes
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: edited; the driver crate's manifest, which now takes the workspace's skrifa
@task_file n=5 path=Cargo.lock
what: edited; the workspace lock, one added dependency edge from the driver to skrifa
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: edited; the driver crate root — declares the canvas label module and states the lint paragraph of the crate doc
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: edited; the driver's lint verb and its defects — the truncated reading now also reads each canvas label against its region
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited; the command schema of escher-session — the lint verb's help text and its findings field's help, served by help and the MCP tool listing
@task_file n=9 path=docs/capabilities.md
what: edited; the capability statement — the visual lint section and the list of where each claim is proven
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; the shared stand test helpers, which now hold the Circle Drawer canvas helpers moved from the canvas commands test
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited; the stand test of the canvas draw commands, now importing its Circle Drawer helpers from the shared helpers
@task_item n=1 part=touches
The canvas verb's line form conventions: a lint finding for a canvas label now reports boxes in CSS px taken from the canvas's paint commands, beside the canvas verb's own reading of those commands.
@task_item n=2 part=touches
The driver verbs and the virtual clock contract for canvas: the canvas's paint commands are now read by a second verb, lint, under the same settle; the canvas verb itself is unchanged.
@task_item n=3 part=touches
The escher-session help command: the lint verb's help and its findings field's help now name the canvas label and its ink box.
@task_item n=4 part=touches
The escher-session mcp role's help tool result and tool listing: the same changed lint help text is served there.
@task_item n=5 part=touches
The escher-session mcp role's other tool results and the command line output line: the lint answer can now hold canvas findings, with its keys from the same closed set and no label text.
@task_item n=6 part=touches
Driver command line output read by agents, and escher-session read by an agent: a canvas label's truncation is now readable by an agent through lint where before only DOM text was.
@task_item n=7 part=touches
Circle Drawer canvas colours as widget constants: Circle Drawer's canvas labels now reach the lint as boxes, and the stand pins that it reads no canvas finding at every state.
@task_item n=8 part=touches
Fonts and typography rows on fallback and system fonts: a canvas label's ink box is read from the glyph outline bounds of the run's own font at its size and variation coordinates, so a missing or unreadable font yields no label rather than a refusal.
@task_item n=9 part=touches
Hit-testing stops where paint clips, and stacking-context clipping: a canvas label's region is cut by every pushed clip, layer or mask path's bounding box and by the element's clip region, a parallel reading of paint clips inside a canvas.
@task_item n=10 part=touches
Actions decided by the verbs' own checks: the lint verb's check now includes canvas labels, which any rule leaning on the lint's checks inherits.
@task_fact n=1 slug=surface
fact: The lint reads a label a canvas widget paints as truncated when its glyph ink box is not inside its region — the element's content box cut by each pushed clip, layer or mask path's bounding box and by its clip region — once per label in paint order, with no label text in the finding (PROVISIONAL, a delegate's answer).
@task_fact n=2 slug=surface
fact: The canvas lint does not read labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region, or a shape with no area; these are left unowned residuals.
@task_item n=1 part=untouched
Colour tokens, the engine's hardcoded colours, color-scheme recascade and the select list colours: no colour value or palette moved.
@task_item n=2 part=untouched
The project type scale, font element hints, lang locale, root-relative units and the WASM bundled font context: no font choice, fallback or unit rule changed.
@task_item n=3 part=untouched
Spacing, depth, border radius and motion rows: no margin, z-index, radius, animation, scroll or virtual clock behaviour moved.
@task_item n=4 part=untouched
The select list rows, its events, accessibility tree and overlay: the select list is not reached.
@task_item n=5 part=untouched
The escher-session exit statuses and exit codes, stderr errors, session error remedy and the tasks list: no status, error or task was added or changed.
@task_item n=6 part=untouched
The screenshot, snapshot, layout, graph, coverage and actions answers and their arguments: their forms are unchanged.
@task_item n=7 part=untouched
The WPT runner output and its result colours, cursor icons, favicon and shell provider window controls: not reached.
@task_item n=8 part=untouched
The web-spa surface's WASM canvas: that page canvas is the render target, not a canvas widget the lint reads.
