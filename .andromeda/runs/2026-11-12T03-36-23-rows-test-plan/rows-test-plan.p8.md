@row_add n=1 row=decision/per-crate-coverage-floors area=quality-gates-coverage-targets rec=1
name: Per-crate line coverage floors
rule: The coverage leg ends by checking three per-crate line floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %; a crate below its floor fails the leg.
why: Ratified as three crates, each floor one point below.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_add n=2 row=decision/keyboard-compat-coverage-exclusion area=quality-gates-coverage-targets rec=1
name: keyboard_compat.rs left out of coverage
rule: Both coverage runs exclude dioxus-native-dom's crate-private keyboard_compat.rs until escher takes upstream's retirement of it; the floors are unchanged by the exclusion.
why: The file is upstream's temporary keyboard-types 0.7/0.8 bridge whose conversion arms no test of ours reaches.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_none n=1 rec=2 type=5
@row_add n=3 row=decision/zero-failure-stand-repeat area=quality-gates-coverage-targets rec=3
name: Zero-failure flakiness budget for stand checks
rule: The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg.
why: Ratified as a repeated job with zero failures.
guard: stand-repeat leg, job Stand checks repeated
@row_add n=4 row=decision/mutation-runs-under-nextest-profile area=quality-gates-coverage-targets rec=4
name: Mutation runs with cargo-mutants under nextest
rule: A mutation run is cargo-mutants with nextest as its test tool under a dedicated mutants profile that stops at the first failure and runs the direct tests first, the default profile untouched.
why: Under plain cargo test, hung integration binaries run before the library tests and make mutants read timeout.
@row_add n=5 row=decision/mutation-baseline-skipped area=quality-gates-coverage-targets rec=4
name: Mutation baseline skipped, read green by hand
rule: Mutation runs skip cargo-mutants' baseline; each unit's mutant test command is first read green on the unmutated tree.
why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, so a test set naming other packages cannot be its baseline and would read every mutant caught.
@row_add n=6 row=decision/mutation-runs-on-dev-host-only area=quality-gates-coverage-targets rec=4
name: Mutation runs on the dev host only
rule: Mutation runs are made on the dev host alone, never in CI.
why: cargo-mutants and cargo-nextest are host tools no lockfile resolves.
@row_add n=7 row=decision/complexity-ceiling-host-only area=quality-gates-coverage-targets rec=5
name: Cognitive-complexity ceiling read on the host
rule: The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted; no CI leg runs it, it binds no function, and engine functions over it are left as they are.
@row_none n=2 rec=6 type=5
@row_add n=8 row=contract/committed-checks/excluded-content area=test-data-fixtures rec=7
boundary: committed recorded checks
item: excluded content
promise: A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.
@row_none n=3 rec=8 type=5
@row_none n=4 rec=9 type=5
@row_none n=5 rec=10 type=5
@row_none n=6 rec=11 type=5
@row_none n=7 rec=12 type=5
@row_none n=8 rec=13 type=5
@row_none n=9 rec=14 type=5
@row_none n=10 rec=15 type=9
@row_none n=11 rec=16 type=5
@row_none n=12 rec=17 type=5
@row_none n=13 rec=18 type=5
@row_none n=14 rec=19 type=5
@row_none n=15 rec=20 type=5
@row_none n=16 rec=21 type=5
@row_none n=17 rec=22 type=5
@row_none n=18 rec=23 type=5
@row_none n=19 rec=24 type=5
@row_none n=20 rec=25 type=5
@row_none n=21 rec=26 type=5
@row_none n=22 rec=27 type=5
@row_none n=23 rec=28 type=5
@row_none n=24 rec=29 type=5
@row_none n=25 rec=30 type=5
@row_none n=26 rec=31 type=5
@row_none n=27 rec=32 type=5
@row_none n=28 rec=33 type=5
@row_none n=29 rec=34 type=5
@row_none n=30 rec=35 type=5
@row_none n=31 rec=36 type=4
relied: The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider.
@row_none n=32 rec=37 type=4
relied: The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag.
@row_none n=33 rec=38 type=5
@row_none n=34 rec=39 type=5
@row_none n=35 rec=40 type=5
@row_none n=36 rec=41 type=5
@row_none n=37 rec=42 type=7
@row_add n=9 row=decision/ci-scripts-unit-tested area=test-scope-summary rec=43
name: CI Python scripts unit-tested
rule: The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg.
guard: ci-scripts leg
@row_add n=10 row=decision/blitz-tests-one-target-per-file area=test-scope-summary rec=43
name: One blitz-tests target per file
rule: The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt.
guard: BlitzTestsTargetsTest
@row_add n=11 row=decision/capabilities-doc-pinned-to-schema area=test-scope-summary rec=43
name: Capability statement pinned to the driver schema
rule: The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables.
guard: CapabilityStatementTest
@row_add n=12 row=decision/bounded-ci-package-installs area=test-scope-summary rec=43
name: Bounded CI package installs
rule: Every package install in the CI workflow is bounded.
guard: test_ci_workflows.py
@row_add n=13 row=contract/escher-session-exit-codes/0 area=test-scope-summary rec=44
boundary: escher-session exit codes
item: 0
promise: A host that served its session and stopped exits 0, with nothing on stdout and no state directory left.
@row_add n=14 row=contract/escher-session-exit-codes/1 area=test-scope-summary rec=44
boundary: escher-session exit codes
item: 1
promise: An argument given to actions is refused with status 1.
@row_add n=15 row=contract/escher-session-exit-codes/2 area=test-scope-summary rec=44
boundary: escher-session exit codes
item: 2
promise: An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
@row_add n=16 row=contract/escher-session-exit-codes/3 area=test-scope-summary rec=44
boundary: escher-session exit codes
item: 3
promise: A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
@row_add n=17 row=contract/escher-session-exit-codes/closed-stdout area=test-scope-summary rec=44
boundary: escher-session exit codes
item: closed stdout
promise: A run whose stdout is closed before it starts keeps its ending's status, printing no panic and creating no state directory.
@row_add n=18 row=contract/session-state-directory/length-bound area=test-scope-summary rec=44
boundary: session state directory
item: length bound
promise: A state directory path of 94 bytes holds a session on Linux and of 90 bytes on other targets; one byte more is refused state-dir-too-long with its bound.
@row_add n=19 row=contract/escher-session-stderr/service-name area=test-scope-summary rec=44
boundary: escher-session stderr
item: service.name
promise: Every stderr line is stamped service.name=seven_guis.
@row_add n=20 row=contract/escher-session-stderr/no-ids-names-or-typed-text area=test-scope-summary rec=44
boundary: escher-session stderr
item: ids, names and typed text
promise: Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
@row_add n=21 row=contract/escher-session-stderr/tool-call-line area=test-scope-summary rec=44
boundary: escher-session stderr
item: tool_call
promise: The mcp role writes one tool_call line per call, holding only its stated field names.
@row_add n=22 row=contract/mcp-server/stdout area=test-scope-summary rec=44
boundary: MCP server
item: stdout
promise: The mcp role writes protocol lines only on stdout.
@row_add n=23 row=contract/mcp-server/tools-list area=test-scope-summary rec=44
boundary: MCP server
item: tools/list
promise: tools/list offers the fourteen verbs in order.
@row_add n=24 row=contract/mcp-server/protocol-revision area=test-scope-summary rec=44
boundary: MCP server
item: protocol revision
promise: The server speaks the newest protocol revision it knows, answering a newer requested revision with that one.
@row_add n=25 row=contract/mcp-server/refused-input area=test-scope-summary rec=44
boundary: MCP server
item: refused input
promise: server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
@row_add n=26 row=contract/mcp-server/actions-argument area=test-scope-summary rec=44
boundary: MCP server
item: actions argument
promise: An actions tool call given an argument is answered isError.
@row_add n=27 row=contract/mcp-server/screenshot area=test-scope-summary rec=44
boundary: MCP server
item: screenshot
promise: A screenshot tool call is answered with an image block then a text block.
@row_add n=28 row=contract/mcp-server/help area=test-scope-summary rec=44
boundary: MCP server
item: help
promise: help is answered before any start and reads the same bytes through MCP and through the command line.
@row_add n=29 row=contract/session-recording/same-check-on-both-surfaces area=test-scope-summary rec=44
boundary: session recording
item: same check on both surfaces
promise: The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
@row_add n=30 row=contract/session-recording/group-open-directory area=test-scope-summary rec=44
boundary: session recording
item: group-open directory
promise: A recording directory open to its group is refused before anything runs.
@row_add n=31 row=contract/session-recording/off-unix area=test-scope-summary rec=44
boundary: session recording
item: off unix
promise: Off unix, --record is refused Unsupported.
@row_add n=32 row=decision/advance-reports-exact-ms area=test-scope-summary rec=44
name: Hosted advance reports exactly the ms asked
rule: A hosted advance moves the timer session with no sleep, and every ask reports exactly the milliseconds asked.
why: Ratified.
guard: host_timer
@row_none n=38 rec=45 type=5
@row_none n=39 rec=46 type=5
@row_none n=40 rec=47 type=5
@row_none n=41 rec=48 type=9
@row_none n=42 rec=49 type=5
@row_none n=43 rec=50 type=5
@row_none n=44 rec=51 type=5
@row_add n=33 row=decision/chosen-select-row-colours area=test-scope-summary rec=52
name: Chosen select row colour pair
rule: A chosen select row is painted with the engine default #c8c8c8 on #000000.
why: The pair is Chromium's inactive selected pair, reading 12.56:1.
guard: a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair
@row_none n=45 rec=53 type=9
@row_none n=46 rec=54 type=4
relied: A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.
@row_none n=47 rec=55 type=5
