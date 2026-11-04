@account n=1 entity=error-handling#1.1 v=carried
why: the scope's «nothing crosses» names cause and session error, so the probe's served task stays inside the closed refusal cause set and the session-error table, a refusal logging only its cause name
@account n=2 entity=error-handling#2.1 v=carried
why: link 2 reads a node's handlers in events.rs, and the probe must read its click listener through the listener map, never through an event-data conversion that hits unimplemented!
@account n=3 entity=error-handling#3.1 v=not_in_scope
why: it rules over shell and runtime setup panics (event loop, window, JS runtime, tokio), which the headless probe never constructs; a re-check every change owes
@account n=4 entity=error-handling#6.1 v=carried
why: it states the MountedData hazard and its coordinate events.rs:162, which the chunk re-verifies at HEAD and names in the founder's reading
@account n=5 entity=error-handling#7.1 v=not_in_scope
why: it rules over flush_queued_mounted_events in dioxus_document.rs, a unit the chunk only re-reads for its poll check; a re-check every change owes
@account n=6 entity=logging-monitoring#1.1 v=carried
why: the scope names «no log field or span» and «never logged», so the probe meets escher's sink with its allowlist unchanged, and its measured panic meets the unscrubbed panic hook the sink does not reach
@account n=7 entity=logging-monitoring#1.2 v=carried
why: the scope names that nothing a probe reads is printed, and this entity states the stdout and stderr paths (test diagnostics, the walk_tree DOM dump, escher-session's lines) the probe's output must keep clear of read values
