@task_prose part=session
The session added tests only: no engine, driver, session, CLI or UI behaviour changed, and no public symbol, verb, command, exit status, config field, colour, font or motion rule was added, moved or retired.
It pinned every 0.3.0 requirement to a named witness test that the stand-repeat CI leg runs, and the sweep now reads the stand-repeat arm of the CI leg script alone; the named stand-repeat targets went from 10 to 17.
It added a new stand test target proving that masked values (a typed password, a file input's path) never appear in the escher-session graph, actions and layout answers, in a kept check and its replay, or in every explored path's check, in both layout modes; over the mcp role the file input's value reads the mask.
It added five unit tests to the browser's document loader proving the dioxus-native configuration it builds carries the root shell provider, an HTML parser provider, the font context and the passed abort signal, and runs style in parallel; the configuration itself is unchanged.
It measured that three verified 0.3.0 ledger refs named renamed tests, and re-counted the agent-run stand and all totals; coverage held all three crates above their floors.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand requirement sweep check: each 0.2.0 and 0.3.0 requirement names its witness test, and every witness runs inside the stand-repeat CI leg's arm (edited; a 0.3.0 witness table and an arm reader added)
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: a new stand check that masked input values stay masked on the escher-session graph, actions and layout answers, kept checks, explored paths' checks and the mcp role's file input value (added)
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser's document loader, which builds the dioxus-native document configuration (shell provider, HTML parser provider, font context, abort signal, parallel style); edited in its unit tests alone, the configuration unchanged
@task_item n=1 part=touches
contract/escher-session-commands/graph-answer, actions-answer and layout-answer: now witnessed by a stand check that these answers hold no masked value in both layout modes; the answers' shape is unchanged.
@task_item n=2 part=touches
contract/actions-line/content and contract/ui-graph-line/content: the lines' content is now checked to hold no typed password or file input path; a row whose guard names the stand checks over these lines may gain the new mask check.
@task_item n=3 part=touches
contract/escher-session-mcp/other-results: over the mcp role a file input's value now has a stand witness reading the mask, the tool result otherwise unchanged.
@task_item n=4 part=touches
contract/escher-session-commands/coverage-answer and contract/coverage-check-files rows: a kept check and every explored path's check are now witnessed to hold no masked value and to replay green; the kept type step reads the masked stand-in.
@task_item n=5 part=touches
contract/escher-session-stderr rows: the new stand check prints nothing and its failure messages name only a mode and an index, so stderr stays content-free; no stderr line changed.
@task_item n=6 part=touches
contract/native-config/font-context: the browser's document configuration is now unit-tested to carry the font context; a row whose guard lists tests over the font context may name it.
@task_item n=7 part=touches
contract/dioxus-native-config rows and decision/default-ua-sheet-via-default-config: the browser's configuration is now tested to carry the root shell provider, an HTML parser provider, the passed abort signal and parallel style; no configuration field, default or value moved.
@task_item n=8 part=touches
contract/shell-provider rows: the browser's configuration is now tested to carry the root shell provider; no shell provider method changed.
@task_item n=9 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: the agent-read answers are now covered by the 0.3.0 requirement table, each capability pinned to a witness the stand-repeat leg runs; no output rule changed.
@task_item n=10 part=touches
contract/escher-session-tasks rows: tasks served under the mask check and the requirement table are only exercised, no task, its id or its behaviour changed.
@task_item n=1 part=untouched
Color palette, typography, spacing, depth and border-radius rows: no colour, select list colour, font, unit, margin, z-index, paint or hit-testing rule was touched.
@task_item n=2 part=untouched
Motion rows (both twin areas): the virtual clock, advance, scroll, settle and select list motion rules are unchanged; no verb's timing moved.
@task_item n=3 part=untouched
Iconography rows: cursor icons, cursor resolution and favicons are untouched.
@task_item n=4 part=untouched
The select list rows (events, accessibility, snapshot and screenshot of a list row, open gestures, overlay): exercised as existing witnesses alone, no rule or contract item moved.
@task_item n=5 part=untouched
escher-session exit status and exit code rows, help, snapshot and screenshot commands, the JS window object, MountedData rows and the WPT runner output and colour rows: no exit status, command, argument or WPT runner output changed.
@task_item n=6 part=untouched
Mobile and WASM rows (mobile targets built not tested, WASM bundled font context, the web-spa canvas surface): no target, platform or WASM path changed.
