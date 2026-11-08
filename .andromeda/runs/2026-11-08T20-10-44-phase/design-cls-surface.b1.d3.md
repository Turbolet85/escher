@answer n=1 entity=surface#1.1 v=no
why: the UA sheet is installed by DocumentConfig::default() in blitz-dom; the chunk makes no document-construction call and the fresh-session replay uses the existing creation path
@answer n=2 entity=surface#1.2 v=no
why: select default-sheet box values are layout only, and the coverage count and owed test read no layout
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout is set and read by nothing the chunk touches
@answer n=4 entity=surface#1.4 v=no
why: multiple-select chosen-row colour; the chunk paints and compares no colour
@answer n=5 entity=surface#1.5 v=no
why: open select list colour constants; the chunk paints no colour and the scope names no select list
@answer n=6 entity=surface#1.6 v=no
why: single-select group display rule is layout-only and the scope names no select
@answer n=7 entity=surface#1.7 v=no
why: the engine's select open on click, Enter or Space is not named by the scope, and the owed test replays no select
@answer n=8 entity=surface#1.8 v=no
why: the select overlay is painted through paint_scene and the coverage read paints nothing
@answer n=9 entity=surface#1.9 v=no
why: the open list's highlight-without-input-or-change rule is not named by the scope, and no replayed step chooses a select row
@answer n=10 entity=surface#1.10 v=no
why: the input-then-change rule on a row choice is an engine select behaviour the scope does not name
@answer n=11 entity=surface#1.11 v=no
why: list close events on Escape or outside click are an engine select behaviour the scope does not name
@answer n=12 entity=surface#1.12 v=no
why: a disabled or hidden select opening nothing is an engine select behaviour the scope does not name
@answer n=13 entity=surface#1.13 v=no
why: a focused select keeping focus after its box is lost is an engine select behaviour the scope does not name and the owed replay does not reach
@answer n=14 entity=surface#1.14 v=no
why: a group header in an open list choosing nothing is an engine select behaviour the scope does not name
@answer n=15 entity=surface#1.15 v=no
why: the tree's ComboBox and expanded reading of an open select is a snapshot-tree value the scope does not name and the chunk does not read for it
@answer n=16 entity=surface#1.16 v=no
why: row bounds and row screenshot crop; the chunk takes no screenshot and reads no bounds
@answer n=17 entity=surface#1.17 v=no
why: the initial theme is never read or painted by the coverage count or the owed test
@answer n=18 entity=surface#1.18 v=no
why: default viewport size and scale are layout inputs the coverage read does not measure
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth, innerHeight and devicePixelRatio are script values the chunk reads nowhere
@answer n=20 entity=surface#2.1 v=no
why: macOS tabstrip padding is browser titlebar styling and the chunk builds no titlebar
@answer n=21 entity=surface#2.2 v=no
why: the shell provider's minimize, maximize and drag exposure is window behaviour the chunk does not change
@answer n=22 entity=surface#3.1 v=no
why: HTML layer opacity over custom GPU content is compositing; the acceptance's canvas limit concerns listeners per (node, event), not opacity
@answer n=23 entity=surface#4.1 v=no
why: devtools overlays are a devtools feature the chunk does not build or read
@answer n=24 entity=surface#5.1 v=no
why: the browser window title following the active tab is window behaviour the chunk does not touch
@answer n=25 entity=surface#5.2 v=no
why: rdme's window title and Ctrl/Cmd+T theme toggle belong to a different app, outside the chunk's files and scope
