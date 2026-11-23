@task_prose part=session
The session fixed where a custom widget (canvas) paints: its paint size is now its content box times scale, and its scene is placed at the content origin inside the element's own transform, unclipped (upstream issue 1578).
A custom widget now hears pointer events in its content-box frame, offset by its left and top border and padding; a sub-document's pointer events are unchanged, still from its border-box origin.
The widget trait gains one defaulted method that hands the widget the document's own shared font context before each paint, and the document gains a public reader of that font context, so canvas text is shaped from the bundled fonts.
Three new integration checks witness the paint placement, the widget pointer frame and the canvas text, in both layout modes; the stand check files move 66 to 69, and the stand repeat leg takes them by prefix with its named-target count unchanged.
The test crate takes parley as a dev-dependency (one lock line, no package added), and cookie moves 0.18.2 to 0.18.3 lock-only for RUSTSEC-2026-0594; no log site, span, field, carrier, verb or accessibility surface is added.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: edited; the paint crate's custom widget scene building, now sizing a widget's paint to its content box and handing it the document's font context before paint
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: edited; the element renderer that appends a custom widget's scene, now at the content origin inside the element's transform
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: edited; the engine's DOM event dispatch, which forwards pointer events to a custom widget, now in the widget's content-box frame
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited; the custom widget trait, gaining a defaulted method that receives the document's font context
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: edited; the engine's base document, gaining a public reader of its shared font context
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: edited; the integration test crate's manifest, gaining a parley dev-dependency
@task_file n=7 path=Cargo.lock
what: edited; the workspace lock file, one dependency line added for the test crate and the cookie version and checksum moved to 0.18.3
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: new; the check that a padded or transformed canvas widget paints its content box at the right size and place
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: new; the check that a padded canvas widget hears pointer events from its content-box origin
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: new; the check that canvas text is shaped from the document's own fonts and matches the bundled route
@task_item n=1 part=touches
Pointer coordinates forwarded to a custom widget changed frame: any row on how a pointer event's position reaches an element's own handling, or on hit position within an element, may need to say a widget now hears it from its content-box origin while a sub-document keeps the border-box origin.
@task_item n=2 part=touches
Range input rows that read a content box (the main-button pointerdown and click value rows, the range pointerdown decision): check they stay a native control's own rule and are not a custom widget's; the sweep found them sharing the content-box token only.
@task_item n=3 part=touches
Driver screenshot rows (an element shot, a row shot) and the screenshot-changes-nothing decision: a canvas widget's pixels inside an element shot now sit at its content box, not its border box.
@task_item n=4 part=touches
Contrast unread past non-colour paint and stand contrast measured from computed styles: a canvas widget's painted content and text are non-colour paint the contrast reader does not reach; check no row claims it measures canvas pixels.
@task_item n=5 part=touches
Hidpi or zoom change reshapes text: canvas text is now shaped from the document's shared font context, which app code can hold and mutate; check whether the reshaping rule claims any reach over widget-drawn text.
@task_item n=6 part=touches
Stand checks repeated three rounds and the accessibility test leg: three new stand check files join the repeat leg by prefix, with no named-target count changed; touch only where a row's cell holds the file count or the target count.
@task_item n=7 part=touches
Assistive technology reach absence and the tier 0 decisions: a canvas widget still exposes no accessibility node of its own and no new adapter or tree surface is added; check these rows stay true and gain no exception.
@task_item n=8 part=touches
Snapshot reads the accessibility tree and the UI graph mirrors the snapshot: no snapshot, actions or UI graph field changed, though a widget's painted text is now real text pixels that leave only as pixels.
@task_item n=9 part=touches
Accessibility checks gate CI: the CI run on the pre-CI commit read green 17 of 17 with the dependency audit job green on cookie 0.18.3; touch only where a row holds the check count or the audit's reading.
@task_item n=1 part=untouched
ARIA role mapping, ARIA property and state reads, hidden and inert content exclusion: no role, name, state or hidden-content rule changed.
@task_item n=2 part=untouched
Keyboard navigation: Tab order, focus rules, keyboard activation, focus fixup and focus-visible matching are unreached; no key handling changed.
@task_item n=3 part=untouched
Single and multiple select, the open select list, its rows, colours, motion and overlay: unreached.
@task_item n=4 part=untouched
Accessible name precedence, label binding, password masking and the snapshot text form: unreached.
@task_item n=5 part=untouched
The lint's contrast finding fields, the WCAG contrast ratio helper and text contrast thresholds: no formula, threshold or finding field moved.
@task_item n=6 part=untouched
Cognitive accessibility, the seven_guis Cells grid and CRUD list contracts, and the UI graph state registration contracts: unreached.
@task_item n=7 part=untouched
The accessibility feature defaults, the platform adapter choice and the stand binaries building no adapter: no feature or dependency of the accessibility stack moved.
