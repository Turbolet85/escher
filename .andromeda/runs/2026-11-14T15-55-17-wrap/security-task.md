@task_prose part=session
Added auto-exploration to the driver: one in-process library call that explores the available actions breadth-first, within a step budget, on a fresh session for each path.
Every call it drives goes through the session's own validation. A panic in a handler is caught for each path. Refused, panicked and diverged paths are never extended.
It can keep each explored path as a replayable kept check in a recording directory that must be owner-only. A directory open to the group or to others is refused before anything boots.
The kept check's opening record gained a third key, format. It is written as 1, read as 1 when absent, and any other value is refused as malformed. The two committed Counter checks were re-recorded with that key alone.
No verb, role, argument, carrier, IPC method, socket, env var, refusal cause, session error, log site or span was added. No dependency moved. Nothing was retired.
The founder confirmed in his own words that exploration is in process only and crosses no carrier. The absence of navigation-following stays provisional.
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the driver's in-process auto-exploration: the step budget, the closed path endings, a fresh session per path, the panic catch, and the explored verbs taken from the actions table with image-result verbs left out
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: unit tests of the exploration step budget bound and of the explored-verb selection by result field kind
@task_file n=3 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand tests of auto-exploration: every served task within budget, a refusing control, a caught panicking handler, each path's kept check replaying green, the group-open recording directory refused, the budget bounding every acting call
@task_file n=4 path=packages/escher-driver/src/lib.rs
what: the driver library's public surface, its re-exports, now adding explore, StepBudget, Exploration, ExploredPath, PathEnding and CHECK_FORMAT
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: the actions answer and the driver verb table's target verbs and per-node call naming, now visible within the crate to exploration
@task_file n=6 path=packages/escher-driver/src/recording.rs
what: the kept check recorder and its strict reader of untrusted check bytes, the opening start record now writing and strictly reading the format key
@task_file n=7 path=packages/escher-driver/src/recording_tests.rs
what: unit tests of the recorded check file's form and its strict reader, including the format rows that are read and those refused as malformed
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: a committed stand-Counter kept check recorded through the command line, now holding format 1 in its opening record
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed stand-Counter kept check recorded through the MCP server, now holding format 1 in its opening record
@task_file n=10 path=docs/capabilities.md
what: the committed capability statement of the driver's carriers and limits, now stating that exploration runs in process only and the kept-check record limit for an explored path
@task_item n=1 part=touches
The kept check file's opening start record goes from two keys (tool, arguments) to three (tool, arguments, format). A row that states the start record's keys or the kept check file's contents is now stale.
@task_item n=2 part=touches
The rules that a recorded check is read strictly and treated as untrusted bytes gain a new strict field. format must be the JSON integer 1. Anything else (1.0, "1", -1, 0, 1.5, null, true, or another number) is refused as malformed, and the refusal holds nothing of the bytes.
@task_item n=3 part=touches
The rules that coverage and every other check reader refuse an unusable check as malformed now cover a format mismatch too, through the same reader.
@task_item n=4 part=touches
The committed stand-Counter checks decision: both committed checks were re-recorded by the binary and differ from before only by the format key.
@task_item n=5 part=touches
The recording directory is owner-only and kept checks are owner-only, under the kept check medium contract. Exploration is a new caller that records into such a directory. It refuses a group- or other-open directory with the existing record-dir-not-private session error before anything boots, and writes nothing.
@task_item n=6 part=touches
The recording bounds that end a recording, and the rule that the recorder removes only its own files: an explored path deeper than a kept check's 400 records ends its recording, and that path's check is removed.
@task_item n=7 part=touches
One validation path for every carrier, and the single command validation gate: exploration drives every call through the session's run and validate, so it adds no second validation path.
@task_item n=8 part=touches
The closed driver verb schema and the driver verb contracts: no verb, role, argument or carrier reaches exploration. It sits beside the absence of a URL-taking driver verb as a further capability that the carriers do not carry.
@task_item n=9 part=touches
The screenshot carried in band, and the screenshot answer's png: exploration leaves out every verb whose result holds an image field. It goes by the field's kind, not its name, which today leaves out screenshot.
@task_item n=10 part=touches
No panic on call-supplied input: exploration runs each path's last call inside a panic catch. A path whose handler panics ends as Panicked, the session is dropped under a second catch, and the exploration goes on.
@task_item n=11 part=touches
The rules that no NodeId is kept between calls and that no process-local handle crosses: the exploration report holds no NodeId, panic payload, screen text or screen digest.
@task_item n=12 part=touches
Driver outcomes are never logged, screen content stays out of logs, there is one command span per call, and the driver installs no subscriber and reads no env. Exploration adds no span, field, log site, print or env read. Each call it drives leaves only its own command span.
@task_item n=13 part=touches
Recorded type to a masked control stays masked, and no caller input is echoed: the text typed on an explored path is only the fixed in-bound values that the per-node call naming gives.
@task_item n=14 part=touches
Actions read changes nothing, and the actions answer: exploration reads the actions answer on each new screen to choose its children. It is a second in-crate caller of the target verbs and the call naming.
@task_item n=15 part=touches
The session errors as one fixed table, and the closed refusal cause set: exploration reuses the existing session error and the eight causes, and adds none. A refused path's ending holds the closed cause.
@task_fact n=1 slug=input-validation
fact: An exploration's step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS). 0 or more than the bound gives no budget. The budget bounds every acting call the exploration runs, re-reach steps included. The first screen is read unrecorded and costs no step.
@task_fact n=2 slug=input-validation
fact: A kept check's opening record carries format, written as 1. A check without the key is read as format 1. Any other value, of whatever JSON type, is refused as malformed, and the refusal holds nothing of the bytes.
@task_fact n=3 slug=input-validation
fact: Exploration explores only the driver table's target verbs whose result holds no image field, chosen by the field's kind and never by its name, so today screenshot is left out. The text typed on a path is only the fixed in-bound value that the call naming gives.
@task_fact n=4 slug=data-protection
fact: An exploration's report holds only closed endings, indices, calls and bools: Accepted with settled, Refused with its cause, Panicked, or Diverged at a step. It holds no panic payload, NodeId, screen text or screen digest, and it is never logged or printed.
@task_fact n=5 slug=data-protection
fact: An exploration keeps each explored path as a kept check only in an owner-only recording directory. A group- or other-open directory is refused as record-dir-not-private before anything boots, with nothing written. A path deeper than a kept check's 400 records ends its recording, and that check is removed.
@task_fact n=6 slug=error-handling
fact: During exploration, a handler panic is caught for each path, which ends as Panicked. The session is dropped under a second catch, and the exploration goes on with the next path. A refused, panicked or diverged path is never extended.
@task_fact n=7 slug=threat-model-summary
fact: Exploration is reachable on no carrier. No verb, role, argument, socket request or MCP tool reaches it, and it is one in-process library call only. This is ratified by the founder, not provisional.
@task_fact n=8 slug=threat-model-summary
fact: Exploration follows no navigation: it drives only the in-page actions of the screen it boots on. This stands PROVISIONAL, pending the founder's choice.
@task_fact n=9 slug=logging-monitoring
fact: Exploration emits no span, field or log line of its own. Each call it drives leaves only that call's own command span.
@task_item n=1 part=untouched
The MCP server's transport, request and result bounds, protocol errors, tool list and idle expiry: no tool, method or bound moved.
@task_item n=2 part=untouched
The session socket's wire, closed request grammar, replies, connection bounds and Unix-only listener: no request, reply or version moved.
@task_item n=3 part=untouched
The escher-session command line's flags, roles, exit codes, stdout and stderr lines: no flag, role or exit was added.
@task_item n=4 part=untouched
Outbound network and fetch controls: schemes, GET and HEAD limits, CORS and same-origin absences, per-host limits, file URL rules and the script fetcher are not reached.
@task_item n=5 part=untouched
Dependency security: audit, licence gate, locked builds and pins. No dependency was added or bumped.
@task_item n=6 part=untouched
Secret management and environment variables: no env var was read or added, and RUST_LOG remains the only one.
@task_item n=7 part=untouched
CI tokens, publish guards, signing material and WPT workflow rows: no CI step, leg or workflow changed.
@task_item n=8 part=untouched
The engine's markup, CSS and select list handling, accessibility adapter, stable id derivation and snapshot text words: none moved.
@task_item n=9 part=untouched
The cold-agent harness rows (its prompt, call allowlist, recordings and logs): no agent-run script changed.
@task_item n=10 part=untouched
The telemetry sink's target allowlist, the content scrub and the network and resource log contracts: no log target or field was added.
@task_item n=11 part=untouched
Browser history, cookies, HTTP cache, clipboard, file dialog and TLS rows: not reached.
