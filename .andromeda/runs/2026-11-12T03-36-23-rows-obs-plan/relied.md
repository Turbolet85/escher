fct-w6etu55s@1 · The escher telemetry crate's formatter scrubs engine and escher targets, drops every other target, and prints one stderr line per event and one per closed span, with a chaining panic hook.
fct-ip3ek2mr@1 · Resource load errors in blitz-dom end in a warn event with an `error` field and are not propagated to callers.
fct-6jm2c55w@1 · A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it.
fct-lrj2spd7@1 · A log record from a target outside the sink's allowlist never reaches stderr.
fct-7au5a6cr@1 · A failed, empty or non-CSS stylesheet is answered as an error, so it leaves the pending critical resources and stops blocking render.
fct-omr4vkvp@1 · A net handler that does not override the error method drops the error, the trait's default doing nothing.
fct-x7v6upkc@1 · The mutation buffer's replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
fct-7tp6qj2l@1 · The actions read acts on nothing and changes nothing in the document.
fct-v53ms3q6@1 · A page with no registered state reaches no signal registry and dispatches as before.
fct-5rpr77we@1 · The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone.
fct-ovjvwkua@1 · Page scripts never see the native search-params parser, which is deleted from globalThis before any page script runs.
fct-ukvug6pr@1 · A failed load's error text reaches only the Resource load failed line, where the sink redacts url, error and the message.
fct-jyfscbc7@2 · A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone.
fct-46rgllvo@1 · A workspace-wide build turns the engine's tracing feature on, so a test child that installs escher's sink at info also receives engine events.
