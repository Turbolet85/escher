@answer n=1 entity=error-handling#1.1 v=yes mode=stale
why: VERBS moved from thirteen to fourteen, so the row's "verb count unchanged" and its schema.rs line citations moved with the session's schema change
@answer n=2 entity=error-handling#2.1 v=no
why: its stylo.rs, events.rs and blitz-shell convert_events.rs panic paths are in files the session did not reach
@answer n=3 entity=error-handling#3.1 v=no
why: its blitz-shell, blitz-vibey-script and dioxus-native setup panics are in files the session did not reach
@answer n=4 entity=error-handling#4.1 v=no
why: the parallel traversal lock lives in blitz-dom stylo.rs, which the session did not reach
@answer n=5 entity=error-handling#5.1 v=no
why: the select list readers in blitz-dom document.rs are not in the session's reached files and their get_node path is unchanged
@answer n=6 entity=error-handling#6.1 v=no
why: the buffered mounted-handle path lives in dioxus_document.rs and events.rs, which the session did not reach
@answer n=7 entity=error-handling#7.1 v=no
why: the MutationBuffer in mutation_buffer.rs is not in the session's reached files
@answer n=8 entity=error-handling#8.1 v=no
why: its limit is the mounted-handle residual on the render-time task path, which the session does not touch
@answer n=9 entity=error-handling#9.1 v=no
why: flush_queued_mounted_events in dioxus_document.rs is not in the session's reached files
@answer n=10 entity=error-handling#10.1 v=no
why: deallocate_anonymous_block in blitz-dom document.rs is not in the session's reached files
@answer n=11 entity=error-handling#11.1 v=no
why: the select list closing paths in blitz-dom document.rs and select_popup.rs are not in the session's reached files
@answer n=12 entity=error-handling#12.1 v=no
why: the net handler trait and ResourceHandler in blitz-traits and blitz-dom net.rs are not in the session's reached files
