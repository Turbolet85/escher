@history_item n=1 marker=amd-e5lmdcaj
**amd-e5lmdcaj** — (2026-10-07-command-and-refusal-schema) made the `Cause` names the command span's cause domain and set the rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. A coverage verb's arguments (check names or paths, if any) and its refusals fall under this rule.
@history_item n=2 marker=amd-3iczuidb
**amd-3iczuidb** — (2026-10-07-act-by-id) added `Outcome` to the `Debug`-deriving types that nothing prints, logs or fields, and kept the executor silent. A coverage `Outcome` variant inherits this, which is the anti-pattern the extract names.
@history_item n=3 marker=amd-cschmkve
**amd-cschmkve** — (2026-10-07-driver-command-spans) built the `command` span: INFO, target `escher_driver`, eight fields recorded only where they apply, and the never-fielded list. It also made `stand_act_spans`' re-run children the one exception to "installs no subscriber". A coverage verb runs under this span unchanged, and the extract's span constraint rests on it.
@history_item n=4 marker=amd-6zxxoq74
**amd-6zxxoq74** — (2026-10-07-driver-command-spans) set the closed-span line's shape and made the sink's scrub judge every span pair the way it judges an event field (later ratified by amd-u2nw2mep). The `tool` word a coverage tool adds is judged pair by pair under this rule (cascade g2 binding 2).
@history_item n=5 marker=amd-dnjkg5io
**amd-dnjkg5io** — (2026-10-10-driver-cli) made a command's printed answer an answer and never a log line, and gave `Outcome::to_json` as the one place an `Outcome` is written. Standing rule: the next surface adds no span or log field that holds what a call supplied or what a screen reads. The coverage answer must go out through `to_json` alone.
@history_item n=6 marker=amd-3jlpr7el
**amd-3jlpr7el** — (2026-10-10-mcp-surface) added the `tool_call` span, ratified, with two closed-word fields (`tool`, `ending`) and `command` as its child by nesting. It also made `stand_mcp_tools`' in-process `Recorder` the second subscriber exception and added `mcp_log` beside `host_log`. A fifteenth tool word lands in this span, and the chunk must add no field to it.
@history_item n=7 marker=amd-hk2vxiq4
**amd-hk2vxiq4** — (2026-10-12-self-description) added `help` to the closed sets of both spans with no field added, and set how a session-level verb records its word beside `unknown-verb`. This is the precedent for widening the closed verb and tool sets by one word.
@history_item n=8 marker=amd-wivka2yb
**amd-wivka2yb** — (2026-10-12-headless-screenshot) added `screenshot` to both spans' closed sets: an accepted call records `verb` alone and the tool call parents its `command`, and `stand_act_spans`' driven verbs moved. This is the precedent for a new verb moving the span words and the stand counts together.
@history_item n=9 marker=amd-vrhzfubf
**amd-vrhzfubf** — (2026-10-19-visual-lint, the gated part) made `lint` an instance verb inside the command span. An accepted `lint` records `verb` alone, its findings are never fielded, and `tool_call` went to twelve words. This was the founder's "no new span field" word (F2). It is the precedent a coverage verb follows, a no-field ruling already given.
@history_item n=10 marker=amd-4q2wobhh
**amd-4q2wobhh** — (2026-10-20-large-screen-in-parts) records that a part read writes `verb` alone, that a part's number and count are never fielded, and that `stand_act_spans` moved 34 to 35 calls per mode. This is the precedent that a read-only answer's content and counts never become fields.
@history_item n=11 marker=amd-2iwce5bg
**amd-2iwce5bg** — (2026-10-31-select-popup) ruled that a verb's help text and a word's meaning are the program's own `'static` text, holding no call text and adding no log site or span field. A coverage verb's `describe` / help entry falls under this rule (2026-10-12) and is not a widening.
@history_item n=12 marker=amd-rbvxics4
**amd-rbvxics4** — (2026-10-19-session-recording) put the recording (`recording_into`, the recorder, `Check::from_bytes`) on the observed-absent list and added the `0700` Recording directory row (call content at rest, local only, ratified). The owed `actions`-replay test records through this path, and any check it keeps must stay within this row.
@history_item n=13 marker=amd-6x37n7n2
**amd-6x37n7n2** — (2026-10-19-recorded-check-replay) put `Check::replay` on the absent list: each replayed step writes its own `command` span and the replay adds none. `Replay` / `Change` hold bools, a step index and closed key words only. `stand_replay` is silent, its failure messages the mode, the file and a step index alone. The sixteenth module `replay` holds no tracing site. It also added the §9 Committed checks row (`tests/blitz-tests/checks/counter-{cli,mcp}.json`, ratified). The owed test's failure, naming a step and a differing key, takes exactly this shape, and committing a new check widens that row.
@history_item n=14 marker=amd-y2gzdcnt
**amd-y2gzdcnt** — (2026-10-19-test-splits) moved the needle helpers (`Needles`, `stand_needles`, `found`, `occurrences`, `recorded_needles`) into the seven_guis shared module `tests/common/mod.rs`. Its assertion messages are a kind and a count, never a needle. Coverage needles for `host_log` / `mcp_log` join this module and its silence.
@history_item n=15 marker=amd-yjzlnn3o
**amd-yjzlnn3o** — (2026-10-07-sink-target-allowlist) set the standing rule that a target is admitted only together with a scrub rule for it, and that an `escher_` target prints with content-named fields redacted. Any span word or field a coverage verb brings is read under this allowlist (cascade g2 binding 2).
@history_item n=16 marker=amd-bcuh7k4j
**amd-bcuh7k4j** — (2026-10-25-capability-statement) records that `docs/capabilities.md` restates the driver's `command` / `tool_call` spans and that its pin reads none of those claims. A verb-set change is made in that document by hand, and the scope lists it among the verb-count pins.
@history_item n=17 marker=amd-6reehbne
**amd-6reehbne** — (2026-10-25-cold-agent-bar) records that the pipe reads `server.log` only to count `tool_call` lines and the counted verbs (`lint` · `screenshot` · `advance`, `span_counts`), and the `capability-unused` reason. `tool_call` keeps its two fields. A coverage tool's place in the counted list belongs to the Cold-agent bar entry.
@history_item n=18 marker=amd-qokfz7t5
**amd-qokfz7t5** — (2026-11-04-dioxus-runtime-probe) recorded the silence row for a blitz-tests target with no `stand_` prefix in log-coverage alone, using the census pattern with `UnixListener|TcpListener|bind\(`. This is the precedent for where a new non-stand coverage or replay check's silence row lands.
@history_item n=19 marker=amd-f6nw3lvy
**amd-f6nw3lvy** — (2026-11-04-dioxus-runtime-probe) records that the probe reads current listeners and components through public dioxus API, and that its failure messages never name a listener read off a node, a value or a node id. The handler-reach record and the coverage tests read the same listener class, so they keep this failure-message rule.
@history_item n=20 marker=amd-qrj3ym2p
**amd-qrj3ym2p** — (2026-11-07-mounteddata-buffered-render) records that `mutation_buffer` and the `DioxusDocument::initial_build` / `Document::poll` lines that render through it hold no log site, print, env read or listener (§6). `MutationBuffer`'s `Debug` prints `len` alone (§8). A handler-reach record added in `dioxus_document.rs` must keep those lines silent (cascade g2 n=6).
@history_item n=21 marker=amd-j2dguaiu
**amd-j2dguaiu** — (2026-11-07-mounteddata-buffered-render) re-pointed the `multiple_select_values` citation `dioxus_document.rs:32-46` after one added line. This is a trap for this chunk: a reach record added in `dioxus_document.rs` moves the §8 citation again.
@history_item n=22 marker=amd-2rlbtdpf
**amd-2rlbtdpf** — (2026-10-31-engine-debts) added the §8 item that a `multiple` select's chosen set leaves only as form entries, the snapshot `value` and tree nodes, and never as a log line or field, with the added-lines census at 0. Cascade g3 binds the reach record's `dioxus_document.rs` lines to that census.
@history_item n=23 marker=amd-w5hr2k4x
**amd-w5hr2k4x** — (2026-10-31-engine-debts) moved the `fixtures` module's readers from 24 to 28 and left `session_common`'s twenty-nine stale against a measured 31. This is a count-drift trap for the reader counts a new coverage or replay check extends.
@history_item n=24 marker=amd-7g47dwps
**amd-7g47dwps** — (2026-10-31-select-popup) caught the `session_common` reader count up to thirty-two and named each new `stand_act_*` check in the no-sink row. This is the precedent for keeping the reader count and the census row right when a check is added.
@history_item n=25 marker=amd-wv4t22sd
**amd-wv4t22sd** — (2026-11-07-ui-graph) added `graph` as the ninth instrumented verb under the one `command` span and made `Outcome::Graphed` a never-logged answer written by `to_json` alone, with `session_common` readers at 33. The coverage read is built on this graph answer and follows the same writing rule.
@history_item n=26 marker=amd-tlxklvv6
**amd-tlxklvv6** — (2026-11-07-ui-graph) ratified the graph answer's carried set (ids, component, state and handler event names, flag words, booleans, never a value) and added it to the Recording directory row. The overseer's confirm was PROVISIONAL, waiting for the founder's word at the epoch boundary. A coverage answer (node id, `owner`, event name) widens past this set, and the founder must ratify it.
@history_item n=27 marker=amd-kn6x2k72
**amd-kn6x2k72** — (2026-11-07-ui-graph) put `graph` into the cold-agent pipe's `VERBS`, uncounted, adding no verdict field or `capability-unused` reason. This is the precedent for a new verb's pipe entry, which is the Cold-agent bar entry's call.
@history_item n=28 marker=amd-jsqwtiq2
**amd-jsqwtiq2** — (2026-11-07-ui-graph) records that the graph verb's `command` line holds no node, state, id or name, that `host_log` / `mcp_log` drive one `graph` call at `trace` with 0 needles (`graph_needles`) under both builds, and that the `graph` tool call parents one `command` with `verb` alone. This is the acceptance shape a coverage verb repeats.
@history_item n=29 marker=amd-htjat4nf
**amd-htjat4nf** — (2026-11-07-ui-graph) records that the `graph` and `signal_registry` modules and `stand_graph` hold no print, log site, subscriber or env read. It also records that `DioxusEventHandler::handle_event` sets the registry's dispatch only where a registry stands, adding no log site. The handler-reach record that must hold on every app moves exactly these files and this dispatch path.
@history_item n=30 marker=amd-b2prlxvh
**amd-b2prlxvh** — (2026-11-07-ui-graph) records that `SignalRegistry` holds no state value and no `NodeId` beyond the dispatch chain, that its `Debug` prints counts alone, and that it has no log site. A reach record kept in or beside the registry must hold to the same (cascade g3 n=5).
@history_item n=31 marker=amd-zc6kgyy6
**amd-zc6kgyy6** — (2026-11-08-what-can-i-do-here) added `stand_actions` as `session_common`'s thirty-fourth reader, plus the silence row over `actions` and `actionable_nodes`, escher-driver keeping 2 `info_span!` sites. The reader count and the census row are what a new coverage or replay check extends.
@history_item n=32 marker=amd-woin57zn
**amd-woin57zn** — (2026-11-08-what-can-i-do-here) re-pointed cited ranges in `stand_mcp_tools.rs`, `json.rs`, `execute.rs`, `command.rs`, `stand_act_spans.rs`, `session_common/mod.rs`, `host_log.rs`, `mcp_log.rs`, the seven_guis `common/mod.rs` and `cli_commands.rs`. A new verb edits these same files, so the citations will need another re-point.
@history_item n=33 marker=amd-quujnooh
**amd-quujnooh** — (2026-11-08-what-can-i-do-here) ratified the actions answer's carried set (ids, owner ids, verb and cause words, booleans, written by `to_json` alone, kept whole in a recorded check) and added it to the Recording directory row. The owed replay test re-asserts exactly this kept answer, and `recording_tests` already reads it back whole.
@history_item n=34 marker=amd-outdow3i
**amd-outdow3i** — (2026-11-08-what-can-i-do-here) moved the pipe's `VERBS` to fourteen with `actions` uncounted. A fifteenth word belongs to the Cold-agent bar entry, per the scope's boundary.
@history_item n=35 marker=amd-k5v64vz2
**amd-k5v64vz2** — (2026-11-08-what-can-i-do-here) moved the `command` verb table to fourteen words, ten of them instance-level, and the `tool_call` `tool` set to fourteen with `actions`. An accepted `actions` records `verb` alone, and its tool call parents one `command` (`stand_mcp_tools`). These are the counts a coverage verb moves to fifteen.
@history_item n=36 marker=amd-ei5tfcg5
**amd-ei5tfcg5** — (2026-11-08-what-can-i-do-here) records that `Session::actions` reads without acting (no `pump` / `tick` / `dispatch`) and changes nothing in the document. It also put `check_target`, `target_of` and `actionable_nodes` on the span-absent list. The owed replay test's equality rests on this read changing nothing, and a coverage read must also open no span.
@history_item n=37 marker=amd-d3eh2inx
**amd-d3eh2inx** — (2026-11-08-what-can-i-do-here) added `actions_needles` to the seven_guis shared module and moved `stand_act_spans` to 38 calls per mode across ten verbs. `host_log` / `mcp_log` each read one `actions` call at 0 needles under both builds. This is the needle and call-count pattern a coverage verb extends.
