@history_item n=1 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — stated the driver's `disabled` cause on the snapshot's presence reading and that the driver's schema defines no role or name set of its own (`role` and `name` are only field names of its result nodes), the source of decision/driver-schema-has-no-role-set; the split writer must not grow a role/name mapping or reword the cause while moving the refusal writer (`impl Refusal`).

@history_item n=2 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `disabled` detected only on the snapshot's `enabled` = `Some(false)`, first screen-level check, nothing dispatched (contract/driver-refusal/disabled, contract/driver-actions/disabled); the refusal's written form moves with `impl Refusal` and must keep its bytes.

@history_item n=3 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the diff is one more reader of the snapshot model, text change reported on the named element, never a text-run entry (contract/snapshot-diff/entries); the diff family (`acted`, `node`, `nodes`) the chunk splits out writes this form and must not re-derive role or name.

@history_item n=4 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set and a fresh boot holds a non-empty set (decision/harness-settle-leaves-changed-set); byte-for-byte witnesses that read a step's diff (`stand_replay`, Counter checks) rest on this, so no move may add a drain or reorder settle and diff.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the obs sink's second record class, one line per closed span of an admitted target; the scope's "no log field or span added, removed or reordered" boundary binds the split modules too.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result reads the snapshot model and nothing else; a `snapshot` block equals `Session::run`'s screen text byte for byte in both layout modes (contract/mcp-result/snapshot); one of the witnesses that would read a moved byte.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — `DiffNode::differs` names readings in node-key order and the node keys' help serves each reading in closed words (contract/snapshot-diff/differs); `help`, `NODE_FIELDS` and `differs` order are unchanged-by-scope surfaces the move must keep.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` reads only bounds, an element shot rounded outward and cropped, and changes nothing (contract/driver-screenshot/element-shot, decision/screenshot-changes-nothing); `shot` / `shot_without_image` (called from `describe.rs`) are among the unnamed forms whose landing P4 settles, re-exported if moved.

@history_item n=9 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (Enter and Space entry) the Book step's diff names seven bounds-only nodes and `flight-book` `[focused, bounds]`; a pinned diff answer the stand checks compare and the split must leave byte-identical.

@history_item n=10 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part) the lint carried as the `lint` verb on every carrier; a `contrast` finding is written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only (contract/lint-contrast-finding/*, contract/visual-lint-finding/content); the lint family (`finding`, `bounds`, `ratio`) is an unnamed form the split relocates, its field order and number forms kept.

@history_item n=11 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a screen over `SNAPSHOT_TEXT_BUDGET` read in parts; an MCP `snapshot` result is `to_text`'s output for a whole screen and the JSON line `{text, part, parts}` for a paged one, walk blocks equal to `Session::run`'s JSON lines (contract/mcp-result/snapshot, contract/snapshot-text/parts); the snapshot family's paged line moves with the split, bytes unchanged.

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the flight booker's select) a driver `click` on a flight-type option is refused `covered` (contract/driver-click-refusals/covered); a refusal answer the writer emits and the replayed checks pin.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the driver on a `multiple` select) `type` focuses with no click and the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors and exits gain nothing; the same closed-set guarantee the scope restates and decision/driver-type-focuses-select-no-click carries.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the select list as engine overlay (decision/single-select-engine-overlay-list), the changed set marking a list's open/close (decision/changed-set-marks), a click under the list refused `covered`, `type` on any select closing the list, and the snapshot's open-list readings: an open row's own `bounds` in snapshot and diff with `differs` naming `bounds`, no `expanded` node key (`NODE_FIELDS` then thirteen), the highlighted row in no snapshot or tree (contract/snapshot/select-list-rows, open-list-row-bounds, open-list-highlighted-row); these are written forms of the snapshot and diff families and must not change.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `NodeState.clickable` added: text token after `focused` only when true, the JSON node always carrying it beside `focused`, `Reading::Clickable` in the diff, `NODE_FIELDS` 13 → 14 (contract/snapshot-node/clickable, contract/snapshot-text/clickable); the read-only UI graph, one node per snapshot node, names and closed words only, never a value (decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values, contract/node-id/reserved-separator); and its citation re-point named escher-driver's `json.rs` among moved files: the JSON node writer and `graph_node` / `graph_state` / `write_readings` are what the split moves, and the plan's `json.rs` citations will need re-pointing again.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb as read-only reader writing ids, owner ids, verb and cause words and booleans only (contract/actions-answer/causes, fields; decision/actions-verb-changes-nothing), `actionable_nodes` with `id`, `owner`, `keyed` (contract/actionable-nodes/listing), the `keyed` wire form (decision/keyed-wire-form), the Cells `actions` answer pinned at 106,088 bytes (a byte witness of the split), and a citation re-point into escher-driver's `json.rs`; `action_node` is an unnamed form the split relocates.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `handler_coverage` answer holds ids, owner ids, event names and two counts (`checks`, `held`) only, no role or name (contract/handler-coverage/answer); `impl Coverage` in `json.rs` is an unnamed form whose landing P4 settles, its bytes kept.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code; the picked rows' citations and handles derive from it, so the split's moved `json.rs` lines are read against these rows, not prose.

@history_item n=19 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's ratification; the `keyed` field on the carriers is now standing, so its written form is frozen for this no-behaviour-change chunk.

@history_item n=20 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (2. A11y Strategy) `~ decision/changed-set-marks` took the select-list qualifier, a scroll that closes a list marking its select; the current wording of a picked row the chunk rests on, untouched by the split.
