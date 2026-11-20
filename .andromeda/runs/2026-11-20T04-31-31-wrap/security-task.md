@task_prose part=session
The session moved the stand checks' repeated boot, act and answer-read setup into the test crate's shared modules, a refactor of test code only.
Every stand check stays one target a file with the same tests, names, messages and literal set; no target was added, renamed or removed.
No product source, manifest, lockfile, CI script, committed check or dependency changed; the shared modules stay silent, with no print, subscriber, env read, log site or span added.
Measured: the stand checks' duplication fell from 75 pairs and 870 lines to 42 and 358, and the whole duplication reading from 4.05 to 3.74 percent; no row holds either count.
The MCP answer-read helper, the recorded-check helper and the element-mark reader now live in shared test modules; the upstream-taken test duplication is kept, untouched, its keep-or-fold question still unanswered.
CI on the pre-CI commit read green, 17 of 17 checks, the windows leg included.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session test helpers of the stand checks — MCP tool calls and served exchanges, the MCP tool result decoder reading isError, held sessions stepped to settle, the visible region and viewport readers, and the recorded check writer
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the shared test harness helpers, now holding the reader of element marks with their accessibility roles
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of refusal text — the refusal JSON cause, meaning, fault and remedy over the session socket and MCP
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of driver refusal causes on acting verbs and MCP protocol errors
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check that calls are served and answered in order over the MCP server
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: the stand check that MCP server results carry masked control values, never a password or file value
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: the stand check of the snapshot's parts — snapshot text, id and name fields, state words — over MCP results
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP server's tools — listing, tool results, isError, size bounds, the covered and fonts-unavailable refusals, and the tool_call span
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that the value mask holds on every new answer and carrier
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: the stand check of session state held between calls, focus included
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: the stand check of the actions answer's keyed flag and the one actionable rule
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of the driver verbs on the non-lean tasks
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer — id, owner, verbs, nodes — and that the actions read changes nothing
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the in-process layout read on a session
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: the stand check of the diff an acting verb answers, its node keys, agreeing across settle modes
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: the stand check of control state readings after acting verbs, across settle modes
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: the stand check of the covered refusal on an obstructed target and its visible region
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of acting on scrolled and off-screen targets and the programmatic scroll
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: the stand check that a click lands on a transformed element through its visible region
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: the stand check of acting verbs that load a page, the bounded load wait and the virtual clock
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of recording and replay — the recorded check file, kept verbs, replay through the session run, recording ending on failure
@task_item n=1 part=touches
Rows whose guard cell names a stand target among the 21 changed checks: each target keeps its name, one target a file, its tests the same; the guard is neither renamed nor moved, only its setup now calls shared helpers. No claim changes.
@task_item n=2 part=touches
Masked values: masked-input-values, password-file-value-masked, mask-holds-on-every-carrier, contract mcp-result masked-control-values, contract masked-input-value and snapshot-value MASKED_VALUE — guarded by the masked MCP and masks-on-new-answers stand checks, whose MCP calls and result reads now go through the shared helpers.
@task_item n=3 part=touches
MCP results and errors: mcp-result-crossing, mcp-bounded-lines, mcp-listing-fixed-text, mcp-errors-fixed-no-data, contract mcp-tool-result is-error, size-bound and screenshot, contract mcp-errors and mcp-protocol-errors — the shared MCP tool result decoder now reads isError for the stand checks; the MCP tools check keeps its own isError assertion.
@task_item n=4 part=touches
Refusal rows: closed-refusal-cause-set, refusal-closed-cause-set, refusal-holds-no-call-input, contract refusal-causes covered, fonts-unavailable, off-screen and the rest, contract refusal-json — guarded by the refusal stand checks, whose served exchanges now come from the shared exchange; messages and reply counts unchanged.
@task_item n=5 part=touches
Recorded check and replay: recorded-check-untrusted, recorded-check-strict-reader, recorded-type-masked, replay-through-session-run, recording-ends-on-failure, recording-bounds-end-recording, contract recorded-check items — the unix-only recorded check writer moved into the shared session helpers; the replay check's assertions folded into private helpers, each message kept.
@task_item n=6 part=touches
Actions answer rows: actions-answer-crossing, actions-keyed-field, actions-read-changes-nothing, one-actionable-rule, contract actions-answer id, owner, verbs, nodes and keyed — guarded by the actions and actionable-keys stand checks, whose lean task boot and held session moved to the shared helpers.
@task_item n=7 part=touches
Snapshot and state readings: snapshot-state-readings, control-states-closed-values-only, toggle-read-per-role, contract snapshot-text items and diff-node keys — guarded by the snapshot-parts, act-states and act-diff stand checks, whose stepping now goes through the shared settled step and mode agreement.
@task_item n=8 part=touches
Visible region and hit rows: visible-region-through-transforms, non-invertible-transform-no-hit, programmatic-scroll-clamped, target-refusal-order, unknown-target-refused — guarded by the obstructed, scroll and transformed stand checks, whose edge, centre, area, inside and viewport readers moved to the shared helpers.
@task_item n=9 part=touches
Load and clock rows: bounded-load-wait, load-wait-bound, failed-load-ends-wait, settle-never-waits-on-loads, advance-moves-virtual-clock-exactly, time-moves-only-on-advance — guarded by the load-page stand check, whose shared assertion tail folded into one private helper.
@task_item n=10 part=touches
Layout and session state: session-layout-read-in-process, no-node-id-between-calls, id-lookup-anew-each-call — guarded by the layout and session-state stand checks, which now declare the shared session module.
@task_item n=11 part=touches
test-harness-no-tracing-no-style: the shared test modules gained helpers with no subscriber, print, env read, log site or span; the MCP tools check's recorder stays where it stood.
@task_item n=12 part=touches
blitz-tests-not-published: the test crate's manifest and library root unchanged; only its test sources moved.
@task_item n=13 part=touches
mcp-tool-call-span and tool-call-span contract rows — guarded by the MCP tools stand check, whose covered and no-font exchanges now call the shared exchange inside the same recorder; reply counts unchanged.
@task_item n=1 part=untouched
Product source and engine behaviour rows: select list, range, focus, keyboard, CSS, HTML parsing, serialization and stable element id derivation — no product file changed.
@task_item n=2 part=untouched
Dependency rows other than the test harness one: manifests, lockfile, pins, audit tool, licence gate, advisory ignores, locked builds — no dependency or version moved.
@task_item n=3 part=untouched
CI and secret rows: CI tokens, actions pinned to SHAs, publish guards, signing material, Actions secrets and environment variables — no CI script or workflow changed and no env read added.
@task_item n=4 part=untouched
Data protection rows: history in sqlite, HTTP cache, cookies, TLS, clipboard and file dialog, kept-check file contents and ownership — no store, cache or kept check changed.
@task_item n=5 part=untouched
Outbound network rows and their absences: blitz-net, per-host limits, CORS and same-origin, request timeout, response size limits, CDP server — nothing outbound changed.
@task_item n=6 part=untouched
The telemetry sink, target allowlists and log scrubbing rows — no log site, sink or target added.
@task_item n=7 part=untouched
Cold-agent and agent-run scripts, coverage floor checker, apt-install and target sweep rows, and their exit codes — no script changed.
@task_item n=8 part=untouched
escher-session CLI flags, roles, exit codes and output contracts, and session socket bounds and replies — the wire and the binary did not change; the stand checks read them as before.
@task_item n=9 part=untouched
WPT runner, CSSOM script API, query API and Dioxus bridge error rows — not reached.
@task_item n=10 part=untouched
Authentication and authorization rows and absences, and the threat model's opt-in rows for JavaScript and file URLs — not reached.
