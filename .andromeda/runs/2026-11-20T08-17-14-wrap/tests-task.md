@task_prose part=session
Codebase-health pass: our files over 800 code lines and our functions over cognitive 15 were brought under those bars by splitting them. No behaviour, public item, verb, field, span or log site changed, and no test was renamed (1631 test paths before and after, none gone, none new).
Unit-test modules moved out of line: the snapshot reader's tests now sit in a sibling test file declared by path, and the driver JSON tests' fourteen fixture builders moved to a fixtures child module. The driver's Key enum moved whole into a private child module, re-exported unchanged.
The command-line reader's role parsing, the MCP server's run loop, the dioxus runtime probe's tree walk and four stand integration checks were split into private helpers. The new stand assertion helpers are track_caller and keep their messages as written.
Measured: files over 800 went from 25 to 22 (ours 10 to 7, the seven left being integration-test targets; upstream's 15 unchanged). Functions over cognitive 15 went from 101 to 94 (ours 7 to 0; upstream's 94 unchanged). Coverage: escher-driver 97.77 to 97.92 %, dioxus-native-dom 90.72 to 89.91 %, blitz-test-harness 96.07 % unchanged, every crate at or above its floor.
The founder ruled on F1 to F4: upstream's oversize files, complex functions and duplicated test pairs are kept as upstream's, and escher's seven oversize integration-test targets are kept as they are, with no route entry.
All gates were green (fast, coverage, stand-repeat 3 of 3 rounds, a11y, doc), and CI was green 17/17 on the pre-CI commit.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: edited; the driver JSON answers' unit tests (the out-of-line test module of the driver's JSON encoding), its fixture builders moved out to a child module
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: new; the driver JSON tests' shared fixture builders (caused items, lint findings, action nodes, layout explanation, UI graph), a test-only child module included by path
@task_file n=3 path=packages/escher-driver/src/command.rs
what: edited; the driver verbs, the verb table and the key names, its Key enum moved out and re-exported, its inline unit tests unchanged
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: new; the driver's Key enum (the twelve named keys, name and from_name), a private child module of the driver verbs
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited; the snapshot reader that builds snapshot nodes (names, values, masked values, states, clickable) from the dioxus document, its unit tests moved out of line
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: new; the snapshot reader's unit tests (masked password and file values, checked, pressed, selected, invalid, select values, clickable), moved whole out of line with their paths kept
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: edited; the escher-session command line reader (the host, mcp and coverage roles and the address form), its role parsing split into private helpers
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: edited; the escher-session MCP role's server, its run loop split into a per-instance and a per-session helper
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited; the dioxus runtime probe integration check that walks the vdom for roots, listeners and dynamic nodes, its walk split into helpers
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: edited; the stand UI graph check (every graph node a snapshot node, no served id in the reserved form, declared writes agree with the code), its assertions moved into track_caller helpers
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: edited; the stand actions check (each served screen lists its actionable nodes with the table's verbs), its assertions moved into track_caller helpers
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: edited; the stand accessibility check (every interactive role element and region reads a name), its assertions moved into track_caller helpers
@task_item n=1 part=touches
Inline unit test modules (the inline cfg-test modules and unit tests in their source file convention): the snapshot reader's test module now lives out of line in a sibling test file declared by path. The driver JSON tests gain a fixtures child module. This is a further exception to the inline rule.
@task_item n=2 part=touches
Split test modules keep their paths: this session split two test modules and moved one test module out of line, and every test path held (1631 listed, 0 gone, 0 new). The rule is exercised and holds.
@task_item n=3 part=touches
Cognitive-complexity ceiling read on the host: our functions over 15 went from 7 to 0, and upstream's 94 are kept by the founder's ruling F2. The row's statement that engine functions over the ceiling are left as they are is now the founder's own ruling.
@task_item n=4 part=touches
Per-crate line coverage floors, and the coverage leg gated by them: the escher-driver reading rose to 97.92 % and the dioxus-native-dom reading fell to 89.91 %. Both stay at or above their floors. Where a row's cell holds a per-crate reading or floor value, it is touched.
@task_item n=5 part=touches
One integration test target per behaviour and one target per file (blitz-tests): four stand checks and the runtime probe were edited in place, and no target was added, split or merged. The founder's ruling F4 keeps the seven oversize integration-test targets whole.
@task_item n=6 part=touches
Shared test modules and shared stand tables in one module: the new stand assertion helpers stay private to each target and none moved into the shared stand modules. A row about where shared stand checks live should be weighed.
@task_item n=7 part=touches
Test failures print no argument value, and call assertions carry a row index only: the new stand assertion helpers are track_caller and keep each message as written, with the index passed in.
@task_item n=8 part=touches
Reserved owner-key id form: the stand graph check that no served id holds the reserved separator was moved into a helper, with its behaviour unchanged.
@task_item n=9 part=touches
One actionable rule for both readers: the stand actions check that every actionable node is listed by both readers was moved into helpers.
@task_item n=10 part=touches
Listeners read from the vdom walk: the runtime probe's walk over roots, listeners and dynamic nodes was split into three recursive helpers.
@task_item n=11 part=touches
Driver verbs (the lookup and the verb table) and the driver argument kind for names: the Key enum and its key names moved into a private child module and are re-exported. No name, verb or argument kind changed.
@task_item n=12 part=touches
Harness re-exports keyboard types: the driver's Key type moved module, but its public path stands. This is indirect.
@task_item n=13 part=touches
escher-session command line contracts (command form, the host, mcp and coverage roles, refused line): the reader that parses them was split into helpers. No form, refusal or exit code changed.
@task_item n=14 part=touches
MCP server and MCP role contracts (tools/list, protocol lines, stdout, refused input): the server's run loop was split into a per-instance and a per-session helper. No answer changed.
@task_item n=15 part=touches
Snapshot node contracts (clickable, masked value, number) and the typed-password-never-returned decisions: the snapshot reader's tests that witness them moved out of line unchanged.
@task_item n=16 part=touches
Stand checks repeated three rounds and the zero-failure budget for stand checks: stand-repeat read 3 of 3 rounds green over the edited stand checks.
@task_item n=17 part=touches
Every requirement witnessed inside stand-repeat, and stand checks run in both layout modes: the edited stand checks keep their witnesses and modes.
@task_fact n=1 slug=quality-gates-coverage-targets
fact: Upstream-authored files over 800 code lines (fifteen) are kept as upstream's, by the founder's ruling F1. Only escher's own files are held under 800 code lines.
@task_fact n=2 slug=quality-gates-coverage-targets
fact: Escher's own files are held under 800 code lines. The exception is the seven integration-test targets over it, which are kept whole with no route entry, by the founder's ruling F4.
@task_fact n=3 slug=quality-gates-coverage-targets
fact: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream's, by the founder's ruling F3. The duplication reading is not held against them.
@task_fact n=4 slug=quality-gates-coverage-targets
fact: A unit-test child module included by path is counted in its crate's coverage reading unless its file is named with the _tests suffix. Only the suffix keeps a test file out of the reading.
@task_fact n=5 slug=test-strategy
fact: A source file's unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg-test module by path, and fixture builders may sit in a child module of that test module. This is the convention's exception to inline test modules.
@task_item n=1 part=untouched
WPT conformance runner, WPT suites, statuses, expectations and the WPT CLI: no WPT surface moved.
@task_item n=2 part=untouched
CI workflow, legs, caches, actions pinning, package installs, clippy legs, MSRV and the CI artifacts: no CI file or leg changed.
@task_item n=3 part=untouched
agent-run verbs, events, selections and exit codes, and the cold-agent pipe, verdict and events: untouched.
@task_item n=4 part=untouched
Session wire, session host, session errors, session recording and the session state directory contracts: no lifecycle, socket or recording behaviour moved.
@task_item n=5 part=untouched
Harness construction, settle, input, inspection, screenshot and contrast contracts, and the harness virtual clock: no harness code changed.
@task_item n=6 part=untouched
Mutation testing decisions (dev host only, baseline, nextest profile): not exercised and not changed.
@task_item n=7 part=untouched
Mocking discipline, network stand-ins, timer ticks and the stand's real pinned font: unchanged.
@task_item n=8 part=untouched
Test data and fixture pages, committed recorded checks and their excluded content, and the windowed end-to-end absence: unchanged.
@task_item n=9 part=untouched
Lint defect and lint finding contracts and the driver refusal causes and their order: no lint or refusal behaviour moved.
