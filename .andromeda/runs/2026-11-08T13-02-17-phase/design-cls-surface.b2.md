@answer n=1 entity=surface#6.1 v=no
why: macOS titlebar of the browser window; desktop-native titlebar is not in the chunk, the read is a driver command
@answer n=2 entity=surface#6.2 v=no
why: transparent example window geometry; desktop-native window is not reached by the read
@answer n=3 entity=surface#6.3 v=no
why: window compositing alpha mode in the windowed launch config; the chunk touches no window config
@answer n=4 entity=surface#7.1 v=no
why: wasm32 canvas sizing from host CSS; the chunk builds no wasm target and canvas nodes stay in 0.4.0
@answer n=5 entity=surface#7.2 v=no
why: MountedData calls made from code the vdom runs during a render; the read dispatches nothing and runs outside any render, so it makes no such call
@answer n=6 entity=surface#7.3 v=no
why: the optional scrollbars feature forwarding to blitz-paint; the chunk adds no feature and the scope names none
@answer n=7 entity=surface#7.4 v=yes
why: the scope names the open select list's rows answered before the walk and its into-view scroll moving only that list; the thumb that takes no hit bounds the covered reading and a scrolled-list stand proof
@answer n=8 entity=surface#8.1 v=no
why: mobile urlbar font and padding tokens; mobile-native is not reached by the driver read
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class driving urlbar sizing; mobile-native is not reached
@answer n=10 entity=surface#10.1 v=no
why: Android hardware back navigation in the browser tab; mobile-native navigation is not reached
@answer n=11 entity=surface#11.1 v=no
why: mobile screenshot default file name without a dialog; mobile capture is not reached
@answer n=12 entity=surface#12.1 v=no
why: iOS and Android targets built but not tested in CI; mobile-native build is not in the chunk
@answer n=13 entity=surface#13.1 v=no
why: WPT runner result colours; the WPT runner is not the driver's agent-read surface
@answer n=14 entity=surface#13.2 v=no
why: WPT runner bright-black test kind and headings; the WPT runner is not the driver's agent-read surface
@answer n=15 entity=surface#13.3 v=no
why: WPT runner OSC 8 hyperlinks; the WPT runner is not the driver's agent-read surface
@answer n=16 entity=surface#14.1 v=no
why: paint_bench example output lines; an example binary, not the driver's agent-read surface, as the task states
@answer n=17 entity=surface#14.2 v=no
why: screenshot example output lines; an example binary, not the driver's agent-read surface, as the task states
@answer n=18 entity=surface#15.1 v=yes
why: the scope names the cli output rule and the new read's answer must follow it: one uncoloured JSON line on stdout, no ESC byte, exit status carrying the class, and the mcp role's reply lines
@answer n=19 entity=surface#15.2 v=no
why: the six-task list in the usage line and the mcp start enum; the read changes no task list and the scope does not name it
@answer n=20 entity=surface#16.1 v=yes
why: the scope names the cli-graph subject; the read is a sibling of graph over the same UI graph, so graph's one-line JSON answer shape and its id, owner and flag-word rules carry over
@answer n=21 entity=surface#17.1 v=yes
why: the scope names the pending unpaged-graph subject, and how graph's answer is shaped and paged on the command line applies to the new read
@answer n=22 entity=surface#18.1 v=no
why: WASM bundled DejaVu font registration; the chunk builds no wasm target and sets no type
@answer n=23 entity=surface#19.1 v=no
why: wasm_hello canvas size, radius and focus; web-spa canvas is not reached
