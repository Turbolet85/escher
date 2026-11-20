@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the code audit's earlier complexity split. The range and keyboard-activation bodies moved into private functions (`take_sign`/`take_digits`/`take_exponent`, `press_range_input_key`, `press_range_input`, `click_button`) with no behaviour change. The rows' claims stood and were only re-cited, and the added lines carried no `unwrap`, `expect`, panic, print, `tracing` site or env read. This is the direct precedent for this chunk's no-behaviour splits of `read` (cli.rs), `run` (mcp.rs) and the oversize files: the claims stay, citations follow the code, and an extraction adds no panic or log site.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script runtime's module split) — upstream split `runtime.rs` into `runtime/*.rs` and `dom/element.rs` into `dom/element/*.rs`. 17 citations were re-pointed by hand to the moved code, with no claim text changed. This is the precedent for a file → module-directory split like `json_tests.rs` → `json/*` tests, `snapshot.rs` and `command.rs`: rows citing moved lines are re-pointed, never re-worded.

@history_item n=3 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (decision · contract · absence · overview), each verified against the code, with guards re-pointed. Rows now name guards (test targets and functions such as `stand_actions`, `stand_graph`, `accessibility_label`, `mcp_log`) rather than file:line citations. A split that renames or moves a guarding test function or target touches those guard cells.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — this entry made the command line an external input surface. Argv is read by `escher_driver::command_line`, a closed reader with fixed-order rules: the call is checked by `validate`/`validate_session` before the session address is asked for, `--session` is required exactly once, there is no panic path on argv, and no ending writes back anything typed. It set exit statuses 0–3 and the socket's `call` grammar under bounds, and it is the source of decision/cli-closed-argv-reader, cli-echoes-nothing-typed, no-echo-of-caller-input and the exit-code contracts. `read` (cli.rs:156, cognitive complexity 25) is this reader, and its extraction must keep the rule order and the check-before-address property.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>` to the argv reader: at most once, only on `start`, the host role and the `mcp` role, otherwise usage (status 2) after `validate`, with the path never written back. It moved `call_of` to `command.rs` (cited `command.rs:32-60`) as the builder that every kept-check step passes back through `validate`. It also added wire v3 and the two session errors. contract/cli-flags/record is this rule. Splitting `command.rs` (826 lines) moves `call_of` and its citations, and splitting `read` must keep the flag's placement rules.

@history_item n=6 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage <app> <check>...` role to the argv reader: one shape, otherwise usage. Each check is read through `File::open(..).take(MAX_CHECK_BYTES + 1)` and `Check::from_bytes`. An unreadable, oversized, non-check or other-app file is refused malformed (status 1) and holds nothing of the path. This is the source of contract/cli-roles/coverage and the coverage answer line, and it is another arm `read` must keep when its complexity is cut. The entry was ratified on 2026-11-15 (2026-11-15T05-21-42-wrap).

@history_item n=7 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added the seventh argument kind `part` (1 to 1000, `u32::try_from`) and `--part` as a one-value flag in the argv reader. This is the source of contract/driver-argument-kinds/part and cli-flags/part, and part of what `read` and the schema in `command.rs` must keep unchanged.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP server's input row. Request lines are ≤ 65,536 bytes (a longer line is read to its terminator, dropped and answered -32700). Non-JSON is -32700 and a bad envelope -32600. The `id` is echoed only as an i64 or a string ≤ 128 bytes. Protocol errors are fixed, with no `data`. Four methods are answered, notifications run nothing, wrong-kind arguments are refused malformed before `validate`, and results are ≤ 1,048,576 bytes. Replies are hand-written and `serde_json` only reads. A sibling entry with the same marker added the `tool_call` span (`tool`, `ending`) as the parent of `command`. This is the source of the decision/mcp-* rows, contract/mcp-* and serde-json-read-only. `run` (mcp.rs:215, cognitive complexity 18) is this loop, and an extraction must keep every bound, error code, echo rule and the span parentage.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — the MCP `initialize` is matched against a closed table of four revisions (any other revision is answered with the newest). `help` is answered by the server with no `command` child. The CLI gained a `help` ending. Session errors became a fixed table with `remedy`. This is the source of contract/mcp-initialize/protocol-revision and decision/session-errors-fixed-table, which are further paths in mcp.rs `run` and cli.rs `read` that a split must preserve.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screenshot crosses in band. An MCP result holding an image is bounded by its whole written form, and a refused shot is one `isError` text block. This is the source of decision/screenshot-in-band, contract/mcp-tool-result/screenshot and size-bound, which are result-shaping paths in mcp.rs that must survive the `run` extraction unchanged.

@history_item n=11 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — `validate` checks a call against a closed table of verbs and bounded argument kinds and answers it in a fixed order. There is no `unwrap`, `expect`, `panic!`, `unreachable!` or index on call-supplied input, and no clock read. A refusal is a closed cause set with fixed texts holding no path. This is the source of decision/closed-driver-verb-schema, no-panic-on-call-input, command-schema-no-clock and refusal-holds-no-call-input. `command.rs` (826 lines) holds this surface, so a split must keep the order and add no panic path.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one INFO `command` span per call under `escher_driver`, with eight fixed-word or count fields (`verb`, `cause`, `settled`, `busy`, `passes`, `added`/`removed`/`changed`). No argument, refusal, fault or screen text is fielded, and only the cause name of a refusal is logged. This is the source of decision/one-command-span-per-call, command-span-content-free and refusal-log-cause-name-only and of the contract/driver-command-span/* rows. The `info_span!` count (two in `packages/escher-driver/src`) is read by later checks, so no extraction may add a span or field.

@history_item n=13 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — after `validate`, the executor refuses an id the screen does not read as stale or not-found, then refuses a click or type as disabled, off-screen or covered, in that order, with nothing run. The session keeps a private, bounded record of ids. This is the source of decision/target-refusal-order, unknown-id-refused-stale-or-not-found and session-id-record-bounded.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set narrowed to eight (`time-unavailable` retired), two returned by `validate` and six by the executor. The set is held at eight in decision/closed-refusal-cause-set and refusal-closed-cause-set. Any test split that counts causes reads eight.

@history_item n=15 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot's one text form, `Snapshot::to_text`, writes ids as quoted `id=` fields and names as quoted strings in `str`'s Debug form. There is no JSON form, no Display form and no parser. It also extended the password mask to file inputs. This is the source of contract/snapshot-text/id-field and name-field and decision/password-file-value-masked. `packages/dioxus-native-dom/src/snapshot.rs` (913 lines) holds the writer and the mask, and its split must keep both byte for byte.

@history_item n=16 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads a password's value as `MASKED_VALUE`, never the text or its length, and its `enabled` reading follows `disabled` presence. This is the origin of decision/masked-input-values and contract/snapshot-value/masked-value, both inside `snapshot.rs`.

@history_item n=17 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — three state readings (selected, pressed, invalid) as closed bare words in the text and booleans in the diff, never attribute text. This set the founder's rule that a closed-value field inside a ratified carrier is no widening. It is the source of decision/snapshot-state-readings and the contract/snapshot-text state-word rows written by `snapshot.rs`.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — an open select list's row crosses only as its own `bounds` in the snapshot and the diff and as pixels in a shot, with no `NodeState` field and no `expanded` key. The select readers resolve their node through `get_node`, `closed_select_of` answers none for an open list, and `SelectListRow.label` is never logged. This is the source of decision/select-list-crossing, select-list-live-reader, select-readers-resolve-node-closed-select-none and select-list-label-never-logged, which bear on how the `snapshot.rs` split keeps the row-bounds path.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added the `graph` verb (no argument, refused malformed with one, unpaged under the 1,048,576-byte bound, guarded by `stand_graph`), the reserved `{owner}///{key}` virtual-node id form (no served id holds `///`), and the snapshot's `clickable` closed reading written in `snapshot.rs` (`NODE_FIELDS` fourteen). This is the source of decision/graph-answer-crossing and graph-answer-unpaged, contract/driver-verbs/graph, contract/stable-element-id/virtual-node-form and contract/snapshot-text/clickable. The two over-ceiling `stand_graph.rs` tests (`every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form`, `every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run`) pin these, so splitting them must keep every assertion.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the `actions` verb (unpaged, guarded by `stand_actions` `the_whole_cells_answer_is_within_the_answer_bound`), one shared `check_target` for every id-taking verb and the actions read, and one actionable rule (`Readings`) shared by `unkeyed_actionable` and `actionable_nodes`. `keyed` was ratified later at 2026-11-13T07-43-19-wrap. This is the source of decision/actions-answer-crossing, actions-answer-unpaged, actions-read-shares-target-check, one-actionable-rule and the contract/actions-answer/* rows. The over-ceiling `stand_actions.rs` test `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` holds part of this, so splitting it must keep the guard names the rows cite.

@history_item n=21 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added the `layout` verb with one required id, so there are eleven instance verbs. The in-process layout read is reached from every carrier, the actions verbs list `layout` with no cause, and a layout or graph or actions call's span records `verb` alone. This is the source of contract/driver-verbs/layout, decision/session-layout-read-in-process, contract/actions-answer/verbs, contract/command-span/verb and contract/recorded-check/tool, all held in `command.rs` and the executor that the split touches.

@history_item n=22 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — an acting step runs under escher-telemetry's scoped log capture, which prints nothing. This is the `except` of decision/driver-installs-no-subscriber and driver-no-subscriber-no-env, and an extraction in the driver must not move the capture or add a subscriber or env read.

@history_item n=23 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's message and `error` field hold what their author wrote, and every other content-named field reads as redacted. This set the current `except` of decision/mask-holds-on-every-carrier and the rule of decision/socket-carries-screen-reading. The delegate confirm was ratified by the founder at 2026-11-17-a-component-in-isolation.

@history_item n=24 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — the guard of decision/mask-holds-on-every-carrier gained `stand_masks_on_new_answers` beside `stand_act_filled` and `stand_mcp_masked`. Test targets named as guards must survive the chunk's test splits under the same names.

@history_item n=25 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — contract/driver-session-reply/no-attribute-text gained its exception for an author's text in a record's message or a caused item's `error`. The entry also ratified the delegate confirms of the error-field and layout cards, so those rows read ratified.

@history_item n=26 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — a host that installs escher's sink drops every outside-target record whole, so no id or name prints at any level. `host_log` is the standing check. Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. This is the source of decision/stable-id-not-logged and accessible-names-not-logged.

@history_item n=27 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added decision/session-layout-read-in-process and extended decision/no-screen-content-in-logs to the layout answer. That row's guards (`stand_act_spans`, `host_log`, `mcp_log` at `RUST_LOG=trace` under both builds) must stay green through the splits.

@history_item n=28 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — added the `aria-labelledby` precedence (cycle-guarded, outranking `<label>` and `aria-label`) and dropped upstream's unfiltered `aria-label` read, leaving one trimmed non-empty read. Descriptions reach the adapter only, and `aria-expanded`, `aria-disabled` and `aria-level` are closed parses. This is the source of decision/aria-labelledby-outranks-label, aria-label-trimmed-once, descriptions-adapter-only and merged-aria-states-closed-parse. The over-ceiling `stand_accessibility_ids.rs` test `every_interactive_role_element_reads_a_name` reads these names, so its split must keep its assertions.

@history_item n=29 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the tree handed to the adapter is asserted headlessly on the stand in both layout modes: every `author_id` is its node's `element_id` and none is empty, all digits or `0x`-bearing. This is the stand ids check whose function `every_interactive_role_element_reads_a_name` this chunk splits. The split must keep the assertion set and both layout modes.

@history_item n=30 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — removed the spawned-task MountedData panic. `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests (one per layout mode) were inverted from `should_panic` and now pin it. `visit` (`tests/blitz-tests/tests/dioxus_runtime_probe.rs:67`, cognitive complexity 22) sits in that target, so its extraction must keep those pins failing on a restored borrowed render.

@history_item n=31 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `coverage_floor.py` gates per-crate floors (`CRATE_DIR=PERCENT`, exit 2 otherwise), and the 30 s answer bound and closed-pipe behaviour are each pinned by a check. This is the source of decision/per-crate-coverage-floors (escher-driver 97, dioxus-native-dom 83) and closed-pipe-drops-answer. Splitting `command.rs`, `snapshot.rs` and the json tests moves line counts in exactly those crates' coverage.

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the committed `docs/capabilities.md` states the MCP crossing (the spawning client as the boundary, no authentication, the carried set, onward use as the client's) and adds no carrier. This is the source of decision/capability-statement-mcp-crossing. A refactor changes no carried set, so the statement stays true.

@history_item n=33 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — restated absence/served-network-api and overview/api-security as "in a default build" because of the opt-in devtools CDP server on loopback. That server is in upstream-authored files this chunk leaves alone (class 2). Context for the API Security overview the picked rows carry.
