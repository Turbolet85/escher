@history_item n=1 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built `validate`: a closed table of verbs and bounded argument kinds, a typed `Command` or a `Refusal` in fixed order, no `unwrap`/`expect`/panic/index on call input, no clock, no `Session`; `Refusal` a closed cause set with no `String` field; and no verb has an argument or result field of a wait. A coverage verb, if added, joins this table under these rules, and any coverage or replay-mismatch refusal must stay a closed cause holding nothing of the call or the file; rule: the first surface reaching `validate` from outside is its own crossing question.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` calls `validate` first and nothing public runs a `Command` that did not come from it. The owed replay test and any coverage read rest on this: every replayed `actions` step goes through `Session::run`.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (what the driver logs) — one `command` span per call with fixed-word, bool and count fields only, never an argument, `Fault` or screen text; of a refusal only its cause's fixed name reaches a log. Binds any coverage field on the span to fixed words, bools and counts.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `escher_telemetry::init`'s three-outcome sink: engine targets print safe fields, `escher_` targets print with the content-named fields redacted, any other target dropped; trap: a "nothing in the logs" check must run against a host that installs the sink (`host_log`). Anything the coverage read or reached-handler record emits passes this allowlist and is measured by `host_log`/`mcp_log`.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the session socket: threat vector, access) — the socket's access is the filesystem's alone: state directory `0700` (a group- or other-open one refused `StateDirNotPrivate`), socket `0600`, no handshake, token or peer check. A coverage call on the socket adds no auth and loosens no mode.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the socket carries calls and answers (a widening ratified by the founder), and the command line became an external input surface read by the closed `command_line` reader, the call checked by `validate`/`validate_session` before the session address is asked for, four fixed exit statuses, no ending writing back anything typed; standing rule: every later surface's input passes `validate` first and a `Refusal`, usage line or session error holds nothing caller-supplied. A coverage verb's CLI arm and any check-naming argument fall under this.

@history_item n=7 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule) — the founder's widening rule: a new field of a closed value set inside an already ratified carrier is not a widening; authored text, an open value set, a new carrier or a new exit still is. This is the test the coverage answer's fields and any new verb/argument are read against at P4.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP stdio crossing ratified: typed tool arguments then `validate`, fixed protocol errors with no `data`, bounded request lines and results, no auth, binds nothing; `serde_json` admitted as a reader only, no `preserve_order`. A coverage tool joins `tools/list` under these bounds, and the replay test's answer comparison reads through this `serde_json`.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` admitted; the program's own fixed `'static` table text crossing out is inside the carriers, while text from a page, a call or a screen, a new carrier or exit still widens (a playbook rule). A coverage verb's help and result-field descriptions are fixed table text under this rule; `SESSION_ERRORS` table form.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (kept check at rest, its directory, its reader, `--record`) — the recording directory row (`0700`, `check-{n}.json` `0600` `create_new`, bounds 400 records / 1,048,576 bytes, ending `recording-failed`, never named by any tool call or field, never logged) and the strict `Check::from_bytes` reader (closed keys, anything else `malformed` holding nothing, no panic path, each `Call` through `call_of` so a replayed step passes `validate` again); a masked `type` kept as `MASKED_STAND_IN`. The owed replay test records into and reads back through exactly these; a path that feeds checks to a coverage command is this reader's input class.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, session errors, `CARGO_MANIFEST_DIR`) — wire `v3` with `recording-failed`, `SESSION_ERRORS` 15, `call_of` in `command.rs`, `serde_json` also reading kept-check bytes, and `CARGO_MANIFEST_DIR` read at build time by escher-driver's unit tests for the workspace `target/tmp`. Precedent for where a test-recorded check may live and which env reads a new test may rely on without a registration.

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer kept whole at rest; `Check::replay` runs each kept step through `Session::run` alone, nothing on another app's session, `Replay`/`Change` holding bools, counts and closed words only; only `counter-cli.json` and `counter-mcp.json` committed (no typed text, host path, name, secret or `screenshot` step), every other check owner-only and never committed; `serde_json` reads the answer line the replay compares. Binds the owed `actions` replay test (not committing its check) and the differing-key report's form.

@history_item n=13 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb on every carrier) — precedent for adding a no-argument read verb onto every ratified carrier: founder-ratified widening, an argument refused `malformed`, one result field written by `to_json` alone, never fielded on the span, kept in a recording with its answer whole, and the verb counts moved in the cold-agent check, recording rows, MCP `tools/list` and logging needles.

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `TMPDIR`, read by escher-driver's client unit test through `std::env::temp_dir()`, registered test-only (operator's answer: no crossing); the library reads none beyond `RUST_LOG`. A replay test placing its owner-only recording under a temp dir rides this precedent; a new env read elsewhere must be registered.

@history_item n=15 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — seven_guis `host_log` became the standing check of a recording session's clients at `RUST_LOG=trace` (`start crud --record`, 0 id, name, sentinel, recording-path and recorded needles, both builds). Where a coverage verb or a check-reading path is added, this is the needle check that must hold.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the live child's kept checks and their second reader) — the pipe became a second reader of kept checks beside `Check::from_bytes`, holding bools and counts only, placed inside the 2026-10-19 recording ruling by the founder. Precedent and constraint for any coverage path that reads a set of checks: counts and closed words out, no content printed or copied.

@history_item n=17 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md`'s «What an MCP client carries onward» states the carried set, adding no carrier; the secret search (`/home/|api[_-]?key|authorization:|bearer `, two e-mail domains) over it and its pin `test_capability_statement.py`, 0 hits, the pin reading no env var. A coverage verb moves this statement and pin, which must keep 0 hits.

@history_item n=18 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the four-term secret search extended to `README.md` (README, `docs/capabilities.md`, the pin, 0 hits). README's tool count moves with a new verb, so the search must stay clean there too.

@history_item n=19 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the stable id's tiers (author key, anchored `{key}//{segment}`, component, document path) and its uniqueness argument; an `UnkeyedActionable` holds a process-local `NodeId`; trap: a wire form of the check must leave `node` behind. The coverage list keys on this id, never a `NodeId`.

@history_item n=20 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the `inert` read) — an inert node is not focusable, takes no hit and leaves the adapter tree and the snapshot (so the graph). The coverage count, reading the graph's handlers, does not count an inert subtree's; the replayed `actions` answer skips inert nodes.

@history_item n=21 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls) — `disabled` read by presence on a form control alone through `ElementData::is_disabled`, read by focusability and click gates. The kept `actions` answer's causes rest on this reading, which the replay must reproduce identically.

@history_item n=22 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the `select` row: a choice dispatches `input` then the engine-emitted `change`, each carrying the value, inside the ratified carriers, no carrier, verb, field or span added. A reached-handler record beside the dispatch counts (element, event) only and never holds the value these events carry.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys, clicks and chosen set) — the set's exits are form entries, `input`/`change` values, the snapshot's `value` and the options' tree nodes, no carrier, field, log field or span added. The reached-handler record in `dioxus_document.rs` must give the chosen set no new exit.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click, so `type` fires no DOM focus events; `Target` holds `select`. A coverage count must not read a select's focus handlers as exercised by a `type`.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select readers and the list's public surface) — `is_select`'s caller `execute.rs`, `clear_select_typeahead`'s one non-test caller `focus_select`, no type, verb or field added. `Session::actions`, exercised by the replay test, reaches these readers; the chunk adds nothing through them.

@history_item n=26 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the spawned-task MountedData panic, removed) — `initial_build` and `poll` render into `MutationBuffer` unborrowed, then take `inner.borrow_mut()` once; pinned by `dioxus_runtime_probe`. A reached-handler record added in `dioxus_document.rs` must keep the single borrow and reintroduce no `RefCell already borrowed` panic.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph joins what the carriers take out) — a `graph` answer's ids, owner ids, component, state and handler event names, closed flag words and booleans cross the socket, CLI, MCP and kept checks, never a control value, typed text or `NodeId`; a widening ratified, the lines' placement provisional for the founder's epoch-boundary batch. The coverage answer must stay inside this ratified set.

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the thirteenth verb in schema, MCP, settle and refusals) — `graph` the ninth instance verb: no argument, running no step, waiting on no load, an argument refused `malformed` with nothing run, the cause set staying eight. The template a coverage verb would follow.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Dioxus bridge's dispatch and the signal registry) — the dispatch sets a registered `SignalRegistry` around `runtime.handle_event`, inert where no state is registered; `use_graph_state`/`SignalRegistry` take dioxus-core types alone, hold no value or `NodeId` beyond the dispatch chain, no log site, print, env read or `unwrap` on app input, `Debug` counts only. The reached-handler record must hold on every app with these same properties.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's size) — `graph` unpaged, no part argument ("which would widen the crossing"), Cells at 293,869 bytes under the 1,048,576-byte bound, the stated limit «An unpaged graph», `res-7cc5bi4k`. The coverage read uses this unpaged answer; a coverage answer stays under the same bound.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a recorded graph answer at rest) — kept checks hold a `graph` answer at rest, names, flag words and booleans only; a widening ratified. A coverage path reading checks meets graph answers at rest.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `NodeState.clickable` (listeners holding `click`, not a disabled form control), a closed bool inside the ratified carrier, no widening under the 2026-10-10 rule. Read by the actions reading the replay re-asserts.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb on the validated surface) — `schema::GRAPH`, `nodes`/`states` the eleventh and twelfth field kinds, written by `to_json` alone, span recording `verb` alone, `tools/list` and `VERBS` thirteen, `DioxusDocument::graph` a read-only reader of the id with no `NodeId`. The coverage read takes its (node, event) handlers from this graph, which keeps its shape.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the verb counts the checks read) — the cold-agent `valid_step` and `VERBS`, the recording row's and the recorded-check reader's instance-verb sets moved together with a new verb. The registries a coverage verb would move again.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — `{owner}///{key}` reserved, no walk calls `virtual_node_id`, no served id holds `///`; a stable-id contract change, ratified. The coverage key stays open to 0.4.0's virtual nodes through this form and serves none now.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's silence in the logs) — `graph` records `verb` alone; dioxus-native-dom's `graph` and `signal_registry` modules hold no log site; `host_log`/`mcp_log` 0 graph needles. A reached-handler record in those modules must keep them log-free, and a coverage verb gets the same needle measurement.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer in the threat model's carried sets) — the `actions` answer `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` crosses the socket, CLI, MCP and kept checks, a widening ratified, `keyed` PROVISIONAL. The shape the owed replay test compares, and the latest carried-set addition a coverage answer would sit beside.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer on the socket and in kept checks) — a `lint` or `actions` step kept with its answer whole; kept checks hold `actions` answers at rest; «the replay of a kept `actions` step is exercised by no test yet, owed on «Handler coverage»» — the CARRY this chunk discharges, its criterion UNMET in part at that chunk.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the verb table at fourteen) — `VERBS` 14, ten instance verbs, MCP `tools/list` 14 with `actions` after `graph`, the cold-agent `valid_step` 14. The counts and pins a new coverage verb moves to fifteen / eleven.

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged actions answer's size) — Cells' `actions` answer 106,088 bytes for 678 nodes, held by `the_whole_cells_answer_is_within_the_answer_bound`, the stated limit «An unpaged actions answer», `res-7cc5bi4k`. The replayed answer stays under the bound.

@history_item n=41 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's validation and its answer lines) — `("actions", [])` → `Command::Actions`, `nodes` the thirteenth field kind written by `action_node` alone, an argument refused `malformed` in process, CLI (status 1) and MCP (`isError`), no cause added, the set eight. The latest template for a coverage verb's validation and field kind.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the shared target check and the actions read) — one crate-private `check_target` for the four id-taking arms and `Session::actions`; `Session::actions` builds calls through `validate` then `check_target`, running nothing, `stale` on an unresolved node, the read changing nothing. A replayed `actions` step equals its kept answer only while this holds.

@history_item n=43 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and `actionable_nodes`) — `Readings` the one actionable rule; `actionable_nodes` returns `ActionableNode { id, owner, keyed }`, no `NodeId`, a removed listener read by neither. The answer the replay test re-asserts.

@history_item n=44 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's silence in the logs) — `actions` records `verb` (plus `cause` when refused), `Session::actions` opens no span, `actionable_nodes` no log site, two `info_span!`, `host_log`/`mcp_log` 0 actions needles. The obs bar a coverage verb or a replay path must keep.
