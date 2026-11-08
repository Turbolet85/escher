@answer n=1 entity=error-handling#1.1 v=yes mode=stale
why: schema.rs and execute.rs are reached whole by the graph verb joining the closed verb table, so the cited schema.rs and execute.rs lines can have moved and the verb count went from twelve to thirteen
@answer n=2 entity=error-handling#2.1 v=no
why: the todo and unimplemented paths sit in stylo.rs, events.rs, convert_events.rs and the accesskit adapters, none of which the session reached
@answer n=3 entity=error-handling#3.1 v=no
why: its setup unwraps sit in blitz-shell, blitz-vibey-script and dioxus-native lib.rs, none reached, and the new app API adds no setup path
@answer n=4 entity=error-handling#4.1 v=no
why: the parallel style traversal lock sits in blitz-dom stylo.rs, which the session did not reach
@answer n=5 entity=error-handling#5.1 v=no
why: the select list readers sit in blitz-dom document.rs, which the session did not reach
@answer n=6 entity=error-handling#6.1 v=yes mode=stale
why: dioxus_document.rs is reached whole by the bridge event dispatch change, so its cited lines 310 and 318 can have moved, though the buffered render and flush are untouched
@answer n=7 entity=error-handling#7.1 v=no
why: the MutationBuffer sits in mutation_buffer.rs, which the session did not reach and whose recorder it leaves untouched
@answer n=8 entity=error-handling#8.1 v=no
why: the limit on mounted handles from a render-polled task is unchanged by the session and stays owned by its own open residual row
@answer n=9 entity=error-handling#9.1 v=yes mode=stale
why: dioxus_document.rs is reached whole, so its cited call site lines 255-259 can have moved with the bridge event dispatch change
@answer n=10 entity=error-handling#10.1 v=no
why: deallocate_anonymous_block sits in blitz-dom document.rs, which the session did not reach, and the registry dispatch is inert without a registry so the key press path holds
@answer n=11 entity=error-handling#11.1 v=no
why: the select list closing paths sit in blitz-dom and select_popup tests, none of which the session reached
@answer n=12 entity=error-handling#12.1 v=no
why: failed-load handling sits in blitz-traits and blitz-dom net.rs, neither reached by the session
