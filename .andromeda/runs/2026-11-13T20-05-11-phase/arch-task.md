@task_prose part=chunk
Restructures three functions without changing what they do, measured against the cognitive-complexity ceiling of 15: blitz-dom's click handler handle_click (97), escher-driver's executor execute (32) and dioxus-native-dom's UI graph reader graph (50).
Each function's reading drops. It ends under 15 where it can, and any function left over 15 names the reason. No helper extracted from it reads over 15, and every helper stays in the file that holds the function today, with no file split and no new module.
Re-measures all three functions and every extracted helper before and after with rust-code-analysis-cli on the dev host, never in CI.
Re-asserts unchanged behaviour through the existing checks: click default actions, driver acting verbs, the UI graph and handler coverage, both layout modes, the incremental oracle and the fast leg. Where no existing check reaches an arm being moved, a check for that arm is added before the move.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom pointer event handling, the click default actions (checkbox and radio input and change, button focus and submit, select arms, links, summary, labels) — edits: the handle_click function
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor, the per-verb dispatch of driver calls with the per-verb target check and the refusal order — edits: the execute function
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph reader, DioxusDocument::graph, which builds the graph the graph verb answers — edits: the graph function
@task_item n=1 part=needs
The cognitive-complexity ceiling of 15, read with rust-code-analysis-cli on the dev host only. It is the measure every restructured function and every extracted helper is read against, and no CI leg, gate or lint is added for it.
@task_item n=2 part=needs
Click default actions in event dispatch: the activation click order, button click focus, checkbox and radio change after input, and label clicks. Their dispatch order and event sequences must come out identical after the restructure.
@task_item n=3 part=needs
Select list default actions reached from a click: click on a select, label click, choosing an option, opening and closing the list, and the single path every select choice is written through. The restructure must keep these arms and their order unchanged.
@task_item n=4 part=needs
Link activation, fragment-only links scrolling in page, and summary toggling, all reached from the click handler. Their outcomes and when a link starts loading stay unchanged.
@task_item n=5 part=needs
The rule that handlers run before the default action, and the order of default-action events, which the click handler's extracted helpers must still follow.
@task_item n=6 part=needs
Disabled and inert readings, plus focus rules (a click focuses a button, focus fixup), that click arms consult. Each must be called at the same point and in the same order as today.
@task_item n=7 part=needs
The driver's per-verb dispatch: the verb table order and the acting verbs (click, type, press, scroll, screenshot, advance, actions, graph, lint, snapshot). The dispatch each verb gets stays exactly as it is.
@task_item n=8 part=needs
Call validation and the per-verb target check: the first failure wins, and the refusal causes (not-found, stale, disabled, covered, off-screen, malformed, unknown-verb, fonts-unavailable) are closed at eight. Refusal order and refusal content do not change.
@task_item n=9 part=needs
The driver acting step and its results (added, changed, removed, settled, busy), the load wait bound, and the clock moves made by advance. These outcomes do not change.
@task_item n=10 part=needs
The command span and its fields (verb, cause, passes, added, changed, removed, settled, busy), plus the rule that the driver opens spans by hand and emits no event. Spans keep the same fields, and none is added.
@task_item n=11 part=needs
The UI graph contract: graph, owner, readers, writers, write resolution, write flag, unconfirmed, repeated registration, and the graph verb's node, state, argument, content and paging. The graph's content stays the same.
@task_item n=12 part=needs
Graph state reached by registration, the graph registry and its declared writes, and signal writes recorded against their causing dispatch. The graph reader keeps consuming these the same way.
@task_item n=13 part=needs
The closed session boundary: no new carrier, verb, argument, key name, node field, refusal cause, session error, exit status, log field or span. Any such addition would widen the ratified boundary, and this chunk adds none.
@task_item n=14 part=needs
The existing checks that hold the restructure: checkable change event, stand keyboard, select interaction, select popup, range input interaction, the stand act checks with escher-driver's unit tests, and stand graph with stand handler coverage. Handler coverage keeps its reached handlers and bubbling dispatch.
@task_item n=15 part=needs
Both layout modes give identical layouts, and the incremental oracle stays green under the restructured code.
@task_item n=16 part=needs
The fast leg (fmt, then clippy, workspace tests, CI scripts and embolden) must pass with the new helpers. Clippy allows stay local, unwrap and expect stay justified, and borrow conflicts are still resolved by taking and restoring.
@task_item n=17 part=needs
DOM mutations still go through DocumentMutator, and the Stylo snapshot is still taken before attribute mutation. Click arms that mutate keep doing so through the same mutator, whose source this chunk does not edit, and the document core is not edited either.
@task_item n=18 part=needs
Upstreamable direct engine changes: restructuring blitz-dom's click handler is an engine change. P3 decides whether the host-only complexity ceiling's clause that engine functions are left as they are is a policy this chunk crosses.
@task_standing n=1
convention: each new integration test lives in blitz-tests, one behaviour per test file and one test target per file
home: design-philosophy · conventions
@task_standing n=2
convention: each new test has a descriptive name, asserts its fixture's condition first, and states shared fixtures once with numbered cases
home: conventions
@task_standing n=3
convention: each new engine scenario runs in both layout modes
home: conventions
@task_standing n=4
convention: unit tests go at the end of the source file, and driver unit tests go in path children; stand-check helpers go in non-target modules
home: conventions · existing-scopes
@task_item n=1 part=not_needed
Browser app areas: chrome, tabs, history service and storage, persistence, about pages, the URL bar and favicons. The chunk touches no browser code.
@task_item n=2 part=not_needed
The WPT runner, its suites, result kinds, statuses, reftests, the PR diff script and its outputs. No WPT tooling changes.
@task_item n=3 part=not_needed
The session socket wire, the MCP server and the mcp role, session host process model, state and recording directories, kept-check files and their reader. The executor's callers and the transport around it are unchanged.
@task_item n=4 part=not_needed
Paint and rendering: the paint scene, the paint tree, scrollbars, gradients, borders, text decoration and the renderer choice. No painting path is restructured.
@task_item n=5 part=not_needed
CSS and style: Stylo, the cascade, presentational hints, fonts, the user-agent stylesheet and the CSS features listed as absences. No style resolution changes.
@task_item n=6 part=not_needed
The JavaScript runtime: Boa, script globals, script fetching, microtasks and timers. No script path is touched.
@task_item n=7 part=not_needed
Networking and loading: the net provider, the per-host cap, the HTTP cache and URL scheme routing. No network code changes.
@task_item n=8 part=not_needed
The stack and dependency choices: new crates, version pins, allocators, the Nix flake, toolchain and MSRV. The chunk adds no dependency and bumps nothing.
@task_item n=9 part=not_needed
The names register (binaries, crates, environment variables), the capability statement and the root README. The chunk adds no name and no capability.
@task_item n=10 part=not_needed
The cold-agent and agent-run tooling: their scripts, events, exit codes, verdicts and model provider use.
@task_item n=11 part=not_needed
Upstream sync, cherry-picks, upstream-only workflows and lockstep release versioning. This is no upstream take or release.
@task_item n=12 part=not_needed
The accessibility tree and platform adapter (AccessKit, ARIA state readings, platform refresh). The click, executor and graph restructure does not touch how that tree is built.
@task_item n=13 part=not_needed
Telemetry bootstrap and scrubbing (init, line format, target filter, panic hook). The binaries' sink installation is unchanged.
