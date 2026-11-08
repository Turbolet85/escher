@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered render, initial_build, poll and flush_queued_mounted_events are unchanged, so a task polled during render still acts on the document as the previous render left it.
@answer n=2 entity=established-decisions#25.3 v=no
why: the render path and the poll arms on dioxus-core 0.7.10 are unchanged, and the registry dispatch set around handle_event does not move which arm polls a task.
@answer n=3 entity=established-decisions#26.1 v=no
why: the owning-component walk and its runtime probe are unedited and still name a component for every task-body element on Counter and CRUD in both layout modes.
@answer n=4 entity=established-decisions#26.2 v=yes mode=stale
why: use_graph_state reaches hook-local state's readers and recorded writes through the SignalRegistry with no fork-level change, so the reading that hook-local state is unreachable without one now needs the registry route, plain use_signal staying unreached.
@answer n=5 entity=established-decisions#27.1 v=no
why: the browser app is untouched by the chunk, and its event path stays inert with no registry present.
@answer n=6 entity=established-decisions#28.1 v=no
why: the browser app is untouched by the chunk, so the tab-lens behaviour holds as before.
@answer n=7 entity=established-decisions#29.1 v=no
why: the WPT runner is untouched by the chunk.
@answer n=8 entity=established-decisions#30.1 v=yes mode=add-to
why: the socket, the command line, MCP and kept checks now also carry the UI graph (ids, component and state names, handler event names, flag words and booleans, never a value or a NodeId), a ratified widening (inputs#I4, I9) the crossing list must name.
@answer n=9 entity=established-decisions#30.2 v=yes mode=contradict
why: select options now cross the graph answer as nodes whose owner is the select's stable id, and the snapshot gains the clickable node key, so the list no longer crosses only as bounds and pixels.
@answer n=10 entity=established-decisions#31.1 v=yes mode=add-to
why: kept checks now also hold the UI graph's answer whole (ids, component and state names, handler event names, flag words), a ratified widening the rest-contents list must name, while the two committed checks exception is unchanged.
