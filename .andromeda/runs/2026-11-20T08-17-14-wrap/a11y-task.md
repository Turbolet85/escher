@task_prose part=session
A code-health refactor: escher's own files over 800 code lines and functions over cognitive complexity 15 were split to meet the audit's bars. No behaviour changed.
Long functions were split into private helpers: the driver's command-line reading of its roles and address, the MCP server's run loop, the Dioxus runtime probe's walk, and three stand checks (the UI graph, actionable nodes, interactive names). Each check keeps its test name and its messages.
The snapshot model's unit tests moved whole out of line into their own test file. The driver's named key set moved whole into a child module of the command set and is re-exported under the same name. The wire-form tests' fixture builders moved into a test-only child.
No public item, signature, verb, field, span or log site changed. Every workspace test keeps its path (1631 listed, none gone, none new). The fast, coverage, stand-repeat (3 of 3 rounds green), a11y and doc legs are green, and CI is green on the final HEAD.
The founder ruled on F1 to F4: upstream's oversize files, its complex functions and its duplicated test pairs stay as upstream wrote them, and there is no route entry for escher's seven oversize integration-test targets.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: the driver's wire-form unit tests, which cover the JSON carriers of driver answers: the keyed wire form, the actions answer, the snapshot diff, lint findings, the UI graph and handler coverage. Edited: its fixture builders moved out.
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: the new test-only fixture builders for the wire-form tests: caused items, actionable nodes, lint findings with bounds, the layout explanation, the UI graph and the snapshot diff.
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command set (its verbs and their arguments). Edited: the named key set moved into a child module and is re-exported from here.
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: the new home of the driver's named key set. It holds the twelve keys a driver press or type can name, for example Enter, Space, Tab and the arrows, with their names and lookup by name.
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot model, which reads the accessibility tree into snapshot nodes (role, name, enabled, checked, value, masked password and file values, clickable). Edited: its unit tests moved out of line.
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: the new out-of-line unit tests of the snapshot model. They cover the accessible name from aria-label and from a bound label, display none absent, disabled read by presence (not enabled, not focusable), checkbox and radio checked, values, password and file masking, range value, aria-selected, aria-pressed, aria-invalid, aria-disabled, the switch role, select value and options, and clickable.
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: the session library's command line, which reads its roles (host, MCP, coverage) and its address. Edited: the reading was split into helpers.
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: the MCP server, which serves the driver's tools and their results. Edited: its run loop was split into an instance run and a session run.
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the Dioxus runtime probe, which walks the vdom for its roots, listeners and dynamic nodes, the walk that listeners are read from. Edited: the walk was split into helpers.
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand check of the UI graph. It checks one graph node per snapshot node, that no served id holds the reserved separator, and that state declarations agree with the code once handlers run (no undeclared or unconfirmed flag). Edited: it was split into assertion helpers.
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check that each served screen lists its actionable nodes with the verbs table, under the one actionable rule. Edited: it was split into assertion helpers.
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: the stand check that every interactive-role element reads a name and every region reads its stated name from the shared table. Edited: it was split into assertion helpers.
@task_item n=1 part=touches
Snapshot model as single source, the snapshot reading the accessibility tree, and one snapshot model with many readers: the snapshot component was restructured, with its tests moved out of line. Its readings are unchanged.
@task_item n=2 part=touches
The snapshot contract items (checked, enabled, value, password input value, file input value, clickable): their unit tests moved to an out-of-line test module. They are unchanged in body and name.
@task_item n=3 part=touches
Password value masked and password text never revealed: the masking tests for password and file inputs moved with the snapshot tests. The rule is unchanged.
@task_item n=4 part=touches
Enabled follows the disabled attribute's presence, and disabled read by attribute presence: the test that a present disabled attribute reads not enabled and not focusable, whatever its value, moved out of line. The rule is unchanged.
@task_item n=5 part=touches
The accessibility tree states and ARIA property reads (aria-selected, aria-pressed, aria-invalid, aria-checked on a switch, aria-disabled still reading enabled): their snapshot tests moved. Toggle state read per role is unchanged.
@task_item n=6 part=touches
The accessible name precedence and the aria-label and label element contract items: the snapshot tests for an aria-labelled button and a label-bound input moved. Names are unchanged.
@task_item n=7 part=touches
Hidden content kept out of the tree (display none): its snapshot test moved. The rule is unchanged.
@task_item n=8 part=touches
Keyboard activation by Enter and Space, and the range input keys: the driver's named key set they are pressed through moved into a child module of the command set. The key names and the twelve-key list are unchanged.
@task_item n=9 part=touches
Interactive nodes carry a name: the stand check enforcing it was split into an interactive-name helper and a region-name helper. The same messages and the same expectation table are used.
@task_item n=10 part=touches
Expected roles and names stated once, and the stand expectation tables: the region-name table is now read by a helper of the stand check. The table is unchanged.
@task_item n=11 part=touches
The actionable nodes listing contract item, the one actionable rule and the actionable-key rule: the stand check that every actionable node is listed with the verbs table was split into helpers. The rule is unchanged.
@task_item n=12 part=touches
UI graph mirrors the snapshot, and the served node ids reserved-separator item: the stand graph check was split into helpers for no flag, the readers agreeing and no reserved separator. The rule is unchanged.
@task_item n=13 part=touches
UI graph state registration (undeclared, unconfirmed): the stand check that declarations agree with the code once handlers run was split into helpers. The verdicts are unchanged.
@task_item n=14 part=touches
Listeners read from the vdom walk: the Dioxus runtime probe's walk was split into root, listener and dynamic-node visitors. What it reads is unchanged.
@task_item n=15 part=touches
The MCP tool results' snapshot item: the MCP server's run loop was split into an instance run and a session run. Results are unchanged.
@task_item n=16 part=touches
Snapshot readers behind the accessibility feature: the moved snapshot tests stay test-only under the same module path.
@task_item n=17 part=touches
The a11y CI leg, the accessibility test leg in the slow tier, and stand checks repeated three rounds: each ran green (a11y exit 0, stand-repeat 3 of 3 rounds green). No threshold or round count moved.
@task_item n=18 part=touches
The keyed wire form on the carriers, the actions answer fields and the snapshot diff entries: their wire-form tests' fixture builders moved into a test-only child. The shapes are unchanged.
@task_item n=1 part=untouched
Cognitive Accessibility (section 8) and its invalid-state rows are not touched. The session's cognitive complexity is a code metric for function size, not cognitive accessibility.
@task_item n=2 part=untouched
Scope and reach rows are not touched: accessibility tier 0, the platform adapter, assistive technology reach, and the defaults of the accessibility features. No feature or manifest changed.
@task_item n=3 part=untouched
Contrast ratio thresholds and colour decisions are not touched: the text contrast required ratio, the WCAG ratio helper, the stand accent colour, the scrollbar thumb stroke and the open list colours. The session measured no contrast.
@task_item n=4 part=untouched
Engine behaviour rows are not touched: Tab and Shift+Tab focus movement, focus events, inert subtrees, hit-testing, double click and the open select list's overlay, motion and row height. No engine code changed.
@task_item n=5 part=untouched
The obs log format and the not-yet-defined a11y violation schema are not touched. No log site or span changed.
@task_item n=6 part=untouched
Rows on deduplication by id and event, or no-duplicate rules, are not touched. The founder's duplication ruling concerns upstream's test clone pairs, not any accessibility rule.
