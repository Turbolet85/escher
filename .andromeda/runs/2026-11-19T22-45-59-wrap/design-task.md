@task_prose part=session
The session merged upstream main at 547d7514b into the 0.4.0 build branch as a two-parent merge, taking upstream's event queue, wheel gestures, text decorations, stylo split, WPT runner manifest and an opt-in Chrome devtools server, and leaving the Dioxus 0.8 move out.
Engine pins moved: Stylo 0.22 to 0.24 (selectors 0.43), Parley 0.12 to 0.13, skrifa 0.44 to 0.49, winit beta.3 to beta.4, svgtypes 0.16.2; Dioxus held at 0.7.3; blitz-net TLS moved to rustls; the arboard clipboard crate removed.
Event paths no longer take a dispatch callback: user-agent events are queued and flushed; programmatic focus now fires blur, focusout, focus and focusin, while the inert fix-up's clear and the driver's select focus stay silent; script-dispatched events read isTrusted false.
An upstream layout defect was fixed in the engine: an inline box inside text no font can shape now takes a private fallback font family, registered in every font context, a given one included; measured to move nothing where a font exists.
The accessibility tree is now refreshed in the shell's redraw after resolve, from the changed set, not in poll; an element with visibility collapse is left out of it.
The browser app and the WPT runner now build Skia by default; the devtools feature is in no default feature set; the parley code editor crate was retired.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest — its members and the engine dependency pins (Stylo, Parley, skrifa, winit, svgtypes, Dioxus)
@task_file n=2 path=Cargo.lock mark=merge
what: the dependency lock — the resolved engine crates and the TLS stack
@task_file n=3 path=deny.toml mark=merge
what: the dependency audit config — the licence table and the advisory ignores
@task_file n=4 path=.github/workflows/ci.yml mark=merge
what: the CI workflow — its legs, the Android job that builds the mobile target without testing it, the wasm job
@task_file n=5 path=.github/workflows/wpt.yml mark=merge
what: the WPT workflow that runs the WPT runner
@task_file n=6 path=apps/browser/Cargo.toml mark=merge
what: the browser app's manifest — its default features (renderer, screenshot, scrollbars), devtools left out
@task_file n=7 path=apps/browser/src/capture.rs mark=merge
what: the browser app's screenshot capture
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=merge
what: the default user-agent stylesheet
@task_file n=9 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the accessibility tree builder — which nodes are hidden from it, the select's chosen option and open state
@task_file n=10 path=packages/blitz-dom/src/config.rs mark=merge
what: the document configuration — the font context given to a document, the devtools provider
@task_file n=11 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — focus and its events, the changed set, the event queue, the font contexts it registers fonts in
@task_file n=12 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the pointer event handling — pointerdown, pointerup, click, the scrollbar thumb drag, the select list row gestures
@task_file n=13 path=packages/blitz-dom/src/events/keyboard.rs mark=merge
what: the keyboard event handling — key presses, the select list's keys
@task_file n=14 path=packages/blitz-dom/src/events/wheel.rs mark=merge
what: the wheel event handling — a wheel gesture's axis lock and its scroll
@task_file n=15 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the event dispatch entry — default actions run after an event unless prevented
@task_file n=16 path=packages/blitz-dom/src/events/focus.rs mark=merge
what: the old focus event generator, deleted
@task_file n=17 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the inline layout builder — text runs, inline boxes and the font families they are shaped with
@task_file n=18 path=packages/blitz-dom/src/lib.rs mark=merge
what: the engine crate root — the placeholder font family name
@task_file n=19 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the scrolling — smooth scroll animation on the virtual clock, into-view scroll, overscroll chaining
@task_file n=20 path=packages/blitz-dom/src/layout/paint_tree.rs mark=merge
what: the paint tree — stacking contexts, positioned paint levels and paint order
@task_file n=21 path=packages/blitz-dom/src/node/select_list.rs mark=merge
what: the select list model — the open list of a single select
@task_file n=22 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the old single stylo integration module, deleted and split
@task_file n=23 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=merge
what: the presentational hints — font element face and size, table cellspacing and cellpadding, lang, hr, SVG presentation attributes
@task_file n=24 path=packages/blitz-dom/src/stylo/prefs.rs mark=merge
what: the Stylo preferences set once
@task_file n=25 path=packages/blitz-dom/src/stylo/user_agent.rs mark=merge
what: the user-agent stylesheet type and the default sheet
@task_file n=26 path=packages/blitz-dom/src/stylo/device.rs mark=merge
what: the style device — viewport and media changes, the system color scheme
@task_file n=27 path=packages/blitz-paint/src/text.rs mark=merge
what: the text painting — glyph runs, synthetic bold, text strokes
@task_file n=28 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the text decoration and text shadow painting
@task_file n=29 path=packages/blitz-paint/src/render/select_list.rs mark=merge
what: the painted select list overlay — its rows and colours
@task_file n=30 path=packages/blitz-paint/src/render/form_controls.rs mark=merge
what: the painted form controls
@task_file n=31 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint pass — clipping, transforms, SVG images, text masks
@task_file n=32 path=packages/blitz-shell/src/window.rs mark=merge
what: the desktop shell window — redraw, the accessibility refresh, the safe area, the viewport
@task_file n=33 path=packages/blitz-shell/src/devtools.rs mark=merge
what: the shell's devtools — starting the CDP server from the environment, the element picker
@task_file n=34 path=packages/blitz-vibey-cdp-server/Cargo.toml mark=merge
what: the CDP devtools server crate — a local listener serving the Chrome devtools frontend
@task_file n=35 path=packages/blitz-traits/src/shell.rs mark=merge
what: the shell provider trait — cursor, window controls, and now opening devtools
@task_file n=36 path=packages/blitz-traits/src/events.rs mark=merge
what: the event types — the dispatch source and isTrusted, the wheel phase and source
@task_file n=37 path=packages/dioxus-native/Cargo.toml mark=merge
what: the dioxus-native manifest — its default features, clipboard left out
@task_file n=38 path=packages/dioxus-native/src/config.rs mark=merge
what: the dioxus-native configuration — devtools keyboard shortcuts
@task_file n=39 path=packages/escher-driver/src/execute.rs mark=merge
what: the driver's verb execution — click, press, type, the select focus
@task_file n=40 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner — its test selection, renderer choice and output
@task_item n=1 part=touches
Any row naming Stylo 0.22, Parley 0.12, skrifa 0.44 or winit beta.3 now reads stale: the pins are Stylo 0.24, Parley 0.13, skrifa 0.49, winit beta.4; Dioxus held at 0.7.3.
@task_item n=2 part=touches
The font fallback rows (unavailable families fall back to the default font, no generic family lays out as sans-serif): an inline box inside text no font can shape now takes a private engine fallback family; a row on fallback may need it beside it.
@task_item n=3 part=touches
The font context rows (dioxus-native configuration FontContext, font context on WASM, bundled font context on WASM, tests built with system fonts): the engine now registers its bullet font under the placeholder family in every font context, a given one included, when absent.
@task_item n=4 part=touches
The presentational hint rows (font element face and size, table cellspacing and cellpadding, lang sets the text locale): the hints moved into the split stylo module and now also cover hr and SVG presentation attributes.
@task_item n=5 part=touches
The default user-agent sheet row: the default sheet moved with the stylo split and the default stylesheet gained upstream rules (SVG not at root among them).
@task_item n=6 part=touches
The smooth scroll and wheel rows (scroll-behavior smooth animates and wheel cancels, wheel and key scroll the open select list): wheel handling is now upstream's gesture model with an axis lock and per-axis overscroll-behavior; smooth scroll progress reads the document clock.
@task_item n=7 part=touches
The virtual clock rows (animation driven by the virtual clock, driver advance moves the one virtual clock): scroll animation progress now reads the document's clock.
@task_item n=8 part=touches
The scrollbar drag row (shown overlay scrollbar takes a pointerdown as drag): a pointerup a handler cancels still ends a scrollbar drag.
@task_item n=9 part=touches
The select list event contracts (open, close, choose, highlight move, close on its select's change): their events are now queued by the user agent and flushed, not dispatched through a callback.
@task_item n=10 part=touches
The focus rows (MountedData set_focus_to during a render, a focused select with no box keeps focus): set_focus_to now fires blur, focusout, focus and focusin; the driver's select focus goes through a silent focus that fires none.
@task_item n=11 part=touches
The driver verb contracts (click, press, type): their events now go through the event queue; a cancelled pointerup still clicks; a cancelled pointerdown still counts towards a double click without selecting.
@task_item n=12 part=touches
The select accessibility contracts (chosen option, highlighted row, open attribute, open select): the accessibility tree is now refreshed in the shell's redraw after resolve, from the changed set, not in poll.
@task_item n=13 part=touches
The select list paint rows (colours as engine constants, chosen row takes Chromium's inactive selected pair, painted after the root render): the select list paint and the paint pass were merged with upstream's changes.
@task_item n=14 part=touches
The paint order rows (positioned auto z-index paint level, stacking-context root groups paint and hit-testing): upstream's paint tree change paints a positioned descendant of a static box above an earlier positioned sibling.
@task_item n=15 part=touches
The screenshot rows (screenshot never paints with text skipped, driver screenshot fonts-unavailable): text painting was rewritten on upstream's text paint options with text decorations and text shadows.
@task_item n=16 part=touches
The mobile targets row (mobile targets built, not tested): the Android CI job gains upstream's NDK steps and an iOS accessibility adapter came in; still built, not tested.
@task_item n=17 part=touches
The shell provider contracts: the shell provider gains an open devtools call (devtools feature).
@task_item n=18 part=touches
The dioxus-native configuration contracts: the configuration gains devtools keyboard shortcuts, and dioxus-native's default features no longer include the clipboard.
@task_item n=19 part=touches
The WPT runner rows (output test name, result colours, secondary text in bright black): the runner now loads tests from the WPT manifest and chooses its renderer, Skia by default.
@task_item n=20 part=touches
The color-scheme recascade row: the style device moved into the split stylo module, deciding device changes from the old and new viewport.
@task_item n=21 part=touches
The name blitz-inline-box-placeholder is now taken as a font family name inside the engine.
@task_fact n=1 slug=typography
fact: An inline box inside text that no available font can shape is laid out with a private engine fallback family named blitz-inline-box-placeholder (the bullet font), registered in every font context the document uses, a given one included; where a font exists nothing moves.
@task_fact n=2 slug=surface
fact: The Chrome devtools (a CDP server on 127.0.0.1 with no authentication, the devtools frontend, the element picker, open-devtools keyboard shortcuts) are an opt-in devtools feature that no default feature set builds.
@task_fact n=3 slug=surface
fact: The browser app and the WPT runner render with Skia by default.
@task_fact n=4 slug=motion
fact: A wheel gesture locks to its dominant axis, a small first movement not deciding it, each gesture deciding its own; wheel events outside a gesture scroll both axes; overscroll-behavior contain stops scroll chaining per axis.
@task_fact n=5 slug=surface
fact: A pointerup a handler cancels still clicks and still ends a scrollbar drag; a cancelled pointerdown still counts towards a double click without selecting text.
@task_fact n=6 slug=surface
fact: An element with visibility collapse is left out of the accessibility tree.
@task_fact n=7 slug=surface
fact: Programmatic focus changes fire blur, focusout, focus and focusin; the inert clear and the driver's select focus fire none; events a script dispatches read isTrusted false.
@task_item n=1 part=untouched
Brand identity, border radius, and the anti-patterns, self-validation and design decisions log areas: no row there and nothing the session brought.
@task_item n=2 part=untouched
The project-wide token absences (colour tokens, type scale, spacing scale, elevation scale, reduced-motion handling): the session added no tokens and no reduced-motion handling.
@task_item n=3 part=untouched
The spacing rows (body rightmargin and bottommargin ignored, open select list placed inside the viewport): no placement or margin rule moved.
@task_item n=4 part=untouched
The escher-session command line rows (exit codes and statuses, stdout, stderr, the mcp role, help, the actions, coverage, graph and layout lines, the session error remedy): no session command or output changed.
@task_item n=5 part=untouched
The escher-session task contracts (counter, cells, crud, flight-booker, timer, load, order, unknown task): the example apps were kept as they were.
@task_item n=6 part=untouched
The JS window contracts (devicePixelRatio, innerWidth and innerHeight, outerWidth and outerHeight) and the favicon from the first icon link: untouched.
@task_item n=7 part=untouched
The coverage check file contracts (read bound, refusal) and the escher-session agent-read and no-colour rows: untouched.
