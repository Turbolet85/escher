@task_prose part=session
The session moved the stand checks' repeated boot, act and answer-read setup into the shared blitz-tests test modules: session_common gained the MCP tool-call exchange and tool-result reader, the lean-task boot, the held task with its settled step, the both-modes agreement assertion, the element geometry readers and the unix-only recorded-check helper; common gained the element marks reader.
Twenty-one test files changed, none added, renamed or removed; every stand check stays one test target a file with the same tests, names and messages (fold guard: files 197, tests 930, literal set equal).
A check keeps a private copy of a helper only where its body or its message differs; three checks gained private helpers of their own for repeated assertions.
No product source, manifest, lockfile, CI script, kept check or upstream-taken test changed; no dependency, schema, verb, carrier or name moved.
Measured: duplication by the audit's jscpd recipe fell from 4.05 to 3.74 percent, the stand checks' own share from 75 pairs over 870 lines to 42 over 358; session_common declarers 44 to 46. The founder's question on keeping upstream's duplicated test pairs stays unanswered.
fast, stand-repeat (3 of 3 rounds) and a11y legs green; the operator pass's CI read green 17 of 17, the windows leg among them.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the shared session test module of blitz-tests that stand checks declare as a non-target module: boot, MCP stdio exchange, tool-result reading, held task stepping, both-layout-modes agreement, element geometry and the recorded kept check helper
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: edited — the shared common test module of blitz-tests: harness helpers, now also the element marks reader over the accessibility roles
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: edited — stand check of the error text an acting verb's refusal carries over the MCP role
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: edited — stand check of acting verb refusals and their causes over the MCP role
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: edited — stand check of the order in which the served session answers calls
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: edited — stand check that masked input values stay masked in MCP tool results
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: edited — stand check of the paged snapshot parts over the MCP role
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: edited — stand check of the MCP role's tool listing, tool calls, tool_call span and covered and no-font exchanges
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: edited — stand check that masking holds on new answers over the MCP role
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: edited — stand check of session state and focus readings
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: edited — stand check that every actionable element reads an author key
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: edited — stand check of the non-lean seven_guis tasks on the stand
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: edited — stand check of the actions verb's answer on the stand tasks
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: edited — stand check of the layout verb's answer on the stand tasks
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: edited — stand check of the snapshot diff an acting verb returns, in both layout modes
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: edited — stand check of node state changes after acting verbs, in both layout modes
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: edited — stand check of the covered refusal for an obstructed target
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: edited — stand check of the scroll verb and visibility after scrolling
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: edited — stand check that clicks land on transformed elements' bounds
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: edited — stand check of the load page whose stylesheet reveals Loaded after the load stand-in answers
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: edited — stand check of recorded sessions kept as checks and replayed through the one run path, unix only
@task_item n=1 part=touches
The convention that stand-check helpers live in non-target shared modules: it now holds many more helpers (tool-call exchange, tool-result reader, held task, geometry readers, recorded check), with private copies kept only where a body or message differs.
@task_item n=2 part=touches
The convention of shared test modules in subdirectories of blitz-tests: session_common and common grew; session_common declarers rose from 44 to 46, which matters only where a row's cell holds that count.
@task_item n=3 part=touches
One test target per file in blitz-tests and one behavior per integration test file: held unchanged, no target added, renamed or removed, every file's tests the same.
@task_item n=4 part=touches
Track-caller assertion helpers: new shared helpers carry track_caller (tool-result reader, held settled step, recorded check), so failures still point at the calling check.
@task_item n=5 part=touches
Scenarios in both layout modes: the both-modes agreement assertion and the held task taking the incremental flag moved into the shared module; the rule still holds for the acting diff and state checks.
@task_item n=6 part=touches
Rows whose guard or witness names a stand check target: every named target still stands under the same name with the same tests; only lines inside those checks moved.
@task_item n=7 part=touches
Recorded sessions kept as replayable checks and replayed through the one run path: their stand check's assertions were folded into private helpers and the unix-only recorded check helper moved to the shared module; behavior unchanged.
@task_item n=8 part=touches
Cross-process session unsupported on Windows and the session socket's non-unix targets: unix-gated helpers moved between modules; the CI windows leg read green as the off-unix compile witness.
@task_item n=9 part=touches
The MCP role over stdio, its tool result and the tool_call span: the stand checks that witness them now read replies through one shared exchange and tool-result reader; the contracts themselves unchanged.
@task_item n=10 part=touches
Capture opened before a check's own recorder: the MCP tools check's covered and no-font exchanges now call the shared exchange inside the same recorder subscriber.
@task_item n=11 part=touches
Stand-repeat zero-failure budget: the stand-repeat leg ran 3 of 3 rounds green over the folded checks; no budget changed.
@task_item n=12 part=touches
Descriptive test names and no echo of typed input: test names and messages stay free of ids, values and typed text; the literal set is equal.
@task_item n=13 part=touches
Integration tests live in blitz-tests and the manifest declares one test table: blitz-tests manifest and library untouched, the change stays inside its tests directory.
@task_item n=14 part=touches
Upstream take of blitz-vibey-cdp-server and blitz-vibey-script: their duplicated tests were deliberately left untouched pending the founder's answer on keeping them.
@task_item n=15 part=touches
Stand readings identical on every CI OS and the stand booting offline with a pinned environment: the shared boot and served helpers are the same boot the checks used before; readings unchanged.
@task_fact n=1 slug=conventions
fact: Stand checks take repeated boot, MCP exchange, held-task stepping, both-modes agreement and element geometry from the shared session_common module and element marks from common; a check keeps its own copy only where its body or message differs from the shared one.
@task_item n=1 part=untouched
Engine decisions and contracts of blitz-dom, layout, style, paint, scrolling, hit testing, select list and events: no product source changed.
@task_item n=2 part=untouched
The driver's verbs, answers, refusal causes, session wire, socket, MCP server protocol and kept check file format: their shapes and bounds are unchanged; only their test witnesses were reorganised.
@task_item n=3 part=untouched
Stack and technologies, dependencies, toolchain and versioning: no manifest, lockfile or dependency moved.
@task_item n=4 part=untouched
CI legs, ci-leg.sh contracts, coverage floors, workflows and the mutation tier: no CI script or workflow changed and no threshold moved.
@task_item n=5 part=untouched
Occupied names (crates, binaries, environment variables, the MCP server name) and occupied resources such as listeners and state directories: none taken or freed.
@task_item n=6 part=untouched
Telemetry, logging and scrubbing rows: no print, subscriber, env read, log site or span was added.
@task_item n=7 part=untouched
Project intent, capability statement, README and upstream sync rows: none changed by this session.
@task_item n=8 part=untouched
The seven_guis app sources and its own tests outside blitz-tests: not changed.
