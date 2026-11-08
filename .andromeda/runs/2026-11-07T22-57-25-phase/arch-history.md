@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — registered `DioxusDocument::element_id` / `element_ids`, computed on demand from a vdom walk joined with a DOM walk and written to neither DOM nor vdom. The grammar: author key `/`-free; component path segments are the last `::` segment of `VComponent.name` with generics stripped, with dioxus-core's four framework scopes adding none; document path `/{tag}:{n}…`; no `NodeId`, `ElementId`, `ScopeId` or pointer in an id. The plan's assumptions (the base scope is `root`, `VComponent.name` is a bare name) were corrected by measurement. This is the grammar the reserved virtual-node separator must not collide with, and the walk that link 1 reuses and that names components.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the fourth tier, the anchored path `{key}//{segment}` (the only id holding `//`, acting inside one component only), and registered `unkeyed_actionable` (focusable · interactive role · `data-dioxus-id`). The founder ruled out a bare `key/segment` and a leading `#`. Trap it set: `UnkeyedActionable.node` is a process-local `NodeId`, so no wire form may carry it. Two rules follow for this chunk. The virtual-node separator must avoid `/`, `//` and the spellings the founder rejected, and must leave the `/` test unchanged. The graph must apply the same no-`NodeId` rule to `Target.node`.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the `Document::accessibility_tree` default, with `DioxusDocument` alone overriding it to set each element node's AccessKit `author_id` to its stable id (none on `TextRun`, root or `Window`). dioxus-native-dom's `accessibility` = `["blitz-dom/accessibility", "dep:accesskit"]`; seven_guis does not name it. This is the agent-visible node the graph keys on. Trap recorded: a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` (check `cargo tree -e features -i`). That matters for the `accessibility`-gated graph reader and for a seven_guis registry reached without that feature.

@history_item n=4 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — registered `DioxusDocument::snapshot` under `accessibility`: `SnapshotNode { id, role, name, state, bounds, children }`, where a node is every element the accessibility tree keeps, resolved through `get_node`, and no field holds a `NodeId`, tree id, `ElementId`, `ScopeId` or pointer. Standing rule: the snapshot derives no role or name of its own, so it cannot disagree with the tree. The «reacts to a click» field joins this model, and the graph keyed on the same node inherits the no-`NodeId` and no-second-derivation rules.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the password `value` reads `MASKED_VALUE` (one fixed marker, the operator's answer), never the text or its length. Weighed and ruled out: a fifth `NodeState` field or a changed `value` type to mark a masked value. Precedent for this chunk on two counts. Masking of any value the graph carries goes through this one form. Adding a `NodeState` field has been weighed against the model before.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text`: fixed state-token order, `SNAPSHOT_TEXT_BUDGET` 10,000, the lean screens' sizes measured. The mask was extended to a file `input`. Standing rule: each later surface asks again when the text first leaves the process. A «reacts to a click» token, if the text form carries one, moves this token order and the pinned lean-task sizes.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — `Snapshot::diff` → `SnapshotDiff { added, removed, changed }`, not derived from the engine-touched changed set (a superset of snapshot-changed). The click field's diff reading must come from the snapshots, not from the changed set.

@history_item n=8 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the one `'static` schema (`VERBS`, `VerbSpec`/`ArgSpec`/`FieldSpec`, `validate`, `Cause` closed at eight, `Fault`, none holding a `String`). Rules set for later chunks: a later entry adds its verbs to `VERBS`, never a second table; `Command`, `Call` and `ArgValue` print ids and typed text under `Debug`, so nothing fields them. The graph verb follows this path.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` is the one entry point: `validate` first, and nothing public runs a `Command` that did not come from `validate`. Nothing resolved (no id, no `NodeId`) outlives a call, and nothing fields an `Outcome`, whose `Debug` prints ids, names and values. Standing rule: the first command carrying a call or an outcome out of the process is a crossing question. A graph `Outcome` falls under these rules. The graph must not let a `NodeId` outlive a call on the session.

@history_item n=10 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — a sixth verb (`scroll`), ratified by the founder as a widening, with `Command::Scroll` and `Outcome::Scrolled`. The session's record of ids, `SeenIds` (`MAX_SEEN_IDS` 4096), is fed from snapshots, and `stale` vs `not-found` is decided from it. Rule restated: a verb joins `VERBS`, never a second table. Two consequences here: a new verb has been a founder's widening every time, and whether a graph read feeds the record of ids must be decided.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span (target `escher_driver`, INFO, `info_span!`, no `#[instrument]`) with its eight fields. `snapshot` records `verb` alone. The never-fielded list covers the screen text or its length, any argument, and any `Command`/`Call`/`ArgValue`/`Outcome`/`Refusal`/`Fault`. The graph verb's span records `verb` (and `cause`) alone and fields no graph content.

@history_item n=12 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES` (`escher_`), drops every other target whole, and on an escher target prints every field not in `CONTENT_FIELDS`. The never-logged rule for component, handler and state names rests on adding no span field at all, since an unlisted field under `escher_driver` prints.

@history_item n=13 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the sink's closed-span line and the diff's three counts on the `command` span. Standing rule: a count of a diff's nodes may reach a log, nothing of its content may. This bounds what the graph read's span may hold.

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the socket carries a call and its answer (ids, names, masked values, snapshot text and diff out, no `NodeId`), ratified by the founder as a second widening; standing: «the next surface that carries them … asks again». Wire `v2`, answers bounded at 1,048,576 bytes. `VERBS` went to two levels. The written forms `Outcome::to_json` etc. are hand-written, keyed by the schema's words, output only. `command_line` reads `VERBS` with no second list. Graph content (component, handler and state names) going out on these ratified carriers is the same kind of crossing question.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the snapshot model and the driver's node) — the precedent for adding a state to a node. `NodeState` grew from four fields to seven. The text form's token order grew. The boot text and hosted line sizes were re-measured. `NODE_FIELDS` went nine → twelve, with a presence rule for each new key. The helps of `changed` and `snapshot`'s `text` were restated. «No help per node key» was the operator's answer at the time. The «reacts to a click» field moves the same set of sites.

@history_item n=16 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the builder's third direct engine change) — the builder writes `selected`/`toggled`/`invalid` in blitz-dom's `accessibility.rs`, ratified by the founder as an engine change for every document. Rule set: a later attribute read keeps this shape, a closed set with no text carried out. This is why the click reading must come from the vdom in dioxus-native-dom and not from a new builder write, and why the boolean is a closed value.

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP crossing ratified, with `tools/list` the `VERBS`. `serde_json` is a reader only (the founder's «serde_json, только чтение») and the written forms stay hand-written. The `tool_call` span holds `tool` and `ending` only (the founder's «Span с tool и ending»). Standing rules: no span field holds what a call supplied or a screen reads; the next surface that carries a call or a screen's content is a new crossing question. The graph's MCP tool and its content fall under all three.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** (the tables) — `NODE_FIELDS` became `[NodeFieldSpec; 13]` `{ name, help }` with `differs` after `bounds`. `Reading` (eleven) is re-exported, and `DiffNode.differs` is filled at one site. `describe()` writes the self-description from the tables alone. MCP tool descriptions carry their result fields. A new node key moves `NODE_FIELDS`, `Reading::ALL`, `differing` and `describe()`'s counts together.

@history_item n=19 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text ratified) — the founder ruled that the program's own fixed `'static` table text on the ratified carriers is no widening, while text from a page, a call or a screen, a new carrier or a new exit still widens, and `differs` crosses as a closed set (rule of 2026-10-10). This is the rule that sorts the click boolean (closed, inside) from component, handler and state names (open, a widening for the founder).

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a full precedent for a new instance verb. It added the `VERBS` row, `Command::Screenshot`, `Outcome::Shot`, a new `FieldKind` (ten), the hand-written written form, the MCP result shape and the `command` span recording `verb` alone. Re-exports and module counts moved, and escher-driver took its fifth dependency (`png`, already in the graph). The image in band was ratified by the founder as fork 1. The graph verb walks the same set of sites.

@history_item n=21 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node) — a re-render-removed node is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so it reads no id and `get_node` returns `None`. Link 1's vdom→DOM mapping and any graph state held across reads must treat a removed node as gone, never as detached and still resolving.

@history_item n=22 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation: Enter and a released Space dispatch a queued Click through `dispatch_activation_click`, so the app's handlers (a Dioxus `onclick`) run before `handle_click`. A click focuses a button. These are direct engine changes, ratified by the founder. The «reacts to a click» reading must agree for a pointer click and a keyboard activation.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the batch, ratified 2026-10-18) — `ElementData::is_disabled` (`can_be_disabled()` and `disabled` present) is the one reading shared by focusability, the pointer gates and Enter/Space. `inert` is honoured by presence: not focusable, no hit, out of the tree and the snapshot. The click reading must state how a disabled control with a handler reads through `is_disabled`, and an inert subtree carries no graph node or field.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `time-unavailable` retired, so `Cause` is closed at eight (`CAUSES: [Cause; 8]`), later ratified by the founder. A cause leaving the set was recorded as a narrowing. The graph read must refuse only with these eight, since a new cause is a widening.

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check: one record per answered call (`tool` · `arguments` · `masked` · `accepted` · `digest`), FNV-1a 64 `answer_digest`, bounds of 400 records / 1,048,576 bytes, a strict `Check::from_bytes` over the instance verbs, never logged. `is_masked_input` was made public as the one masked-control test. Ratified by the founder as a widening. The graph read's recorded step and its masking rest here, and a new instance verb moves the verbs the recording admits.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is kept whole (`answer`, the driver's line masked as on every carrier). `Check::replay` runs every step through `Session::run` and compares the live line read by `serde_json::from_str`. The two committed Counter checks (`counter-cli.json`, `counter-mcp.json`) are written by `record_checks.sh`, replayed by `stand_replay`, and kept fresh byte for byte by `recording` in `rc-fresh`. A new node key changes every acting answer, so these committed checks must be re-recorded or `stand_replay` goes red.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint, and its gated part) — first `Session::lint` in process, with the record of ids also fed from the snapshot it reads. Then the founder's F1 «Новая команда lint» made `lint` the twelfth verb (eighth instance verb) on every carrier and recorded: `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` form, an MCP tool, `command` span `verb` alone, `describe` 12 verbs / 8 causes, the cold-agent `isolated` twelve tools. Its findings crossing was ratified as a widening, with no new carrier, cause, field or dependency. This is the latest precedent the graph verb follows, and the decision whether a non-acting read feeds the record of ids.

@history_item n=28 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the five acting verbs settle through the load wait, while `snapshot` and `screenshot` (and later `lint`) wait on nothing. `session.rs` is the one clock reader. A graph read runs no step, so it waits on nothing and reads no clock.

@history_item n=29 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a screen over `SNAPSHOT_TEXT_BUDGET` is read in parts (`part` argument, `Outcome::Part`, `MAX_PART` 1000), ratified by the founder as a widening with no new verb, carrier or cause. Cells (676 data cells) passes the budget, so a whole-screen graph of Cells must fit the 1,048,576-byte answer bound or be read in parts. This is the precedent and the constraint for bounding the graph answer.

@history_item n=30 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and the capability-statement pin (`CapabilityStatementTest`). The pin reads verb names from `VERBS`, `NODE_FIELDS`'s length and the other counts, each count in words, plus README's «the {word} driver commands as tools» (README «twelve»). A new verb and a new node key each move this pin, the statement and the README together.

@history_item n=31 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand serves Cells (`boot_cells`, no `LeanTask`) and the load page (`boot_load`, `LoadNet`), and `escher-session` serves six tasks. The cold-agent `passed` needs `lint`, `screenshot`, `advance` and a part read (else `capability-unused`) over the isolated `mcp__escher__{verb}` set. `session_common` has 31 readers. The stand's signals moving onto the registry span these six boots, and a new MCP tool moves the pipe's isolated tool set. Whether the graph tool joins `capability-unused` is the plan's to decide.

@history_item n=32 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — `stand_non_lean_tasks` (3 tests) is registered, and `unkeyed_actionable` is empty on every stand task. The chunk re-pins this check's «react and state no role» reading.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid, its keys and its count) — the count of elements that react and state no role went 676 → 677, «react to input»: the 676 data cells plus `cells-grid` with its `onkeydown`, one `onmounted` per data cell. 677 was measured and disproved the forecast of 676. The scope's Cells 676 must be reconciled with this 677 before the click field's count is pinned.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the MountedData hazard of a spawned task) — a key-handler-spawned task that writes a signal and calls `MountedData` panicked (`RefCell already borrowed`), so Cells calls its handles synchronously. This is the hazard behind the borrow-order rule that the registry's write-time record and the graph's vdom walk must respect.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers) — the operator chose not to add an `expanded` node key: `NodeState` gains no field for the list, and `NODE_FIELDS` keeps its thirteen. The founder ratified the shot's page text as a widening. This is the latest precedent of a node key weighed and declined. The click field is the first to move the thirteen since then.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's checks and counts) — sixteen `stand_act_*` checks, `session_common` 32 declarers, `fixtures` 31, and `stand-repeat`'s named targets outside the prefix are seventeen. A new graph check moves these counts and, if outside the `stand_` prefix, must join `stand-repeat`'s literal list.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the public select readers) — `closed_select_of` and `is_multiple_select` are public and re-exported from dioxus-native-dom because escher-driver has no blitz-dom dependency, with `is_masked_input` as the precedent. This is the precedent for how a registry or graph reader in dioxus-native-dom is re-exported for the driver and the stand.

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (Cells' screen bytes) — Cells' screen is 47,089 bytes (was 47,949, stale since an earlier chunk), 740 lines, 5 parts. Standing trap: a plan quoting a flagged-old reading did not re-read it. The chunk must re-measure Cells wherever the text form carries the click field.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the queued `mounted` event's guard) — `flush_queued_mounted_events` skips an element removed since its event was queued, read through `try_element_to_node_id`. Upstream's 0.8 test was left out (it reads 1 mounted event on Dioxus 0.7). Link 2 reports `mounted` listeners exactly against this bridge behaviour.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component and a held signal's readers) — link 1 reads through public dioxus-core 0.7.10 API (`base_scope` → `try_root_node` → `template.roots` / `dynamic_nodes` → `mounted_root` → `mounted_scope`, named by `VComponent.name`), and a deleted CRUD row resolves to none. Link 4 is `Readable::subscribers()` inside `VirtualDom::in_runtime` → `ReactiveContext::origin_scope`. Hook-local `use_signal` state is unreachable without a fork (`Scope.hooks` `pub(crate)`). These are the measured mechanisms the graph reader and the signal registry are built on, with no fork.

@history_item n=41 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the bridge's listener attributes over-report) — the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `mounted_dynamic_attribute`) read current listeners exactly, `mounted` included. The bridge's event attribute and `data-dioxus-id` mark stay after a re-render removes a listener, and `mounted` writes none, so `unkeyed_actionable` over-includes (the conservative side). Link 2 must read the vdom walk (the founder's fork 2 A), never the attributes.

@history_item n=42 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the `dioxus_runtime_probe` target) — 10 tests over Counter, CRUD and Cells, reading public API alone. It has no `stand_` prefix, so it sits outside `run stand`, `stand-repeat` and `a11y`. It stays the witness of links 1–4. The chunk's graph checks extend or rest on it without moving it into the stand legs.

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render recorded as a contract, and the hazard closed) — `DioxusDocument.buffered_mutations: MutationBuffer`. `initial_build` and `poll` render with the `RefCell` unborrowed, then borrow once and replay through one `MutationWriter`. This is a ratified direct engine change for every Dioxus document, not upstreamable. Its merge surface (`dioxus_document.rs`, `mutation_buffer.rs`, `lib.rs`) is owned by `res-3v42legm`, absorbed into `v030-03`. Its witnesses are the probe's two `…_spawned_acts_*` tests. The registry and graph reader land beside it and must keep this render order, its witnesses and its merge surface untouched.

@history_item n=44 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the limit and dioxus-core 0.7.10's first-poll paths) — a task polled during render sees the document as the previous render left it (a node created in the same render is not yet in it). On 0.7.10 only an event-handler task that also dirties a scope is first polled inside `render_immediate`'s `Work::PollTask` arm. This is an open residual. Link 3's write-time record at the registry wrapper, made from a handler, runs inside this scheduling.

@history_item n=45 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the take read at `f9cbb2756`) — git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, owned by a residual. Stylo stays "0.22.0", Taffy "0.15" and Dioxus "0.7.3", with no git dependency. The registry and graph must be built on Dioxus 0.7.3 with no fork and no version move.

@history_item n=46 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the `stand-repeat` leg was added: three rounds of every `stand_*.rs` target plus named targets, zero-failure, no retry, on the founder's «Повторное задание, ноль сбоев». The chunk's re-pinned stand checks run here, and any new target outside the prefix must be named in its literal list.

@history_item n=47 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one target per file) — every file directly under `tests/` is its own target, and `mod common;` / `mod session_common;` resolve only that way (the operator's choice over upstream's single binary). New graph checks are new files with their own targets.

@history_item n=48 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — escher-driver's `mcp` has private children (framing, listing, result) and `schema` keeps its shape types in child `spec`, with out-of-line `#[path]` test modules. A new verb's `VerbSpec`, its listing row and its MCP result land in these child modules.

@history_item n=49 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the two table-asserting tests' row helpers) — the MCP listing test keeps its outer assertions and row walk in the test function, with private `#[track_caller]` row helpers that name the row and index only. The graph tool's listing row extends this test in that shape.
