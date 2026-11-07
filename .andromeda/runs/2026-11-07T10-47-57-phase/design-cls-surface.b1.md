@answer n=1 entity=surface#1.1 v=yes
why: the scope re-asserts document built, and this states that every document, the Dioxus one included, gets the blitz user-agent sheet once through DocumentConfig::default(), which the built document and its layout read
@answer n=2 entity=surface#1.2 v=no
why: select's default border, padding and background; the chunk touches no select and the scope names no select behaviour
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout in the default sheet; no select is touched and the scope names none
@answer n=4 entity=surface#1.4 v=no
why: multiple-select chosen-row colour; a colour rule and no select is touched
@answer n=5 entity=surface#1.5 v=no
why: open select list colour constants on SelectListView; no select list is touched
@answer n=6 entity=surface#1.6 v=no
why: single-select group display rules in the default sheet; no select is touched
@answer n=7 entity=surface#1.7 v=no
why: select list opening, label and size reading; the chunk changes no select behaviour
@answer n=8 entity=surface#1.8 v=no
why: select list overlay reaching the screen through paint_scene; the chunk changes mutation application, not paint, and the scope names no select list
@answer n=9 entity=surface#1.9 v=no
why: select open and highlight movement on keys and pointer; no select is touched
@answer n=10 entity=surface#1.10 v=no
why: select choose firing input and change; the scope names mounted events, not select input and change
@answer n=11 entity=surface#1.11 v=no
why: select close on Escape, outside click and scroll; no select is touched
@answer n=12 entity=surface#1.12 v=no
why: disabled, hidden or inert select opens nothing; no select is touched
@answer n=13 entity=surface#1.13 v=no
why: focus kept on a select whose box was dropped; the scope's focus rule is set_focus moving focus to a named input, not this select path
@answer n=14 entity=surface#1.14 v=no
why: list group header and label reading; no select is touched
@answer n=15 entity=surface#1.15 v=no
why: select-specific accessibility and snapshot rule for the highlighted row; the scope names snapshot generically and names no select
@answer n=16 entity=surface#1.16 v=no
why: open list row bounds in the snapshot; a select-specific snapshot rule and the scope names no select
@answer n=17 entity=surface#1.17 v=no
why: initial window theme; the buffer changes no theme path and the scope names no theme value
@answer n=18 entity=surface#1.18 v=yes
why: the default viewport size, scale, zoom and Light scheme is the size the chunk's get_client_rect and layout-identity checks are measured at, so the value is an input the chunk reads
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth, innerHeight and devicePixelRatio in blitz-vibey-script; neither the chunk's code nor its tests read JS window values
@answer n=20 entity=surface#2.1 v=no
why: macOS-only browser tabstrip padding and height; the browser check is on documents, element ids and mounted events, not its CSS
@answer n=21 entity=surface#2.2 v=no
why: shell titlebar provider controls; the chunk does not touch window chrome
@answer n=22 entity=surface#3.1 v=no
why: custom GPU widget opacity in the custom_widget example; the scope names no custom content and the chunk paints none
@answer n=23 entity=surface#4.1 v=no
why: devtools overlays for the hovered node; the chunk touches no devtools path
@answer n=24 entity=surface#5.1 v=yes
why: the windowed launch sets the window title from the document's title node when set before the window exists, and the buffered replay moves when that node's mutations land, so the windowed launch must keep this
@answer n=25 entity=surface#5.2 v=no
why: rdme's own title string and Ctrl/Cmd+T theme shortcut; the chunk does not touch rdme and the shared title mechanism is covered by surface#5.1
