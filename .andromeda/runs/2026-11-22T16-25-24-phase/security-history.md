@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and keyboard-activation rows cite the named functions) — the code audit's complexity split moved bodies of the keyboard and range handlers into private functions (`take_sign`/`take_digits`/`take_exponent`, `press_range_input_key`, `press_range_input`, `click_button`) with no behaviour changed; the rows' claims stood and only their citations moved, the added lines holding no `unwrap`, `expect`, panic, print, `tracing` site or env read. The direct precedent for this chunk's move of escher's additions out of `handle_key_or_input_event` and the other two upstream bodies: a behaviour-preserving split is no widening, and the new helper functions must carry no new panic path or log site.

@history_item n=2 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** (Input Validation: 1 row) — the earlier cognitive-load corrective chunk changed no rule; it only dropped a false why on `decision/unresolvable-stylesheet-href-no-load`. Precedent that a complexity corrective's security amendment is limited to re-reading rows against the moved code, not new rows.

@history_item n=3 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — established the driver command schema: `validate` checks a call against a closed verb table and bounded argument kinds, one answer per call in a fixed order, no `unwrap`/`expect`/`panic!`/`unreachable!` or index on call input, no clock read, no `Session` in its signature; `Command`, `Call`, `ArgValue` print ids and typed text under `Debug`, which nothing prints, logs or fields. Basis of `decision/closed-driver-verb-schema`, `decision/no-panic-on-call-input`, `decision/command-schema-no-clock`, `decision/refusal-holds-no-call-input`; the split of `packages/escher-driver/src/command.rs` must keep every one of these, and keep `Debug` output unlogged.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` validates first and nothing public runs a `Command` that did not come from `validate`; an id is looked up anew each call, never by index. Basis of `decision/one-validation-path-for-every-carrier`, `decision/single-validation-gate`, `decision/id-lookup-anew-each-call`; a module split must not make any `Command` constructor or run path public outside the crate.

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — after `validate` the executor refuses with nothing run: stale or not-found, then disabled, off-screen, covered in that order; the session holds a private bounded record of id text. Basis of `decision/target-refusal-order`, `decision/unknown-id-refused-stale-or-not-found`, `decision/session-id-record-bounded`; the order and the record's privacy must survive the split.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (what the driver logs: one span, eight fixed-word or count fields) — one INFO `command` span per call under `escher_driver`, fields `verb`, `cause`, `settled`, `busy`, `passes`, `added`/`removed`/`changed`; never an argument, refusal, fault or screen text; only a refusal's cause name logged. Basis of `decision/one-command-span-per-call`, `decision/command-span-content-free`, `decision/refusal-log-cause-name-only` and the `contract/driver-command-span/*` rows; moving span code between modules must leave exactly these fields and the two `info_span!` sites the logging rows count.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's span record class and that a diff's three sizes may reach a log while its content never does. Basis of `decision/diff-counts-only-in-logs`; unchanged by a restructure.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers; the command line is an external input surface; a refusal has a written form) — wire `call v2` decoded strictly and run through `Session::run` alone; argv read by `escher_driver::command_line`, a closed reader checking the call before the session address; four exit classes; no ending echoes caller input; `Refusal::to_json` and `SessionError` from fixed strings. Basis of `decision/session-socket-carries-calls`, `decision/driver-socket-closed-grammar`, `decision/cli-closed-argv-reader`, `decision/cli-echoes-nothing-typed`, `decision/no-echo-of-caller-input` and the exit-code contracts; `escher_driver::command_line` is one of the public paths the split must keep resolving.

@history_item n=9 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — the founder's rule that a new closed-value field inside a ratified carrier is no widening, while authored text, an open set, a new carrier or exit still is. Basis of `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`; the scope adds no field at all, so the rule stands as the yardstick for any incidental change.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (a Markup attributes row for the three control states) — the in-process accessibility builder reads `aria-selected`/`aria-pressed`/`aria-invalid` trimmed, lower-case, against a closed set, no log site or panic path, exiting only as closed enums on the adapter and the snapshot. Basis of `decision/aria-state-lowercase-exact-match`, `decision/control-states-closed-values-only`, `decision/attribute-readers-no-log-or-panic`; these are escher's additions inside `build_accessibility_node` that the chunk moves out, and must stay closed parses with no log site.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing, ratified; `serde_json` reviewed; the `tool_call` span) — `serve_mcp` over stdio, bounded request and result lines, tool arguments typed before `validate`, a `tool_call` span parenting `command`. Basis of `decision/mcp-server-stdio-only`, `decision/mcp-stdio-no-auth`, `decision/mcp-bounded-lines`, `decision/mcp-no-print-or-panic`, `contract/mcp-tool-arguments/value-kinds`, `contract/mcp-tool-call-span/tool`; MCP reaches the schema through `command.rs`'s items, which the split must keep in place.

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and the program's fixed text; a session error's remedy and its table) — the verb table's help and result-field texts and `SESSION_ERRORS` are fixed `'static` table text crossing on ratified carriers. Basis of `decision/mcp-listing-fixed-text`, `decision/session-errors-fixed-table`, `decision/session-error-fixed-table`; moving tables between modules must not change any text.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest, its directory, its reader and `--record`; wire v3, fifteen session errors, `call_of`'s home) — `Check::from_bytes` builds each `Call` through `command::call_of`, so a replayed step passes `validate` again; the schema and MCP rows cite `call_of` at `command.rs` (was `mcp/result.rs`); recording directory 0700, never named or logged. `call_of` lives in the file being split: its new module must keep replay passing `validate` and the rows' citation follows it (`decision/recorded-check-untrusted`, `decision/recording-dir-owner-only`, `decision/recording-dir-never-named`).

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` runs each kept step through `Session::run` alone, nothing on another app's session. Basis of `decision/replay-through-session-run` and `decision/recorded-check-strict-reader`; unchanged by the split.

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb's findings on every carrier) — `lint` joined the validated table; findings hold ids, closed defect words and numbers, never fielded or logged. Basis of `decision/lint-findings-crossing` and `decision/lint-findings-closed-content`.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the seventh argument kind `part` (1 to 1000, `MAX_PART`); a part's number and count never fielded. Basis of `contract/driver-argument-kinds/part`; the kinds table moves with the split unchanged.

@history_item n=17 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting step waits on a load for at most `LOAD_WAIT` (5 s), moving no time, the session's only wall-clock read for it. Basis of `decision/load-wait-bound`, `decision/bounded-load-wait`, `decision/driver-installs-no-subscriber`'s clock list; any moved code must not add a clock read.

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set closed at eight (`time-unavailable` retired), two by `validate` and six by the executor; `advance` accepted on every session. Basis of `decision/closed-refusal-cause-set`, `decision/refusal-closed-cause-set` and the `contract/refusal-causes/*` rows; the split adds no cause.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — `off-screen`'s fixed meaning restated, remedy naming `scroll`; cause set and names unchanged. Bears on `contract/refusal-causes/off-screen`; fixed texts must move verbatim.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the select verbs' help) — `covered`'s meaning names an option of a closed select; `click`/`press`/`type` help restated, each a fixed literal of the verb table. Bears on `contract/refusal-causes/covered`; the texts move verbatim.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph joins the carriers; the thirteenth verb `graph`; its silence in the logs) — `graph` in the schema after `lint`, refused `malformed` with an argument, recording `verb` alone, a ratified widening onto every carrier. Basis of `decision/graph-answer-crossing`, `contract/graph-answer/nodes-states`, `contract/command-span/verb`.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer; the verb table at fourteen; the actions verb's validation; the shared target check) — `validate` reads `("actions", [])`; one crate-private `check_target` shared by the four id-taking arms and `Session::actions`, which spells each call by `call_naming` through `validate`; answer unpaged under the 1,048,576-byte bound; `keyed` field. Basis of `decision/actions-answer-crossing`, `decision/actions-keyed-field`, `decision/actions-read-shares-target-check`, `decision/actions-answer-unpaged` and the `contract/actions-answer/*` rows; the single shared check and its crate-private visibility must survive the split.

@history_item n=23 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (Input Validation: 2 rows) — `~ decision/actions-keyed-field` provisional → standing; `~ contract/actions-answer/keyed` drops its provisional clause, on the founder's ratification. The picked rows' current state.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, a kept check named on the command line) — `escher-session coverage <app> <check>...` reads checks through `Check::from_bytes`, replay through crate-private `replay_each`, refusals `malformed` holding nothing of the path, cause set eight. Basis of `decision/coverage-refuses-bad-checks-malformed` and the coverage-line contract; ratified by `2026-11-15T05-21-42-wrap`.

@history_item n=25 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Error Handling: 1 row) — `+ decision/reach-error-closed-set`: reach errors are a closed set with fixed texts echoing nothing supplied.

@history_item n=26 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Input Validation: 2 rows) — `~ decision/driver-installs-no-subscriber` and `~ decision/driver-no-subscriber-no-env` gain the exception that an acting step runs under escher-telemetry's scoped log capture, which prints nothing; `~ contract/driver-answer/settled` scoped to the busy reading.

@history_item n=27 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/caused-text-crossing` (error kept within the text bound, every other content-named field redacted), `~ decision/mask-holds-on-every-carrier` (a caused item's message and error field as authored), `~ decision/socket-carries-screen-reading` (caused scrubbed by the answer's own field names); ratified by `2026-11-17-a-component-in-isolation`. The current wording of three picked rows the moved answer-writing code must keep.

@history_item n=28 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring: 1 row) — `~ decision/no-screen-content-in-logs` names a step's caused text among what reaches no log at any level.

@history_item n=29 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/driver-verbs/layout`, `~ decision/session-layout-read-in-process` (through the layout verb on any carrier), `~ decision/socket-carries-screen-reading` (layout readings), `~ contract/actions-answer/verbs` (layout listed with no cause), `~ contract/command-span/verb` (layout records verb alone), `~ contract/recorded-check/tool` (eleven instance verbs).

@history_item n=30 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Input Validation: 1 row) — `~ contract/driver-session-reply/no-attribute-text`: a reply carries no attribute text except what an author wrote into a record's message or a caused item's `error`.

@history_item n=31 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (Input Validation: 1 row) — `~ decision/mask-holds-on-every-carrier` guard gains `stand_masks_on_new_answers`; one of the existing checks the plan can name to prove the split moved no mask.

@history_item n=32 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/driver-verbs/graph` (optional part 1 to 1000), `~ contract/graph-answer/nodes-states`, `~ decision/graph-answer-crossing` (part and parts numbers), `+ contract/cli-flags/ctrl`, `~ contract/refusal-causes/malformed`, `~ contract/command-span/verb` (never the part asked), `− decision/graph-answer-unpaged → decision/graph-answer-paged`. This chunk's 72 lines in `command.rs` that pushed it to 848 code lines; the rows it moved are the newest wording of the code being split.

@history_item n=33 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check's `type` masking) — a `type` whose element resolves masked is kept as `MASKED_STAND_IN`, never the text. Basis of `contract/masked-input-value/masked-stand-in`; must hold across the split.

@history_item n=34 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Input Validation: 1 row) — `~ contract/recorded-check/start-record`: the opening record gains a strictly read `format`; the kept-check reader the split leaves in place.

@history_item n=35 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (Threat Model Summary: 1 row) — `~ decision/driver-session-unix-socket-only`: the socket exists on unix only while an in-process session runs on linux, windows and macOS; any module split must keep the unix-only gating of socket code.

@history_item n=36 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the committed `docs/capabilities.md` states the MCP crossing and its carried set, adding no carrier; carried sets reconciled. Basis of `decision/capability-statement-mcp-crossing`.

@history_item n=37 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — under `accessibility` each element node carries its stable id as AccessKit `author_id`, never on a text run, root or window, leaving only through the adapter. Basis of `decision/author-id-on-accessibility-nodes`; this write is among escher's additions in `build_accessibility_node` (`packages/blitz-dom/src/accessibility.rs`).

@history_item n=38 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the tree handed to the adapter is asserted headlessly on the stand in both layout modes (`author_id` equals `element_id`, never a NodeId form); the AT witness stays unread. An existing check that proves a move of `build_accessibility_node`'s additions changed nothing.

@history_item n=39 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — upstream's `aria-labelledby`, `aria-describedby`/`aria-description` and `aria-expanded`/`aria-disabled`/`aria-level` reads are upstream's behaviour, ratified; escher dropped upstream's unfiltered `aria-label` read for one trimmed read. Basis of `decision/aria-labelledby-outranks-label`, `decision/aria-label-trimmed-once`, `decision/descriptions-adapter-only`, `decision/merged-aria-states-closed-parse`. Trap: these are upstream text in `build_accessibility_node` and must stay; the single trimmed `aria-label` read is an escher edit to upstream's lines, to be told apart by diff against `547d7514b`.

@history_item n=40 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the `inert` read and the toggle per role) — `aria-checked` read on the five checkable roles, `aria-pressed` on every other, `role="switch"` → `Role::Switch`; inert subtrees out of the tree; ratified 2026-10-18. Basis of `decision/toggle-read-per-role`; escher's additions in `build_accessibility_node`.

@history_item n=41 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls) — presence on a form control alone through `ElementData::is_disabled`, read by focusability, pointer, click and the activation arms. Basis of `decision/disabled-read-by-presence`; the gate the keyboard additions use.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkable's checked state in the tree) — `build_accessibility_node` writes `Toggled::True/False` where `is_native_checkable && is_checkable(role)`, pinned by `accessibility_states`. An escher addition in the function this chunk splits, with its named check.

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded`, and an author's `open` and `aria-expanded`) — a single select writes `set_expanded(open)` from engine state and reads no `aria-expanded`; pinned by `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`. Basis of `decision/select-expanded-from-engine-state`; an escher addition in `build_accessibility_node`.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup` and its `label`) — a group takes `Role::Group` and its non-empty label is its tree name. Basis of `decision/optgroup-options-and-label`; an escher addition in the accessibility builder, pinned by `select_interaction`, `stand_act_select`.

@history_item n=45 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the single `select` row: choice as `CHECKED`, closed keys and the 1 s typeahead under the `is_disabled` gate, `input` then engine `change`; a select's option reads no `aria-selected`; label binding by a closed element-name match. Basis of `decision/single-select-selectedness`, `decision/select-typeahead-private`, `decision/select-choice-exits`; its keys are escher additions in `handle_key_or_input_event`, and the option `aria-selected` exception lives in the accessibility builder.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys, clicks and chosen set; the `select` row's rules) — `press_multiple_select_key` closed match, `apply_select_choice` one write path, set exits. Basis of `decision/multiple-select-keys-and-clicks`, `decision/select-choice-one-write-path`, `decision/select-keys-closed-match`, `decision/multiple-select-set-exits`; keyboard additions this chunk moves.

@history_item n=47 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `select` row: Enter, Space and a click open the list; an open list's state, keys, pointer and close) — `press_select_key` opens the list; `press_open_list_key` one closed match, Tab chooses then closes before focus moves; `live_select_list` reader. Basis of `decision/select-list-keys-and-painted-labels`, `decision/select-list-tab-chooses-and-closes`, `decision/select-list-live-reader`, `decision/select-list-closing`; dispatch of these arms sits in `handle_key_or_input_event`.

@history_item n=48 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (which select opens a list, what a row takes, and hostile markup; scroll; readers; labels) — `select_list_allowed` gate, disabled row changes nothing, 20 rows, hostile markup finite, own-offset scroll, `SelectListRow.label` never logged; each with named guards. Basis of `decision/select-list-open-gate`, `decision/select-list-row-choice`, `decision/select-list-twenty-rows`, `decision/select-list-hostile-markup-finite`, `decision/select-list-scroll-own-offset`, `decision/select-list-label-never-logged`.

@history_item n=49 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a shot of an open list holds labels as pixels; `optgroup`'s `label` painted as a header; a page's markup reaches the select list) — labels as pixels on socket and MCP, guarded by `cli_commands`. Basis of `decision/open-list-shot-pixels`.

@history_item n=50 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — `size_rows` via `parse_unsigned_integer`, capped at 65534, a valid value hinting a `height` of size × 1.2em; pinned by `size_reads_a_whole_number_above_zero_capped`. Basis of `decision/multiple-select-size-rows`, `decision/select-size-hint-cap`; the size hint is an escher addition to `compute_presentational_hints` (`packages/blitz-dom/src/stylo/presentational_hints.rs`).

@history_item n=51 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (Input Validation: 4 + 3 rows) — `~ decision/select-size-hint-cap` (any select), `~ decision/multiple-select-size-rows`, `~ decision/single-select-selectedness`, `~ decision/select-list-open-gate` (drop-down only), `~ decision/select-list-keys-and-painted-labels`, `~ decision/select-expanded-from-engine-state` (a list box states none). The newest wording of escher's additions in all three upstream functions this chunk splits: the presentational size hint, the drop-down key gate and the expanded write.

@history_item n=52 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click or DOM focus events, clears the typeahead, closes any list; `type`'s help fixed text. Basis of `decision/driver-type-on-select`.

@history_item n=53 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — range input current value apart from its attribute, sanitized on every seed and move, arrows/Home/End with no `disabled`, one `input` per change, no print, log site, `unwrap`, panic or env read. Basis of `decision/range-current-value-apart`, `decision/range-keyboard-actions`, `decision/range-value-sanitized`, `decision/range-value-exits`; the range arm is an escher addition reached from `handle_key_or_input_event`.

@history_item n=54 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter on the focused control dispatches a Click (after the text-input and range arms) for button, summary, submit/reset/button input or `a[href]`; released Space through `handle_key_up`; closed name and type match plus the `disabled` presence gate, no log site or panic. Basis of `decision/enter-activates-focused-control`, `decision/space-keyup-activates-focused-control`; escher's arms in `handle_key_or_input_event`, their arm order part of the behaviour to keep.

@history_item n=55 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the engine's `change` on a checkbox and a radio) — `handle_click`'s checkable arms send `change` after `input`, pinned by `checkable_change_event`. Basis of `decision/engine-change-event-checkable`; reached by the keyboard activation path.

@history_item n=56 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears children's `layout_parent`, so a key press on a control whose block was dropped does not panic, witnessed by `key_press_after_container_lost_its_box`. Basis of `decision/dropped-anonymous-block-clears-parent`; a guard for the keyboard function's move.

@history_item n=57 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — `~ decision/dropped-anonymous-block-clears-parent` (a re-focused control), `~ decision/programmatic-focus-events` except (post-layout focus clear silent). Newest wording of keyboard-path rows.

@history_item n=58 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Input Validation: 5 rows) — `+ decision/event-is-trusted-by-source`, `+ decision/programmatic-focus-events`, upstream's event-queue model, ratified as an upstream widening. Upstream behaviour now inside the keyboard path; upstream's text there stays upstream's.

@history_item n=59 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Input Validation: 2 rows) — `+ decision/presentational-hints-typed-parse`: the font face/size, valign, cellspacing, cellpadding hints read by a typed parse, upstream's behaviour. These are upstream's lines in `compute_presentational_hints`, to be left in its body while escher's additions move out.

@history_item n=60 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every record from a target outside its allowlist at every level; an author-key id and an accessible name print at no level (`host_log`). Basis of `decision/stable-id-not-logged`, `decision/accessible-names-not-logged`, `decision/log-sink-redacts-and-allowlists`, `decision/telemetry-sink-target-allowlist`; the moved helpers must add no log site that could carry an id or name.

@history_item n=61 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — `RUST_LOG` registered as the one env value read, escher's stderr sink installed by the binaries. Basis of `decision/driver-reads-only-rust-log`, `contract/environment-variables/rust-log`, `decision/stderr-only-telemetry-sink`; the split must add no env read.
