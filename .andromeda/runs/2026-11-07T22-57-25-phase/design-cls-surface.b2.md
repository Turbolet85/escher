@answer n=1 entity=surface#6.1 v=no
why: macOS browser titlebar of apps/browser; the chunk changes no shell or browser app surface
@answer n=2 entity=surface#6.2 v=no
why: transparent example window decoration and shell close; no window or shell surface is reached
@answer n=3 entity=surface#6.3 v=no
why: window compositing alpha mode in the dioxus-native config; the chunk changes no window setup
@answer n=4 entity=surface#7.1 v=yes
why: the scope names the web-spa surface rendering into one canvas, and the reserved canvas-element-plus-virtual-key id form rests on the wasm surface being a canvas appended to the page; its host-CSS sizing is not named
@answer n=5 entity=surface#7.2 v=no
why: MountedData calls under the buffered render; the chunk's vdom walk calls no MountedData method and mutation_buffer and dioxus_document are not reached
@answer n=6 entity=surface#7.3 v=no
why: optional scrollbars feature forwarding to blitz-paint; the chunk uses no scrollbars
@answer n=7 entity=surface#7.4 v=no
why: scrolled select list thumb painting; the chunk touches no select list painting
@answer n=8 entity=surface#8.1 v=no
why: mobile urlbar font and padding tokens in browser.css; the chunk changes no mobile browser surface
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class drives urlbar sizing in apps/browser; that file is not reached
@answer n=10 entity=surface#10.1 v=no
why: Android hardware back button navigation in the browser; the chunk changes no navigation
@answer n=11 entity=surface#11.1 v=no
why: mobile screenshot default filename in apps/browser capture; the chunk takes no screenshot
@answer n=12 entity=surface#12.1 v=no
why: iOS and Android are not tested in CI; the chunk adds no mobile target or OS-only check
@answer n=13 entity=surface#13.1 v=no
why: result colours of the wpt runner, a different CLI from the driver; wpt/runner is not reached
@answer n=14 entity=surface#13.2 v=no
why: bright-black tokens of wpt runner output, a different CLI from the driver; wpt/runner is not reached
@answer n=15 entity=surface#13.3 v=no
why: OSC 8 hyperlinks gated on a stdout terminal test in the wpt runner; that is the wpt runner's rule, not the driver's, and wpt/runner is not reached
@answer n=16 entity=surface#14.1 v=no
why: paint_bench output lines; examples/paint_bench is not reached and is not the driver
@answer n=17 entity=surface#14.2 v=no
why: examples/screenshot output lines; the chunk takes no screenshot and this is not the driver's screenshot verb
@answer n=18 entity=surface#15.1 v=yes
why: task item 2 names the escher-session cli surface component patterns; the graph read is a new verb whose answer and refusals cross this surface, so its JSON-only stdout, no-ESC rule, exit classes 0 1 2 3 and its MCP role bind, and its verb list must record the new verb
@answer n=19 entity=surface#15.2 v=no
why: escher-session's six-task list and usage and enum text in session_host.rs; the chunk adds no task, edits no usage or enum, and moves Cells' signals without changing which tasks are served
@answer n=20 entity=surface#16.1 v=no
why: WASM bundled DejaVu fonts and the font context; the chunk's not_needed item excludes font loading and the graph read measures no text
@answer n=21 entity=surface#17.1 v=no
why: wasm_hello example canvas styling and tabindex focus; the scope names the canvas listener owner case, not its focus or outline
