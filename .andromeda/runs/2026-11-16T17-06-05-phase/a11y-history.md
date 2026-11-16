@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — measured the obs log format for escher's own sink: one text line per event on stderr behind the allowlist scrub, printing only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` for `blitz*` and `accesskit_xplat` targets and redacting `text`, `value`, `html`, `attrs`. This is the scrub by field name in `packages/escher-telemetry/src/format.rs` that the chunk reuses on every `caused` record. A record from an engine target crosses with its safe fields alone, so the chunk's rule (a hidden field shown by name with the marker, the rest of the message still crossing) has to be settled against this one.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — added the sink's third outcome: a record from any target outside `blitz*`, `accesskit_xplat` and `escher_*` is dropped whole at every level, and `escher_*` targets redact the four content fields. The chunk wants app, framework and engine records with their target and fields in the answer. The answer must widen what it captures without moving this drop on the log sink's own lines, which the scope's boundary keeps unchanged. A future a11y violation record also reaches the log only under an engine or `escher_` target.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — restated the sink as two record classes: event lines, and one line per closed span of an admitted target, every pair judged by the same allowlist. It also added `tracing` as escher-driver's third dependency, with no feature. The scope keeps the `command` and `tool_call` spans' fixed fields, and error text must reach no span field. This entry is where that span class and its scrub are stated.

@history_item n=4 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the sink's second record class (closed-span lines) now reads ratified by the founder in §3's note. The span line format the chunk must leave untouched is ratified, not provisional.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `value` from the element, and a password input reads `MASKED_VALUE` (contract/snapshot/password-value). The scope says the ratified mask is unmoved and a password's value never crosses. Error, log or panic text carrying a typed password (for example an `Err` that quotes the input's value) must not bypass this mask through `caused`.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the mask to a file input: it reads the marker where its `value` is non-empty, its path in no snapshot field (contract/snapshot/file-input-value). It also put `Snapshot::to_text` under the `accessibility` feature as a reader of the model. A file path that turns up in an error message or log field is the same secret, and the chunk's scrub must keep it out of the answer.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — measured a password typed through MCP `type` reading `MASKED_VALUE` in the tool result, the sentinel 0 times, in both layout modes. It also added the node keys' help as a reader of the snapshot model, closed words only. This is the precedent for the chunk's MCP acceptance (a sentinel count of 0 over the whole result) and for `describe()`'s help naming new fields in closed words.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password's text is in no painted frame either (decision/password-never-revealed, ratified fork 2). The screenshot is one more reader of the model's bounds and changes nothing. This completes the "never revealed" rule that the error text must not break on any carrier.

@history_item n=9 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the MCP masked-password input moved to the shared page `fixtures::password_named` in `tests/blitz-tests/tests/fixtures/mod.rs`. This is the ready fixture for the chunk's acceptance that a password value never crosses in `caused`.

@history_item n=10 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the workspace takes the Dioxus crates with `default-features = false`, so a crate gets `accessibility` only by naming it (decision/dioxus-accessibility-feature-by-name). dioxus-native-dom's feature gates its tree override. If the chunk reads boundary errors in `packages/dioxus-native-dom/src/boundary.rs`, it must decide whether that reading is gated, and this entry is where the feature wiring is set.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — at that point escher-driver named no dioxus-native-dom feature, and the entry that first read the snapshot through a session was to decide the feature. This is the starting state that act-by-id later reversed.

@history_item n=12 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver now names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table of its own, so the feature reaches both seven_guis binaries (decision/driver-requires-accessibility). No platform adapter is built, and the lockfile was byte-identical. The chunk's new `caused` text in `packages/escher-driver/src/caused.rs` runs under this always-on feature. Any new dependency must be re-read against the stand graph's adapter probe (`accesskit_(xplat|winit)` 0 rows).

@history_item n=13 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — `DioxusDocument::snapshot` takes role, name and focus from the accessibility tree, with no second role or name mapping (decision/snapshot-model-single-source). The chunk's text joins acting answers beside the diff. It must add no role or name reading of its own, only error and log text.

@history_item n=14 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — `Snapshot::diff` is one more reader of the snapshot model, also gated by `accessibility`. An acting call's answer carries this diff (contract/mcp-result/snapshot). The `caused` text rides the same JSON line, beside the diff and apart from it.

@history_item n=15 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result reads the snapshot model and nothing else: a `snapshot` result is `to_text`'s output, and an acting call's is its JSON line with the diff (contract/mcp-result/snapshot). It also added escher-driver's `serde_json` dependency and re-read the stand graph. The chunk widens that acting JSON line with error text on the ratified MCP crossing. The contract row has to state the addition, and the 1,048,576-byte bound holds over it.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` is a further reader under `accessibility`, and an MCP `snapshot` result is the JSON line `{text, part, parts}` for a paged screen. A password reads `MASKED_VALUE` in every part. This is the precedent for bounding an answer by a budget, and the MCP contract row the chunk's carrier change sits beside.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave) — ratified that an option's label and a group's `label` leave an open list only as painted pixels and the snapshot's `name`, with no log site (decision/option-labels-leave-only-as-pixels). The chunk carries log records and their fields into the answer. It must not open a new route for page-authored labels through a captured log record or message.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` on a `multiple` select focuses it with no click, adding no verb, argument, key name, result field, cause, session error or exit to the driver's closed sets (decision/driver-type-focuses-select-no-click). The chunk adds result fields under `caused`. This row's list of closed sets is where that addition has to be squared, and the scope's boundary of no new verb, cause, session error or exit matches it.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — widened the `type` row to any select (`focus_select`, which also closes any list), still with no closed-set addition. This is the current form of the row named in item 18.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader) — put `DioxusDocument::graph` behind `accessibility`, with the `signal_registry` module not gated (decision/agent-readers-behind-accessibility). It answers names and closed words only, never a value. This is the gating line and the "never a value" precedent the chunk's text in `caused` breaks on purpose, inside the founder's answer.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — added `actionable_nodes` to the gated set, sharing the crate-private actionable rule with `unkeyed_actionable`. The gated readers' list is extended here; any reader the chunk adds to dioxus-native-dom (such as the boundary error's message) belongs on this list or beside the ungated ones.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — the `actions` answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId`, measured with a typed sentinel. This is the precedent for a sentinel acceptance on a driver answer. The chunk's `caused` text is the first answer field to carry free text beyond that rule.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — `unkeyed_actionable` and `actionable_nodes` read one rule, with the listener leg the vdom walk's exact current listeners (decision/actionable-key rule, guard `unkeyed_actionable`). This is relevant if the chunk's fixtures add failing handlers: each actionable element must carry an author key or the guard reports it.

@history_item n=24 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — set the actionable-key rule (founder) and its check `unkeyed_actionable`. Trap: never satisfy the check by adding a `tabindex` or a role; key the element. This applies to any button the chunk adds to a fixture or stand page to raise a panic or `Err`.

@history_item n=25 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record) — dioxus-native-dom's `reached` module (`ReachedHandler`, `record_reached`, `reached_handlers`, crate-private `element_listener_ids`) is ungated, written on `handle_event`'s path before `runtime.handle_event`, because the wasm build compiles dioxus-native-dom without `accessibility`. `caused`, built by 2026-11-16-errors-with-the-action by handler, rests on this record. Any new boundary or error reading on that path must likewise compile without the feature.

@history_item n=26 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (241), each verified against the code. The picked rows (driver-requires-accessibility, agent-readers-behind-accessibility, password-value-masked, contract/mcp-result/snapshot and the rest) took their current wording here. Any `~ {handle}` change this chunk owes, such as the MCP acting-result contract gaining error text, edits these rows.
