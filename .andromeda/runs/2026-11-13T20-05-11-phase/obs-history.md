@history_item n=1 marker=amd-amti4mhy
**amd-amti4mhy** — (2026-10-26-codebase-health) the direct precedent: a chunk that brought functions under the cognitive-complexity ceiling of 15 with private items only had its extracted helpers recorded `observed absent` in log-coverage, with the silence census read over every file it changed (exit 1, last line 0) and escher-driver's 2 `info_span!` sites re-counted. This chunk's helpers in `pointer.rs`, `execute.rs` and `graph.rs` take the same census and the same row, and the row's findings stay in log-coverage, not the body's §3.

@history_item n=2 marker=amd-cschmkve
**amd-cschmkve** — (2026-10-07-driver-command-spans) set the `command` span as built: `info_span!` at target `escher_driver`, entered before `validate`, eight fields each recorded only where it applies, and a never-fielded list (any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`, any argument, the screen text). The `execute` restructure must keep the span's entry point, the per-verb recording sites and their order (the rows `contract/command-span/*` and `contract/escher-driver-command-span/*`).

@history_item n=3 marker=amd-e5lmdcaj
**amd-e5lmdcaj** — (2026-10-07-command-and-refusal-schema) the standing rule for the executor: the span's cause field takes a `Cause::name`. Never field a `Command`, `Call` or `ArgValue` with `Debug`, and never field an argument by its schema name. A helper split out of `execute` must not record anything other than the closed words (row decision/command-span-no-call-content, guard `stand_act_spans`).

@history_item n=4 marker=amd-3iczuidb
**amd-3iczuidb** — (2026-10-07-act-by-id) recorded the executor `execute` / `Session::run` as installing no subscriber, reading no env var or clock and printing nothing. It added `Outcome` to the `Debug`-deriving types nothing prints, logs or fields, and restated the rule for the span's owner. Moved helpers keep that silence (row decision/debug-types-never-logged).

@history_item n=5 marker=amd-oqhtag4e
**amd-oqhtag4e** — (2026-10-07-refusal-detection) all eight cause names are returned by code, most of them by the executor. The session's record of ids has no reader outside the crate, no `Debug` and is never printed or fielded (row decision/session-id-record-unprinted). The refusal order the restructure must not change is where these causes come from (rows `contract/refusal-cause/*`).

@history_item n=6 marker=amd-npbuors6
**amd-npbuors6** — (2026-10-07-settle-detection) the settle wait (`Harness::settle`) and the session step are silent and add no span of their own. The `command` span covers them. An `execute` helper that runs settle must not add a span, event or log line there.

@history_item n=7 marker=amd-xzye3ehn
**amd-xzye3ehn** — (2026-10-14-quality-gates) `busy` is driven on a real line: `stand_act_spans`' `press tab` re-key fixture reads `settled=false busy="render"` with no `passes` and no `cause`. This is the check that holds the acting-field recording branches `execute` keeps (rows contract/command-span/busy · settled · passes).

@history_item n=8 marker=amd-lo6ldhab
**amd-lo6ldhab** — (2026-10-20-load-wait) on a step that waited on a load, `passes` is the settle that went quiet. The driver's load wait (`settle_waiting`) runs inside the `command` span with no span of its own. The executor's load-wait branch must keep this (row contract/command-span/passes).

@history_item n=9 marker=amd-wggssyt7
**amd-wggssyt7** — (2026-10-18-one-virtual-clock) eight causes with `time-unavailable` retired. An accepted `advance`, accepted on every session, records `verb` and the acting fields with no cause, and its `tool_call` parents one `command` child. The `advance` arm of `execute` holds this (row decision/advance-accepted-every-session).

@history_item n=10 marker=amd-bbjl3fhw
**amd-bbjl3fhw** — (2026-10-18-engine-debts) the founder ratified the one-virtual-clock marks: the eight cause names and `advance` accepted on every session. These are behaviour the executor restructure must not move.

@history_item n=11 marker=amd-wivka2yb
**amd-wivka2yb** — (2026-10-12-headless-screenshot) `screenshot` joined the closed sets. `fonts-unavailable` is returned before anything is painted, and an empty crop is refused `off-screen`. A shot records `verb` alone, or `verb` and `cause` when refused, and never its image or size. These are executor arms and refusal-order facts the restructure must keep (rows contract/refusal-cause/fonts-unavailable · off-screen, contract/command-span/screenshot, decision/shot-image-leaves-only-in-answer).

@history_item n=12 marker=amd-vrhzfubf
**amd-vrhzfubf** — (2026-10-19-visual-lint) the `lint` verb runs inside the `command` span with no field. An accepted `lint` records `verb` alone and the findings are never fielded. `Outcome::Linted` is written by `to_json` alone (rows contract/command-span/lint, contract/command-answer/linted).

@history_item n=13 marker=amd-4q2wobhh
**amd-4q2wobhh** — (2026-10-20-large-screen-in-parts) a part read (`Outcome::Part`) records `verb` alone, and a `part` outside 1 to 1000 is refused `malformed`. The part number or count is never fielded. This is an executor arm (rows contract/command-span/snapshot-part, decision/part-answer-never-logged).

@history_item n=14 marker=amd-hk2vxiq4
**amd-hk2vxiq4** — (2026-10-12-self-description) a `help` handed to `Session::run` is refused `unknown-verb` with its table word recorded. Session-level words appear only beside `unknown-verb`. This is the verb-recording branch of the refusal path in `execute` (row contract/command-span/verb).

@history_item n=15 marker=amd-3jlpr7el
**amd-3jlpr7el** — (2026-10-10-mcp-surface) the `tool_call` span (two closed fields, `tool` and `ending`) was ratified. `command` is its child by nesting and the executor is unchanged. An `execute` restructure must keep the `command` span nesting under `tool_call` (rows contract/tool-call-span/*, contract/escher-driver-tool-call-span/*).

@history_item n=16 marker=amd-dnjkg5io
**amd-dnjkg5io** — (2026-10-10-driver-cli) an `Outcome` is written in one place, by `Outcome::to_json`, as a command's answer on stdout and never a log line. Hosted calls run through `Session::run` with no span of their own. Standing rule: no span or log field holding what a call supplied (rows decision/debug-types-never-logged, contract/escher-session-stdout/command-answer, decision/hosted-call-through-session-run).

@history_item n=17 marker=amd-ei5tfcg5
**amd-ei5tfcg5** — (2026-11-08-what-can-i-do-here) `Session::actions` validates each named call and then puts it through the shared `check_target`. The `actions` verb and the four id-taking verbs reach `check_target` and `target_of` inside the call's `command` span, which open no span of their own. The scope names `check_target` as part of the `execute` restructure, so the shared helper's span-less reach and the read-changes-nothing fact must hold (row decision/actions-read-changes-nothing).

@history_item n=18 marker=amd-k5v64vz2
**amd-k5v64vz2** — (2026-11-08-what-can-i-do-here) `actions` joined the `command` verbs (fourteen, ten instance-level) and the `tool_call` tools. An accepted `actions` records `verb` alone and parents exactly one `command` child. This is an executor arm (row contract/command-span/actions).

@history_item n=19 marker=amd-d3eh2inx
**amd-d3eh2inx** — (2026-11-08-what-can-i-do-here) `stand_act_spans` drives 38 calls per mode across ten verbs, and `host_log` / `mcp_log` read 0 needles at `trace` under both builds. These are the guards that hold the executor's span lines through a behaviour-preserving move.

@history_item n=20 marker=amd-quujnooh
**amd-quujnooh** — (2026-11-08-what-can-i-do-here) the actions answer's carried set and shape are a ratified boundary widening and are never logged or fielded. The `execute` move must not change them (rows contract/actions-answer/*).

@history_item n=21 marker=amd-wv4t22sd
**amd-wv4t22sd** — (2026-11-07-ui-graph) the `graph` verb runs under the same `command` span. `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}` and is never logged or fielded. `stand_graph` reads `session_common`. The `graph` restructure must keep the answer's content and shape (rows contract/command-answer/graphed, contract/graph-answer/shape, decision/graph-answer-never-logged).

@history_item n=22 marker=amd-jsqwtiq2
**amd-jsqwtiq2** — (2026-11-07-ui-graph) a graph's line is `span="command" verb="graph"` and holds no node, state, id or name. `host_log` and `mcp_log` read 0 graph needles (`graph_needles`) at `trace` under both builds. These guard the graph function's output never reaching a log (row contract/command-span/graph).

@history_item n=23 marker=amd-htjat4nf
**amd-htjat4nf** — (2026-11-07-ui-graph) dioxus-native-dom's modules `graph` (gated `accessibility`) and `signal_registry` hold no print, log site, subscriber or env read, with the silence census at 0. A page with no registered state reaches no registry. Helpers extracted inside `graph.rs` re-enter this census (row decision/no-state-page-skips-registry).

@history_item n=24 marker=amd-tlxklvv6
**amd-tlxklvv6** — (2026-11-07-ui-graph) the graph answer's carried set is a ratified boundary widening: ids, component names, registered state and handler event names, the closed flag words `undeclared` / `unconfirmed` and booleans, never a value. A typed sentinel occurs 0 times. The restructure of `graph` must leave this set exactly as is (row contract/graph-answer/content).

@history_item n=25 marker=amd-b2prlxvh
**amd-b2prlxvh** — (2026-11-07-ui-graph) `SignalRegistry` holds no state value, its `Debug` prints counts alone, and it holds no log site. `graph` reads it, and a helper must not add a print or a `Debug` of it (row decision/state-registry-holds-no-values).

@history_item n=26 marker=amd-c6l4ytcq
**amd-c6l4ytcq** — (2026-11-07-ui-graph) the snapshot's `clickable` reading is a closed bool that carries no page text. It is a click-adjacent reading that the click and graph restructures must leave unchanged (rows contract/snapshot-node/clickable, contract/snapshot-text/clickable).

@history_item n=27 marker=amd-q624ly3m
**amd-q624ly3m** — (2026-11-08-handler-coverage) the coverage role replays each step of a kept check through `Session::run`, each writing its own `command` line. Handler coverage adds no span. This is PROVISIONAL (delegate overseer) for the founder's epoch-boundary batch. `stand_handler_coverage`, named in the scope as a guard for `graph`, rests on it (rows decision/coverage-line-never-logged, contract/coverage-role-stdout/coverage-line).

@history_item n=28 marker=amd-ob2zaxca
**amd-ob2zaxca** — (2026-11-08-handler-coverage) the `coverage` module and dioxus-native-dom's `reached` record hold no log site. The record holds stable ids and event names only, no `NodeId`, and its `Debug` prints its count alone. It is written in `DioxusEventHandler::handle_event` beside the `SignalRegistry` arm (row decision/reached-record-ids-and-events-only).

@history_item n=29 marker=amd-coxkueit
**amd-coxkueit** — (2026-11-08-handler-coverage) the coverage line's carried fields and shape are a widening that is PROVISIONAL (delegate overseer). Graph-derived ids feed it, so the `graph` restructure must not move them (rows contract/coverage-line/*).

@history_item n=30 marker=amd-u6k3oci4
**amd-u6k3oci4** — (2026-11-08-handler-coverage) the `escher-session` coverage role is a further reach of the same sink install. A `malformed` refusal holds nothing of the path. This is PROVISIONAL (delegate overseer) (rows contract/coverage-role-stdout/coverage-line, contract/escher-session-coverage/accepted).

@history_item n=31 marker=amd-flbwaun3
**amd-flbwaun3** — (2026-11-08-handler-coverage) `host_log`'s coverage run reads 0 needles at `trace` under both builds (`coverage_needles`, which are the CRUD screen's UI-graph ids). This guard is the guard named on the coverage-line-never-logged row.

@history_item n=32 marker=amd-il5krtas
**amd-il5krtas** — (2026-10-31-engine-debts) the engine changes inside `handle_click`'s arms recorded `no site added`: native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change`. The `handle_click` restructure must keep those arms free of log sites, prints, env reads and panic paths. Trap: the added-lines census's carve-out for `blitz-shell/src/event.rs`'s inline test module.

@history_item n=33 marker=amd-2rlbtdpf
**amd-2rlbtdpf** — (2026-10-31-engine-debts) these are the founder's forks F6, F7 and F10. The engine's `change` on a checkbox is always sent after `input`; on a radio it is sent only where the radio was not checked before the click. A page hears one `change`, and a `multiple` select's chosen set leaves only as form entries. These are `handle_click`'s checkbox, radio and select arm sequences, and the restructure must not change them (rows contract/page-events/*, contract/multiple-select-form-entries/chosen-option, contract/snapshot/multiple-select-value, decision/select-chosen-set-never-logged).

@history_item n=34 marker=amd-z52p4vmm
**amd-z52p4vmm** — (2026-10-31-engine-debts) `checkable_change_event`, which the scope names as a guard of the checkbox and radio arms, and its sibling checks are recorded silent in a §6 observed-absent row. A check added before a move gets a row of the same form.

@history_item n=35 marker=amd-t7lc5vnn
**amd-t7lc5vnn** — (2026-10-31-engine-debts) the engine's `focussed node` INFO line fires once per focus move under the workspace build and holds a node number. The focus fixup adds no log site. A button arm that focuses must keep that line's count and content (row decision/engine-tracing-by-invocation).

@history_item n=36 marker=amd-cmorfrzt
**amd-cmorfrzt** — (2026-10-31-select-popup) the `Focussed node` line's count through the sink on the workspace build reads 0 with no base reading, so it is neither moved nor unmoved. A before/after count of that line would be the first such count with a base.

@history_item n=37 marker=amd-2iwce5bg
**amd-2iwce5bg** — (2026-10-31-select-popup) the select list adds no log site or span. `is_select` is read by `execute.rs`'s `locate` alone, and the driver's `focus_select` and `close_select_list` open no span. Both `execute` and `handle_click`'s select arms touch these.

@history_item n=38 marker=amd-7g47dwps
**amd-7g47dwps** — (2026-10-31-select-popup) `select_popup`, `select_popup_painted` and `stand_act_popup`, which hold the select arms the scope moves, are silent. `host_log` and `mcp_log` read a list opened and chosen at `trace` with 0 needles. `SelectListRow.label` is never logged.

@history_item n=39 marker=amd-hv3i7j3n
**amd-hv3i7j3n** — (2026-10-31-select-popup) an open list was read through the host and the mcp role at `trace` with 0 option needles (`option_needles`). This guards a select click arm leaking page text (row decision/select-list-label-never-logged).

@history_item n=40 marker=amd-twhohqtg
**amd-twhohqtg** — (2026-10-31-select-popup) a ratified widening: an open list's shot carries option and `optgroup` labels as pixels, and `size` is read on a `multiple` select. The open-list behaviour that `handle_click`'s select arms drive must not change (rows decision/open-list-shot-carries-labels, decision/optgroup-label-crossing).

@history_item n=41 marker=amd-tjocwgx3
**amd-tjocwgx3** — (2026-10-31-select-popup) an open list's row crosses by its own `bounds` within the closed `differs` set. `SelectListRow.label` is never logged (rows contract/snapshot-diff/open-list-row-bounds, decision/select-list-label-never-logged).

@history_item n=42 marker=amd-nsxi7wwt
**amd-nsxi7wwt** — (2026-10-20-select-by-id) `select_interaction` and `stand_act_select` are silent, and the select model adds no log site or span. These are the guard checks the scope names for the select arms.

@history_item n=43 marker=amd-sdp6gopp
**amd-sdp6gopp** — (2026-10-13-range-input-interaction) `range_input_interaction`, named in the scope as a guard of `handle_click`, is silent in both layout modes. The range value leaves only through the ratified carriers.

@history_item n=44 marker=amd-x754ad24
**amd-x754ad24** — (2026-10-13-stand-keyboard-harness) `stand_keyboard`, named in the scope as a guard of the button and focus arms, is in the silent census, and the shared module's `tab_order` and `focused` are probed.

@history_item n=45 marker=amd-r5f3yres
**amd-r5f3yres** — (2026-10-31-engine-debts) `stand_keyboard`'s stated reach is the four lean tasks, the load page, Cells and its own fixtures. A new arm check placed there changes this stated reach.

@history_item n=46 marker=amd-zihybibd
**amd-zihybibd** — (2026-10-31-engine-debts) the trap: obs rows cite lines in `execute.rs` (§6, §4) and blitz-dom `events/pointer.rs` (§6, §1, §8), and an edit that moves those lines owes a re-point with no claim change. This chunk's restructure moves both files.

@history_item n=47 marker=amd-z2cdlvn4
**amd-z2cdlvn4** — (2026-10-31-select-popup) citations into `events/pointer.rs` and `execute.rs` were re-pointed after added lines. This is the same re-point duty that falls on this chunk.

@history_item n=48 marker=amd-woin57zn
**amd-woin57zn** — (2026-11-08-what-can-i-do-here) cited ranges in `execute.rs`, `stand_act_spans.rs` and `session_common/mod.rs` were re-pointed. This is the latest precedent for the citation sweep the `execute` move will need.

@history_item n=49 marker=amd-6zxxoq74
**amd-6zxxoq74** — (2026-10-07-driver-command-spans) the closed span's line shape is the span's fields in the order they are recorded. A content-named span field is never stored. Reordering the recording sites in `execute` would change the line's field order (rows contract/span-line/field-order, contract/escher-log-line/span-line, decision/content-span-fields-never-stored).

@history_item n=50 marker=amd-u2nw2mep
**amd-u2nw2mep** — (2026-10-09-founder-rulings) the founder ratified the closed span line as a second record class, together with the ruling that a diff's three sizes (`added` · `removed` · `changed`) may reach a log while its content never does (rows decision/closed-span-lines, contract/command-span/added · removed · changed).

@history_item n=51 marker=amd-yjzlnn3o
**amd-yjzlnn3o** — (2026-10-07-sink-target-allowlist) the sink's target allowlist drops outside targets at every level. The rule for later chunks: a target is admitted only together with a scrub rule for it. This bounds any helper that would log under a new target (row decision/sink-target-allowlist).

@history_item n=52 marker=amd-7v2ajbq2
**amd-7v2ajbq2** — (2026-10-06-telemetry-bootstrap) escher-telemetry is ungated while engine `tracing` is feature-gated. Engine targets print only the safe fields. This governs any engine log site in `pointer.rs` (rows decision/engine-target-safe-fields, decision/engine-tracing-feature-gated, decision/escher-crates-tracing-ungated).

@history_item n=53 marker=amd-vm35icer
**amd-vm35icer** — (2026-10-07-driver-session) `escher-session` is the second sink installer. Trap: a zero read from a host that installs no sink says nothing about one that does. Behaviour-preservation readings on the host must be taken through a sink-installing host (rows decision/scrub-sink-reach, decision/session-logs-carry-no-page-content).

@history_item n=54 marker=amd-gftbr5vi
**amd-gftbr5vi** — (2026-10-10-driver-cli) typed text was measured absent in a sink-installing host's log at `trace` under both builds, with by-level line counts. These are the `host_log` / `mcp_log` readings that hold the rows' guards.

@history_item n=55 marker=amd-rbvxics4
**amd-rbvxics4** — (2026-10-19-session-recording) the `command` span keeps its eight fields whether or not a recording is held. A call after which the recording ended ends its `tool_call` `error` (row contract/tool-call-ending/error, contract/kept-check/arguments).

@history_item n=56 marker=amd-6x37n7n2
**amd-6x37n7n2** — (2026-10-19-recorded-check-replay) each replayed step writes its own `command` span through `Session::run`, and the replay adds none. Kept answers are written whole at rest. The replay depends on the executor's span behaviour staying unchanged (row decision/coverage-line-never-logged).

@history_item n=57 marker=amd-fldwfipp
**amd-fldwfipp** — (2026-10-06-id-persistence) the one `println!` exception, a re-run child whose ids the parent reads. It belongs to the checks-silent row's except-clause, and any arm check this chunk adds before a move stays inside that row (row decision/checks-silent).

@history_item n=58 marker=amd-zc6kgyy6
**amd-zc6kgyy6** — (2026-11-08-what-can-i-do-here) `stand_actions` and escher-driver's `actions` module are recorded silent, and `session_common` has thirty-four readers (thirty-five after handler coverage, amd-srw72242). These reader counts move if the chunk adds a session check.

@history_item n=59 marker=amd-i4np4ecm
**amd-i4np4ecm** — (2026-11-12T03-36-23-rows) the masters became rows, each verified against the code at edd0cfaea7f0 / f84ccd9f58bc on the founder's approval. The picked rows this chunk is bound by (decisions, contracts, guards `stand_act_spans`, `host_log`, `mcp_log`, `stand_mcp_tools`) are those rows.
