@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** (the HTML `id` gains the stable-element-id reader) — set the author-key / path derivation and "no `id` makes two ids equal", no engine id or pointer in the id; the layout read names its element by this id, so it must resolve it through the same reader (contract/stable-element-id/derivation, no-collision, unresolved-node).

@history_item n=2 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** (the CRUD row's id key is a model-assigned u64) — an id is never a list index, pointer, hash, clock or process-local value; any id the layout answer carries (the element, its parent or containing block, a canvas owner) must keep contract/stable-element-id/stability.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** (the stable id reaches the platform accessibility API) — standing rule: any further consumer of the id beyond the platform adapter re-opens the `id` row; a new layout read is a further reader and crossing of the id (decision/stable-id-exits).

@history_item n=4 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** (the snapshot is an in-process reader of ids and names) — recorded new in-process readers in the `id` row's reader list; trap: the first wire form of a reader is a crossing for ids and must be escalated then — the layout read's first carrier is that moment.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** (the `id` row states four tiers and a third in-process reader) — anchored `{key}//{segment}` tier and three-way uniqueness argument (contract/stable-element-id/anchored-path, no-collision); trap: a wire form must leave `NodeId` behind (decision/no-process-handle-crosses).

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (a password's snapshot value is masked) — origin of `MASKED_VALUE`; a layout answer must carry no control value, and if it ever reads a text extent or inline content of a password input it must not leak length/glyph widths (decision/masked-input-values).

@history_item n=7 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** (the snapshot has one text form; a file input's value is masked) — the mask extended to file inputs and fields written in `Debug` form so no line break or bare `"` lands; standing rule that the first command exposing a reading is a new crossing, asked again.

@history_item n=8 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** (the diff as an in-process reader) — diff values come from snapshots only so the mask holds by construction; the precedent for a new reader inheriting the mask rather than re-reading values (decision/mask-holds-on-every-carrier).

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session socket: threat vector, access, input rows) — the Unix socket as the one listener, `0700`/`0600`, closed grammar; standing: a command carrying an id, name, value or screen reading reopens the crossing question (decision/session-socket-sole-listener, session-socket-closed-grammar).

@history_item n=10 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (outside-target records dropped) — a sink-installing host logs no id or name at any level; trap: a "nothing in the logs" check must run against a host that installs the sink (`host_log`) — owed for a layout-explain verb (decision/stable-id-not-logged, accessible-names-not-logged).

@history_item n=11 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (a settle reports a load in flight and never waits on one) — rule: a driver command that would wait with a clock is a new decision, not an extension; a read-only layout verb must wait on nothing and move no time (decision/time-moves-only-on-advance, no-wait-verb-fields).

@history_item n=12 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (a driver call is validated before anything runs) — closed verb table with bounded argument kinds, no panic on call input, no clock, refusal texts fixed and free of call content; any new verb and its `id` argument go through `validate` (decision/closed-driver-verb-schema, no-panic-on-call-input, command-schema-no-clock, refusal-holds-no-call-input).

@history_item n=13 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the command schema is executed in process; an id is taken in) — an id-taking command looks the id up anew on every call through `element_ids()`/`get_node`, never by index, no node handle kept (decision/id-lookup-anew-each-call, no-node-id-between-calls).

@history_item n=14 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (the detected causes after `validate`; the session holds id text) — unknown id refused `stale` or `not-found` via the session's bounded record of ids; the sixth verb ratified as a widening of the validated surface — the precedent a layout verb follows (decision/unknown-id-refused-stale-or-not-found, session-id-record-bounded).

@history_item n=15 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fixed-word or count fields) — one `command` span per call, `verb` the table's word, a refusal logged by cause name only, no argument or answer content fielded; the scope's span rule (verb, cause alone) rests here (decision/one-command-span-per-call, refusal-log-cause-name-only).

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the sink's second record class and the diff's lengths, PROVISIONAL) — one sink line per closed span, content-named fields never stored (decision/closed-span-lines); a new span field would ride this record class.

@history_item n=17 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (the sink's second record class and the diff's sizes in a log, ratified) — standing rule: a count may reach a log, no node, id, name or value; the scope keeps even the read's size off the span — any size/count field is a new question.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers) — wire grammar with request/answer bounds (16,384 / 1,048,576 bytes, 30 s read); a layout answer travels under these bounds (decision/session-socket-carries-calls, session-socket-bounds).

@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line is an external input surface) — closed argv reader, call checked before the session address is asked, no echo of caller input; standing rule: every later surface passes `validate`, refusals/usage/session errors hold nothing a caller supplied — a new verb's CLI arm and flags follow this (decision/cli-closed-argv-reader, no-echo-of-caller-input).

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer) — the driver's answers became an exit of the id, typed text measured absent from the host's log; the content class a layout answer would add to these exits is judged against this ratified set.

@history_item n=21 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — origin of the widening rule: a closed-value field in a ratified carrier is no widening; authored text, an open value set, a new carrier or exit still is — the rule the scope says decides which parts of the layout answer are the founder's (decision/boundary-widening-rule, closed-value-field-no-widening).

@history_item n=22 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing, ratified) — bounded request/result, typed arguments before `validate`, `-32602` for unknown tools; "the next carrier is a new question"; a new verb becomes an MCP tool under these rules (decision/mcp-bounded-lines, mcp-result-crossing, contract/mcp-tool-arguments/value-kinds).

@history_item n=23 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` reviewed; the `tool_call` span) — `serde_json` reads only, replies hand-written; `tool_call` span holds `tool` and `ending` only — a layout answer must be hand-written JSON and add no span field (decision/serde-json-read-only, contract/tool-call-span/*).

@history_item n=24 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and the program's fixed text through the validated surfaces) — the program's own fixed table text crossing is ratified; `tools/list` descriptions are each verb's help plus result fields; a new verb's help and field descriptions in `describe.rs` ride this, while page/call text does not (decision/mcp-listing-fixed-text, session-errors-fixed-table).

@history_item n=25 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the screen's pixels in band on the three ratified carriers) — precedent: an open value set (pixels) in an answer is a widening ratified by the founder; also `fonts-unavailable` refusing a reading before anything is read — guides whether layout values are closed numbers or open content.

@history_item n=26 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes, `advance` on every session) — `time-unavailable` retired, the cause set closed at eight; a layout verb's refusals must come from the eight, adding none (decision/closed-refusal-cause-set).

@history_item n=27 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest, its directory, its reader and `--record`) — a call no record can spell is not kept; the id's exits gain kept checks; a new instance verb must be recordable and re-validated on read (decision/kept-check-owner-only, recorded-check-strict-reader, contract/recorded-check/tool).

@history_item n=28 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors, `call_of`'s home, serde_json) — wire `v3` replies (`accepted`/`refused`/`oversize`/`recording-failed`); a new verb rides v3 unchanged (contract/session-socket-reply/*, contract/session-wire/recording-failed).

@history_item n=29 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, the committed checks, the replay) — kept checks hold each answer whole, masked; replay through `Session::run` alone; the committed stand-Counter checks compared byte-for-byte with a fresh recording — a new verb's answer will be at rest and its replay owed (decision/kept-check-answer-whole, replay-through-session-run, committed-counter-checks).

@history_item n=30 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint as an id reader, and the text extent reader) — `BaseDocument::text_extent`: read-only, `None` for unresolved/no inline layout through `get_node`, no log/panic, geometry only; the lint link the scope names, and the shape of a read-only layout reader (decision/text-extent-read-only-none-when-unresolved).

@history_item n=31 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb's findings on every carrier) — the `lint` precedent the scope cites: a new no-step verb ratified as a widening onto every carrier, findings ids/closed words/numbers only, never fielded, `fonts-unavailable` refusing before a read, counts updated across cold-agent, recording and MCP (decision/lint-findings-closed-content, lint-findings-crossing).

@history_item n=32 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (the acting step's bounded load wait) — only acting verbs wait `LOAD_WAIT`; reading verbs wait on no load — a layout read must stay with the reading verbs (decision/time-moves-only-on-advance, contract/driver-command-line/answer-bounds).

@history_item n=33 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (`snapshot`'s `part` and the paged answer on every carrier) — paging an answer is an argument widening ratified by the founder; part number/count never fielded — if a layout answer needs paging or an argument beyond `id` it is a fork.

@history_item n=34 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (a recording session's clients read at `trace` by a standing check) — `host_log` drives each new verb at `RUST_LOG=trace` with 0 id/name/sentinel needles; a layout verb owes a line in it (decision/no-screen-content-in-logs).

@history_item n=35 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the carried sets name a lint's findings and the diff) — the carried sets of the socket, MCP, stdout and kept checks, and `docs/capabilities.md`'s committed statement, are reconciled per new content class; a layout answer must be added to each carried set and the statement (decision/capability-statement-mcp-crossing).

@history_item n=36 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the live child's kept checks and their second reader, the paged decode) — the pipe decodes a paged answer only as exactly `text`·`part`·`parts`; any new answer shape must not collide with it (decision/paged-answer-closed-decoding).

@history_item n=37 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the agent's check as input) — the agent-written check: ≤ 65,536 bytes, ≤ 400 steps, closed keys, `tool` a known verb; a new verb joins this verb set (decision/cold-agent-check-validation).

@history_item n=38 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the capability statement's secret search extended to README.md) — the four-term secret search reads over `README.md`, `docs/capabilities.md` and its pin; an edit of those files re-runs it.

@history_item n=39 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the owed checks discharged) — the 30 s answer bound and the closed stdout pipe each pinned by a check (decision/closed-pipe-drops-answer).

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — clipping regions read through transforms, exact under translate/scale, rotation as bounding box, non-finite edge `None`; ratified as an engine change for every document — a layout answer naming a transform or scroll must agree with this reading and not change the engine (decision/visible-region-through-transforms).

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — a subtree under a determinant-0 or non-finite transform takes no hit; an explanation of a transformed box must handle the non-invertible case without panic (decision/non-invertible-transform-no-hit).

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's readers and its close add no panic) — list readers go through `get_node` and call no `layout_style()`, so they add no panic path; trap for a reader of computed style/layout on nodes with no styles (decision/skip-snapshot-of-unstyled-node, stale-node-ids-never-panic).

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — an open list's row crosses only as its own `bounds` in snapshot/diff and as pixels, no `expanded` key — a node placed by an engine overlay rather than Taffy layout; the layout answer must not add a list field (decision/select-list-crossing).

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — a list row's `visible_region` is the list's scrollport and its scroll is the list's own `scroll_y`, apart from the page's boxes; a placement explanation must not attribute these rows to Taffy.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears each child's `layout_parent`, curing stale layout-parent readers; a walk up the layout parent chain for the containing block rests on it (decision/dropped-anonymous-block-clears-parent).

@history_item n=46 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (unknown alignment flags) — in `stylo_taffy`, an unknown alignment flag maps to `AlignContent::NORMAL` or none rather than panicking; reading the parent's alignment inputs from the converted Taffy style must keep that no-panic mapping.

@history_item n=47 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (dependency security: a second `unsafe`) — `stylo_taffy`'s public `resolve_calc_value` reads a calc value through a raw pointer; a read of computed sizing that resolves calc lengths touches this unsafe path.

@history_item n=48 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Dependency Security: 3 rows) — the Taffy-only git exception to the no-git rule; the layout read sits on the Taffy taken under it.

@history_item n=49 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Dependency Security: 4 rows) — Taffy is the git pin `e90751fd`, integrity resting on the rev; the read of `Node::final_layout` is against this pin, and «Taffy past #1372» will move it again (decision/engine-dependency-pins, no-git-dependency).

@history_item n=50 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads: descriptions to the adapter) — descriptions cross only to the accessibility adapter; no driver/MCP carrier, verb or field carries one — a layout answer must not carry them (decision/descriptions-adapter-only).

@history_item n=51 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph joins what the carriers take out) — the `graph` precedent: a new content class added to every carried set (socket, MCP, capability statement) as a ratified widening (decision/socket-carries-screen-reading).

@history_item n=52 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the thirteenth verb, graph, in the schema, MCP, the settle and the refusals) — a no-argument read verb: argument refused `malformed`, no cause added, runs no step, waits on no load (contract/driver-verbs/graph, contract/refusal-causes/malformed).

@history_item n=53 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's size under the answer bound) — unpaged answer, a part argument judged a widening; size held under 1,048,576 bytes by a Cells check, limit owned by `res-7cc5bi4k` — a whole-screen layout answer would face the same (decision/graph-answer-unpaged).

@history_item n=54 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a recorded graph answer at rest) — a new verb's answer becomes kept-check content at rest, recorded as a ratified widening in Data Protection (decision/kept-check-call-content).

@history_item n=55 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — a closed `bool` added to `NodeState` as no widening under the 2026-10-10 rule; precedent for closed layout-mode words as non-widening fields.

@history_item n=56 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb on the validated surface) — `schema::GRAPH`, a new `FieldKind`, `to_json` alone writing, feeding the session's record of ids, span records `verb` alone; the full checklist for a new verb (decision/id-record-bounded-and-session-fed).

@history_item n=57 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the verb counts the checks read) — cold-agent `valid_step`/`VERBS`, the recording directory's instance-verb set and the kept check's `tool` set each count verbs; a new verb moves every one (contract/recorded-check/tool).

@history_item n=58 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — `{owner}///{key}` reserved, no served id holds `///`; the scope's open placement kind «placed by the app inside canvas X» names the owner by its stable id, reserved like this form (contract/stable-element-id/virtual-node-form).

@history_item n=59 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's silence in the logs) — `graph` records `verb` alone, two `info_span!` in escher-driver, `host_log`/`mcp_log` 0 needles — the logging bar a layout verb meets.

@history_item n=60 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer in the threat model's carried sets) — the `actions` precedent's crossing onto every carrier, ratified by the founder, a provisional field held apart — the shape for adding a layout answer to the carried sets.

@history_item n=61 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer on the socket and in kept checks) — answer under the 1,048,576-byte bound, kept whole in checks, its replay owed until tested (decision/kept-check-answer-whole).

@history_item n=62 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the verb table at fourteen, ten instance verbs) — the current counts a new verb moves: `VERBS` 14, instance verbs 10, MCP tools 14, cold-agent `VERBS`; the read waits on no load (contract/actions-answer/verbs).

@history_item n=63 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged actions answer's size) — Cells' answer measured under the bound, the size a stated limit in `docs/capabilities.md` owned by `res-7cc5bi4k` (decision/actions-answer-unpaged).

@history_item n=64 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's validation and its answer lines) — argument refused `malformed` in process, CLI (status 1) and MCP (`isError`), a new `FieldKind` written by `json.rs` alone, a provisional field held apart from the ratified verb (contract/driver-verbs/actions).

@history_item n=65 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the shared target check and the actions read) — one `check_target` shared by id-taking arms and `Session::actions`, the read changing nothing; a layout verb taking an `id` should resolve and refuse through the same check (decision/actions-read-shares-target-check, actions-read-changes-nothing).

@history_item n=66 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's silence in the logs) — the read opens no span of its own, the reader holds no log site, `host_log`/`mcp_log` drive it with 0 needles.

@history_item n=67 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, a kept check named on the command line) — the coverage role reads kept checks as untrusted through `Check::from_bytes` and replays them; a kept check holding a new verb's step must replay under it (decision/coverage-check-files-untrusted, coverage-refuses-bad-checks-malformed, contract/cli-roles/coverage).

@history_item n=68 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay now exercised) — the owed replay of a new verb's kept step was discharged by a `stand_replay` test; a layout verb's kept step owes the same.

@history_item n=69 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's content and its silence in the logs) — the coverage line holds closed content only and adds no span; exit codes 0/1/2/3 as the role's endings (contract/escher-session-exit-codes/*).

@history_item n=70 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the coverage role and its line) — the coverage role's crossing and line moved from PROVISIONAL to ratified by the founder; they now stand as settled rows.

@history_item n=71 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Input Validation and Error Handling rows) — added decision/fixture-name-closed-table and decision/reach-error-closed-set; a stand fixture for a flex, grid, positioned or scrolled case, if added as a named state, enters this closed table and its fixed reach errors.

@history_item n=72 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Input Validation: the kept check's start record gains `format`) — a kept check opens with `format`, any other value refused malformed; a new verb's step is read by this strict reader (contract/recorded-check/start-record).
