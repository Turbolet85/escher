@history_item n=1 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (type and press on a multiple select) — recorded that `press` carries no Ctrl, so a non-adjacent `multiple` select set cannot be built through the driver, and that `press`'s help states it ("a set of rows that are not adjacent needs Ctrl, which this verb does not carry"); this is the founder's P5 word that made it a limit and residual `res-hfvjj77l`. Debt 1 either retires this record and the help text, or keeps it.

@history_item n=2 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select's chosen set, keys and pointer) — modelled the `multiple` select: arrows/Home/End choose one row alone, Shift spans, Ctrl or Meta move the current row without choosing, Space flips the current row, Ctrl/Meta-click flips, Ctrl+A unmodelled. These are rows decision/multiple-select-row-keys, decision/multiple-select-row-clicks and decision/select-ctrl-a-unmodelled. A Ctrl on `press` reaches exactly these engine arms, and they decide what a Ctrl-press builds.

@history_item n=3 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the keyboard-types 0.8 surfaces) — the harness re-exports `Key`, `Modifiers` and `NamedKey`, and the driver's key types come through them. `keyboard_key` maps the named keys, and the macOS `ACTION_MODIFIER` reads `Modifiers::META`, `CONTROL` elsewhere. Rows decision/key-types-through-harness, decision/harness-reexports-key-types and decision/platform-action-modifier: a Ctrl/Meta modifier on `press` is built from these re-exports and takes no keyboard dependency of the driver's own.

@history_item n=4 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8) — moved the workspace to keyboard-types 0.8 while Dioxus carries its own 0.7, so two versions stand in the graph and a prelude-globbing module names the workspace crate as `::keyboard_types`. This is rows decision/keyboard-types-named-by-absolute-path and decision/workspace-keyboard-types-path, a trap for any new modifier code beside the Dioxus prelude.

@history_item n=5 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (`type` replaces; two engine limits said by the tool) — `type` presses the engine's select-all as `a` with Control, Super/Meta on macOS, behind a second macOS `cfg` gate in the executor. This is the existing precedent for the driver pressing a modifier. The same entry set the rule that a measured limit is stated where an agent reads the tool (decision/measured-limits-stated-in-the-tool), which is why `press`'s Ctrl limit and coverage's stop limit sit in help texts.

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built the one schema table: `press` takes `key` and an optional `shift`, `ArgKind` `Key`/`Flag`, the `Fault` set and the fixed validation order (verb, then arguments as passed, then the schema's order). Its rule that a later entry adds to `VERBS` and never to a second table governs how a Ctrl flag or a graph `part` joins the schema (contract/validation-order/first-failure, contract/argument-kinds/flag).

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (wire `v2`, the client's `call`) — the call line `call v2 <verb> [<name>=<kind>:<value>]…` with kinds `t`·`n`·`f`, validated before sending, under the 16,384-byte request bound. A new `press` flag or a graph `part` number rides these kinds on the socket (now `v3`, contract/session-wire/call-request) with no new wire form.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (nine verbs, the written forms, `command_line`) — the command line's flag spelling and reader. The three session verbs and a new argument kind were ratified by the founder as a widening of the validated verb set (decision/validated-verb-set-widened-by-ratification). This is the precedent that a new argument on a ratified verb is a ratification question, as the scope's Boundaries state for Ctrl.

@history_item n=9 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the direct precedent for `graph --part N`. It added `snapshot`'s optional `part` (`ArgKind::Part`, 1 to `MAX_PART` 1000), `Outcome::Part` with `part` and `parts` written only when paged, a part past the last answering empty text, and the command line's `--part`. Over MCP the paged answer is its JSON line with `part` an integer 1-1000 in the input schema, and the part's number is never fielded on a span. It was ratified as a boundary widening with no new carrier (decision/large-screen-in-parts). The founder's ruled form "graph --part N, paged as snapshot is" copies this shape.

@history_item n=10 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — set `SNAPSHOT_TEXT_BUDGET` (10,000 bytes), a ceiling callers compare against and `to_text` never enforces (decision/snapshot-text-budget). This is the budget `text_parts` cuts within and the measure a graph part's cut must choose against, beside the 1,048,576-byte answer bound the scope names as the acceptance.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the `graph` verb in the Driver session contract) — `graph` became the ninth instance verb with no argument, an argument refused `malformed` with nothing run, and result `nodes`/`states` (two `FieldKind`s). An accepted `graph` records `verb` alone, and a graph is never fielded. Rows contract/graph-verb/arguments, contract/graph-verb/argument, contract/driver-graph/argument and contract/driver-verbs/graph state "no argument". `graph --part N` amends each.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph and its Cells size) — `graph` answers the whole screen at once. Cells reads 293,869 bytes under the answer bound, and the remainder is the stated limit "An unpaged graph", owned by residual `res-7cc5bi4k`, the residual debt 2 discharges (contract/graph-verb/unpaged). It records the size history (547,685 → 522,692 → 293,869 bytes), the measurement a part's acceptance re-reads.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers) — the graph crosses socket, command line and MCP and rests in a kept check, holding names, closed flag words and booleans, never a value or `NodeId`. It added no new carrier, exit, cause, session error, span field or dependency (decision/ui-graph-crossing). A part argument must stay inside this crossing, as the founder's ruling said.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader) — `DioxusDocument::graph` returns one node per snapshot node in pre-order, with handlers, owners, readers and the write resolution and `unconfirmed` rules (contract/ui-graph/graph, contract/ui-graph/unconfirmed, contract/ui-graph/write-resolution). This is the order and shape a graph part would cut. Its `states` list sits apart from `nodes`, a question for how parts split it.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — `handle_event` sets the `SignalRegistry` dispatch around `runtime.handle_event` only where a registry stands. Each signal write is recorded against its dispatch, and hook-local `use_signal` stays unreachable (decision/dioxus-event-path-records-opt-in, decision/signal-write-recorded-against-dispatch, decision/graph-state-by-registration). Debt 3's stop reading would sit on this same dispatch path.

@history_item n=16 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (stand_graph) — `stand_graph` (6 tests) pins the verb's answer equal to `DioxusDocument::graph` and the whole Cells answer within the answer bound. `stand_mcp_tools` gained a `graph` tool call. These are the checks the scope names for a paged graph.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the log checks read the graph's names) — a `graph` call's `command` span records `verb` alone, and `host_log`/`mcp_log` read 0 graph needles at `trace`. A `part` argument must stay unfielded (contract/command-span/graph-call, contract/command-span/never-fielded).

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (thirteen verbs where a surface counts them) — the recording keeps a `graph` step with its answer whole, and the CLI, MCP and cold-agent surfaces count verbs. A changed graph argument shape changes what a kept graph step's `arguments` hold, and so its strict read (contract/kept-check/arguments).

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Cells screen in six parts) — the Cells snapshot reads 53,859 bytes walked in 6 parts. This is the precedent measurement of paging Cells by the snapshot budget, and the cells-not-a-lean-task row's reason.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged actions answer) — `actions` answers unpaged, Cells 106,088 bytes. "An unpaged actions answer" was widened into the same residual `res-7cc5bi4k` beside "An unpaged graph" (contract/actions-verb/unpaged). Discharging the graph half of that residual must state what becomes of the actions half.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer crosses; fourteen verbs) — the most recent precedent of a read verb that takes no argument added on every carrier and kept whole in a check, ratified as a boundary widening. It shares `graph`'s field name `nodes` but not its `FieldKind`.

@history_item n=22 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (verb table, kept-check reader, explored verbs) — the latest widening of a verb onto the carriers. It moved the counts of `VERBS`, `tools/list`, the kept-check strict reader (eleven instance verbs) and the explored-verbs rule, which selects by field kind and never by name. These are the rows (contract/kept-check-reader/strict-read, contract/mcp-server/tools-list) a schema change to `press` or `graph` must keep true.

@history_item n=23 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (kept-check opening record) — kept checks carry a `format` number `CHECK_FORMAT` (now 1), and a reader refuses any other value (contract/kept-check/opening-record). A new argument on `press` or `graph` in a kept check raises whether the format moves.

@history_item n=24 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — kept checks keep the calls as they crossed, the `press` key among them, at rest owner-only. This was ratified as a boundary widening (decision/recorded-session-checks, decision/recorded-session-kept-check). A Ctrl flag on `press` would be kept and replayed the same way.

@history_item n=25 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is whole at rest, replayed through `Session::run` alone, stopping at the first differing step. The two committed stand-Counter checks are compared byte for byte (decision/kept-check-replays-through-run, decision/committed-stand-checks). A changed `press`/`graph` schema must keep those committed checks fresh.

@history_item n=26 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` a reader only; the `tool_call` span) — MCP arguments are read by `serde_json` alone, and answers are hand-written. The `tool_call` span holds only `tool` and `ending`, and no span field holds what a call supplied (decision/serde-json-reader-only, decision/tool-call-span-tool-and-ending). This binds how a Ctrl flag and a graph `part` enter over MCP.

@history_item n=27 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span's fields, where any argument and any `Command`/`Call` are never fielded. This is the rule that keeps a new `press` modifier or graph part number off every span.

@history_item n=28 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cold-agent pipe over six apps) — a paged `snapshot` result's text is escaped inside its JSON line, so a reader decodes the line before matching. A paged graph answer over MCP raises the same reading for its parts.

@history_item n=29 marker=2026-10-12-self-description
**2026-10-12-self-description** — `describe()` and `help` write the self-description from the tables alone: verbs, node keys, key names and counts. Each tool's description carries its result fields (decision/one-self-description-for-cli-and-mcp, decision/fixed-program-text-on-carriers). A new argument or a dropped limit sentence changes the served help and the MCP tool listing.

@history_item n=30 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created `docs/capabilities.md` and its pin `CapabilityStatementTest`, which holds verb and cause names and five counts to the schema's tables (decision/capability-statement-matches-schema). The statement names the three debts' limits (coverage's stop at `docs/capabilities.md:279-281`), so each fix or keep restates it under the pin.

@history_item n=31 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — widened the capability-statement pin to also read the a11y leg's file count and stand-repeat's named-target count from the CI leg runner (decision/capability-statement). A new check target in either leg moves a pinned count.

@history_item n=32 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader and its stated limits) — `handler_coverage` reads the UI graph at boot and after each replayed step. It lists the three stated limits, one being that a bubbling dispatch counts every ancestor listener it passed with `stopPropagation` unread, owned by an open residual (`res-4dr3wt62`, debt 3) (contract/handler-coverage/bubbling). Debt 3 fixes or keeps exactly this limit.

@history_item n=33 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch) — `ReachedRecord` is written in `handle_event` before `runtime.handle_event`, taking each chain node whose current listeners hold the event, so a handler whose run removes its element still counts. It reads the ungated `element_listener_ids` and holds no borrow across render (decision/reached-record-before-dispatch, decision/reached-recorded-before-handlers). Reading a stop means reading after handlers run, against this "before" rule.

@history_item n=34 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's kept-check files on argv) — the `coverage` role reads named kept-check files through `Check::replay_each`, ratified as a boundary widening (decision/coverage-reads-named-checks, decision/replay-hook-after-each-step). A changed coverage reading leaves its line on this carrier.

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the escher-session coverage role) — the role's shape, its endings 0/1/2/3, one process with one fresh session per check, and no listener (contract/cli-coverage-role/*, decision/coverage-role-in-process). These are the coverage surfaces a stop-aware fix must keep.

@history_item n=36 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage) — `stand_handler_coverage` (8 tests), with the `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` tables. A stop-aware reached record would move these tables, the checks the scope names.

@history_item n=37 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — the founder's own word ratified both coverage widenings (the role's kept-check files and its shape and endings), replacing the delegate confirm. The coverage surfaces debt 3 touches are therefore ratified, not provisional.

@history_item n=38 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — added the public `take_step_reached` on the reached record, draining handlers reached since the last drain, read by an acting step's `caused` (its first reached handler) (contract/reached-record/take-step-reached). A change to what the reached record counts under a stop also moves `caused`'s first reached handler.

@history_item n=39 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the bridge's listener attributes over-report) — measured that the vdom walk reads current listeners exactly while the bridge's listener mark over-reports. That gives rows decision/listeners-read-from-vdom and decision/listener-mark-outlives-listener. Any stop-aware reading of listeners must read the vdom walk, not the mark.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component and a held signal's readers) — owning components and registered signals read through Dioxus 0.7.10 public API with no fork-level change, while hook-local state needs one (decision/owning-component-via-public-api, decision/graph-state-by-registration). Whether a `stopPropagation` is readable from public API faces the same constraint.

@history_item n=41 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (events queued on the document; the cancelled pointerup) — events reach dispatch through the document's queue, not a callback (decision/events-queued-on-document, retiring decision/dispatch-event-callback). The default-action order gained the exception that a cancelled pointerup still clicks. This is the current shape of `packages/blitz-dom/src/events/` where a stop is decided, measured just before this chunk.

@history_item n=42 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter and a released Space dispatch a queued Click so the app's handlers run before the default action (decision/keyboard-activation-enter, decision/keyboard-activation-space, decision/click-focuses-button, contract/event-dispatch/activation-click-order). These are the key paths a `press` with Ctrl travels and the dispatch order a stop reading sits in.

@history_item n=43 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — the eight ratified direct engine changes for every document, each flagged upstreamable with nothing sent, their merge surface owned by the upstream-sync CARRY (decision/hit-focus-form-engine-changes, decision/direct-engine-changes-flagged-upstreamable). A fix that reads a stop in blitz-dom's dispatch is an engine change on these terms.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the public select readers and the row rules) — `select_values`, `select_of` and the public `is_multiple_select`/`closed_select_of`, re-exported because escher-driver has no blitz-dom dependency (decision/is-select-public-re-export, contract/base-document/select-values). These are the readers a Ctrl-press check of a chosen set would use.

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select in the bridge and the snapshot) — `multiple_select_values` fills `values` per chosen option on Input/Change, and rows read their own bounds (contract/dioxus-form-events/multiple-select-values, contract/snapshot/multiple-select-bounds). This is what a non-adjacent set built by a Ctrl press would show on the carriers.

@history_item n=46 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model: the choice held as `CHECKED`, one `apply_select_choice` path firing input then change, and the keyboard model and typeahead (decision/select-choice-single-path, decision/select-keyboard-model, decision/select-choice-as-checked-state). A Ctrl press on a select goes through these.

@history_item n=47 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's verbs on a select) — `focus_select` focuses any select with no click, and `click`'s, `type`'s and `press`'s help each read as a fixed literal of the verb table (contract/driver-verbs/click). Dropping or keeping `press`'s Ctrl sentence edits that literal.

@history_item n=48 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded) — the single select's list opens by click, Enter or a Space with no typed text. Its closing and choosing keys are recorded, and it is a ratified engine change (decision/select-list-painted-overlay, decision/single-select-list-opening, contract/select-list-default-actions/open-keys). These are key paths a modifier on `press` must not disturb.

@history_item n=49 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit) — a row of an open list is never taken as a thumb drag. This qualifies decision/click-after-scroll-advances-past-fade.

@history_item n=50 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served-page checks, the overlay-scrollbar hazard) — measured that a driver click in a just-scrolled box needs an `advance` past the fade (decision/click-after-scroll-advances-past-fade). The served Cells and load checks use it, and so must any new driver check that scrolls before a press or click.

@history_item n=51 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — the select list crosses only as a row's bounds and pixels, with no new key, field or span (decision/select-list-crossing). It is the precedent that a new closed reading inside a ratified carrier is a ratification question.

@history_item n=52 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Established Decisions; Standard Contracts) — the previous debts chunk added the closed bool node key `highlighted` (NODE_FIELDS fifteen) and rewrote decision/select-list-crossing and the node and answer-JSON rows (contract/node-keys/highlighted, contract/answer-json/node). This is the latest shape of the carriers' node keys and of a debt fixed through the founder's forks.

@history_item n=53 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Cross-cutting Patterns) — a highlight move to another row became a changed-set write site (decision/select-list-changed-set, decision/changed-set-write-sites). A Ctrl press moving a `multiple` select's current row without choosing must agree with these write sites.

@history_item n=54 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes; `disabled` on form controls) — `ElementData::is_disabled` is the one disabled reading shared by focusability, pointer gates and the activation keys (decision/one-disabled-reading). A Ctrl press, like every key path, reads it.

@history_item n=55 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and button arms as private named functions) — split the range and button arms of `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` into private named functions, leaving the parents at their upstream floor so upstream sync diffs stay out of escher-owned arms. Any new stop or modifier reading in the event handlers follows this placement rule.

@history_item n=56 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Established Decisions) — decision/driver-spans-only now reads "installs no global subscriber": an acting step runs under escher-telemetry's scoped capture. A `press` change runs inside that capture.
