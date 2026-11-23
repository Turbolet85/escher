@task_prose part=session
The session makes what an app paints from Rust through a custom widget appear in a harness shot and in a driver shot, whole and cropped to the element, in process only.
The test harness now turns on the paint crate's custom-widget feature, so the widget paint pass is compiled into every build selection that reaches the harness: the harness, the driver and the session binary, per package and in the workspace build.
A new stand check, stand_canvas_shot, measures it in both layout modes: a red widget pixel inside the canvas, white outside, the element shot 200 by 100 all red, the whole driver shot equal to the harness raster.
No verb, argument, result field, cause, session error, exit, log field or span was added; no product source changed; the stand still builds no platform accessibility adapter (the probe reads 0).
The test crate takes the colour-type library as a dev-dependency (one lock line, no new package); CI went green 17 of 17, the a11y leg and the repeated stand checks included.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: a new stand check, a stand_* target: a widget painted from Rust shows in a harness shot and in a driver shot, whole and as an element shot, in both layout modes
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: the test harness's dependencies and feature set; edited to name the paint crate's custom-widget feature
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the stand checks crate's manifest and dev-dependencies; edited to add the colour-type library as a dev-dependency
@task_file n=4 path=Cargo.lock
what: the workspace lock file; edited with one line, no package added
@task_item n=1 part=touches
No platform adapter on the stand binaries: the harness, driver and session binary feature set moved (custom-widget paint added) and the platform-adapter probe still reads 0, so the decision holds under the new feature set.
@task_item n=2 part=touches
A platform accessibility adapter absent from every stand binary (assistive technology reach): re-measured under the moved feature set, still absent.
@task_item n=3 part=touches
Stand checks repeated three rounds: a new stand_* target joins the repeated stand checks by the existing glob; no leg or script changed.
@task_item n=4 part=touches
The a11y CI leg and accessibility checks gating CI: green on the push with the moved harness feature set; no leg changed.
@task_item n=5 part=touches
A screenshot changes nothing: the driver shot now carries app-painted widget pixels, whole and cropped, and still changes nothing.
@task_item n=6 part=touches
The driver's screenshot, an element shot: an element shot of a widget-painted element is measured, its bounds 200 by 100 and every pixel the painted colour.
@task_item n=7 part=touches
Option labels leave an open list only as pixels: the precedent beside which an app's canvas content now also leaves a session only as pixels in a shot.
@task_item n=8 part=touches
Session library always enables accessibility and snapshot readers behind the accessibility feature: the driver's build now also compiles the widget paint pass; accessibility features themselves unmoved.
@task_item n=9 part=touches
Accessibility tree built through the document's own method and the adapter reading the document's own tree: a page now holding a custom widget is shot; no reading of the tree was added or changed for it.
@task_fact n=1 slug=screen-reader-support
fact: An app's canvas content, painted from Rust through a custom widget, leaves a session only as pixels in a harness or driver shot; no snapshot field, carrier, cause or log field carries it.
@task_item n=1 part=untouched
ARIA role mapping, accessible name precedence, states and property reads: no role, name or state rule moved.
@task_item n=2 part=untouched
Keyboard navigation, focus, Tab order, select list and range input contracts: no input handling changed.
@task_item n=3 part=untouched
Contrast measurement, the lint's contrast finding and stand colours: no colour, threshold or lint rule changed.
@task_item n=4 part=untouched
Hidden and inert content rules, hit-testing and driver click refusals: untouched.
@task_item n=5 part=untouched
The snapshot text form, snapshot diff, actions answer, UI graph and MCP results: no field, token or cause added.
@task_item n=6 part=untouched
Cognitive accessibility rows and the accessibility tier 0 statement: unmoved.
