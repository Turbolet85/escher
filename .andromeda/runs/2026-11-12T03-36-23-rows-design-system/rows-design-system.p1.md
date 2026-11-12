@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=S
@row_none n=3 rec=3 type=S
@row_add n=1 row=decision/select-list-colours-engine-constants area=color-palette rec=4
name: Select list colours as engine constants
rule: The open select list's colours are engine constants, not design tokens and not in the default stylesheet; its highlight is the same pair as a multiple select's chosen row.
why: The engine holds no design tokens.
@row_none n=4 rec=5 type=S
@row_none n=5 rec=6 type=S
@row_none n=6 rec=7 type=S
@row_none n=7 rec=8 type=S
@row_add n=2 row=decision/screenshot-never-skips-text area=typography rec=9
name: Screenshot never paints with text skipped
rule: A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
@row_add n=3 row=contract/driver-screenshot-refusals/fonts-unavailable area=typography rec=9
boundary: driver screenshot refusals
item: fonts-unavailable
promise: A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
@row_add n=4 row=decision/test-engine-with-system-fonts area=typography rec=10
name: Test engine built with system fonts
rule: The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_add n=5 row=decision/wasm-bundled-font-context area=typography rec=11
name: Bundled font context on WASM
rule: On WASM a font context with bundled fonts must be provided, a single-font context for one font.
@row_none n=8 rec=12 type=S
@row_none n=9 rec=13 type=S
@row_none n=10 rec=14 type=S
@row_none n=11 rec=15 type=S
@row_none n=12 rec=16 type=S
@row_none n=13 rec=17 type=S
@row_none n=14 rec=18 type=S
@row_none n=15 rec=19 type=S
@row_none n=16 rec=20 type=9
@row_none n=17 rec=21 type=S
@row_none n=18 rec=22 type=S
@row_add n=6 row=decision/hit-testing-stops-at-paint-clip area=depth-strategy rec=23
name: Hit-testing stops at a paint clip
rule: Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box.
except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
@row_none n=19 rec=24 type=S
@row_none n=20 rec=25 type=S
@row_none n=21 rec=26 type=S
@row_none n=22 rec=27 type=9
@row_none n=23 rec=28 type=S
@row_none n=24 rec=29 type=S
@row_none n=25 rec=30 type=S
@row_none n=26 rec=31 type=S
@row_none n=27 rec=32 type=9
@row_none n=28 rec=33 type=S
@row_add n=7 row=decision/into-view-scroll-instant-in-nested-boxes area=motion-calibrated-to-expression-level-unrecorded rec=34
name: Into-view scroll instant in nested boxes
rule: A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone.
except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.
@row_add n=8 row=decision/driver-scroll-instant area=motion-calibrated-to-expression-level-unrecorded rec=34
name: Driver scroll is instant
rule: The driver's scroll is instant in every scrolling box and in the viewport.
@row_none n=29 rec=35 type=S
@row_add n=9 row=decision/settle-does-not-wait-on-animation area=motion-calibrated-to-expression-level-unrecorded rec=36
name: Settle does not wait on animation
rule: A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
@row_add n=10 row=decision/animation-driven-by-virtual-clock area=motion-calibrated-to-expression-level-unrecorded rec=37
name: Animation driven by the virtual clock
rule: The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
@row_add n=11 row=contract/driver-verbs-and-virtual-clock/advance area=motion-calibrated-to-expression-level-unrecorded rec=37
boundary: driver verbs and the virtual clock
item: advance
promise: The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
@row_add n=12 row=contract/driver-verbs-and-virtual-clock/click area=motion-calibrated-to-expression-level-unrecorded rec=37
boundary: driver verbs and the virtual clock
item: click
promise: The driver's click leaves the VirtualClock where it was.
@row_add n=13 row=contract/driver-verbs-and-virtual-clock/type area=motion-calibrated-to-expression-level-unrecorded rec=37
boundary: driver verbs and the virtual clock
item: type
promise: The driver's type leaves the VirtualClock where it was.
@row_add n=14 row=contract/driver-verbs-and-virtual-clock/press area=motion-calibrated-to-expression-level-unrecorded rec=37
boundary: driver verbs and the virtual clock
item: press
promise: The driver's press leaves the VirtualClock where it was.
@row_add n=15 row=decision/load-wait-moves-no-clock area=motion-calibrated-to-expression-level-unrecorded rec=38
name: Load wait moves no clock
rule: While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
@row_none n=30 rec=39 type=S
@row_add n=16 row=absence/reduced-motion-handling area=motion-calibrated-to-expression-level-unrecorded rec=40
capability: Reduced-motion handling
@row_none n=31 rec=41 type=S
@row_none n=32 rec=42 type=S
@row_none n=33 rec=43 type=5
@row_none n=34 rec=44 type=S
@row_add n=17 row=decision/cursor-resolution area=iconography rec=45
name: Cursor resolution
rule: The cursor is the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included.
why: The shell keeps a visible arrow where the pointer is over no box.
@row_none n=35 rec=46 type=S
@row_none n=36 rec=47 type=S
@row_none n=37 rec=48 type=S
@row_none n=38 rec=49 type=5
@row_none n=39 rec=50 type=S
@row_none n=40 rec=51 type=5
@row_none n=41 rec=52 type=S
@row_none n=42 rec=53 type=5
@row_add n=18 row=decision/driver-output-for-agents area=surface-cli rec=54
name: Driver command line output read by agents
rule: The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not.
why: These surfaces are read by an agent, not a person.
@row_add n=19 row=contract/escher-session-stdout/command-line-line area=surface-cli rec=54
boundary: escher-session stdout
item: command line output
promise: One JSON line per command: the answer, the refusal or a session error's object.
@row_add n=20 row=contract/escher-session-stdout/mcp-line area=surface-cli rec=54
boundary: escher-session stdout
item: mcp role output
promise: Newline-delimited JSON-RPC 2.0 lines.
@row_add n=21 row=contract/escher-session-exit-codes/0 area=surface-cli rec=54
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
@row_add n=22 row=contract/escher-session-exit-codes/1 area=surface-cli rec=54
boundary: escher-session exit codes
item: 1
promise: The command was refused.
@row_add n=23 row=contract/escher-session-exit-codes/2 area=surface-cli rec=54
boundary: escher-session exit codes
item: 2
promise: A usage error.
@row_add n=24 row=contract/escher-session-exit-codes/3 area=surface-cli rec=54
boundary: escher-session exit codes
item: 3
promise: A session error; an mcp role ending in Err also exits 3.
@row_none n=43 rec=55 type=S
