@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired `escher_telemetry::init`'s scrub and fixed the eleven content-named redacted fields (`url` … `panic.payload`). This is the set the logging rule leans on: a new `escher_` span field outside those eleven prints as written, so no graph field may carry a node, component name, handler, state or value.
@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — added the stable-element-id reader: an author key only when non-empty, `/`-free and first in pre-order, else a `/`-bearing path, "so no `id` makes two ids equal", with no panic and no engine id or pointer. This is the guarantee the reserved virtual-node form (`{canvas element}` + `{virtual key}`) must keep.
@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — a keyed list row's segment carries its Dioxus key `{tag}[{key}]`, and a key never comes from a pointer, hash or process counter. This is part of the id grammar a new virtual-key separator must not be able to collide with, and the precedent that a new key value class inside the grammar is no widening.
@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the id leaves as AccessKit `author_id`, with a CARRY to assert that it never carries a `NodeId`/`ElementId`/pointer. Standing rule: any further consumer of the id beyond the platform API re-opens the row. Keying the graph on the accesskit node makes the graph a new consumer.
@history_item n=5 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot became an in-process reader of ids and names, resolved through `get_node` and never by index. Its trap: a first wire form of snapshot data is a crossing for ids, names and values. That is the shape the graph read and its new snapshot field rest on.
@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — set the four tiers (author key, anchored `//`, component path, document path) and the three-way uniqueness argument the virtual-node form must extend. It also scoped "no `NodeId`" to the id and the snapshot, since `UnkeyedActionable` holds a `NodeId`. Its trap: a wire form must leave `node` behind, which bears directly on moving off `Target.node: NodeId`.
@history_item n=7 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — introduced `MASKED_VALUE` for a password's snapshot value: never the text or its length. Its trap: the mask covers the snapshot only, and a file input's host path reads unmasked through the same reader. Any value the graph carries must pass the same mask.
@history_item n=8 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the mask to `type=file` (compared ASCII-case-insensitively; `""` / `None` cases), and Data Protection now states that the file's host path never reaches the snapshot or its text. Standing rule: the first command exposing snapshot data is a new crossing. This binds the graph's state and control values.
@history_item n=9 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — a diff takes its values from the snapshots only, so masked values stay `MASKED_VALUE` in a diff entry, with no `NodeId`/`ElementId`/`ScopeId` in `DiffNode`. The new reacts-to-a-click field rides the diff under this construction.
@history_item n=10 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — a Dioxus key that is empty or holds `/` is refused, falling back to the positional `{tag}:{n}` segment, and a unit test witnesses each refusal. The virtual-node form's separator must hold against this segment grammar, and needs its own named witnesses in the same way.
@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (socket vector; host-log reading) — created the owner-only socket (`0700` dir, `0600` socket, no handshake). Standing rule: a command carrying screen content reopens the crossing question. Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously, so the graph verb's log check must run on a sink-installing host.
@history_item n=12 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink's three outcomes: engine targets print safe fields only, `escher_` targets print with the content-named fields redacted, and any other target is dropped whole at every level. `host_log` is the standing check against a sink-installing host. This is the rule a graph call's host-log read must pass.
@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — created `validate` over a closed table: typed `Command` or `Refusal` in a fixed order, no `unwrap`/`expect`/panic/index on call input, no `Session` in its signature, and a refusal holding no `String` and no call content. The graph verb enters this table. The rule that no verb takes a wait argument or a wait result field starts here.
@history_item n=14 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` validates first, and nothing public runs a `Command` that did not come from `validate`. Ids are looked up anew on every call through `get_node`, never by index. The graph read must take the same path and keep no `NodeId` between calls.
@history_item n=15 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — a sixth verb (`scroll`) widening the validated surface was ratified by the founder. This is the precedent that a new verb is a founder-ratified widening. It also added the session's private, bounded record of id text, never logged, and detected causes after `validate` in a fixed order.
@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fields) — one `command` span per call, holding only fixed words, causes and counts. Arguments, screen text, the record of ids and the outcome are never fielded. A graph verb may add no field carrying graph content without a new crossing question.
@history_item n=17 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the sink's second record class) — span lines print every field outside the content-named set, and a non-content span field's value is held in memory until the span closes. Any span field the graph adds would therefore print.
@history_item n=18 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the span record class and the diff's sizes in a log. Standing rule: a count of a diff's nodes may reach a log; no node, id, name or value may, and a field holding one is a new crossing question. This applies directly to graph counts against graph content.
@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (socket carries calls) — the socket's trust boundary was widened to carry calls and answers (ids, names, masked values, snapshot text, diffs; no `NodeId`), with wire grammar, 16,384-byte request and 1,048,576-byte answer bounds and `oversize`. The graph answer must fit inside this ratified crossing and its bounds.
@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (command line) — `escher_driver::command_line` is a closed reader: the call is checked by `validate`/`validate_session` before the session address is asked for, with four fixed exit statuses and nothing typed written back. Standing rule: every later surface's input passes `validate` first. The graph verb's CLI form reuses this path.
@history_item n=21 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names, values leave) — ids, names and masked values leave in an answer, and `Refusal::to_json`/`SessionError` are written from fixed strings. Kept: no log, event or file carries an id, name, value, snapshot text or diff content. The graph answer extends this exit under the same rules.
@history_item n=22 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule) — the founder's widening rule: a closed-value field inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is. It put three closed state readings into the text/diff and the node keys. The reacts-to-a-click boolean is no widening under this rule; component, handler and state names are the founder's fork.
@history_item n=23 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (control-states row) — a closed in-process reader: values trimmed and matched exactly against a closed set, carrying no attribute text, with no log site or panic path even on a 5,500-character value, and exiting as closed enums/booleans. This is the pattern the reacts-to-a-click reading can follow.
@history_item n=24 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (MCP crossing) — the MCP server row: request and result bounds, fixed protocol errors with no `data`, the `id` echoed as the one client value, typed tool arguments refused `malformed` and then `validate`. Results reach the model provider through the client. The graph verb's MCP tool must go through this row; MCP was its own crossing question, and the next carrier is a new one.
@history_item n=25 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json`, `tool_call` span) — `serde_json` is featureless and read-only. A second span, `tool_call` (`tool`, `ending`), was ratified as a new log field. The graph tool reads its arguments through that reader and adds no span field without ratification.
@history_item n=26 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help`) — `help` became the tenth verb, a founder-ratified widening. The program's own fixed `'static` table text crossing the ratified carriers is inside them; text from a page, a call or a screen still widens. `differs` moved node keys from twelve to thirteen as a closed field. The graph verb's help, `describe()` and the `tools/list` descriptions are fixed text under this rule, and `NODE_FIELDS` moves again.
@history_item n=27 marker=2026-10-12-self-description
**2026-10-12-self-description** (session error table) — the `SESSION_ERRORS` table rows have fixed kind, meaning and remedy with no path. Any session error the graph verb adds joins this fixed table.
@history_item n=28 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (in-band pixels) — the eleventh verb, `screenshot`. Pixels are no closed value set, so it was a founder-ratified widening across all three carriers under the unchanged bounds. This is the direct precedent for an open-content verb's answer being the founder's fork.
@history_item n=29 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (causes, no image in a log) — a new cause joined the closed set as fixed text. A shot's content and size are never fielded, which needle checks on the host and the `mcp` role prove. This is the pattern for the graph's content staying off the log.
@history_item n=30 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter/Space activation dispatches a Click through closed element-name/type gates, and `disabled` is read by presence (ratified). The reacts-to-a-click reading must agree with these click paths and gates.
@history_item n=31 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the tree handed to the adapter is asserted headlessly: every `author_id` is its node's `element_id`, none empty, all-digit or `0x`. Keying the graph on the accesskit node rests on this proof that no `NodeId` form rides the agent-visible node.
@history_item n=32 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the audit is `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace, with per-ID ignores only. A registry or graph reader using dioxus-core's public API must leave this audit green with no new package or ignore.
@history_item n=33 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent check row: `valid_step` names `tool` by the verb count (then eleven, now twelve), at most 400 steps, closed keys. A graph verb must join that verb list or be stated outside it.
@history_item n=34 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`inert`) — `inert` is read by presence on an element or ancestor: such a node is not focusable, takes no hit, and its subtree is out of the adapter tree and the snapshot. The graph keyed on the agent-visible node, and the reacts-to-a-click reading, inherit this exclusion.
@history_item n=35 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls) — `ElementData::is_disabled` is presence on a form control only, read by focusability, `ClickTarget::Disabled`, `handle_click`'s walk gate and the activation arms, so a disabled ancestor `div` stops no walk. The reacts-to-a-click field's disabled reading must agree with this or name the fork.
@history_item n=36 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `time-unavailable` retired, so the cause set is closed at eight. A graph refusal must fall among these eight in the fixed order.
@history_item n=37 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (kept check, reader, `--record`) — the recording directory (`0700`/`0600`, bounds) and the strict `Check::from_bytes` (closed keys, `malformed`, no panic, replay through `call_of`/`validate`). A call no record can spell is not kept. A masked `type` is kept as `MASKED_STAND_IN`. The kept check became the id's fourth exit. A graph step is recorded, or stated unrecorded, under these rules.
@history_item n=38 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, session errors) — wire `v3` and `SESSION_ERRORS` at 15. `serde_json` also reads kept checks while writing nothing. The counts a graph verb's carriers and any new session error would move.
@history_item n=39 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is held whole, masked as on every carrier. The committed `counter-cli.json` / `counter-mcp.json` hold no typed text, host path, name or secret, are read only by `Check::from_bytes`, and are compared byte for byte with a fresh recording. The reacts-to-a-click field changes their snapshot text, so they must be re-recorded under these rules.
@history_item n=40 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (in-process reader) — `Session::lint` was first a PROVISIONAL in-process reader of the id that "crosses nothing", returning ids, closed words and numbers with no `NodeId`. This is the precedent for building a graph reader in process before its verb.
@history_item n=41 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the `lint` verb) — the twelfth verb and eighth instance verb, a founder-ratified widening onto every carrier at once: the socket, CLI, MCP `tools/list`, the cold-agent verb set, the recorder's instance verbs and the kept check. Its content is never fielded, and `host_log` / `mcp_log` drive it with 0 needles. This is the exact template for adding the graph verb.
@history_item n=42 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the seventh argument kind (`part`) moved the arguments count. A part's number, count and text are never fielded, with needle checks on both logs. This is the precedent for any argument or paging the graph verb takes, and for the counts the schema row states.
@history_item n=43 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `host_log` became a standing check reading a recording session's clients at `trace` with 0 id, name, sentinel and recording needles. The graph verb's no-log acceptance should extend this check.
@history_item n=44 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — reconciled the carried sets on every carrier, and `docs/capabilities.md` states the MCP crossing's carried set as a pinned statement. If graph content is ratified onto MCP, this statement and its sites must name it.
@history_item n=45 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the live child's kept checks) — the cold-agent pipe is a second reader of kept checks, holding bools and counts only, and its verdict fields are counts and closed words. A graph step's kept answer would sit at rest there and must stay unprinted.
@history_item n=46 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six slugs, flight booker) — the flight booker's `select#flight-type` writes `flight_type` by a closed match over its two options. Moving the stand's signals onto the registry must keep this and the 7GUIs validations.
@history_item n=47 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' `cells-grid` keys) — `cells-grid` takes `tabindex` 0 and a role-less `onkeydown` with a closed key match, clamped, and a click inside an open editor returns early. These are the role-less handlers the reacts-to-a-click reading (Cells 676) and the registry move touch.
@history_item n=48 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses) — an open row's `bounds` is a closed field inside the ratified crossing, so no widening. `NodeState` holds no field for it, `NODE_FIELDS` stays thirteen, and no `expanded` node key stands (the operator's call). This is the latest precedent and count the reacts-to-a-click field moves.
@history_item n=49 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list in the log) — `packages/escher-driver/src` holds two `info_span!` and `stand_act_spans` reads eight span fields; page text reaches no log under host or role. The graph verb must keep these counts or re-pin them.
@history_item n=50 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (mounted event of a removed element) — `flush_queued_mounted_events` skips a removed element through the option-returning `try_element_to_node_id`. The graph reader and the registry resolve elements the same way, so a stale element is skipped and never panics. The sync was named for this chunk.
@history_item n=51 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (MountedData hazard) — re-measured the `RefCell already borrowed` panic from a task polled inside `render_immediate`, and pinned it. This is the probe chunk whose measured mechanisms (vdom walk, `subscribers()`) the scope rests on, and whose `dioxus_runtime_probe` link 1 reads.
@history_item n=52 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (zip ignores, nine) — the ignore count is nine, by ID with written reasons. The audit must stay at nine with no ignore added by the registry or graph work.
@history_item n=53 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories after a green audit) — standing rule: CI's audit can go red on new advisories over an unchanged lock, so run `bash .github/scripts/ci-leg.sh audit` locally before a push.
@history_item n=54 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (buffered render) — `initial_build` / `poll` render into `MutationBuffer` unborrowed, then replay and flush. A registry hook or graph read inside `dioxus-native-dom` runs on this render path and must not reintroduce a borrow across `render_immediate`.
@history_item n=55 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (recorder silence) — `MutationBuffer` has no panic path and no log site, print or env read, and its `Debug` prints `len` alone. This is the latest precedent in `dioxus-native-dom` for a new in-process holder of page content: the registry and the graph reader follow the same silence.
@history_item n=56 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (limit for a task polled during render) — a task polled during render acts on the previous render's document. This is an open residual: a registry write recorded in such a task, or a reader resolving a node created in the same render, meets this limit.
