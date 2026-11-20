@task_prose part=session
The session moved the stand checks' repeated boot, act and answer-read setup out of each check file into the shared test modules; each check keeps one target and the same tests, with no test added, renamed or removed and no message changed.
The shared session module gained the tool-call request line, the MCP JSON-RPC exchange over the mcp role, the tool result decoder, the lean-task lookup and held session boot, the settled step with its diff, the mode-agreement assertion, the element edge, centre, area and viewport readers, and a unix-only kept-check recorder; the shared page module gained an element mark reader.
No product source, manifest, lockfile, CI script, committed check, schema or dependency changed; no span, event, subscriber, print, env read or log site was added, and the shared modules stay silent.
Measured: the stand checks' duplication fell from 75 pairs and 870 lines to 42 and 358, the whole tree's from 4.05 to 3.74 percent; the upstream-taken tests' duplicated pairs were kept untouched while the founder's question on them stays unanswered.
Gates green: fast, stand-repeat 3 of 3 rounds, a11y, the fold guard, and the operator pass's CI 17 of 17 including the windows leg.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session module of the stand checks: headless session boot, the MCP role exchange and tool result decoding, settled steps and diffs, element geometry reads, and the kept-check recorder
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the shared page-harness module of the integration checks, now holding the element mark reader (ids with their roles)
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of the act refusals' error text and remedy over the MCP role
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of act refusals and their cause names over the MCP role
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check of the order in which served tool calls are answered over the MCP role
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: the stand check that the MCP role's answers mask what the caller typed
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: the stand check of the snapshot part reads over the MCP role
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP role's tools, its tool_call and command spans under a recording subscriber, and the covered and no-font refusals
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that new answers keep masking typed values
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: the stand check of the session state and its focused element
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: the stand check of the actionable keys read from the page's elements
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of the non-lean tasks booted in a held session
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer, which changes nothing in the document
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the layout answer
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: the stand check of the snapshot diff an act returns, agreeing across full and incremental modes
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: the stand check of element states across settled act steps
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: the stand check of the covered and off-screen act refusals by element geometry
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of scrolling an element into the viewport before an act
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: the stand check that a click lands on a transformed element
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: the stand check of loading a page by a click or a press
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of session recording and replay of kept checks, unix only
@task_item n=1 part=touches
The silent checks and shared modules decision: the shared test modules gained many helpers and must still hold no print, subscriber, env read, log site or span; the fold guard's literal set was read equal.
@task_item n=2 part=touches
The headless stand runs with no sink decision: the held session boot and lean-task lookup that stand checks use moved into the shared session module.
@task_item n=3 part=touches
The driver library installs no subscriber decision and the driver emits no event decision: the MCP tools check still installs its recording subscriber only around the shared exchange, as a test-side scope.
@task_item n=4 part=touches
The tool_call span contracts (tool, ending, command child) and the command span contracts: their witness stand check now reads recorded tool calls through a private helper and the shared exchange; targets and messages unchanged.
@task_item n=5 part=touches
The mcp role stdout JSON-RPC lines contract and the mcp role exit status contracts: the JSON-RPC request line and the stdout exchange checks use are now one shared helper, and the tool result decoder is shared.
@task_item n=6 part=touches
The refusal cause names covered, off-screen and fonts-unavailable: the checks witnessing them now read element geometry and the covered and no-font exchanges through shared helpers.
@task_item n=7 part=touches
The kept check file contracts (arguments, answer, answer digest, accepted) and the recording file check file contract: the unix-only kept-check recorder moved into the shared session module and the replay check's kept-and-holding assertion became a private helper.
@task_item n=8 part=touches
The replay children uninstrumented and recording directory private decisions: the replay check's recorded-counter setup that clears and recreates the record directory is now a helper.
@task_item n=9 part=touches
The answers echo nothing typed and part answer never logged decisions: the masked-answer checks now share the tool call and result decoding; the rule that no id, name, value or typed text appears in a test name or message held, the name lists and literal set equal.
@task_item n=10 part=touches
The snapshot and diff contracts and the actions read changes nothing decision: the settled step, its diff and the full-versus-incremental mode agreement are now one shared helper used by the act diff and act states checks.
@task_item n=11 part=touches
The snapshot node clickable and snapshot text clickable contracts: the element mark reader of roles by id moved to the shared page module.
@task_item n=12 part=touches
Rows whose guard or witness cell names a stand check target by name: every target stands, one target a file with the same tests; no guard was renamed or moved.
@task_item n=1 part=untouched
Telemetry strategy: the tracing crate choice, feature-gated engine tracing and the ungated escher crates are unchanged; no product crate or manifest moved.
@task_item n=2 part=untouched
Escher's sink log line, its stream, scrub, escaping, span line layout, resource keys and RUST_LOG configuration: no sink or telemetry code changed.
@task_item n=3 part=untouched
Metric coverage and the absences of metrics exporters, metric emission, telemetry backends, OTel export and an error-reporting service: none added.
@task_item n=4 part=untouched
The cold-agent pipe and agent-run harness: their events, verdict fields, counted verbs, reasons, transcripts and cleanup are not reached.
@task_item n=5 part=untouched
CI integration: workflows, uploaded artifacts, the package-install diagnostic line and the build-directory sweep line are unchanged; no CI script changed.
@task_item n=6 part=untouched
Error capture: the script error surface, window error event, panic hooks and WPT report contracts are not reached.
@task_item n=7 part=untouched
PII scrubbing in the engine and session logs: scrub sets, engine target fields, select and optgroup label handling and the state registry are unchanged.
@task_item n=8 part=untouched
escher-session exit codes, commands, coverage role and host wire version: no product behaviour changed.
