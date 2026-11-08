@answer n=1 entity=surface#6.1 v=no
why: the macOS browser titlebar is a window the chunk does not change
@answer n=2 entity=surface#6.2 v=no
why: the transparent example window is a shell the chunk does not build
@answer n=3 entity=surface#6.3 v=no
why: window compositing alpha mode is a renderer config the chunk does not set or read
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 page canvas is the page the app renders into, not the in-app canvas, and no canvas renderer is built
@answer n=5 entity=surface#7.2 v=no
why: the buffered-render rule governs MountedData calls made from vdom render code, which the chunk neither adds nor makes and which names no unit the chunk edits
@answer n=6 entity=surface#7.3 v=no
why: the scrollbars feature forwards to blitz-paint and the chunk adds no scrolling
@answer n=7 entity=surface#7.4 v=no
why: the select list's painted scroll thumb is a paint the chunk does not touch, and the coverage read dispatches no pointer
@answer n=8 entity=surface#8.1 v=no
why: mobile urlbar font tokens, and the chunk touches no mobile build
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class drives mobile urlbar sizing, and the chunk touches no mobile build
@answer n=10 entity=surface#10.1 v=no
why: the Android hardware back button navigates the browser tab, and the chunk touches no mobile build
@answer n=11 entity=surface#11.1 v=no
why: mobile screenshot file naming, and the chunk takes no screenshot and touches no mobile build
@answer n=12 entity=surface#12.1 v=no
why: mobile targets' CI status, and the chunk touches no mobile build
@answer n=13 entity=surface#13.1 v=no
why: WPT runner result colours, and the coverage command is a driver surface that paints no colour
@answer n=14 entity=surface#13.2 v=no
why: WPT runner terminal heading colours, and the chunk paints no colour and is not the WPT runner
@answer n=15 entity=surface#13.3 v=no
why: WPT runner OSC 8 hyperlinks, and the chunk is not the WPT runner
@answer n=16 entity=surface#14.1 v=no
why: paint_bench stdout lines, and the coverage command is not that example binary
@answer n=17 entity=surface#14.2 v=no
why: screenshot example stdout lines, and the chunk takes no screenshot
@answer n=18 entity=surface#15.1 v=no standing=S1
why: the one-line JSON stdout, no-colour, no-ESC and exit-status convention every driver answer owes
@answer n=19 entity=surface#15.2 v=no
why: the six served task names are the host's app list, which the coverage count and the replay test do not read, since the test uses the app its recorded check names
@answer n=20 entity=surface#16.1 v=yes
why: the graph verb's answer shape with handler event names and owner ids is the UI graph the coverage command reads unchanged
@answer n=21 entity=surface#17.1 v=yes
why: the actions verb's answer shape and its actionable-node listing is the kept answer the owed test compares whole
@answer n=22 entity=surface#17.2 v=yes
why: the actions answer's cause and stale values are what the owed test's comparison of the kept answer reads
@answer n=23 entity=surface#18.1 v=yes
why: the pending unpaged graph answer is the handler set the coverage count reads whole, as item 4 names
@answer n=24 entity=surface#19.1 v=yes
why: the pending unpaged actions answer is the kept answer the owed test compares whole, as item 4 names
@answer n=25 entity=surface#20.1 v=no
why: WASM bundled fonts, and the chunk measures no text and builds no WASM host
