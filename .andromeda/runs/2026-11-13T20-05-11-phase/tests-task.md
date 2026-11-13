@task_prose part=chunk
A behaviour-preserving restructure of three functions toward the cognitive-complexity ceiling of 15: blitz-dom's click default-action handler, escher-driver's executor, and dioxus-native-dom's UI graph builder.
Each function's reading must drop; one left above 15 names why, and no extracted helper reads over 15; helpers stay in the same file, with no file split and no new module.
Behaviour is unchanged for every document and every driver call: dispatch order, event sequences, default actions, refusal order, outcomes, span fields, snapshot and graph content all stay as they are.
It re-measures the three functions and each extracted helper before and after with rust-code-analysis-cli on the dev host, never in CI, and keeps those readings as evidence.
It re-runs the existing checks that hold each moved arm (both layout modes, the incremental oracle, the fast leg); where no check reaches an arm, a check that could fail is added before the move.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, holding the click default actions (checkbox and radio input and change, button focus and submit, select, links, summary, labels) — edits: the handle_click function, into helpers in the same file
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor, the per-verb dispatch with its target check and refusal order — edits: the execute function, into helpers in the same file
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph, the DioxusDocument graph answer of listeners and task state — edits: the graph function, into helpers in the same file
@task_item n=1 part=needs
The cognitive-complexity ceiling of 15 is read on the dev host only; no CI leg, gate or lint is added for it. The scope leaves to P3 whether the rule's clause that engine functions over it are left as they are is crossed here.
@task_item n=2 part=needs
Click default actions in the engine: checkbox and radio fire input and change, buttons take focus and submit, plus the select, link, summary and label arms. Change events still come from the engine alone, in the same event sequence.
@task_item n=3 part=needs
The driver executor's per-verb dispatch and target check, with the eight refusal causes in their fixed order. A refusal still changes nothing and echoes no input.
@task_item n=4 part=needs
Driver session verbs over a held instance (act, acting and reading verbs, run, run carriers, settle) keep their outcomes. They are held by the stand_act integration checks and escher-driver's unit tests.
@task_item n=5 part=needs
The UI graph answer keeps its content: task state registered for the graph, listeners read from the vdom walk, the reserved owner-key id form, and an answer that holds no value. It is held by stand_graph.
@task_item n=6 part=needs
Handler coverage measured by replay stays the same, held by stand_handler_coverage. That includes bubbling dispatch counting every ancestor listener and the limits on canvas handlers and unreached screens.
@task_item n=7 part=needs
The driver command span and per-call log line keep their fields and fixed words. The restructure adds no span, log field, carrier, verb, argument, key name, cause, session error or exit.
@task_item n=8 part=needs
Stand checks and layout assertions run in both layout modes. The incremental layout differential oracle stays green.
@task_item n=9 part=needs
The fast CI leg stays green: fmt, clippy over the workspace and upstream blitz-dom, workspace tests, CI scripts and embolden.
@task_item n=10 part=needs
Existing engine checks hold the moved click arms: checkable change event, keyboard stand, select interaction, select popup and range input interaction. Harness click and dispatch helpers drive them.
@task_item n=11 part=needs
Any check added for an arm no existing check reaches must be red-before-green, able to fail against a broken move, and land before the move.
@task_item n=12 part=needs
The mutation baseline is read per unit on the dev host. It is relevant where the restructure moves units the mutation runs read.
@task_standing n=1
convention: each new check is a test that could fail, controlled red-before-green
home: unit-test-strategy
@task_standing n=2
convention: a new unit test sits in an inline test module of its source file
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=3
convention: a new integration check is its own test target, one target per file, with shared fixtures and stand tables kept in their shared module
home: 1-test-scope-summary · test-strategy
@task_standing n=4
convention: a new engine check is harness-driven on a bundled font with no skip, and runs in both layout modes
home: 2-test-strategy · test-strategy
@task_standing n=5
convention: new test data is stated in code with nothing seeded, and blitz-dom fixtures are built without the HTML parser
home: 7-test-data-fixtures · 3-test-harness-contract
@task_standing n=6
convention: a new check uses no sleep or wall-clock read; time moves only by the virtual clock
home: integration-test-strategy
@task_standing n=7
convention: a new check's failure prints no argument value and no ids, names or typed text in its logs
home: unit-test-strategy · 3-test-harness-contract
@task_item n=1 part=not_needed
WPT conformance runner rows: its CLI, outputs, status set, pinned commit, Thai-font reftests and upstream-only workflow. The restructure changes no WPT run or expectation.
@task_item n=2 part=not_needed
Cold-agent pipe rows: its events, exit codes, verdict, isolation, prompt, recordings and bounds. The chunk runs no cold agent.
@task_item n=3 part=not_needed
Agent-run harness rows: its verbs, events, selections, exit codes, state files and Windows entry. The chunk changes no agent-run surface.
@task_item n=4 part=not_needed
Session wire, session host lifecycle, session state directory, idle expiry and session recording rows (check files, record fields, directory permissions). The executor's restructure touches none of the socket or recording transport.
@task_item n=5 part=not_needed
escher-session command-line and MCP role rows: usage line, help, tools/list order, protocol revision, MCP tool results and CLI exit statuses. No command, role or protocol surface is changed.
@task_item n=6 part=not_needed
CI workflow plumbing rows: actions pinned to SHAs, bounded and retried apt installs, cache save branches, trigger branches, secrets and permissions, artifact upload, and the dependency audit. No workflow file is touched.
@task_item n=7 part=not_needed
Screenshot and pixel rows: CPU-buffer pixel tests, shot PNG determinism, box-shadow expectations, select list painting, scrollbar fade and font embolden paint. No paint path is restructured.
@task_item n=8 part=not_needed
Snapshot text budget, lint defect kinds and contrast rows (WCAG formula, required ratio, contrast reader). The graph and executor restructure changes none of those readings.
@task_item n=9 part=not_needed
Password masking and typed-password rows, and file: URL loading rows. No arm moved by the restructure reaches them as a new behaviour.
@task_item n=10 part=not_needed
Upstream blitz-tests manifest rows and upstream buffered-render or mounted-flush test rows. The chunk adds and drops no upstream test.
@task_item n=11 part=not_needed
The windowed end-to-end absence and the real MCP client operator pass. The chunk runs no window and no external client.
