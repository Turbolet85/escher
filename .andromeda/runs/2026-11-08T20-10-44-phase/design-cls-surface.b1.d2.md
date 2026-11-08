@answer n=1 entity=surface#1.1 v=no
why: the user-agent stylesheet install lives in blitz-dom config.rs, which the chunk does not reach, and the chunk builds no document config
@answer n=2 entity=surface#1.2 v=no
why: select default border, padding and background are engine paint defaults that the coverage count and the owed test never read
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout is engine default styling, and no handler count or replay reads it
@answer n=4 entity=surface#1.4 v=no
why: the multiple-select chosen-row colour is paint, and the coverage command and replay test compare no colour
@answer n=5 entity=surface#1.5 v=no
why: the open select list colours are engine constants on the paint path, not read by the count
@answer n=6 entity=surface#1.6 v=no
why: single-select group and chosen-option display is layout, which the count and the test do not read
@answer n=7 entity=surface#1.7 v=no
why: opening the list on click, Enter or Space fires no input or change, and the count does not depend on the popup being painted
@answer n=8 entity=surface#1.8 v=no
why: the list is an engine overlay painted through paint_scene, and the chunk paints nothing
@answer n=9 entity=surface#1.9 v=yes
why: keys and pointer on an open single select choose nothing and fire no input or change, which decides whether a replayed actions step exercises a select change handler
@answer n=10 entity=surface#1.10 v=yes
why: Enter, Space, Tab or a click fires one input then one change, while a disabled row or the chosen option fires nothing, the events the count records for select handlers
@answer n=11 entity=surface#1.11 v=yes
why: Escape, a second click or a click outside closes the list with no input or change, so the count must not treat those as exercising select handlers
@answer n=12 entity=surface#1.12 v=no
why: a disabled, inert or hidden select opens nothing, and a label click opens nothing, a state no handler count reads
@answer n=13 entity=surface#1.13 v=no
why: a focused select whose box was lost keeps focus and takes a key with no panic, a focus case the coverage read and the owed test never hit
@answer n=14 entity=surface#1.14 v=no
why: a group header in the list is an accessibility reading and chooses nothing through a path the count does not record
@answer n=15 entity=surface#1.15 v=yes
why: the snapshot the chunk reads without changing its shape carries no highlighted row and the tree's ComboBox and expanded state for an open select, which is the chunk's own re-read
@answer n=16 entity=surface#1.16 v=yes
why: the snapshot the chunk reads carries each open-list row's own bounds, a snapshot value the coverage read leaves as it is
@answer n=17 entity=surface#1.17 v=no
why: the initial window theme is Light and reaches no handler count or replay
@answer n=18 entity=surface#1.18 v=yes
why: the fresh session's default viewport decides which nodes are agent-visible, and the scope leaves that size open for the owed replay test
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth, innerHeight and devicePixelRatio are page-script values that the coverage read and the owed test never read
@answer n=20 entity=surface#2.1 v=no
why: the macOS browser tabstrip padding and height are titlebar layout, which the chunk does not build
@answer n=21 entity=surface#2.2 v=no
why: the shell's minimize, maximize and drag provider is window chrome the chunk does not change
@answer n=22 entity=surface#3.1 v=no
why: the custom widget's HTML layer opacity is a compositing value, and the canvas listener limit is not this entity's rule
@answer n=23 entity=surface#4.1 v=no
why: devtools browser-style overlays are a display feature the coverage read does not touch
@answer n=24 entity=surface#5.1 v=no
why: the browser window title follows the active tab, a window value the chunk does not change
@answer n=25 entity=surface#5.2 v=no
why: rdme window title and its Ctrl or Cmd+T theme toggle are an app shell behaviour the chunk does not reach
