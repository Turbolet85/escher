@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the `disabled` row split: parsed as a bool for focusability only, its presence alone (`disabled="false"` included) setting the DISABLED state and making the element ignore clicks; the first statement of the presence reading the read's `disabled` cause must share with the call.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — added the stable-element-id reader: the HTML `id` is an author key only when non-empty, `/`-free and first in pre-order, else the element reads its `/`-bearing path; computed on demand, written nowhere, no engine id; the grammar the L12 author-key / positional-path reading rests on.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the id's first crossing (the platform adapter's `author_id`), ratified by the operator, with the standing rule that any further consumer of the id beyond the platform accessibility API re-opens the row; the new read is such a consumer and is weighed against it.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added `DioxusDocument::unkeyed_actionable` as an in-process reader (id, tag, role, never text, name or attribute value) holding a process-local `NodeId` in `node`; trap: a wire form of the check is a crossing for ids and must leave `node` behind, escalated then; this is the L12 fold's binding.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a password's snapshot value reads the fixed `MASKED_VALUE`, never the text or its length; the snapshot's `enabled` follows the presence reading of `disabled`; the falsy boolean-attribute clear recorded; trap that the first wire form re-reads the mask — the read must open no new path for a control value.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the mask extended to a file input's value (a host path); the snapshot's text form; standing rule that the first command exposing a reading is a new crossing; kept: `unkeyed_actionable` still has no serialized form — the L12 fold would give it one.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the Driver command schema row: `validate` against a closed table, typed `Command` or `Refusal`, first failure in a fixed order, no `unwrap`/`expect`/panic/index on call input, no clock, no `Session`; the `Refusal` bullet's closed causes with fixed texts and no `String` field; rule: keep every refusal text fixed and free of call content — the read's per-verb cause must be one of these.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` validates first and nothing public runs a `Command` that did not come from `validate`; an id naming no element is refused with nothing run; the id looked up anew each call, never by index — the one validation path the read must enter through, no second path beside it.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — after `validate` the executor refuses `stale` / `not-found`, then a `click` or `type` on a target that cannot take it as `disabled`, `off-screen`, `covered` in that order, snapshot and focus unchanged; `scroll` applies none of the three; ratified as a widening of the validated surface — the fixed order the read's causes must agree with by one shared path.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one `command` span per call with eight fields (`verb` the table's word, `cause`, `settled`, `busy`, `passes`, `added`/`removed`/`changed`), never any argument, `Refusal`, `Fault` or screen text; exactly one thing of a refusal reaches a log, its cause's fixed name — the span rule the read's span must keep.

@history_item n=11 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the sink's span record class ratified; standing rule that a count of a diff's nodes may reach a log while no node, id, name or value may, a field holding any being a new crossing question — the read's span must log neither its listed ids, verbs nor causes beyond `verb` and a refused `cause`.

@history_item n=12 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's own sink: engine targets print safe fields only, `escher_` targets with the eleven content-named fields redacted, any other target dropped whole; trap: a "nothing in the logs" check must run against a host that installs the sink (`host_log`) — the read's silence witness follows it.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers) — the trust boundary restated: an id and typed text in; ids, names, masked control values, snapshot text and diffs out, no `NodeId`; the decoded call run through `Session::run` alone under the 1,048,576-byte answer bound — the crossing the read's answer must stay inside, a founder-ratified widening.

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line is an external input surface) — `command_line` checks a call by `validate` or `validate_session` before the session address is asked for, four fixed endings 0·1·2·3, no ending writes back anything the caller typed; standing rule that every later surface's input passes `validate` first — the read's CLI arm follows it.

@history_item n=15 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer; a refusal has a written form) — `Refusal::to_json` written from its cause's fixed strings on the socket and stdout (status 1); the id gains the driver's answers as an exit; `unkeyed_actionable` still has no command — the precedent for the read's refusal-cause words crossing.

@history_item n=16 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — `covered`'s meaning and two help texts restated as fixed strings, the cause set unchanged; no boundary moved because no verb, field or cause was added — the test the read's verb-help and any new `VerbSpec` text is held to.

@history_item n=17 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — three state readings inside the ratified carriers, and the founder's widening rule set with the ruling: a new field of a closed value set inside an already ratified carrier is no widening; authored text, an open value set, a new carrier or a new exit still is — the rule that decides the read's carrier and the verb-table change.

@history_item n=18 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing) — the MCP stdio vector and row: bounded request lines and results, fixed protocol errors with no `data`, typed tool arguments then `validate`; ratified, with "the next carrier is a new question" — the read's MCP tool and result follow this row.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the `tool_call` span) — a second `escher_driver` span, `tool_call` (`tool`, `ending`, closed words, outside `CONTENT_FIELDS`), parent of `command`, a ratified new log field — an MCP call of the read writes one such line.

@history_item n=20 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` as the tenth verb and the program's fixed `'static` table text crossing out on the ratified carriers, ratified; `tools/list` descriptions are the verb's help plus result fields; text from a page, a call or a screen, a new carrier or exit still widens — the rule any new `VerbSpec` target or refusal metadata and its `describe.rs` serving must pass.

@history_item n=21 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` as a new instance verb on the three carriers (pixels a widening, the founder's fork); `fonts-unavailable` added as an executor cause, `id` looked up as a `click`'s; never-fielded list grew — a precedent for a new verb's carrier ruling and for a cause the executor returns before anything runs.

@history_item n=22 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate) — the Dioxus bridge removes a falsy value of the 28 names of upstream's `is_bool_attr` (`disabled`, `inert` among them), so a Dioxus `disabled: false` never reaches the presence reading — the read's disabled cause on the stand rests on it.

@history_item n=23 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — focusability reads `disabled` by presence for the Tab walk, autofocus and `unkeyed_actionable`; the snapshot's `enabled` and focusability read the same presence (the founder's «По наличию») — the one reading the actionable set and the disabled cause share.

@history_item n=24 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `covered`'s meaning states two measured limits (the one-axis clip added), cause names unchanged, fixed program text inside the ratified carriers — the read naming `covered` inherits these limits.

@history_item n=25 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — `aria-expanded`, `aria-disabled`, `aria-level` read as closed values; `aria-disabled` moves no snapshot reading — the read's `disabled` cause must not come from `aria-disabled`, as the call's does not.

@history_item n=26 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the `inert` read) — `inert` by presence on an element or an ancestor: not focusable (Tab walk, autofocus, `unkeyed_actionable` skip it), no hit at it (`hit_inner`), out of the tree and the snapshot — an inert node stays outside the actionable set and reads no hit, as the call reads it.

@history_item n=27 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls) — presence on a form control alone through `ElementData::is_disabled`, read by focusability, `ClickTarget::Disabled`, `handle_click`'s gate and the activation arms; refusal texts restated as fixed strings — the presence reading the read's disabled cause and `clickable` share.

@history_item n=28 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `time-unavailable` retired, the cause set closed at eight (two by `validate`, six by the executor), ratified by the founder 2026-10-18 — the closed set the read's per-verb cause must be drawn from, a new cause being a widening.

@history_item n=29 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording directory (`0700`, `RecordDirNotPrivate`, `check-{n}.json` `0600`, a call no record can spell not kept, named by no field, never logged) and the strict `Check::from_bytes` reader (closed keys, each `Call` built through `command::call_of` so a replayed step passes `validate`) — a kept step of the read must fit both.

@history_item n=30 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer whole at rest, masked as on every carrier; the two committed stand-Counter checks the one exception to owner-only, no typed text, host path, secret or `screenshot` step, compared byte for byte with a fresh recording — recording edits for the read must keep them so and add no committed check.

@history_item n=31 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint as an id reader) — `Session::lint` first held in process only (PROVISIONAL, delegate), feeding the record of ids, findings of ids, closed words and numbers — the precedent for a read built in process first with its carrier left to the founder.

@history_item n=32 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb's findings on every carrier) — `lint` as a no-argument read-only verb on the socket, CLI, MCP and a kept check, a ratified boundary widening; its span records `verb` and, refused, `cause`; counts across the schema, cold-agent check, recorder and MCP listing moved together — the pattern this read's verb follows.

@history_item n=33 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting verbs wait on a load for at most `LOAD_WAIT`, while `snapshot` and `screenshot` wait on none and no verb has an argument or result field of a wait — the read, running no step, must wait on no load.

@history_item n=34 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot`'s `part`, the seventh argument kind, a ratified widening; a part's number and count never fielded — paging the read's answer would be such a widening, the alternative to an unpaged answer under the bound.

@history_item n=35 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the carried sets reconciled to name a lint's findings; a `boundary-mcp-stdio` line names `docs/capabilities.md` «What an MCP client carries onward» and its pin; the secret / host-path searches over the statement and pin, 0 hits — the read's MCP crossing must be named there with the pin passing.

@history_item n=36 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the four-term secret search extended to `README.md` beside `docs/capabilities.md` and `.github/scripts/test_capability_statement.py`, 0 hits; the pin reads no env var — any statement of the read added there must keep these at zero.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — `visible_region` reads clipping boxes through transforms, its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`; `off-screen`'s meaning restated, cause set eight — the read's off-screen cause must come from this same reader, a new caller recorded.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `Node::hit_inner` answers `None` for a non-invertible transform's subtree — the read's `covered` cause must come from the call's hit test under this rule.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`type` on a `multiple` select, and the public select readers) — `select_of`, `select_values`, `clear_select_typeahead`, `closed_select_of`, `is_multiple_select` through `get_node`, their callers named, no type, verb or field added — a new caller from the read must be recorded in that list.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `select` row's rules, limits and pins) — an option of a closed single select has no click arm and stays refused `covered`, under a group too — the read must report that cause for such an option, as the call returns it.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys, clicks and chosen set) — `click_select_row` behind `handle_click`'s `option` arm: a disabled select stops it, a disabled row changes nothing — the owner-served rows whose acceptance the read reports.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup` and its `label`) — `options` walks a select's `option` and `optgroup` children, `owner_select` and `option_disabled` the one shared rule — the owner-served sub-targets the read lists under a select.

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the select verbs' help) — `covered`'s meaning gains an option of a closed select, `click`, `press` and `type` help restated, each a fixed literal under the 2026-10-12 rule, cause set eight, verb count unchanged — the cause text the read names for a closed option.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (which select opens a list, what a row takes, and hostile markup) — `select_list_allowed` keeps a disabled, inert, hidden or empty select from opening; a disabled row changes nothing, a group header chooses nothing; hostile `size`, 2,000 options and a 5,500-character label panic nowhere, `MAX_ROWS` 20 — the read over a hostile select must stay bounded and agree with these.

@history_item n=45 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `select` row: Enter, Space and a click open the list) — a click on a single select opens or closes its list; an option of a closed single select keeps no click arm, refused `covered` (F9) — what the read reports `click` does on a select and its options.

@history_item n=46 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click, `Target` holds `select`, no verb, argument, field, cause or exit added, `type`'s help fixed text — the `type` path the read reports for a select.

@history_item n=47 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select readers and the list's public surface) — `closed_select_of` answers `None` for an open list, `is_select`'s one caller `execute.rs` `locate`; `close_select_list` the one public writer; `SelectListRow.label` holds page text, never logged — the read must write nothing and carry no row label.

@history_item n=48 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — `visible_region` answers an open list's scrollport for its rows — the off-screen cause for an open list's row must read through it.

@history_item n=49 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — an open row's own `bounds` inside the ratified crossing under the closed-field rule, `NODE_FIELDS` thirteen then; the highlighted row crosses no structured carrier — a precedent for a node field added without a widening.

@history_item n=50 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph joins what the carriers take out) — the socket and MCP vectors and the `docs/capabilities.md` carried set gain a UI graph (ids, owner ids, component, state and handler event names, closed flag words, booleans, never a control value, typed text or `NodeId`), a widening ratified, provisional for the founder's batch — the node model and crossing the read draws on.

@history_item n=51 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the thirteenth verb, graph, in the schema, MCP, the settle and the refusals) — `VERBS` 12→13, instance verbs 8→9, MCP tools 13 with `graph` after `lint`, an argument refused `malformed`, cause set staying eight, no wait — the counts the read's verb would move.

@history_item n=52 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb on the validated surface) — `schema::GRAPH` read as `Command::Graph`, argument refused `malformed` on every carrier, fields `nodes`/`states`, span `verb` alone, CLI status 0/1, `tools/list` thirteen; `DioxusDocument::graph` a read-only id reader with no `NodeId` — the direct precedent this read follows, ratified provisional for the founder's batch.

@history_item n=53 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `NodeState.clickable`, a closed bool after `focused`, true where listeners hold `click` and no form control `is_disabled` reads disabled; `NODE_FIELDS` 13→14, `Reading::ALL` 11→12; no widening under the 2026-10-10 rule — the actionable set's input and the count any new node field moves.

@history_item n=54 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's size under the answer bound) — `graph` unpaged with no part argument, which would widen the crossing; Cells' answer 293,869 bytes within 1,048,576 in both layout modes, the stated limit «An unpaged graph» in `docs/capabilities.md`, residual `res-7cc5bi4k` — the bound and paging question the read's answer meets.

@history_item n=55 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a recorded graph answer at rest) — kept checks hold a `graph` answer's UI graph, never a control value or typed text, a widening ratified — a kept step of the read joins this class.

@history_item n=56 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the verb counts the checks read) — the cold-agent check's thirteen verbs, a new cold-agent row, the recorder's and `Check::from_bytes`' nine instance verbs — the closed lists a new verb must join by the same table or stay out of.

@history_item n=57 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — `{owner}///{key}` reserved, `virtual_node_id` spelling it, no walk calling it so no served id holds `///`; ratified stable-id contract change — the read lists only served ids and produces no virtual-node id, the shape kept open for 0.4.0.

@history_item n=58 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's silence in the logs) — `graph` records `verb` alone, one `tool_call` parenting it, two `info_span!` in escher-driver, `host_log` and `mcp_log` each one call with 0 id, name and graph needles at `trace` under both builds — the silence witness the read's call repeats.

@history_item n=59 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Dioxus bridge's dispatch and the signal registry) — the registry and `DeclaredWrite` hold no state value and no `NodeId` beyond the dispatch chain, `Debug` counts alone; the graph's handlers and states the read draws its actionable set from come from here.
