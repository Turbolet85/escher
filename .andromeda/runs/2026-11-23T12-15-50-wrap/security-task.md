@task_prose part=session
The session fixed upstream issue 1578: a custom widget is now painted at its content box (border box less border and padding, times scale), its scene placed at the content origin inside the element's own transform and left unclipped.
A custom widget now hears pointer events in its content-box frame; the sub-document (iframe) forwarding arm is unchanged.
The Widget trait gains a defaulted fonts method, and the document gains a public font_ctx accessor behind the custom-widget feature: before each paint the widget is handed the document's own shared, mutable font context, so app widget code can shape canvas text from the bundled fonts.
blitz-tests takes parley as a workspace dev-dependency (one lock line, no package added), and three new stand checks witness placement, pointer frame and canvas text.
RUSTSEC-2026-0594 is fixed by moving cookie 0.18.2 to 0.18.3 lock-only: the version and checksum lines edited by hand, no audit ignore, deny.toml unchanged, the syn edges kept at 3.0.6; the audit leg and CI read advisories ok, licenses ok.
No IPC method, socket, verb, argument, env var, refusal cause, session error, exit code, log field or span is added.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: the engine paint crate's custom widget paint step behind the custom-widget feature, which sizes a widget's scene and hands the widget the document's font context before paint
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: the engine paint crate's element renderer, which places a custom widget's scene inside the element's transform, unclipped
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: the DOM event dispatch that forwards pointer events to a custom widget or a sub-document with coordinates adjusted to their own space
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget object and its public Widget trait, the app-code boundary a widget implementor fills
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine's base document, holder of the shared font context used for font resolution
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: the unpublished test harness crate's manifest and its dev-dependencies
@task_file n=7 path=Cargo.lock
what: the workspace lockfile read by locked builds and by the cargo-deny audit leg, holding the cookie crate behind the cookie store
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: a stand check that a custom widget paints its content box, inside its transform
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: a stand check that a custom widget hears pointer events from its content box
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: a stand check that canvas text is shaped from the document's own font context and matches the bundled fonts
@task_item n=1 part=touches
custom-widget-initial-attribute-replay: the custom widget boundary it rules over gains a defaulted Widget method that hands app code the document's shared, mutable font context, and the widget's paint size, scene placement and pointer frame move to the content box; a new decision stands beside it.
@task_item n=2 part=touches
canvas-content-crosses-as-shot-pixels: canvas content still crosses only as shot pixels, but the pixels are now placed at the content box and canvas text may be shaped from the document's fonts; the row is checked for staleness.
@task_item n=3 part=touches
harness-blitz-paint-custom-widget: the custom-widget feature the harness turns on now also gates the document's public font context accessor and the widget fonts handoff.
@task_item n=4 part=touches
cookie-store-behind-feature and cookies-and-cache-default-features: the cookie crate behind the cookie store moves 0.18.2 to 0.18.3 for RUSTSEC-2026-0594, a reviewed lock pin.
@task_item n=5 part=touches
dependency-audit-ci-gate, dependency-audit-cargo-deny, cargo-deny-audit-tool and local-audit-before-push: the audit leg read red on RUSTSEC-2026-0594 and green after the lock move, locally and in CI.
@task_item n=6 part=touches
advisory-ignores-by-id and audit-ignores-per-id-with-reason: the advisory was fixed with no ignore and deny.toml unchanged.
@task_item n=7 part=touches
locked-builds: the lockfile was moved by hand-editing two lines rather than by cargo update, and every gate's cargo run passed locked.
@task_item n=8 part=touches
engine-dependency-pins and upstream-sync-dependency-set: a dependency version moved by the fork's own lock edit, apart from an upstream sync, with the syn edges deliberately kept at 3.0.6.
@task_item n=9 part=touches
workspace-inherited-dependency-versions: blitz-tests takes parley with workspace = true, the workspace's one registry version.
@task_item n=10 part=touches
blitz-tests-not-published and test-harness-no-tracing-no-style: the unpublished harness gains a parley dev-dependency; the rows are checked against it.
@task_item n=11 part=touches
absence/automated-dependency-updates: the advisory fix was found by the audit leg and applied by hand; the absence still holds and is checked.
@task_item n=12 part=touches
no-agent-urls-through-font-path and hintless-font-face-source-sniffed: the document's font context is now reachable by app widget code through a mutable handle; the font-source rules over that context are checked against it.
@task_item n=13 part=touches
non-invertible-transform-no-hit and visible-region-through-transforms: a custom widget's scene is now composed inside the element's transform and its pointer coordinates are offset by border and padding; the transform-reading rules are checked.
@task_item n=14 part=touches
engine-target-field-allowlist and accessible-names-not-logged: the engine paint and DOM components changed but no log site, span or field was added.
@task_fact n=1 slug=input-validation
fact: A custom widget's paint size is its content box times scale (border box less border and padding, floored at 0), its scene is placed at the content origin inside the element's transform and left unclipped, and its pointer events arrive in its content-box frame while a sub-document's stay in its border-box frame; these are layout values, not input.
@task_fact n=2 slug=input-validation
fact: Before each paint a custom widget is handed the document's own shared, mutable font context (the Widget trait's defaulted fonts method, and a public font_ctx accessor on the base document, both behind custom-widget), so app widget code holds a live handle to the engine's font context; no input crosses and the default ignores it.
@task_fact n=3 slug=dependency-security
fact: cookie is pinned at 0.18.3 as a reviewed lock pin fixing RUSTSEC-2026-0594, lock-only with no audit ignore and deny.toml unchanged.
@task_fact n=4 slug=dependency-security
fact: A lock-only patch bump on this lock is made by hand-editing the crate's version and checksum lines, since cargo update -p re-resolves unrelated edges (it moved the syn edges of bindgen and cssparser-macros 3.0.6 to 2.0.119); those syn edges are kept at 3.0.6.
@task_fact n=5 slug=dependency-security
fact: blitz-tests takes parley as a workspace-inherited dev-dependency; it was already in the lock and no package is added.
@task_item n=1 part=untouched
The driver session socket, its request grammar, replies, bounds and lifecycle: no verb, argument, field or reply changed.
@task_item n=2 part=untouched
The MCP server, its transport, requests, results and protocol errors.
@task_item n=3 part=untouched
Refusal causes, session errors and exit codes of escher-session and its scripts: none added or moved.
@task_item n=4 part=untouched
Secret management and environment-variable contracts: no env var, secret or credential is read or added.
@task_item n=5 part=untouched
Stable element ids, snapshot text, graph and actions answers, and masked control values.
@task_item n=6 part=untouched
Select list, range input and other form-control rules: the content box shared by name with range input is not reached.
@task_item n=7 part=untouched
Recording, kept check and cold-agent rows, and the coverage floor.
@task_item n=8 part=untouched
Outbound HTTP, fetch, navigation, CORS and response-limit rows: the cookie move changes no network behaviour.
@task_item n=9 part=untouched
CI tokens, publish guards, CI action pinning and signing material: no workflow changed.
