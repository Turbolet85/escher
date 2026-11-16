@task_prose part=session
The session added an in-process layout read: for an element, why it sits where it does at its size, read from the style and layout chain (v030-11, built and not claimed).
The engine gains a read-only per-document reader answering how a box was placed (Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select, SelectList), what shifts it (Scroll, Transform), and why each axis has its size (Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content, Layout).
The driver gains Session::layout, answering by stable string id with closed words and numbers; it runs no step, pump, tick or settle, moves no clock, opens no span and changes nothing on screen.
No verb, escher-session command, MCP tool, socket call, kept check, argument, result field or exit status was added; the verb list still holds fourteen, and the absence of every carrier of the read is PROVISIONAL on the delegate's answer, for the founder's word.
An element the engine cannot explain answers Block with its snapshot bounds and both causes Layout in process; it owes a closed word of its own before the read crosses any carrier.
A closed select's option reads placed by Select at its select's box; a closed select's optgroup reads nothing.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: new; the engine's layout read, which explains an element's placement, shifts and size causes from the computed style and the layout geometry
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: edited; the engine document crate's root, its module list and public re-exports
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the dioxus-native DOM integration crate's root and the engine types it re-exports
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: new; the driver's in-process layout explanation for an element by its stable id
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: new; the driver layout explanation's unit tests
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: edited; the driver's verb execution, the run path that resolves an id to its element and refuses stale or not-found
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: edited; the driver crate's root, its session, modules and public exports
@task_file n=8 path=tests/blitz-tests/tests/layout_reason.rs
what: new; the engine test of every placing word and size cause on fixture pages, in both layout modes
@task_file n=9 path=tests/blitz-tests/tests/stand_layout.rs
what: new; the stand test of the driver layout read on the six served task screens, joining the stand-repeat run
@task_file n=10 path=docs/capabilities.md
what: edited; the capability statement, what the project can and cannot do, including its absences
@task_item n=1 part=touches
The escher-session command list and the MCP tool results: no layout command or tool was added, so a row stating the full command set stands, but the layout read is now a capability with no carrier on either.
@task_item n=2 part=touches
The decision that escher-session is read by an agent and that driver command line output is read by agents: the new read is reachable only from Rust code in process, so an agent at the command line cannot ask why an element sits where it does.
@task_item n=3 part=touches
The decision that actions are decided by the verbs' own checks: the layout read resolves its id through the same run-path resolve the verbs use, refusing stale or not-found.
@task_item n=4 part=touches
The select rows over a closed select and its list as an engine overlay: the layout read names Select for a closed select's option, read at its select's box, and SelectList for a row of the open list.
@task_item n=5 part=touches
The snapshot's bounds of an open select list row and the open list placed inside the viewport: the layout read answers with the snapshot's bounds and an offset chain from its container held to 1/64 px.
@task_item n=6 part=touches
The virtual clock and settle rows (driver advance moves the one clock, load wait moves no clock, settle does not wait on animation): the layout read moves no clock and leaves delivered timer ticks unapplied.
@task_item n=7 part=touches
The scroll rows (driver scroll is instant, into-view scroll instant in nested boxes, smooth scroll-behavior): the layout read reports a scrolled holder as a Scroll shift and leaves the holder's own bounds unmoved.
@task_item n=8 part=touches
The stacking and paint rows over transforms: the layout read reports a transformed holder as a Transform shift, including a scale of zero.
@task_item n=9 part=touches
The escher-session tasks rows (cells, crud and the other served screens): every snapshot node on the six served screens answers the layout read; a cells grid cell reads Grid and a crud list row reads Flex with width Stretch.
@task_item n=10 part=touches
The logical viewport and replaced-element layout readings: the layout read names Replaced for a replaced element's size and resolves calc through the engine's one resolver.
@task_item n=11 part=touches
The color scheme, select colour and text locale rows that mention layout reads of their own subject: the new read restates none of them and changes no document's behaviour.
@task_fact n=1 slug=surface-cli
fact: No escher-session command, MCP tool, socket call or kept check carries the layout read; why an element sits where it does is read in process only, an absence held PROVISIONAL for the founder's word.
@task_fact n=2 slug=surface
fact: The layout read answers an element by its stable id with a placing word, its shifts and a size cause per axis, all closed words and numbers; a closed select's option reads Select at its select's box.
@task_item n=1 part=untouched
Brand identity, the color palette tokens, typography and the project type scale: no colour, font or type value moved.
@task_item n=2 part=untouched
Iconography and cursor resolution, the favicon and the shell provider's window controls: none is reached.
@task_item n=3 part=untouched
The escher-session exit statuses and exit codes, stderr session and usage errors and the remedy object: no status, error or remedy was added.
@task_item n=4 part=untouched
The WPT runner output and result colours, and the coverage, actions and UI graph lines: no line or colour moved.
@task_item n=5 part=untouched
The screenshot verb and its refusals, hit-testing and paint order: nothing painted or hit moved.
@task_item n=6 part=untouched
The web-spa surface, mobile targets and the WASM font context: no target or platform build was reached.
