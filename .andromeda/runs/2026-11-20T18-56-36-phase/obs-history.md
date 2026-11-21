@history_item n=1 marker=amd-e5lmdcaj
**amd-e5lmdcaj** — (2026-10-07-command-and-refusal-schema) made `Cause::name` the domain of the span's `cause` field and laid down the rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. Any new `press` modifier or `graph` `part` argument must stay out of the span, its refusal fielded only as one of the closed cause words.

@history_item n=2 marker=amd-3iczuidb
**amd-3iczuidb** — (2026-10-07-act-by-id) restated the rule for the span's owner: field a cause by `Cause::name`, never a `Command`/`Call`/`ArgValue`/`Outcome` with `Debug`. An `Outcome` fielded with `Debug` would print an id, a name or a control's value. This binds a Ctrl-press outcome and a graph part outcome.

@history_item n=3 marker=amd-cschmkve
**amd-cschmkve** — (2026-10-07-driver-command-spans) built the `command` span: eight fields recorded only where they apply, and never any argument, Command, Call, ArgValue, Outcome, Refusal or Fault. These are the rows `contract/escher-driver-command-span/*` and `decision/driver-spans-carry-no-content`. A modifier or a part number must not become a ninth field.

@history_item n=4 marker=amd-u2nw2mep
**amd-u2nw2mep** — (2026-10-09-founder-rulings) the founder ratified the closed span line. Only the sizes of a diff (`added`/`removed`/`changed`) reach a log, never its content. This is the basis for `decision/closed-span-lines` and the `command-span/line` contract that new verbs' lines follow.

@history_item n=5 marker=amd-dnjkg5io
**amd-dnjkg5io** — (2026-10-10-driver-cli) what the command line prints is an answer under the ratified crossing, never a log line. It made the standing rule that no new surface adds a span or log field holding what a call supplied or a screen reads. A modifier or part argument on the command line, the socket or MCP inherits this rule.

@history_item n=6 marker=amd-3jlpr7el
**amd-3jlpr7el** — (2026-10-10-mcp-surface) ratified the `tool_call` span with exactly two closed fields, `tool` and `ending`, `command` as its child. A new argument on an existing MCP tool adds no field. This is the basis for `decision/tool-call-span-two-closed-fields` and the `tool-call-span/*` rows.

@history_item n=7 marker=amd-hk2vxiq4
**amd-hk2vxiq4** — (2026-10-12-self-description) `help` joined the closed sets, and an argument to it is refused `malformed`. The self-description and remedies are an answer on stdout, never a log. This applies when `press`'s help (its stated no-Ctrl limit) and `graph`'s description change.

@history_item n=8 marker=amd-xzye3ehn
**amd-xzye3ehn** — (2026-10-14-quality-gates) `busy` is driven by `stand_act_spans` running `press tab`, whose line carries `settled=false busy="render"`. A `press` with a new modifier is an acting verb whose line must keep to the acting fields only.

@history_item n=9 marker=amd-wggssyt7
**amd-wggssyt7** — (2026-10-18-one-virtual-clock) set the cause set at eight names, the executor returning six, and the span keeps eight fields. A refusal for a bad modifier or an out-of-range graph part must reuse a cause of the eight, `malformed` for a bad argument, not add one.

@history_item n=10 marker=amd-oqhtag4e
**amd-oqhtag4e** — (2026-10-07-refusal-detection) the session's record of ids (at most 4096, no reader outside the crate, no `Debug`) is never printed, logged or fielded. This is `decision/session-id-record-unprinted`, which a graph part read touching that record must keep.

@history_item n=11 marker=amd-rbvxics4
**amd-rbvxics4** — (2026-10-19-session-recording) recording opens no span and logs nothing, and a kept check is call content at rest in a 0700 directory. This is the ground of the `kept-check/*` and `recording-dir-private` rows. A new modifier or `part` argument is kept in a check's arguments, never in a log.

@history_item n=12 marker=amd-6x37n7n2
**amd-6x37n7n2** — (2026-10-19-recorded-check-replay) each replayed step writes its own `command` span through `Session::run`, and the replay adds none. The recorder keeps the answer whole as `answer`. The committed Counter checks are public. A kept `graph --part` or Ctrl-`press` step replays under the same span, and committed checks must hold no private data.

@history_item n=13 marker=amd-vrhzfubf
**amd-vrhzfubf** — (2026-10-19-visual-lint, gated part) the founder ruled the precedent «Без поля» (no new span field) for a new verb. An accepted `lint` records `verb` alone. A ruled verb or argument extension adds no span field.

@history_item n=14 marker=amd-4q2wobhh
**amd-4q2wobhh** — (2026-10-20-large-screen-in-parts) a part read (`Outcome::Part`) records `verb` alone. A part's number and the part count are never fielded. A refused `part` outside 1 to 1000 is `malformed`. `host_log`/`mcp_log` each drive one part read at `trace` with 0 needles. This is the direct template for `graph --part N` (rows `command-span/snapshot-part`, `decision/part-answer-never-logged`).

@history_item n=15 marker=amd-a6pxq634
**amd-a6pxq634** — (2026-10-24-stand-requirement-sweep) `stand_act_spans` counts calls per mode, part reads of `many_items` among them, with ids and buttons' text as needles. The client's stderr is read at `trace`. A graph part read added there moves the call count and needs its needles read.

@history_item n=16 marker=amd-zogtm42c
**amd-zogtm42c** — (2026-10-25-cold-agent-bar) measured the paged part read through the host's own sink: `host_log` `serve cells` with `snapshot --part 2` at `trace`, 0 needles under both builds, recording `verb` alone. This precedent applies to measuring a hosted `graph --part` on Cells, whose graph is the 293,869-byte case.

@history_item n=17 marker=amd-6reehbne
**amd-6reehbne** — (2026-10-25-cold-agent-bar) introduced the verdict's `part_reads` (paged snapshot reads) and `capability-unused` for an unused part read. These are the rows `cold-agent-verdict/part-reads` and `capability-unused`. A graph part read must be placed deliberately inside or outside that count.

@history_item n=18 marker=amd-2rlbtdpf
**amd-2rlbtdpf** — (2026-10-31-engine-debts) a `multiple` select's chosen set leaves only as form entries, the snapshot's first-chosen `value` and the options' tree nodes, never a log line or field. These are the founder's F6/F7/F10 rows `decision/select-chosen-set-never-logged` and `multiple-select-form-entries/chosen-option`. A Ctrl-`press` building a non-adjacent set must keep this.

@history_item n=19 marker=amd-il5krtas
**amd-il5krtas** — (2026-10-31-engine-debts) the `multiple` select and its readers (`select_values`, `is_multiple_select`) add no log site. This is the chunk that kept `press` without Ctrl as residual `res-hfvjj77l`. A Ctrl fix touches these surfaces and must keep the no-site reading.

@history_item n=20 marker=amd-nsxi7wwt
**amd-nsxi7wwt** — (2026-10-20-select-by-id) the select checks `select_interaction` and `stand_act_select` are recorded silent. The select model adds no log site or span field. A multiple-key check extended for Ctrl joins this silence census.

@history_item n=21 marker=amd-2iwce5bg
**amd-2iwce5bg** — (2026-10-31-select-popup) `press`'s and `click`'s help and `covered`'s meaning are the program's own `'static` text, adding no log site or span field, and `command` keeps its eight fields. Rewriting `press`'s help (its Ctrl limit) falls under the 2026-10-12 fixed-text rule.

@history_item n=22 marker=amd-bcuh7k4j
**amd-bcuh7k4j** — (2026-10-25-capability-statement) `docs/capabilities.md` restates the driver's spans and the scrub's limits. Its pin reads none of them, so a change to the span set is made there by hand. This is `decision/capability-statement-updated-by-hand`. The chunk edits `docs/capabilities.md:279-281` (the stopped-dispatch limit) and possibly the press and graph lines.

@history_item n=23 marker=amd-wv4t22sd
**amd-wv4t22sd** — (2026-11-07-ui-graph) `graph` runs under the one `command` span. `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}`, never logged or fielded, and kept whole in a recorded check. A paged graph answer must keep this.

@history_item n=24 marker=amd-jsqwtiq2
**amd-jsqwtiq2** — (2026-11-07-ui-graph) a graph's line is `span="command" verb="graph"` (with `cause` when refused; an argument is refused `malformed`, since `graph` takes none today), holding no node, state, id or name. `host_log`/`mcp_log` use `graph_needles`. Adding `--part` turns the today-refused argument into an accepted one, and the line must still hold no part number (row `command-span/graph`).

@history_item n=25 marker=amd-tlxklvv6
**amd-tlxklvv6** — (2026-11-07-ui-graph) the graph answer's carried set (ids, component names, state and handler event names, closed flag words, booleans, never a value) is a boundary widening, ratified later by amd-spdvdmvv. A graph part must carry that set alone, inside the ratified `graph --part` crossing.

@history_item n=26 marker=amd-spdvdmvv
**amd-spdvdmvv** — (2026-11-15 wrap) ratified the graph answer's carried set by the founder's own word, replacing the delegate confirm. This is the standing basis of `contract/graph-answer/content`.

@history_item n=27 marker=amd-htjat4nf
**amd-htjat4nf** — (2026-11-07-ui-graph) dioxus-native-dom's `graph` and `signal_registry` modules and the bridge dispatch add no log site, and a page with no registered state reaches no registry. This is `decision/no-state-page-skips-registry`, which a reached-record change in the dispatch path must keep.

@history_item n=28 marker=amd-u6k3oci4
**amd-u6k3oci4** — (2026-11-08-handler-coverage) the `escher-session coverage` role is a further reach of the same sink install. `handler_coverage` and the reached record install no subscriber, read no env or clock and add no span or field. The coverage line is a stdout answer. Ratified by amd-qs55m2hv.

@history_item n=29 marker=amd-ob2zaxca
**amd-ob2zaxca** — (2026-11-08-handler-coverage) the `coverage` module and dioxus-native-dom's `reached` module (written in `DioxusEventHandler::handle_event`) hold no log site. The record holds stable ids and event names only, no `NodeId`, and its `Debug` prints its count alone. Reading a `stopPropagation` into the record must keep the record ids-and-events-only (row `decision/reached-record-ids-and-events-only`).

@history_item n=30 marker=amd-q624ly3m
**amd-q624ly3m** — (2026-11-08-handler-coverage) the `command` span's reach gains the coverage role, each replayed step writing its own `command` line, and handler coverage adds no span (rows `decision/coverage-line-never-logged`). Ratified by amd-fznt7mdc.

@history_item n=31 marker=amd-flbwaun3
**amd-flbwaun3** — (2026-11-08-handler-coverage) `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` reads 0 id, name, coverage, sentinel, path and recorded needles with `coverage_needles`. This is the guard a stopped-dispatch coverage change re-runs.

@history_item n=32 marker=amd-coxkueit
**amd-coxkueit** — (2026-11-08-handler-coverage) the coverage line carries ids, owner ids, event names and two counts only, a ratified widening (ratified by amd-dhxza76a). A stop-aware record must change which handlers read exercised, not the line's shape.

@history_item n=33 marker=amd-iir7mv4r
**amd-iir7mv4r** — (2026-11-15 wrap) ratified the coverage role's endings (shape, bounded read, `malformed` refusals holding nothing of the path), which are the rows `escher-session-coverage/*` this chunk rests on.

@history_item n=34 marker=amd-rszctrp5
**amd-rszctrp5** — (2026-11-08-handler-coverage) trap: a new `host_log` test inserted ahead moved the paged-part test's ordinal (4 to 5) in four sections. A new host-log test for `graph --part` or the coverage stop shifts ordinals the same way.

@history_item n=35 marker=amd-csae2x3f
**amd-csae2x3f** — (2026-11-14-auto-exploration) the coverage role's `malformed` list gained "a format other than 1" (`contract/escher-session-coverage/malformed`). If a stop-aware reached record or new kept-check arguments change the check format, this refusal row is affected.

@history_item n=36 marker=amd-ypcavwq4
**amd-ypcavwq4** — (2026-11-17-layout-on-the-carriers) `command-span/verb` is fifteen words, and a per-verb line contract (`command-span/layout`) was added for a new verb. This is the precedent for any per-argument line contract for `graph --part`. No verb is added by this chunk, so the count stays.

@history_item n=37 marker=amd-vstyxann
**amd-vstyxann** — (2026-11-17-layout-on-the-carriers) `contract/command-span/settled` lists the verb-alone reads (snapshot, screenshot, lint, graph, actions, layout, a part read). `tool-call-span/tool` is one of fifteen. A graph part read must be named among the verb-alone reads.

@history_item n=38 marker=amd-wjfn5pka
**amd-wjfn5pka** — (2026-11-17-layout-on-the-carriers) a read reached by its verb runs inside that call's `command` span and opens none of its own. This is the pattern for a paged graph read (`decision/hosted-call-through-session-run`).

@history_item n=39 marker=amd-etvpmur5
**amd-etvpmur5** — (2026-11-17-layout-on-the-carriers) `decision/sink-host-read-at-trace` grew its needle set (layout-answer keys). A graph part read through the host or the mcp role may need its own needles named in that row.

@history_item n=40 marker=amd-6r5naojb
**amd-6r5naojb** — (2026-11-18-cold-agent-bar) reversed the uncounted-verbs decision: `graph` is a counted verb. The pipe reads the server log by the `tool_call` span's tool and ending and the `command` span's cause word only. `refused_by_cause` counts the eight causes. A graph part read's lines are counted from these span lines.

@history_item n=41 marker=amd-bwz3xm3u
**amd-bwz3xm3u** — (2026-11-18-cold-agent-bar) added `graph_calls` and `refused_by_cause` to the verdict (`contract/cold-agent-verdict/graph-calls`, `refused-by-cause`). `graph --part` calls count as accepted graph `tool_call` lines unless deliberately split.

@history_item n=42 marker=amd-h766vjv5
**amd-h766vjv5** — (2026-11-18-cold-agent-bar) `capability-unused` reads a counted verb, a part read or a list opening going unused, later extended by amd-ie77k2lt and amd-shqfkh5t with `caused_by_kind`. A new part-read form is a decision on whether the requirement set grows.

@history_item n=43 marker=amd-g62h26kx
**amd-g62h26kx** — (2026-11-16-error-text-in-the-answer) reworded `decision/driver-library-installs-no-subscriber` (the scoped capture and its chaining panic hook) and `decision/answers-echo-nothing-typed`, which now has a caused-item exception. An acting `press` with Ctrl runs under that capture and echoes nothing typed.

@history_item n=44 marker=amd-5rrv4k4t
**amd-5rrv4k4t** — (2026-11-16-error-field-in-the-answer) set `decision/answers-echo-nothing-typed`'s current exception: a caused item's `error` field shows as written, cut at the text bound. Ratified by amd-mxcnbjck. A refusal for a new argument must still echo nothing the caller typed.

@history_item n=45 marker=amd-z4s54auy
**amd-z4s54auy** — (2026-11-16-error-text-in-the-answer) set the current `decision/driver-emits-no-event` rule: the scoped capture keeps the step's records and panics in process and prints nothing, and the driver's telemetry is its spans alone. A driver fix adds no event.

@history_item n=46 marker=amd-vm35icer
**amd-vm35icer** — (2026-10-07-driver-session) trap: a zero read from a host that installs no sink says nothing about one that does. Needle readings for the new arguments must be taken on `escher-session` (`host_log`/`mcp_log`), not only in process.
