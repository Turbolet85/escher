# Classified — arch · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · the chunk changes no platform adapter; it writes through the tree's builder, which is in blitz-dom
existing-scopes#1.2 · no · debug_timer is not blitz-dom and the chunk adds no timer; the clock rule is blitz-dom's own
existing-scopes#1.3 · no · lists modules only and states no behaviour; the chunk reaches no blitz-traits file and adds no crate or event type
existing-scopes#1.4 · yes · the select rules, events, accessibility, clock and config code of the chunk land in these blitz-dom modules, with clock carrying the time rule and node holding select
existing-scopes#1.5 · no · cssom editing and getComputedStyle are script-facing and the scope names no reader the chunk's colour check uses; the UA sheet is read through the harness contrast reader
existing-scopes#1.6 · no · form.rs owner reset and submission is not reached, and the bridge's form data for select events lives in dioxus-native-dom, not here
existing-scopes#1.7 · yes · hit testing under the overlay, focus with a list open, and the client-rect, visible-region and scroll-into-view readers of an open list's options sit in these document.rs interaction parts
existing-scopes#1.8 · yes · accessibility.rs tree building is a reached file, and the list's open state, expanded and option names are written through this builder
existing-scopes#1.9 · yes · keyboard.rs and pointer.rs are reached, carrying Enter, released Space, arrows, typeahead, clicks and the focus order of the list
existing-scopes#1.10 · yes · the visible-region reader and into-view scrolling are the driver's readings of an open list's option
existing-scopes#1.11 · yes · Device management carries the headless ColorScheme::Light the default colours read, and Stylo styling resolves the UA sheet
existing-scopes#1.12 · yes · versioned node storage is what the chunk's NodeIds are over, and a removed node's open state is cleared or retargeted before the node is freed
existing-scopes#1.13 · yes · damage propagation on open and close, and the layout-generated boxes that persistent state must not reference, sit in this layout module
existing-scopes#1.14 · yes · select is the reached crate-private select model holding the choice rules, and its node file is edited for the choice path
existing-scopes#1.15 · no · HTML parsing is the component's own guarantee; the scope names no select or optgroup parsing behaviour and the chunk changes no parser
existing-scopes#1.16 · no · the list loads nothing, and blitz-net sits in the chunk's Not needed list
existing-scopes#1.17 · yes · render.rs and lib.rs of blitz-paint are reached: the overlay in paint_scene, its layer cap, and the painting of option labels and default colours
existing-scopes#1.18 · no · the chunk adds no script surface; Boa's runtime and script timers are in the chunk's Not needed list
existing-scopes#1.19 · yes · the windowed path's redraw request and poll loop, and the shell's accessibility module refreshing the platform tree, carry the list's changes
existing-scopes#1.20 · no · a re-export layer the chunk does not edit; the select readers reach the driver through dioxus-native-dom's snapshot re-exports
existing-scopes#1.21 · no · the overlay is painted and not laid out through taffy, so no stylo_taffy conversion is reached
existing-scopes#1.22 · yes · the windowed path of dioxus-native builds its documents Parallel and shows the list through the engine overlay, in these config and renderer modules
existing-scopes#1.23 · yes · snapshot.rs is edited, element_id and unkeyed_actionable key the options, snapshot text and diff read the screen, and the bridge carries input, change and form data
existing-scopes#1.24 · yes · the harness's screenshot, screenshot_rect, text_contrast and settle are the chunk's own checks and must never move time
existing-scopes#1.25 · no · the formatter prints log records; the chunk adds no log target, span or field the allowlist must admit
