@task_prose part=session
The session fixed where an app widget painted from Rust paints (upstream #1578): its paint size is now its content box times scale, and its scene sits at the content origin inside the element's transform, unclipped.
A widget now hears pointer events in its content-box frame (border and padding taken off); a sub-document's pointer events are unchanged, still from its border-box origin.
The widget trait gained a defaulted fonts method that hands the widget the document's own font context before each paint, and the document gained a public font_ctx accessor; no existing widget changed.
Three new blitz-tests integration targets witness the three changes, each in both layout modes, each read red before its fix, with measured post-fix literals pinned; the text check also passes under font-embolden.
blitz-tests took parley as a dev-dependency (one lock line, no package added), and the lock moved cookie 0.18.2 to 0.18.3 by hand (version and checksum lines only) to clear RUSTSEC-2026-0594; the audit leg reads advisories ok, licenses ok, with the deny config unchanged.
No log site, span, field, IPC verb, carrier or exit code was added; no capability was claimed.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: edited — the scene painter's custom-widget path: the size a widget is handed to paint at, and the handoff of the document's font context before paint
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: edited — the element renderer that places an app widget's painted scene inside the element's transform
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: edited — the document's event dispatch that forwards pointer events to an app widget with coordinates in its own space
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the app widget trait (the custom-widget object an app paints from Rust), gaining a defaulted fonts method
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: edited — the base document, gaining a public accessor to its shared font context
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: edited — the blitz-tests manifest, gaining a parley dev-dependency
@task_file n=7 path=Cargo.lock
what: edited — the workspace lock: one added dependency line for blitz-tests, and the cookie entry's version and checksum moved to 0.18.3
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: new — the stand check of an app widget's paint size and paint origin in a headless shot, both layout modes
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: new — the stand check that an app widget hears pointer events from its content box, both layout modes
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: new — the stand check that canvas text is shaped from the document's own fonts, both layout modes and under font-embolden
@task_item n=1 part=touches
App widget painted from Rust shows in shots: the widget's painted size and position in a shot moved to its content box, inside its transform.
@task_item n=2 part=touches
Canvas handlers and handler coverage limits (canvas handler, canvas handlers not counted): a canvas widget's pointer events now arrive in its content-box frame; check whether a row states the frame.
@task_item n=3 part=touches
Harness input helpers, pointer coordinates: a pointer event dispatched at a widget is now heard less the widget's border and padding.
@task_item n=4 part=touches
One test target per file / per behaviour, and one blitz-tests target per file: three new targets, one file each.
@task_item n=5 part=touches
blitz-tests manifest holds one test target, and the blitz-tests manifest's difference from upstream: the manifest gained a parley dev-dependency.
@task_item n=6 part=touches
Stand checks run in both layout modes, and layout assertions in both layout modes: the three new checks loop both modes.
@task_item n=7 part=touches
Stand checks on the bundled font, no skip; stand checks keep a real pinned font; font-metric tests never skip: the canvas text check shapes from the bundled fonts through the document's own font context.
@task_item n=8 part=touches
Embolden witness leg without a CI job, and the local witness for font-embolden paint: the canvas text check is also run green under font-embolden.
@task_item n=9 part=touches
Every requirement witnessed inside stand-repeat, and stand checks repeated three rounds: stand-repeat takes the three new targets by their prefix; its named-target count is unchanged.
@task_item n=10 part=touches
Shared test modules are no target, session checks share one module, shared fixtures in one module: the new files declare neither shared module, so no declarer count moves.
@task_item n=11 part=touches
Tests controlled red-before-green, and expected value pinned as a literal: each new check read red before its fix, and the literals pinned are the measured post-fix values.
@task_item n=12 part=touches
Regression tests named for and documenting their bug: the placement check answers upstream #1578.
@task_item n=13 part=touches
Pixel tests on a CPU buffer, headless screenshot from the harness, PNG decoding in checks, frames compared whole: the placement and text checks read shot pixels and a byte-equal region.
@task_item n=14 part=touches
Dependency audit (advisories and licenses): RUSTSEC-2026-0594 read red on cookie 0.18.2 and is cleared by the lock-only 0.18.3 move, no ignore added.
@task_item n=15 part=touches
Cargo legs run locked: the lock was edited by hand, and every gate's cargo ran with --locked, green with no warning.
@task_item n=16 part=touches
Build directory held by a sweep, target sweep safe around a build, shared workspace test cache: a stale build on the shared target read a crate Fresh against edited sources and was cleared by a per-crate clean.
@task_item n=17 part=touches
No ids or names in stand logs, and log silence read on a sink host: the three new checks read silent and add no log site.
@task_fact n=1 slug=test-strategy
fact: An app widget's paint placement is witnessed by stand_canvas_placement: it is handed its content box times scale and its scene paints at its content origin inside the element's transform, in both layout modes.
@task_fact n=2 slug=test-strategy
fact: An app widget's pointer frame is witnessed by stand_canvas_pointer_frame: it hears pointer events from its content box, page and client alike, in both layout modes.
@task_fact n=3 slug=test-strategy
fact: Canvas text from the document's fonts is witnessed by stand_canvas_text: the widget is handed the document's own font context, and its text region is non-blank and byte-equal to the bundled-font route, in both layout modes and under font-embolden.
@task_fact n=4 slug=ci-integration
fact: When another cargo shares the build directory, cargo can read a crate Fresh against edited sources and neither a re-run nor a touch clears it; a per-crate clean does, and gate readings taken before it read the pre-edit code.
@task_item n=1 part=untouched
The session wire, session host, session state directory and session errors contracts: no verb, bound, field or error was added.
@task_item n=2 part=untouched
The escher-session command line, CLI and MCP role contracts, their exit codes and stderr lines: no command, argument or output changed.
@task_item n=3 part=untouched
Driver refusal causes, driver verbs and the driver session's acting and reading verbs: no cause or verb moved.
@task_item n=4 part=untouched
The agent-run and cold-agent contracts, verbs, events, exit codes and verdicts.
@task_item n=5 part=untouched
WPT runner, status set, conformance and pinned commit rows.
@task_item n=6 part=untouched
Coverage floors, mutation testing, complexity ceiling and oversize-file rows: no floor, budget or threshold moved.
@task_item n=7 part=untouched
Harness settle, virtual clock, advance and timer rows: no time or settle behaviour changed.
@task_item n=8 part=untouched
Recorded checks, check replay, session recording and committed-check content rows.
@task_item n=9 part=untouched
Lint defects, snapshot node, contrast reading and UI graph rows.
@task_item n=10 part=untouched
CI workflow shape rows (action pins, triggers, permissions, secrets, package installs, caches): no workflow or CI script changed.
