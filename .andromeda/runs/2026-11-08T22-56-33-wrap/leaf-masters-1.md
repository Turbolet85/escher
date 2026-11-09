@leaf_group group=masters-1
@leaf_read leaf=CLAUDE.md lines=153
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=33 class=omission
leaf-text: whose dispatch the bridge's `handle_event` sets around the vdom's only where a registry stands;
source: fct-srb7slak «writes the ungated reached record (`DioxusDocument::record_reached`, `reached_handlers`, `ReachedHandler`) only where it was switched on, inert elsewhere at one `has_context` lookup per dispatch»
replace-with: whose dispatch the bridge's `handle_event` sets around the vdom's only where a registry stands, and that `handle_event` writes the ungated reached record (`DioxusDocument::record_reached` switching it on, `reached_handlers` → `ReachedHandler { id, event }` in first-reached order, deduplicated by (id, event), empty when never switched on) only where it was switched on, inert elsewhere at one `has_context` lookup per dispatch;
@leaf_edit n=2 leaf=CLAUDE.md line=37 class=omission
leaf-text: its `Replay` and `Change` holding no reading (ratified by the founder, 2026-10-19);
source: fct-elnqiowc «`handler_coverage` replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised»
replace-with: its `Replay` and `Change` holding no reading (ratified by the founder, 2026-10-19); `handler_coverage(checks, boot)` (module `coverage`, re-exported with `Coverage { unexercised, checks, held }` and `HandlerRef { id, owner, event }`) replays kept checks, each on a fresh session through the crate-private `Check::replay_each`, a check of another app running nothing and contributing nothing, reads the UI graph at boot and after each step (`mounted` left out) and the reached record after the replay, and lists the handlers no check exercised, written by hand by `Coverage::to_json` as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`;
@leaf_edit n=3 leaf=CLAUDE.md line=37 class=omission
leaf-text: nothing the caller typed written back. `serve_mcp` is the MCP server
source: fct-movjyb23 «The command line's `coverage` role: its reader takes the first word `coverage` after `mcp` and before the verb table, in one shape, `coverage <app> <check>...`»
replace-with: nothing the caller typed written back; its `coverage` role, `coverage <app> <check>...` (no `--session`, no `--record`, any other shape usage), reads each named kept check bounded (`MAX_CHECK_BYTES`) and through `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` (status `1`) holding nothing of the path or the file, boots the app once per check through `handler_coverage` and prints the coverage line on stdout (status `0`), a failed boot the session error (status `3`) — a ratified boundary widening, no new socket message, MCP tool, verb, cause, span field or dependency. `serve_mcp` is the MCP server
@leaf_edit n=4 leaf=CLAUDE.md line=38 class=omission
leaf-text: is an MCP server over its standard streams, booting the same six through the same boot
source: fct-qkp2micb «in its `coverage` role, `coverage <app> <check>...`, the handlers its named kept checks left unexercised, the same boot closure called once per check»
replace-with: is an MCP server over its standard streams, booting the same six through the same boot, and in its `coverage` role `escher-session coverage <app> <check>...` prints the handlers its named kept checks left unexercised, the same boot closure called once per check
@leaf_edit n=5 leaf=CLAUDE.md line=44 class=omission
leaf-text: (`100644`, public) the one exception to owner-only
source: fct-cz244xre «the answer line carries ids, owner ids, event names and the checks and held counts only — a boundary widening, ratified»
replace-with: (`100644`, public) the one exception to owner-only — and the `escher-session coverage <app> <check>...` role reads kept-check files its caller names on argv as untrusted input, bounded and strictly through `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` holding nothing of the path, its stdout answer a coverage line of ids, owner ids, event names and two counts (a ratified boundary widening)
@leaf_edit n=6 leaf=CLAUDE.md line=102 class=omission
leaf-text: one line of JSON on stdout, exit `0` · `1` · `2` · `3`
source: fct-movjyb23 «accepted, it prints the coverage line on stdout, status `0`, and a boot that fails is the session error, status `3`»
replace-with: one line of JSON on stdout, exit `0` · `1` · `2` · `3`; `target/debug/escher-session coverage {app} {check}...` (no `--session`, no `--record`) replays each named kept check on a fresh session and prints the coverage line `{"unexercised":[…],"checks":n,"held":m}`, the handlers no check exercised
@leaf_edit n=7 leaf=CLAUDE.md line=85 class=omission
leaf-text: `packages/escher-driver/src/{session,host,client,wire,error,recording,replay}.rs`
source: fct-h37xmzho «The coverage reader: the public `handler_coverage(checks: &[Check], boot: impl FnMut() -> Result<Session, SessionError>) -> Result<Coverage, SessionError>` boots a fresh session for each check»
replace-with: `packages/escher-driver/src/{session,host,client,wire,error,recording,replay}.rs` · the coverage reader `packages/escher-driver/src/coverage.rs` (`handler_coverage`; check `tests/blitz-tests/tests/stand_handler_coverage.rs`) and the reached record `packages/dioxus-native-dom/src/reached.rs`
@leaf_edit n=8 leaf=CLAUDE.md line=112 class=omission
leaf-text: proven on the stand in both layout modes by `stand_graph`).
source: fct-ldnjdynp «`stand_handler_coverage` (8) holds `escher_driver::handler_coverage` over the six served tasks in both layout modes»
replace-with: proven on the stand in both layout modes by `stand_graph`), and handler coverage (built — `handler_coverage` replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised, read through dioxus-native-dom's reached record, switched on for it alone; the `escher-session coverage <app> <check>...` role prints that coverage line — ids, owner ids, event names and two counts — on stdout, a ratified boundary widening; proven on the six served tasks in both layout modes by `stand_handler_coverage`).
