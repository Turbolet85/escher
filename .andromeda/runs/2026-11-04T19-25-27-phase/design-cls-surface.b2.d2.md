@answer n=1 entity=surface#6.1 v=no
why: the macOS titlebar of the browser app; the chunk's stand is seven_guis, not apps/browser
@answer n=2 entity=surface#6.2 v=no
why: the transparent example's decoration-less window; the probe opens no transparent window
@answer n=3 entity=surface#6.3 v=no
why: compositing alpha mode is a launch config the probe neither sets nor reads; it measures nodes, handlers and state
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 canvas path; the probe is native and headless
@answer n=5 entity=surface#7.2 v=no
why: the scrollbars feature forwards to paint; the probe reads nodes and layout and does no scrolling
@answer n=6 entity=surface#7.3 v=no
why: the select-list scroll thumb is paint under a feature; the probe reads no select list and does no scrolling
@answer n=7 entity=surface#8.1 v=no
why: mobile urlbar font and padding; mobile-native is not reached by the chunk
@answer n=8 entity=surface#9.1 v=no
why: the mobile frame class drives the mobile urlbar; mobile-native is not reached by the chunk
@answer n=9 entity=surface#10.1 v=no
why: the Android hardware back button in the browser tab; mobile-native is not reached by the chunk
@answer n=10 entity=surface#11.1 v=no
why: mobile screenshots saved without a dialog; mobile capture is not reached by the chunk
@answer n=11 entity=surface#12.1 v=no
why: iOS and Android build and CI coverage; the probe runs headless on desktop and reaches no mobile target
@answer n=12 entity=surface#13.1 v=no
why: WPT runner result colours; CLI binary terminal colours are dropped by the task's not-needed list and the WPT runner is not reached
@answer n=13 entity=surface#13.2 v=no
why: WPT runner test-kind and heading colours; CLI binary terminal colours are dropped by the task's not-needed list
@answer n=14 entity=surface#13.3 v=no
why: WPT runner OSC 8 hyperlinks; the probe does not touch the WPT runner
@answer n=15 entity=surface#14.1 v=no
why: paint_bench output lines; the probe prints nothing and the example is not reached
@answer n=16 entity=surface#14.2 v=no
why: screenshot example output lines; the example is not reached by the probe
@answer n=17 entity=surface#15.1 v=yes
why: the scope names escher-session stdout and mcp output staying as they are, so this one-line JSON, no-ESC and exit-status contract binds the probe's no-crossing bound
@answer n=18 entity=surface#15.2 v=yes
why: the probe drives the Counter or CRUD task plus a served task, and this entity lists the six served tasks and the unknown-task usage ending
@answer n=19 entity=surface#16.1 v=no
why: WASM font-context registration; the probe is native and headless, and the chunk's font convention is not stated here
@answer n=20 entity=surface#17.1 v=no
why: the wasm_hello canvas; web-spa is not reached by the chunk
