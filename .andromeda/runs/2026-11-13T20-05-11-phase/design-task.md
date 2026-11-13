@task_prose part=chunk
Restructures three functions without changing behaviour, to bring their cognitive complexity under or toward the ceiling of 15: blitz-dom's click handler (handle_click), escher-driver's executor (execute) and dioxus-native-dom's UI graph builder (graph).
The extracted helpers stay in the file that holds each function today; there is no file split and no new module, and no new helper reads over 15.
Nothing observable changes: dispatch order, event sequences, click default actions, refusal order, outcomes, span fields, snapshot content and graph content all stay the same, and no verb, argument, key, field, cause, session error, exit or log field is added.
The chunk re-measures the three functions and every extracted helper before and after with rust-code-analysis-cli on the dev host, and re-runs the existing checks that cover each moved arm, adding a check first for any arm that no check reaches.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, which holds the click default actions: checkbox and radio input and change, button focus and submit, the select arms, links, summary and labels — edits: the handle_click function, plus helpers extracted from it in the same file
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor, which holds the per-verb dispatch, the target check and the refusal order — edits: the execute function, plus helpers extracted from it in the same file
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph, which builds the document's graph of nodes and handlers — edits: the graph function, plus helpers extracted from it in the same file
@task_item n=1 part=needs
The click default actions of a single select must stay the same: click, Enter or Space opens the list, selects that open no list keep doing so, and a focused select with no box keeps its focus and keys.
@task_item n=2 part=needs
A single select's list events (open, close, choose, choosing the chosen option, choosing a disabled row, highlight move, and closing when its select changes) must keep their event sequences after the handle_click arms move.
@task_item n=3 part=needs
The select list as an engine overlay: the open list answers hits first and is placed in the viewport. The select arms of the click handler act on it, and its behaviour must not change.
@task_item n=4 part=needs
The snapshot and accessibility tree of a select (chosen option, highlighted row, open attribute, open select) and the snapshot and screenshot of a row of an open select list must stay the same after the click arms are restructured.
@task_item n=5 part=needs
The driver verbs and the virtual clock: click, press, type and advance go through the executor's per-verb dispatch. Their clock behaviour and their order must be preserved, and the executor's single advance moves the one virtual clock.
@task_item n=6 part=needs
Actions are decided by the verbs' own checks: the actions line (its content, nodes and stale marker) is computed from the same target checks that the executor runs. Moving the target check into a helper must not change what it accepts or refuses.
@task_item n=7 part=needs
The UI graph line's content, and escher-session's graph answer and graph arguments, are produced from the graph function's output. Graph content must stay byte-identical.
@task_item n=8 part=needs
The coverage line and escher-session's coverage answer read the handler graph, which is checked by handler coverage. Restructuring graph must not change which handlers appear.
@task_item n=9 part=needs
Driver command-line output is read by agents and has no colour. Because the restructure adds no key, field or line, the executor's answers must keep the same shape on stdout and in the mcp role.
@task_item n=10 part=needs
escher-session's session error object and its remedy, and the exit statuses read from the schema's table: the executor's refusal order decides which error and exit an invalid call gets, and that order must be preserved.
@task_item n=11 part=needs
MountedData calls during a render (document state, set_focus_to) and the buffered render that is replayed under one borrow: a focus move made by a button or label click arm must keep happening at the same point.
@task_item n=12 part=needs
Driver scroll is instant, and into-view scroll is instant in nested boxes. A link or label arm that scrolls a target into view must keep that behaviour.
@task_standing n=1
convention: every new check added to cover an arm before it moves builds the engine with system fonts, the same as every other test
home: typography
@task_standing n=2
convention: every new check that drives clicks, presses or typing goes through the driver verbs on the virtual clock, never through wall-clock waits
home: motion-calibrated-to-expression-level-unrecorded · motion
@task_item n=1 part=not_needed
The colour palette (engine colours, select list colours, colour-scheme recascade, colour tokens): the restructure changes no colour or paint value.
@task_item n=2 part=not_needed
Spacing, border radius and brand identity: no spacing, radius or brand value is involved in the three functions.
@task_item n=3 part=not_needed
The WPT runner's output and result colours: the chunk does not touch the WPT runner.
@task_item n=4 part=not_needed
The shell provider's window controls (maximize, minimize, decorations toggle, window drag) and the window's compositing alpha mode: these are window-level surfaces, not click default actions, the executor or the graph.
@task_item n=5 part=not_needed
The JS window object's dimensions and device pixel ratio, and the logical viewport computed from scale: the chunk changes no viewport sizing.
@task_item n=6 part=not_needed
Favicon reading, mobile targets that are built but not tested, and the web-spa canvas surface: none of these is reached by the three functions.
@task_item n=7 part=not_needed
Overlay scrollbar fade timings, smooth scroll-behavior with wheel cancel, and animation driven by the clock: the chunk changes no scrolling or animation timing.
@task_item n=8 part=not_needed
escher-session's task catalogue (counter, crud, cells, timer, flight-booker, load, unknown task): the chunk adds and changes no task.
