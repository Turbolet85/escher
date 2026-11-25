@task_prose part=session
The session added canvas lint: the lint verb now reads truncated once per label a canvas widget paints whose ink box is not inside its region, in paint order, chained after the DOM truncated reading and before contrast.
The region is the element's content box cut by each pushed clip, layer or mask path's bounding box and by the element's clip region; a label's ink box is read from its glyph outline bounds through skrifa.
A canvas finding carries only numbers (the label's ink box and its visible part) under the closed key set; no label text or glyph id crosses, and the six-defect set and the answer's written form are unchanged.
Circle Drawer's labels now reach the lint as truncated boxes with no label text, and Circle Drawer reads no canvas finding at boot, after a click and at 12 and 100 circles, in both layout modes.
The new stand target stand_canvas_lint joins the stand-repeat rounds by the existing glob; the named-target count stays seventeen and CI read green 17/17.
Four lint-like checks were left out and unowned: labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region, and a shape with no area.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: new; the canvas label reader of the visual lint, taking each painted label's ink box and the clip box it is painted under
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: new; unit tests of the canvas label reader (clip, layer and mask boxes, a pop on an empty stack, an unreadable font)
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: new; the stand check proving the canvas lint on a planted page of numbered cases and on Circle Drawer at four states, and that the lint verb's line carries no label text
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: edited; the session library's dependency list, now taking skrifa
@task_file n=5 path=Cargo.lock
what: edited; the workspace lock, one added dependency edge for skrifa
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: edited; the session library root, declaring the canvas lint module and its crate doc's lint paragraph
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: edited; the visual lint (the truncated, clipped and contrast readings and their findings), now reading canvas label truncations before contrast
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited; the driver schema's verb help, the lint verb's help and its findings field naming the canvas label and its ink box
@task_file n=9 path=docs/capabilities.md
what: edited; the capability statement, its visual lint section and the list of where each claim is proven
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; the shared stand test helpers, now holding the Circle Drawer canvas helpers moved in
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited; the Circle Drawer canvas draw-command stand check, now importing its helpers from the shared module
@task_item n=1 part=touches
Circle Drawer's drawing area as one named canvas node: the row saying its labels reach no lint is now stale; its labels reach the lint as truncated boxes with no label text, while the node and its pixels-only reach to the accessibility tree are unchanged.
@task_item n=2 part=touches
The visual lint finding's content: a finding's numbers now include a canvas label's ink box and its visible part, under the same closed key set, with no text.
@task_item n=3 part=touches
Stand reads no lint finding at boot: Circle Drawer now also reads no canvas finding at boot, after a click and at 12 and 100 circles, in both layout modes.
@task_item n=4 part=touches
Lint contrast through one formula and lint contrast sharing the harness reader: the lint's readings order changed, canvas truncations now chained before contrast; the contrast reading itself and its formula are unchanged, so confirm only.
@task_item n=5 part=touches
Contrast unread past non-colour paint: a canvas label's contrast against the fill under it is still not read; the new canvas reading checks only containment, so that limit now extends to canvas labels.
@task_item n=6 part=touches
Stand checks repeated three rounds: the stand-repeat set gains stand_canvas_lint by the glob; the named-target count of seventeen is unchanged; stand-repeat read 3 of 3 rounds green.
@task_item n=7 part=touches
Option labels leave an open list only as pixels and other pixels-only rows: the lint now reads painted canvas labels' boxes, but only canvas widget labels, not an open select list's rows; check the row's wording names no lint reach that is now wrong.
@task_item n=8 part=touches
The a11y CI leg, the accessibility test leg and accessibility checks gating CI: the new stand target joins the test and stand-repeat legs by the existing glob with no leg change; confirm no row's list of stand targets is now stale.
@task_fact n=1 slug=visual-design-verification
fact: The visual lint reads no contrast for a label a canvas widget paints against the fill under it (nor overlap between canvas labels); that check was left out with no owner, and a canvas label is read only for being cut from its region.
@task_fact n=2 slug=screen-reader-support
fact: A canvas widget's painted labels reach the lint only as numeric boxes under truncated; their text and glyph ids never cross any carrier, and they still give no accessibility tree node beyond the canvas element's own.
@task_item n=1 part=untouched
The accessibility tree itself, its roles, names, states, hidden and inert content, and the AccessKit adapter: no tree builder, role mapping or adapter changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, Tab order, activation keys, select and range input key handling: no input or focus behaviour changed.
@task_item n=3 part=untouched
The snapshot, its text form, the actions answer, the UI graph and the screenshot: no reader of the snapshot model changed.
@task_item n=4 part=untouched
The lint's contrast finding contract items (bounds, defect, id, ratio, required) and the WCAG thresholds and ratio helper: unchanged.
@task_item n=5 part=untouched
The accessibility feature defaults, the platform adapter absence on the stand binaries and the tier-0 reach: unchanged.
@task_item n=6 part=untouched
Cognitive accessibility rows and the select list colour, motion and row-height rows: not reached.
