@answer n=1 entity=surface#1.1 v=yes
why: the UA sheet comes once through DocumentConfig::default(), so the reached dioxus_document.rs must not add it again in DioxusDocument::new, and the fresh-session replay builds documents
@answer n=2 entity=surface#1.2 v=no
why: a select's engine CSS border, padding and background; the chunk lays out and paints no select
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout from the engine sheet; no select or layout value is in the scope
@answer n=4 entity=surface#1.4 v=no
why: the chosen-row colour of a multiple select; the chunk paints and compares no colour
@answer n=5 entity=surface#1.5 v=no
why: the open list's engine colour constants; the chunk paints and compares no colour
@answer n=6 entity=surface#1.6 v=no
why: single-select group layout as display contents; no select or layout is named by the scope
@answer n=7 entity=surface#1.7 v=no
why: a select's list opening and its label and size painting; the scope names no select
@answer n=8 entity=surface#1.8 v=no
why: the select list overlay reaches the screen through paint_scene; the coverage read dispatches and paints nothing on the screen
@answer n=9 entity=surface#1.9 v=no
why: a select's highlight and open-state events; the scope names no select and the count reads only registered graph handlers
@answer n=10 entity=surface#1.10 v=no
why: a select's choose fires engine input and change events, which are not graph-registered handlers the scope counts
@answer n=11 entity=surface#1.11 v=no
why: a select list's close behaviour; the scope names no select
@answer n=12 entity=surface#1.12 v=no
why: a disabled, inert or multiple select opens nothing; the scope names no select
@answer n=13 entity=surface#1.13 v=no
why: a focused select keeping focus when its container loses its box; the scope names no focus or select rule
@answer n=14 entity=surface#1.14 v=no
why: a group header in the select list; the scope names no select
@answer n=15 entity=surface#1.15 v=no
why: the open select's accessibility tree and highlight; the scope reads the snapshot's shape only and this entity states no shape the chunk reads
@answer n=16 entity=surface#1.16 v=no
why: the bounds of a row of an open list in the snapshot; the chunk reads no row bounds
@answer n=17 entity=surface#1.17 v=no
why: the initial theme is Light; the coverage count and the owed test carry no colour scheme
@answer n=18 entity=surface#1.18 v=yes
why: the owed test replays on a fresh session at this default viewport, and whether the actions answer read there depends on layout, a reading the scope leaves open
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth, innerHeight and devicePixelRatio values; the chunk runs no script and reads none of them
@answer n=20 entity=surface#2.1 v=no
why: the macOS tabstrip titlebar; the chunk builds no shell or window change
@answer n=21 entity=surface#2.2 v=no
why: the shell's titlebar provider for minimize, maximize and drag; the chunk builds no window change
@answer n=22 entity=surface#3.1 v=no
why: the opacity of custom GPU content layers; the custom widget's listener count is the scope's rule, not its compositing
@answer n=23 entity=surface#4.1 v=no
why: devtools browser-style overlays; the chunk builds no devtools change
@answer n=24 entity=surface#5.1 v=no
why: the browser window title follows the active tab; the chunk changes no window or title
@answer n=25 entity=surface#5.2 v=no
why: rdme's window title and theme toggle; the chunk changes no app or window
