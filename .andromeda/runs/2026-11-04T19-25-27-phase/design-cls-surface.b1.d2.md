@answer n=1 entity=surface#1.1 v=no
why: the UA sheet moves layout only; the chunk reads DioxusDocument's element ids and listeners, not its construction, and no link reading is geometric
@answer n=2 entity=surface#1.2 v=no
why: select default box layout is layout only, and no select or layout value is named in the scope
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout is layout only, and the link readings are not geometric
@answer n=4 entity=surface#1.4 v=no
why: chosen-row colour, and the probe paints and styles nothing
@answer n=5 entity=surface#1.5 v=no
why: open-list colour constants, and the probe paints nothing
@answer n=6 entity=surface#1.6 v=no
why: optgroup display rules are layout only, and no link reading touches them
@answer n=7 entity=surface#1.7 v=yes
why: a click opens a single select's list; the scope names the click verb, and if CRUD presents a select (a fork the scope leaves open) the probe's click lands on it
@answer n=8 entity=surface#1.8 v=no
why: the list is an engine overlay painted through paint_scene; paint is not a named verb and the windowed list is read, not run
@answer n=9 entity=surface#1.9 v=yes
why: keys move the highlight and choose nothing; the scope names the press verb, and an open list takes a press before a Dioxus key handler sees it
@answer n=10 entity=surface#1.10 v=yes
why: choosing fires input then change on the select node; the scope names click and press, and these are the listener-map events the probe reads
@answer n=11 entity=surface#1.11 v=yes
why: Escape, a second click and a click outside close the list with no input or change; carried by the click and press verbs (scrolling is not in scope)
@answer n=12 entity=surface#1.12 v=yes
why: a disabled, hidden or multiple select opens nothing and a label click opens nothing; carried by the click verb
@answer n=13 entity=surface#1.13 v=yes
why: a key press into a focused select whose container lost its box must not panic and still moves the choice; carried by the press verb
@answer n=14 entity=surface#1.14 v=yes
why: a click on a group header chooses nothing; carried by the click verb (the tree reading of the header is not in scope)
@answer n=15 entity=surface#1.15 v=no
why: the accessibility tree (ComboBox, expanded, selected) is a reading the scope does not name; the probe reads element ids and listeners
@answer n=16 entity=surface#1.16 v=no
why: row bounds in the snapshot and row screenshots are not named, and the probe does no snapshot or screenshot
@answer n=17 entity=surface#1.17 v=no
why: the initial window theme belongs to the windowed shell path, read only where a reading needs it, and the probe paints nothing
@answer n=18 entity=surface#1.18 v=no
why: the default window viewport belongs to the windowed shell path; the probe runs headless and names no viewport value
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth, innerHeight and devicePixelRatio come from the window bootstrap, and no served-task read of them is named
@answer n=20 entity=surface#2.1 v=no
why: macOS browser tabstrip padding; apps/browser is outside the scope
@answer n=21 entity=surface#2.2 v=no
why: shell provider window controls for custom titlebars; the titlebar pattern is not_needed and the windowed shell path is read only where a reading needs it
@answer n=22 entity=surface#3.1 v=no
why: custom GPU widget opacity layers; custom-widget patterns are not_needed
@answer n=23 entity=surface#4.1 v=no
why: devtools hover overlays; the scope names no overlay and the probe reads no overlay
@answer n=24 entity=surface#5.1 v=no
why: window title follows the active tab's title; the browser app and windowed shell are outside the scope
@answer n=25 entity=surface#5.2 v=no
why: rdme window title and Ctrl/Cmd+T theme toggle; apps/readme is outside the scope
