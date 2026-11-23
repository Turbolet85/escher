@task_prose part=session
The session fixes upstream Blitz issue 1578: a custom widget is now painted at its content box. Its paint size is the border box less border and padding, times the scale, and its scene is placed at the content origin inside the element's own transform, still unclipped.
A custom widget's pointer events now arrive in its content-box frame: the position plus the left and top border and padding, in CSS px, untransformed. The sub-document forwarding arm is unchanged.
The Widget trait gains a defaulted public method, fonts, which hands the widget the document's own shared font context before each paint. BaseDocument gains a public font_ctx reader behind the custom-widget feature. No implementor changed.
Three new stand checks witness the fixes in both layout modes, the text one also under font-embolden. blitz-tests takes parley as a workspace dev-dependency.
The lock moves cookie 0.18.2 to 0.18.3 by a hand edit of two lines, which fixes advisory RUSTSEC-2026-0594. The syn edges stay at 3.0.6 and deny.toml is unchanged.
No IPC method, verb, argument, carrier, env var, log field or span is added, and nothing is retired.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: blitz-paint's crate root, which builds custom widget scenes: the widget's paint size, the font hand-off before paint, and the call to the widget's paint
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: blitz-paint's element rendering, where a custom widget's scene is appended into the paint scene under the element's transform
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: blitz-dom's DOM event dispatch, which forwards pointer events to custom widgets and sub-documents with adjusted coordinates
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the blitz-dom Widget trait, the public contract a custom widget implements (paint, handle_event, and now fonts)
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: BaseDocument, the core DOM document and its public readers, now including font_ctx
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests integration test crate's manifest, with its dev-dependencies and test targets
@task_file n=7 path=Cargo.lock
what: the workspace lockfile, holding the resolved dependency versions the locked cargo legs and the dependency audit read
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: a new blitz-tests stand check, the guard for a custom widget's content-box paint size and placement inside its transform
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: a new blitz-tests stand check, the guard for a custom widget's pointer events in its content-box frame
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: a new blitz-tests stand check, the guard for canvas text shaped from the document's own font context
@task_item n=1 part=touches
decision/custom-widget-object: the custom widget as an object element now paints at its content box, not its border box, and its scene sits at the content origin inside the element transform. A decision on paint placement belongs beside it.
@task_item n=2 part=touches
contract/nodes-widgets/widget-trait: the Widget trait gains the defaulted public method fonts. Its standing promises still hold, and a contract item for fonts belongs beside them.
@task_item n=3 part=touches
decision/forwarded-event-coordinates: a widget's forwarded pointer coordinates are now in its content-box frame, a finer statement than the standing one.
@task_item n=4 part=touches
decision/sub-document-event-forwarding: the widget arm now receives content-box coordinates, while sub-documents keep the border-box origin. The rule over both arms now differs per arm.
@task_item n=5 part=touches
contract/dom-events/pointer-events and contract/event-dispatch/targeting: pointer-derived events delivered to a custom widget change their coordinate frame.
@task_item n=6 part=touches
The BaseDocument contracts (contract/base-document/*): BaseDocument gains a public font_ctx reader behind custom-widget, and a contract item belongs beside these.
@task_item n=7 part=touches
decision/feature-gated-widgets: every new paint, event and font-context item is behind the custom-widget feature.
@task_item n=8 part=touches
decision/harness-paints-custom-widgets and decision/harness-composites-over-white: the three new checks read widget pixels through the harness screenshot with custom-widget paint on.
@task_item n=9 part=touches
decision/canvas-crosses-as-pixels: canvas text and placement are still witnessed only as pixels. This is indirect, through the widget paint path.
@task_item n=10 part=touches
decision/parley-text-layout and decision/stylo-parley-type-aliases: a widget is now handed Parley's FontContext, the document's own, behind a shared Mutex.
@task_item n=11 part=touches
decision/stand-uses-bundled-font and decision/stand-typography-dejavu-sans: canvas text drawn from the document's fonts is byte-equal to the bundled-font route.
@task_item n=12 part=touches
decision/font-embolden-local-witness and contract/ci-leg-legs/embolden: the canvas text check is also green under blitz-paint/font-embolden.
@task_item n=13 part=touches
decision/fork-additions-in-own-functions: the paint size and pointer origin are escher's own functions, each a single call in upstream's body.
@task_item n=14 part=touches
decision/direct-engine-changes-flagged-upstreamable: the paint placement, the widget pointer frame, Widget::fonts and BaseDocument::font_ctx are flagged upstreamable, with nothing sent.
@task_item n=15 part=touches
decision/paint-to-paint-scene and contract/paint/paint-scene: a widget's scene is now appended in the element's transform at the content origin (M times T, not T times M).
@task_item n=16 part=touches
decision/blitz-tests-one-target-per-file, decision/integration-tests-in-blitz-tests and decision/blitz-tests-single-test-target: three new integration targets, one file each, added to the blitz-tests manifest.
@task_item n=17 part=touches
decision/both-layout-modes and decision/scenarios-both-layout-modes: each new check runs in both layout modes.
@task_item n=18 part=touches
contract/ci-leg-legs/stand-repeat and decision/stand-repeat-zero-failure-budget: stand-repeat takes the three new stand checks by prefix. Its named-target count does not change.
@task_item n=19 part=touches
decision/workspace-dependencies and decision/workspace-shared-dependencies: blitz-tests takes parley with workspace = true as a dev-dependency. The lock gains one line and no package.
@task_item n=20 part=touches
decision/cargo-deny-advisory-gate, decision/dependency-and-licence-audit and contract/ci-leg-legs/audit: RUSTSEC-2026-0594 is fixed by the lock-only cookie 0.18.3 move, with no ignore. The audit reads advisories ok and licences ok.
@task_item n=21 part=touches
decision/cargo-legs-locked and decision/locked-cargo-legs: the lock was edited by hand on its version and checksum lines, and every locked leg is green on it.
@task_item n=22 part=touches
decision/public-item-docs and decision/module-and-item-docs: the two new public items, Widget::fonts and BaseDocument::font_ctx, carry item docs.
@task_item n=23 part=touches
contract/custom-widget-embed/object-data: an embedded widget's paint size and event frame are now its content box.
@task_fact n=1 slug=established-decisions
fact: A custom widget's paint size is its content box times the scale (the border box less border and padding, floored at 0). Its scene is appended at the content origin inside the element's transform and stays unclipped. This follows upstream issue 1578, and the guard is stand_canvas_placement.
@task_fact n=2 slug=cross-cutting-patterns
fact: A custom widget hears pointer events in its content-box frame: the position plus left and top border and padding, in CSS px, untransformed. A sub-document still hears them from its border-box origin. The guard is stand_canvas_pointer_frame.
@task_fact n=3 slug=standard-contracts
fact: The Widget trait's defaulted fonts method hands the widget the document's own shared Parley font context before each paint. The default ignores it, and the guard is stand_canvas_text.
@task_fact n=4 slug=standard-contracts
fact: BaseDocument::font_ctx, public and behind custom-widget, returns a clone of the handle to the document's shared font context (Arc Mutex FontContext). App code holding it shares and can mutate the engine's own font context.
@task_item n=1 part=untouched
The driver's session carriers, verbs, arguments, answer JSON, MCP role, session socket wire, refusal causes and session errors: no carrier, verb, argument or field is added.
@task_item n=2 part=untouched
Telemetry, logging and spans: no log site, span or field is added, and the new checks read silent.
@task_item n=3 part=untouched
The accessibility tree, ARIA readings and snapshot node keys: nothing in them changed.
@task_item n=4 part=untouched
The select list, form controls, range input and password masking: their paint and event rules do not change.
@task_item n=5 part=untouched
Scroll, hit testing, visible region and CSSOM geometry readers: the hit walk and geometry readings are unchanged, and only the coordinates forwarded to a widget moved.
@task_item n=6 part=untouched
The WPT runner, its outputs and its CI artifacts.
@task_item n=7 part=untouched
Occupied names (crates, binaries, env vars, servers): no crate, binary, env var or port is taken or freed.
@task_item n=8 part=untouched
The browser app, its history, about pages and persistence.
@task_item n=9 part=untouched
Style, Taffy layout, incremental layout and damage: layout values are only read, never changed.
@task_item n=10 part=untouched
CI workflow structure, action pins, coverage floors and the Nix flake: no leg or job is added or changed.
