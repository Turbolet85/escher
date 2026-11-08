@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered render, initial_build and poll are unchanged, so the previous-render limit still holds
@answer n=2 entity=established-decisions#25.3 v=yes mode=add-to
why: the bridge's handle_event now sets the registry dispatch around runtime.handle_event, and a handler-spawned task's first poll in render_immediate falls outside that window
@answer n=3 entity=established-decisions#26.1 v=no
why: the vdom walk and its Counter and CRUD probe are unchanged, and the graph only reads the component through that walk
@answer n=4 entity=established-decisions#26.2 v=yes mode=contradict
why: use_graph_state and the SignalRegistry now reach a registered hook-local state's writers and readers with no fork change, against the claim that they are unreachable
@answer n=5 entity=established-decisions#27.1 v=no
why: the browser app is untouched by the chunk
@answer n=6 entity=established-decisions#28.1 v=no
why: the browser app is untouched by the chunk
@answer n=7 entity=established-decisions#29.1 v=no
why: the WPT runner is untouched by the chunk
@answer n=8 entity=established-decisions#30.1 v=yes mode=stale
why: the UI graph now crosses the socket, command line and MCP as a ratified widening, and the unpaged Cells graph answer of 293,869 B moves the 22,404-byte largest whole-screen answer reading
@answer n=9 entity=established-decisions#30.2 v=yes mode=contradict
why: the graph carries option and optgroup ids and their select_of owner ids across the carriers, so the select list no longer crosses only as bounds
@answer n=10 entity=established-decisions#31.1 v=yes mode=add-to
why: kept checks now also hold the UI graph answers at rest, a ratified widening of what a kept check carries
