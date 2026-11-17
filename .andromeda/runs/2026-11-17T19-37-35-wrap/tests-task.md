@task_prose part=session
The driver's hand-written JSON answer writer was split by answer family into seven private child modules (snapshot, diff, caused, layout, graph, actions, lint); the parent keeps the primitives, the shot, coverage, the outcome dispatch, refusal, session error and the session lines.
Every carrier's answer stays byte for byte unchanged: no written key, order or byte moved, no public symbol, re-export or caller path changed, no dependency, schema or committed check changed.
The writer's inline unit test module moved out of line into a #[path] test file beside it; its 9 tests and 49 literal JSON pins stand there with the test paths json::tests::* unchanged.
Moved items reached by the parent or the tests are widened to pub(super) and no further; the rest stay private to their child.
Measured: the unit tests 9 passed, stand replay 5 passed, the fast and doc CI legs green, the session agent-run passed; CI green 17/17 at attempt 3 after two iOS jobs cancelled with no runner; escher-driver line coverage 97.68 % against its 97 % floor.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: the driver's answer writer, written by hand in a crate-private module — the JSON primitives (string, number, list, object), the shot without its image, the coverage line, the outcome dispatch, the refusal and session error answers and the session lines; edited, now the parent of the family modules
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: the snapshot answer family of the driver's answer writer — a snapshot node with its schema fields, its reading word and masked value; new
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: the acting call result family of the driver's answer writer — settled, busy and the snapshot diff of changed node states; new
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: the caused items family of the driver's answer writer — the caused and caused-past items an acting call carries (panic, error, log); new
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: the layout verb answer family of the driver's answer writer — placement, placer, shift and size-cause words and lengths of a layout explanation; new
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: the graph answer family of the driver's answer writer — graph nodes and task state with declared and confirmed writes, holding no value; new
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: the actions answer family of the driver's answer writer — one actionable node with its owner, holding no value; new
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: the lint findings family of the driver's answer writer — one finding with its defect, bounds and reason; new
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: the driver answer writer's unit tests (json::tests), moved out of line to a #[path] test file — table-driven tests with literal expected JSON pins for strings, numbers, nodes, each outcome, layout words, refusals, session errors, session answers and the coverage line; new
@task_item n=1 part=touches
The convention that unit tests sit inline in their source file (inline cfg(test) modules): the driver answer writer's 9 tests moved out of line to a #[path] test file, a further instance of the out-of-line exception.
@task_item n=2 part=touches
The rule that a split test module keeps its paths: the moved tests keep json::tests::* names, 9 of them, unchanged.
@task_item n=3 part=touches
The rule that shared test modules or #[path] test children are no test target: the new test file is a child of the crate's lib, not a target.
@task_item n=4 part=touches
Expected values pinned as literals: the 49 literal JSON pins of the driver answers moved with the tests and pass unchanged.
@task_item n=5 part=touches
Table-driven tests assert their row count and carry a row index: the writer's table tests moved whole, their bodies unchanged.
@task_item n=6 part=touches
Driver JSON answers contract items (number, masked value) and the snapshot node contract: the code writing them moved to family modules, bytes unchanged.
@task_item n=7 part=touches
The driver refusal causes, their printed form and fixed strings, and session error messages as fixed text: the refusal and session error writers stay in the parent, tested by the moved tests.
@task_item n=8 part=touches
Lint defects and lint findings order: the finding writer moved to the lint family module, its output unchanged.
@task_item n=9 part=touches
Every layout reader word read on a fixture and the layout verb answer: the placer, shift, cause words and placement moved to the layout family module; the words test moved with them.
@task_item n=10 part=touches
Graph answer holds no value and actions answer holds no value: the graph and actions writers moved to their own family modules, unchanged.
@task_item n=11 part=touches
Acting call result and caused items on an acting call (run, diff, changed node states): the diff and caused writers moved, unchanged.
@task_item n=12 part=touches
The coverage line shape and owner: the coverage line writer stays in the parent, its test moved out of line.
@task_item n=13 part=touches
Per-crate line coverage floors: escher-driver reads 97.68 % against its 97 % floor; the split children read 98.91 % to 100 %.
@task_item n=14 part=touches
Recorded checks replayed as tests and committed checks byte-equal to the recording: stand replay of both committed Counter checks compared whole, 5 passed, checks untouched.
@task_item n=15 part=touches
Bridge tests without widening visibility: items moved to children are widened to pub(super) only, some only because the out-of-line tests call them (layout placement among them).
@task_item n=16 part=touches
Doc leg gates a public doc comment edit: two doc comments now stand on their own items (graph node, shot without image), text unchanged, the doc leg green.
@task_item n=17 part=touches
Capability statement pinned to the driver schema and claim section names held to the schema: no schema, field or capability changed; no capability claimed.
@task_item n=18 part=touches
Stable tree serialization: the hand-written serialization of the driver answers keeps every key and order.
@task_fact n=1 slug=unit-test-strategy
fact: Items a split module's children expose to the parent or to the out-of-line tests are widened to pub(super) and no further; the tests name the parent's former imports instead of reaching them through use super::*.
@task_fact n=2 slug=ci-integration
fact: A CI job cancelled with no runner acquired is re-run, not fixed, and recorded before each re-run; a red on anything but a missing runner stops the pass.
@task_item n=1 part=untouched
The session wire, session host, socket, state directory and session recording contracts: no protocol, bound or file layout moved.
@task_item n=2 part=untouched
The harness construction, settle, input, inspection, screenshot and contrast contracts and the virtual clock: no harness code changed.
@task_item n=3 part=untouched
agent-run and cold-agent verbs, events, exit codes, verdict and isolation rows: no script changed.
@task_item n=4 part=untouched
WPT runner, its status set, outputs and conformance rows: untouched.
@task_item n=5 part=untouched
escher-session command line, MCP role, exit codes and stderr rows: no command, output or exit status changed.
@task_item n=6 part=untouched
Mocking discipline, stand-ins, fixtures and test data rows: no stand-in or fixture changed.
@task_item n=7 part=untouched
CI workflow shape, package installs, caches, actions pinning, legs membership and mutation testing rows: no workflow, leg or floor value changed.
