@carried n=1 label=architecture/res-network-ports-and-listeners fact=fct-crxfkvwh
why: whatever shape the P4 fork gives the coverage read, its answer crosses only these carriers, holds no NodeId and stays inside the request and answer bounds, with no second listener.
quote: serving three requests at wire version `v3`, a call's replies including the fixed `recording-failed` — `hello`, `stop` and, since 2026-10-10-driver-cli, `call` — so a call's element id and typed text cross in and an answer's ids, accessible names, control values (a password's or a file input's as the fixed mask), snapshot text and diff cross out, no `NodeId` among them, a request line at most 16,384 bytes and an answer at most 1,048,576; still no second served listener;
@carried n=1 run=2 label=architecture fact=fct-uu6lok3l
quote: one local listener, the driver session's Unix-domain socket `{state_dir}/session.sock` (`0600`, in a `0700` state directory), unix only; the `escher-session mcp` role speaks over its own stdio, with no listener; still no second served listener.
@carried n=2 label=architecture/res-network-ports-and-listeners fact=fct-o232y5ww
why: the coverage list is read from the graph's handlers and carries no more than the graph answer already carries — stable ids, owner ids and handler event names — never a value, typed text or NodeId.
quote: Both carriers, and a kept check, also take the screen's UI graph out as the `graph` verb's answer — stable ids, owner ids, component names, handler event names, registered state names, the declared writes the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`; `graph` takes no argument, and a call that gives one is refused `malformed` with nothing run;
@carried n=3 label=architecture/res-network-ports-and-listeners fact=fct-jvyhausr
why: the owed replay test keeps an actions step in a check and compares the replayed answer to the kept one key by key in exactly this shape.
quote: Both carriers, and a kept check, also take the screen's actionable nodes out as the `actions` verb's answer — each actionable node's stable id, owner id and `keyed` boolean, the table's verb words and the closed cause words, never a value, typed text or `NodeId`; `actions` takes no argument, and a call that gives one is refused `malformed`;
@carried n=4 label=architecture/res-filesystem fact=fct-xzzhfdyl
why: the replay test's recording scratch directory sits with the recording checks' scratch directories under target/tmp, local only and uploaded by no CI leg.
quote: under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, beside the recording checks' scratch directories and escher-driver's unit tests', which reach the workspace `target/tmp` through the compile-time `CARGO_MANIFEST_DIR`, local only and uploaded by no CI leg
@carried n=5 label=architecture/res-filesystem fact=fct-p7m5lrau
why: the replay test records into this medium, and the fork on how a set of checks reaches the coverage command may not name a check path by a tool call, verb, argument or result field without a founder-ratified widening.
quote: The recording directory: a path the launcher names by `--record <dir>` on `start`, the host role or the `mcp` role (`Session::recording_into` in process) — never an env read or a default, and named by no tool call, verb, argument or result field — created `0700` when absent, its parent existing, and refused `RecordDirNotPrivate` when present and open to its group or others; it holds one `check-{n}.json` per recorded session (`0600`, `create_new`, `n` the smallest from 1 naming no file, so no file is replaced), rewritten whole after every call through `.check-{n}.json.tmp` (`0600`) and a rename;
@carried n=5 run=2 label=architecture fact=fct-kjd5y7c4
quote: The recording directory is a path the launcher names by `--record <dir>`, never an env read or a default and named by no tool call, created `0700` and refused when open to its group or others;
@carried n=6 label=architecture/res-filesystem fact=fct-bw7ybeh6
why: the committed checks are recorded checks the stand replays and the one owner-only exception, so a check the chunk compares across OSes holds no screenshot step and any new committed check joins this exception.
quote: The committed checks: `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, git mode `100644` in the public repository — the one exception to a kept check's owner-only medium: two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the `mcp` role, holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name or secret and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577);
@carried n=7 label=architecture/res-process-wide-state-and-threads fact=fct-drud2scu
why: a coverage call run through Session::run emits under the one existing escher_driver command span and adds no log target.
quote: log target `escher_driver`, under which escher-driver's command span is emitted at INFO since 2026-10-07-driver-command-spans and its `tool_call` span since 2026-10-10-mcp-surface
@carried n=8 label=architecture/res-process-wide-state-and-threads fact=fct-jzxi3spj
why: if the coverage read lands as a verb, its witness in host_log and mcp_log reads its needles from this one shared module, stated once, as the graph and actions calls do.
quote: `graph_needles` (each served component and registered state name, for the one `graph` call each log check makes) and `actions_needles` (the CRUD screen's listed author-key ids holding a `-`, for the one `actions` call each log check makes)
@carried n=9 label=architecture/res-process-wide-state-and-threads fact=fct-j4gmcppp
why: the new exercised record is held per document like SignalRegistry, adding no process-wide state, log site, print or env read.
quote: dioxus-native-dom's `SignalRegistry` is a Dioxus root context of one document's virtual DOM, not process-wide state, with no log site, print or env read
@carried n=10 label=architecture/res-process-wide-state-and-threads fact=fct-3eiirvbg
why: the replayed actions step and any coverage call keep a command span recording the verb alone, read with zero id and name needles.
quote: An `actions` call's `command` span records `verb` alone (`cause` when refused), its `tool_call` parenting it under the `mcp` role, and `host_log` and `mcp_log` each make one actions call and read 0 id, name and actions needles at `trace` under both builds;
@carried n=11 label=architecture/crosscut-config-management fact=fct-dw3hon6z
why: the exercised record must hold on every app, so it is taken where no feature gate removes it, while the coverage read beside the graph stays under accessibility.
quote: `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules, its `signal_registry` module ungated
@carried n=11 run=2 label=architecture fact=fct-nmnb5xux
quote: optional capabilities are compile-time Cargo features forwarded down the crates, `accessibility` also gating the dioxus-native-dom snapshot modules;
@carried n=12 label=architecture/crosscut-logging-and-timing fact=fct-4onw7te5
why: execute.rs and mcp.rs, edited for any new verb, keep escher-driver's spans hand-built through tracing, ungated, installing no subscriber.
quote: escher-driver emits its command span and, since 2026-10-10-mcp-surface, its `tool_call` span through `tracing` ungated and installs no subscriber (packages/escher-driver/src/execute.rs:109-124);
@carried n=12 run=2 label=architecture fact=fct-wkx6y4nz
quote: escher-driver emits its spans ungated and installs no subscriber;
@carried n=13 label=architecture/crosscut-service-wiring fact=fct-biqhin7y
why: a page with no registered state has no SignalRegistry, so the record of exercised handlers must not live in the registry alone.
quote: The first `use_graph_state` hook of a page provides a `SignalRegistry` context at `ScopeId::ROOT`, which the bridge's dispatch looks up with `runtime.has_context`; a page that registers no state has none
@carried n=14 label=architecture/crosscut-event-processing fact=fct-klon5sda
why: the record counts the handlers a dispatch reached; zero-handler kinds are skipped by the bridge, and a mounted firing, outside the graph's handlers, is no check's exercise.
quote: dioxus skips event kinds with zero handlers (packages/dioxus-native-dom/src/dioxus_document.rs:369-375; packages/dioxus-native-dom/src/mutation_writer.rs:349-359); `mounted` listeners fire after `initial_build` and each `poll`
@carried n=14 run=2 label=architecture fact=fct-p37qjcmu
quote: Dioxus skips event kinds with zero handlers;
@carried n=15 label=architecture/crosscut-event-processing fact=fct-56u6r4qr
why: the reached (element, event) record is taken in the bridge's handle_event on every app, registry or not, beside the registry's dispatch, leaving the buffered render, initial_build, poll and the mounted flush untouched.
quote: Where a `SignalRegistry` stands at the root, `DioxusEventHandler::handle_event` sets the registry's dispatch — the chain head first, and its event name — around `runtime.handle_event` and clears it after, so a signal write is recorded against the dispatch that caused it; a page with no registered state dispatches as before, and the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched
@binding n=1
**crosscut-event-processing ↔ crosscut-service-wiring ↔ crosscut-config-management**: the record of exercised (element, event) handlers is taken in the bridge's dispatch path on every app — where no `SignalRegistry` stands as where one does — ungated like `signal_registry`, mounted firings and zero-handler kinds never counted, while the coverage read that pairs it with the graph's handlers stays under `accessibility` (architecture/crosscut-event-processing fct-56u6r4qr) (architecture/crosscut-service-wiring fct-biqhin7y) (architecture/crosscut-config-management fct-dw3hon6z) (architecture/crosscut-event-processing fct-klon5sda)
@binding n=2
**res-network-ports-and-listeners ↔ the scope's boundaries**: the coverage answer carries only stable ids, owner ids and handler event names the `graph` answer already carries, no `NodeId`, value or typed text, inside the 1,048,576-byte answer bound on the existing carriers; any new verb, argument kind or result field it needs is a boundary widening escalated at P4 (architecture/res-network-ports-and-listeners fct-o232y5ww) (architecture/res-network-ports-and-listeners fct-crxfkvwh)
@binding n=3
**res-filesystem ↔ res-network-ports-and-listeners**: the owed replay test records a session holding an `actions` step into an owner-only recording directory under `target/tmp`, keeps it, reads it back and replays it, comparing the `actions` answer in its carried shape; a set of checks reaches the coverage command without a tool call naming a check path, and a check compared across OSes holds no `screenshot` step (architecture/res-filesystem fct-p7m5lrau) (architecture/res-filesystem fct-xzzhfdyl) (architecture/res-filesystem fct-bw7ybeh6) (architecture/res-network-ports-and-listeners fct-jvyhausr)
@binding n=4
**res-process-wide-state-and-threads ↔ crosscut-logging-and-timing**: a coverage call and a replayed `actions` step run under the existing hand-built `command` span recording the verb alone, no new log target, subscriber or log site; the exercised record is per document, never process-wide; a new verb's log witness reads its needles from the shared seven_guis module (architecture/res-process-wide-state-and-threads fct-3eiirvbg) (architecture/res-process-wide-state-and-threads fct-j4gmcppp) (architecture/res-process-wide-state-and-threads fct-drud2scu) (architecture/crosscut-logging-and-timing fct-4onw7te5) (architecture/res-process-wide-state-and-threads fct-jzxi3spj)
