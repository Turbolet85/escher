@answer n=1 entity=surface#6.1 v=no
why: macOS browser titlebar in apps/browser; the chunk changes no window or shell
@answer n=2 entity=surface#6.2 v=no
why: transparent example window; the chunk builds no window change
@answer n=3 entity=surface#6.3 v=no
why: window compositing alpha mode; the chunk sets and reads no alpha mode
@answer n=4 entity=surface#7.1 v=no
why: wasm32 canvas sizing from host CSS; the chunk builds no wasm target
@answer n=5 entity=surface#7.2 v=yes
why: the chunk reaches dioxus_document.rs, where the buffered render keeps MountedData calls borrow-clean during a render, and its new code there must not reintroduce a borrow
@answer n=6 entity=surface#7.3 v=no
why: scrollbars feature forwarding to blitz-paint; the chunk adds no scrolling
@answer n=7 entity=surface#7.4 v=no
why: select list scrollbar thumb paint; the chunk paints nothing and scrolls nothing
@answer n=8 entity=surface#8.1 v=no
why: mobile urlbar tokens; the chunk touches no mobile build
@answer n=9 entity=surface#9.1 v=no
why: mobile frame class drives urlbar sizing; the chunk touches no mobile build
@answer n=10 entity=surface#10.1 v=no
why: Android hardware back button; the chunk touches no mobile build
@answer n=11 entity=surface#11.1 v=no
why: mobile screenshot default file name; the chunk takes no screenshot
@answer n=12 entity=surface#12.1 v=no
why: iOS and Android CI status; the chunk touches no mobile build
@answer n=13 entity=surface#13.1 v=no
why: WPT runner result colours; the chunk does not touch the WPT runner
@answer n=14 entity=surface#13.2 v=no
why: WPT runner bright black tokens; the chunk does not touch the WPT runner
@answer n=15 entity=surface#13.3 v=no
why: WPT runner OSC 8 hyperlinks; the chunk does not touch the WPT runner
@answer n=16 entity=surface#14.1 v=no
why: paint_bench output lines; the coverage command is a driver surface, not that binary
@answer n=17 entity=surface#14.2 v=no
why: screenshot output lines; the coverage command is a driver surface, not that binary
@answer n=18 entity=surface#15.1 v=yes
why: the escher-session command-line answer list is where the new coverage verb's one-line JSON answer must be recorded, as an instance the convention's list must record
@answer n=19 entity=surface#15.2 v=no
why: the six served tasks list; the chunk adds no task and the scope names no app
@answer n=20 entity=surface#16.1 v=yes
why: the graph verb's answer shape (nodes with handlers, owner ids, event names) is what the coverage command reads and leaves unchanged
@answer n=21 entity=surface#17.1 v=yes
why: the actions verb's answer shape is what the owed test keeps and compares whole on replay
@answer n=22 entity=surface#17.2 v=yes
why: the actions answer's causes come from the same validate and check_target the test compares whole on replay, and the scope names that verb's answer
@answer n=23 entity=surface#18.1 v=yes
why: the pending unpaged graph item is named in the scope, and the coverage count reads the graph's handler set whole
@answer n=24 entity=surface#19.1 v=yes
why: the pending unpaged actions item is named in the scope, and the kept actions answer is compared whole
@answer n=25 entity=surface#20.1 v=no
why: WASM bundled fonts; the chunk measures no text and builds no web-spa target
