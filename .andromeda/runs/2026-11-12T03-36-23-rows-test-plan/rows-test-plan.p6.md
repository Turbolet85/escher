@row_none n=1 rec=1 type=5
@row_add n=1 row=decision/per-crate-coverage-floors area=ci-integration rec=2
name: Per-crate coverage floors
rule: The coverage leg fails unless escher-driver holds at least 97, dioxus-native-dom at least 83 and blitz-test-harness at least 90 percent line coverage, keyboard_compat.rs excluded from the measure.
guard: coverage_floor.py chained in the coverage leg
@row_add n=2 row=contract/coverage-floor-exit-codes/0 area=ci-integration rec=2
boundary: coverage floor script exit codes
item: 0
promise: Every crate named is at or above its floor; the verdict reads `coverage floor: N of N crates at or above their floors`.
@row_add n=3 row=contract/coverage-floor-exit-codes/1 area=ci-integration rec=2
boundary: coverage floor script exit codes
item: 1
promise: Some crates are under their floors; the verdict reads `coverage floor: K of N crates under their floors`.
@row_add n=4 row=contract/coverage-floor-exit-codes/2 area=ci-integration rec=2
boundary: coverage floor script exit codes
item: 2
promise: A crate named has no coverage record, or an argument is malformed.
@row_add n=5 row=contract/cli-exit-codes/1 area=e2e-test-strategy rec=3
boundary: escher-session CLI exit codes
item: 1
promise: A call a target cannot take is refused with its cause.
@row_add n=6 row=contract/cli-exit-codes/2 area=e2e-test-strategy rec=3
boundary: escher-session CLI exit codes
item: 2
promise: A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
@row_add n=7 row=contract/cli-exit-codes/3 area=e2e-test-strategy rec=3
boundary: escher-session CLI exit codes
item: 3
promise: A session error: a command with no session, a state directory too long, or a group-open recording directory.
@row_add n=8 row=contract/cli-help/output area=e2e-test-strategy rec=3
boundary: escher-session CLI help
item: help
promise: `help` answers status 0 and one line equal to `describe()`, reaching no session.
@row_add n=9 row=contract/cli-refusal/remedy area=e2e-test-strategy rec=3
boundary: escher-session CLI refusals
item: remedy
promise: Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
@row_add n=10 row=contract/cli-state-dir/max-length area=e2e-test-strategy rec=3
boundary: escher-session CLI state directory
item: length bound
promise: A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.
@row_add n=11 row=contract/cli-snapshot/budget area=e2e-test-strategy rec=3
boundary: escher-session CLI snapshot
item: size budget
promise: A snapshot answers the in-process text as one line inside its 10,000-byte budget.
@row_add n=12 row=contract/cli-snapshot/part area=e2e-test-strategy rec=3
boundary: escher-session CLI snapshot
item: --part
promise: A part read by number answers one uncoloured JSON line, status 0; a part of 0, 1001 or a non-number exits 1 with the `malformed` refusal line, nothing typed written back.
@row_add n=13 row=contract/cli-schema/refused-call area=e2e-test-strategy rec=3
boundary: escher-session CLI schema
item: refused call
promise: A call the schema refuses reaches no session.
@row_add n=14 row=contract/cli-stdout/closed area=e2e-test-strategy rec=3
boundary: escher-session CLI stdout
item: closed stdout
promise: A run with its stdout closed before it starts keeps its ending's status, prints no panic and writes nothing typed back.
@row_add n=15 row=contract/cli-record/permissions area=e2e-test-strategy rec=3
boundary: escher-session CLI recording
item: --record
promise: `start <task> --record <dir>` keeps one `0600` check in a `0700` directory; a group-open recording directory is refused, status 3, with nothing spawned.
@row_add n=16 row=decision/no-screenshot-in-recorded-check-scripts area=e2e-test-strategy rec=3
name: No screenshot step in committed recorded checks
rule: The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems.
why: a shot's PNG bytes differ by OS
guard: the_committed_checks_are_what_the_binary_records
@row_add n=17 row=contract/cli-coverage-role/stdout area=e2e-test-strategy rec=4
boundary: escher-session CLI coverage role
item: coverage line
promise: Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
@row_none n=2 rec=5 type=5
@row_add n=18 row=contract/cli-actions/output area=e2e-test-strategy rec=6
boundary: escher-session CLI actions verb
item: output
promise: `actions` against a started host answers one JSON line of the actionable nodes.
@row_add n=19 row=contract/cli-actions/argument area=e2e-test-strategy rec=6
boundary: escher-session CLI actions verb
item: argument
promise: An argument to `actions` is refused, status 1.
@row_add n=20 row=contract/mcp-tools-list/order area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: tools/list
promise: `tools/list` offers the fourteen verbs in order, `graph` after `lint` and `actions` after `graph`.
@row_add n=21 row=contract/mcp-tool-result/screenshot area=e2e-test-strategy rec=7
boundary: escher-session MCP tool results
item: screenshot
promise: A `screenshot` tool call answers an `image/png` image block then a text block, `isError` false.
@row_add n=22 row=contract/mcp-tool-result/refused area=e2e-test-strategy rec=7
boundary: escher-session MCP tool results
item: refused call
promise: A refused tool call answers a single text block, `isError` true.
@row_add n=23 row=contract/mcp-protocol/revision area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: protocol revision
promise: The role speaks the newest MCP protocol revision.
@row_add n=24 row=contract/mcp-protocol/errors area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: protocol errors
promise: A protocol error runs nothing.
@row_add n=25 row=contract/mcp-help/parity area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: help
promise: `help` is answered before any `start`, the same bytes through MCP and the command line.
@row_add n=26 row=contract/mcp-snapshot/parity area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: snapshot
promise: A fresh Counter's snapshot reads the same through MCP and the command line.
@row_add n=27 row=contract/mcp-record/parity area=e2e-test-strategy rec=7
boundary: escher-session MCP role
item: --record
promise: With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
@row_add n=28 row=contract/mcp-actions/argument area=e2e-test-strategy rec=8
boundary: escher-session MCP role
item: actions
promise: The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
@row_add n=29 row=contract/command-span/actions area=e2e-test-strategy rec=8
boundary: driver command span
item: actions
promise: An `actions` tool call parents one `command` span that records `verb` alone.
@row_add n=30 row=decision/buffered-render-unborrowed-document area=e2e-test-strategy rec=9,11
name: Buffered render with the document unborrowed
rule: The Dioxus bridge renders into a mutation buffer with the document unborrowed and replays it afterwards, so a task polled during render, such as a MountedData call from a key handler's task, reaches the document without a borrow panic.
why: upstream's buffered render ported onto the Dioxus 0.7 bridge, its Dioxus 0.8 move not taken
guard: mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental in dioxus_runtime_probe
@row_none n=3 rec=10 type=4
relied: The buffered render's key-handler pins discriminate on Dioxus 0.7 because a task an event handler spawns while dirtying a scope is first polled inside `render_immediate`.
@row_none n=4 rec=12 type=4
relied: The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own.
@row_none n=5 rec=13 type=7
@row_add n=31 row=absence/windowed-end-to-end-tests area=e2e-test-strategy rec=14
capability: An automated test that launches a window
@row_add n=32 row=decision/layout-tests-in-both-layout-modes area=integration-test-strategy rec=15
name: Layout assertions in both layout modes
rule: Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do.
guard: incremental_oracle
@row_add n=33 row=decision/file-urls-only-for-file-documents area=integration-test-strategy rec=16
name: file: URLs loadable only from a file: document
rule: A `file:` URL is loaded by `fetch()`, a module `import` or `<script src>` only for a `file:` document, and refused with a `FetchError` otherwise.
guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls
@row_add n=34 row=decision/change-event-from-engine-only area=integration-test-strategy rec=17
name: change events come from the engine alone
rule: A page's script hears one `change` per change, fired by the engine; the script runtime synthesizes none.
guard: radio_click_fires_change_only_where_it_was_not_checked
@row_none n=6 rec=18 type=5
@row_add n=35 row=contract/session-act/settle area=integration-test-strategy rec=19
boundary: escher_driver Session
item: act
promise: A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
@row_add n=36 row=decision/load-wait-bound area=integration-test-strategy rec=19
name: Acting steps wait out loads in flight
rule: An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`.
guard: stand_load_wait
@row_add n=37 row=contract/session-run/diff area=integration-test-strategy rec=19
boundary: escher_driver Session
item: run
promise: A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
@row_add n=38 row=contract/session-run/screenshot area=integration-test-strategy rec=19
boundary: escher_driver Session
item: screenshot
promise: A `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster, and changes nothing.
@row_add n=39 row=decision/advance-accepted-on-every-session area=integration-test-strategy rec=19
name: advance accepted on every session
rule: The `advance` verb is accepted on every session, never refused for the session it addresses.
why: ratified
@row_add n=40 row=decision/refusal-leaves-instance-unchanged area=integration-test-strategy rec=19
name: Refusals change nothing and echo no input
rule: A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
@row_add n=41 row=contract/refusal-cause/unknown-verb area=integration-test-strategy rec=19
boundary: driver refusal causes
item: unknown-verb
promise: A call naming no known verb is refused `unknown-verb`, the instance unchanged.
@row_add n=42 row=contract/refusal-cause/malformed area=integration-test-strategy rec=19
boundary: driver refusal causes
item: malformed
promise: A malformed call is refused `malformed`, the instance unchanged.
@row_add n=43 row=contract/refusal-cause/not-found area=integration-test-strategy rec=19
boundary: driver refusal causes
item: not-found
promise: A call naming an id not on the screen is refused `not-found`, the instance unchanged.
@row_add n=44 row=contract/refusal-cause/stale area=integration-test-strategy rec=19
boundary: driver refusal causes
item: stale
promise: A call naming a stale id is refused `stale`, the instance unchanged.
@row_add n=45 row=contract/refusal-cause/disabled area=integration-test-strategy rec=19
boundary: driver refusal causes
item: disabled
promise: A call on a disabled target is refused `disabled`, the instance unchanged.
@row_add n=46 row=contract/refusal-cause/off-screen area=integration-test-strategy rec=19
boundary: driver refusal causes
item: off-screen
promise: A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
@row_add n=47 row=contract/refusal-cause/covered area=integration-test-strategy rec=19
boundary: driver refusal causes
item: covered
promise: A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
@row_add n=48 row=contract/refusal-cause/fonts-unavailable area=integration-test-strategy rec=19
boundary: driver refusal causes
item: fonts-unavailable
promise: A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
@row_add n=49 row=contract/session-run/scroll area=integration-test-strategy rec=19
boundary: escher_driver Session
item: scroll
promise: A `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view.
@row_add n=50 row=decision/typed-password-never-returned area=integration-test-strategy rec=19
name: Typed passwords never returned
rule: A password typed through the driver occurs nowhere in what the call returns, reads `MASKED_VALUE`, and never leaves as pixels in a screenshot.
guard: stand_act_screenshot, stand_act_filled
@row_add n=51 row=decision/no-sleep-or-wall-clock-in-checks area=integration-test-strategy rec=19
name: No sleep or wall-clock read in checks
rule: Session checks take no sleep and read no wall clock; time moves by `advance` alone.
except: stand_virtual_clock's one 800 ms sleep and stand_load_wait's clock reads and 200 ms stand-in sleep, both checks whose subject is wall time
@row_add n=52 row=decision/driver-log-fixed-words-only area=integration-test-strategy rec=19
name: Driver log lines hold fixed words and counts only
rule: At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line.
guard: stand_act_spans
@row_add n=53 row=contract/session-run/type area=integration-test-strategy rec=19
boundary: escher_driver Session
item: type
promise: A `type` replaces what a filled control holds and an empty text clears it; a masked control's written form holds the mask, never the typed text.
@row_add n=54 row=contract/session-run/changed-node-states area=integration-test-strategy rec=19
boundary: escher_driver Session
item: acting call result
promise: An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
@row_add n=55 row=contract/session-run/type-on-select area=integration-test-strategy rec=19
boundary: escher_driver Session
item: type on a select
promise: A `type` on a single-choice select chooses the first option whose label starts with the text, its diff naming the select with `value` and the options whose `selected` moved.
@row_none n=7 rec=20 type=4
relied: A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`.
@row_add n=56 row=decision/reserved-graph-id-form area=integration-test-strategy rec=21
name: Reserved owner-key id form
rule: No served id holds the reserved `{owner}///{key}` form.
guard: stand_graph
@row_add n=57 row=contract/session-run/graph area=integration-test-strategy rec=21
boundary: escher_driver Session
item: graph
promise: The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
@row_add n=58 row=decision/handler-coverage-holds-no-typed-text area=integration-test-strategy rec=22
name: Handler coverage line holds no typed text
rule: Text typed during a replayed check never reaches the handler coverage line.
guard: stand_handler_coverage
@row_add n=59 row=contract/handler-coverage/refused-step area=integration-test-strategy rec=22
boundary: handler coverage
item: refused step
promise: A refused step exercises no handler; a keyboard activation exercises the element's click handler.
@row_add n=60 row=contract/handler-coverage/other-app-check area=integration-test-strategy rec=22
boundary: handler coverage
item: check of another app
promise: A check recorded on another app contributes nothing to the coverage.
@row_add n=61 row=decision/handler-coverage-reads-every-replayed-screen area=integration-test-strategy rec=23
name: Handler coverage reads every replayed screen
rule: Handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step.
state: provisional
@row_add n=62 row=decision/handler-coverage-canvas-uncounted area=integration-test-strategy rec=24
name: Canvas handlers not counted by handler coverage
rule: Handler coverage does not count a handler inside a canvas.
@row_add n=63 row=decision/handler-coverage-unreached-screens area=integration-test-strategy rec=24
name: Handlers on unreached screens not listed
rule: Handler coverage does not list a handler on a screen no replay reached.
why: by construction
@row_add n=64 row=decision/handler-coverage-bubbling-counts-ancestors area=integration-test-strategy rec=24
name: Bubbling dispatch counts every ancestor listener
rule: A bubbling dispatch counts every ancestor listener it passed as exercised, `stopPropagation` unread.
