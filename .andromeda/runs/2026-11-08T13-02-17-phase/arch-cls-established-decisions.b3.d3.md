@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered render's focus limit concerns task polling and set_focus_to, and the read runs no step and dispatches nothing.
@answer n=2 entity=established-decisions#25.3 v=no
why: the task-spawn timing on dioxus-core 0.7.10 concerns event and render ordering, which the read never drives.
@answer n=3 entity=established-decisions#26.1 v=no
why: the component-naming walk from base_scope is not named by the scope, and the read's owner is the handler element, not a component.
@answer n=4 entity=established-decisions#26.2 v=yes
why: DioxusDocument::graph, which the scope names, reads the registered state's readers and recorded writes, and graph.rs is reached whole.
@answer n=5 entity=established-decisions#27.1 v=no
why: the event dispatch set in DioxusEventHandler::handle_event is on the event path, and the read dispatches nothing and touches no handler.
@answer n=6 entity=established-decisions#28.1 v=no
why: the browser app's history, favicon and about pages are outside the chunk, which reaches only the driver, the Dioxus bridge's reads and the stand.
@answer n=7 entity=established-decisions#29.1 v=no
why: tab lenses in the browser app are outside the chunk's reach.
@answer n=8 entity=established-decisions#30.1 v=no
why: the WPT runner's rules are outside the chunk, which runs and changes no WPT output.
@answer n=9 entity=established-decisions#31.1 v=yes
why: the driver session's socket, carriers, boundary widening, serde_json reader-only rule and command span bound the new read's answer and its wire crossing.
@answer n=10 entity=established-decisions#31.2 v=yes
why: the select list's crossing as a row's bounds and the rule of no new node key bind the owner-served select options and groups the read lists.
@answer n=11 entity=established-decisions#31.3 v=yes
why: the UI graph's crossing rule (stable ids, owner ids, closed flag words, booleans, never a NodeId, clickable bool) binds the new read's answer, which uses the same GraphNode model.
@answer n=12 entity=established-decisions#32.1 v=yes
why: the kept check's recording and Session::run replay of answered calls bind the new read, which is recorded and replayed in recording.rs and replay.rs.
