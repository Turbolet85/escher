@task_prose part=chunk
Settles three driver debts left by 0.1.0–0.3.0, each put to the founder as a fork with every option's cost, then fixed and proven on the stand through the driver's carriers or kept as a stated residual by his word.
Debt 1: the driver's press verb holds no Ctrl, so a non-adjacent set of a multiple select cannot be built through the driver; the fork adds a Ctrl (and by kin Meta/Alt) modifier beside the Shift one, or keeps the limit stated in help.
Debt 2: the UI graph read is answered whole (293,869 bytes on Cells at boot); its ruled form is graph read in numbered parts, paged as the snapshot text is, every part within the 1,048,576-byte answer bound.
Debt 3: handler coverage reads an ancestor's handler as exercised when a bubbling dispatch was stopped short of it; the fork reads the stop into the reached-handler record, or keeps the stated limit.
Re-reads the capability statement's stated limits and its pin, and re-runs the stand's graph, handler-coverage, MCP-tool and act checks over the changed verbs.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema — the verb table with each verb's argument kinds and bounds — edits: the press verb's modifier argument, the graph verb's part argument
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's verb specs, argument kinds and their bounds — edits: the press and graph verb entries
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command form of each verb carried on the session socket's call — edits: the press and graph commands
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description of its verbs, the help that states the press limit — edits: the press and graph descriptions
@task_file n=5 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's execution of each verb against the session — edits: press with a modifier, graph by part
@task_file n=6 path=packages/escher-driver/src/cli.rs mark=edits
what: the driver's command line carrier of the verbs — edits: the press and graph arguments
@task_file n=7 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP carrier and its tool listing — edits: the press and graph tools' arguments
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the kept check's recording of verb calls and their argument shapes
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check's replay of recorded verb calls and their argument shapes
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the handler coverage answer and the coverage line of escher-session coverage
@task_file n=11 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph, one node per snapshot node, read-only — edits: the graph read split into parts
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the Dioxus document's reached-handler record (record_reached, reached_handlers) — edits: the record of a dispatch's chain under a stopped propagation
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: the engine's event dispatch where a stopped propagation is decided
@task_file n=14 path=docs/capabilities.md mark=edits
what: the capability statement, edited by hand — edits: the stated limits of press, graph and handler coverage
@task_file n=15 path=.github/scripts/test_capability_statement.py mark=edits
what: the pin of the capability statement's text
@task_file n=16 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the UI graph read
@task_file n=17 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=edits
what: the stand check of handler coverage
@task_file n=18 path=tests/blitz-tests/tests/stand_mcp_tools.rs mark=edits
what: the stand check of the MCP tool listing and its tools' arguments
@task_file n=19 path=tests/blitz-tests/tests/stand_act_keys.rs mark=edits
what: the stand check of the driver's press verb and its keys
@task_file n=20 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand check of driver acts on selects, the multiple select's key check
@task_item n=1 part=needs
The forks file: each debt's fix-or-keep is the founder's, every option costed first; technical forks that change nothing of what the product is are the operator's.
@task_item n=2 part=needs
A multiple select's chosen set built by keys and row clicks: a Ctrl modifier on press must reach the multiple select's key handling so a non-adjacent set is built through the driver, its form event value the full set.
@task_item n=3 part=needs
The keyed field on the carriers: a new modifier on press and a new part on graph cross as arguments of a closed value set on the ratified verbs, on the session socket's call, the command line, MCP and a kept check alike, and no new carrier.
@task_item n=4 part=needs
The UI graph mirrors the snapshot one node per node, read-only, carrying no values: a part-paged graph read keeps those rules in every part.
@task_item n=5 part=needs
The snapshot text form's parts: graph parts are paged as the snapshot's are, each within the 1,048,576-byte answer bound and the MAX_PART bound of 1000.
@task_item n=6 part=needs
The UI graph state registration (use_graph_state, undeclared, unconfirmed, no registered state) as read in each part of a paged graph.
@task_item n=7 part=needs
The Dioxus document's reached-handler record (record_reached, reached_handlers): under a fix, an element of a dispatch's chain past a stopPropagation is not recorded as reached.
@task_item n=8 part=needs
The handler coverage answer and escher-session coverage line: an ancestor handler a stopped dispatch never reached must not read exercised, or the limit stays stated.
@task_item n=9 part=needs
Listeners read from the vdom walk: the reached record judges a handler by the element's current listeners for the event, the source a stop reading must keep.
@task_item n=10 part=needs
Keyboard activation's handler order and Enter/Space as a click: a modifier on press must not change which handlers a plain press runs or their order.
@task_item n=11 part=needs
Tab and Shift+Tab move focus: a new modifier sits beside the existing Shift one and leaves the Shift+Tab rule as it is.
@task_item n=12 part=needs
The MCP tool results and tool listing: the press and graph tools carry the new arguments, the graph tool's answer per part.
@task_item n=13 part=needs
The driver schema defines no role or name set: new arguments add closed value sets of modifiers and part numbers only.
@task_item n=14 part=needs
The capability statement is edited by hand: its stated limits of press, graph and handler coverage change only by the founder's ruling, and its pin follows.
@task_item n=15 part=needs
seven_guis' cli_commands check and the select's multiple-key check re-run over the changed press and graph verbs.
@task_standing n=1
convention: each new stand check runs as a Rust test over the accessibility tree and the driver
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand check states its expected nodes, names and orders once in a shared table
home: screen-reader-support
@task_standing n=3
convention: each new stand check repeats three rounds
home: ci-integration
@task_item n=1 part=not_needed
Text contrast, lint contrast findings, accent colours and scrollbar strokes: the chunk paints nothing.
@task_item n=2 part=not_needed
ARIA state and property reads and HTML-AAM role mapping: no node's role, name or ARIA reading changes.
@task_item n=3 part=not_needed
Hidden and inert content rules and hit-testing of inert elements: no exclusion rule changes.
@task_item n=4 part=not_needed
Range input, Cells grid and CRUD list key contracts: the press modifier targets the multiple select's set.
@task_item n=5 part=not_needed
The single-choice select's open and closed list keys, its list colours, motion and row heights: the single select is out of scope.
@task_item n=6 part=not_needed
Platform adapter reach, accessibility tier and the accessibility feature's defaults: no stand binary gains an adapter.
@task_item n=7 part=not_needed
Invalid state beyond colour or class, and other cognitive rules: no form state changes.
@task_item n=8 part=not_needed
Password masking, IME enabling and the driver's screenshot rules: none of those verbs or readings change.
