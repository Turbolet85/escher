@task_prose part=session
A behaviour-preserving refactor: escher's own additions inside three upstream engine functions moved out into private functions of escher's own, each reached by one call in upstream's body; no public symbol, verb, answer or event changed.
The accessibility tree builder's escher-added state reads (expanded, selected, label/name, toggled, invalid) now stand in their own functions, each ARIA attribute read once.
The keyboard handler's escher-added control-key chain (range, single list box, drop-down select, multiple select, Enter activation) and the close-the-open-list-before-Tab step moved into their own functions, same order and gates.
The presentational hints' select size and list-box row handling moved into one function of its own; the driver's command validation tests moved whole into a child test file, the validation itself unchanged.
The lock bumped url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3, closing RUSTSEC-2026-0592; the lock keeps two syn edges cargo would otherwise move, on the operator's ruling.
Measured: escher-driver coverage 97.97 to 97.80 percent against its floor 97; cognitive complexity of the three upstream functions fell; CI green 17 of 17.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: the driver's command validation: the verb table that admits or refuses an escher-session call, each verb checked at its own level, a malformed call refused with the rule it broke
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: the unit tests of the driver's command validation (admitted calls, unknown verbs, malformed session and instance calls), moved whole out of the validation module
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder: the role, name, expanded, selected, toggled and invalid states each node carries in the snapshot and accessibility tree, including a select's chosen option, open state and highlighted row
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: the keyboard event handler: Tab, Enter, Space and arrow keys on controls, including opening a single select, moving and choosing in an open select list or list box, range input keys and Enter activation, and closing an open list before Tab moves focus
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hints: HTML attributes turned into style, such as the font element's face and size, table cellspacing, cellpadding and valign, and a select's size attribute giving a list box its height, overflow and rows
@task_file n=6 path=Cargo.lock
what: the workspace dependency lock: the resolved versions of every crate the engine, driver and examples build against
@task_item n=1 part=touches
The snapshot and accessibility tree of a select contract items (chosen option, highlighted row, open attribute, open select): the reads producing them moved into new functions of the accessibility tree builder; each claim should still hold, behaviour unchanged.
@task_item n=2 part=touches
Click, Enter or Space opens a single select, and Wheel and key scroll the open select list: the key arms behind these decisions moved into a new function of the keyboard handler, same order and gates.
@task_item n=3 part=touches
A single select's list events (open, highlight move, choose, choose a disabled row, choose the chosen option, close): the key path firing them moved into its own function; the close before Tab now stands in its own function too.
@task_item n=4 part=touches
Selects that open no list (list box, multiple select) and Select list as an engine overlay: the list-box size, overflow and row computation moved into a function of its own in the presentational hints; a select holding several size attributes in different namespaces now reads each, as the old loop did.
@task_item n=5 part=touches
Font element face and size as presentational hints, and Table cellspacing, cellpadding and valign hints: decisions over the presentational hints component the session restructured; their own arms did not move.
@task_item n=6 part=touches
actions decided by the verbs' own checks, and the escher-session commands contract items: they rule over the driver's command validation, whose tests moved to a child file; the validation and every answer unchanged.
@task_item n=7 part=touches
Driver command line output read by agents and escher-session exit status or exit code rows: indirect, through the command validation whose refusals feed them; unchanged.
@task_item n=8 part=touches
Select chosen row takes Chromium's inactive selected pair and Select list colours as engine constants: indirect, a single list box's rows now computed in the moved list-box function of the presentational hints; colours unchanged.
@task_item n=1 part=untouched
Colour tokens, the color-scheme recascade, engine colour constants and the base_color configuration: no colour value or token moved.
@task_item n=2 part=untouched
Motion and the virtual clock: advance, settle, load wait, smooth scroll, overlay scrollbar fade timings and instant scrolls are untouched.
@task_item n=3 part=untouched
Screenshot fonts and font context rows: fonts-unavailable refusals, system fonts in tests, WASM bundled font context and document.fonts are untouched.
@task_item n=4 part=untouched
Depth and hit-testing: stacking contexts, auto z-index, paint clips and the open select list answering hits first are untouched.
@task_item n=5 part=untouched
Iconography and the shell provider: cursor icons, favicon, window drag, maximize, minimize and decorations are untouched.
@task_item n=6 part=untouched
WPT runner output and result colours, the JS window object, MountedData calls and the logical viewport are untouched.
@task_item n=7 part=untouched
The escher-session tasks rows, the mcp role results and the coverage, actions and UI graph line contents: no answer or format changed.
