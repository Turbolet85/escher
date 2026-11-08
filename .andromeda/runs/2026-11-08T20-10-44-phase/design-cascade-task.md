@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of kept checks has been replayed on the stand, it lists exactly the UI graph's handlers (a graph node's id with its optional owner, and a handler's event name) that none of them exercised.
States the canvas limit in the acceptance: only DOM listeners on agent-visible nodes count, one per (node, event); handlers a canvas dispatches inside one listener are not counted, and 0.4.0 extends the count to virtual nodes.
Writes the owed replay test (CARRY cry-sxrivdk3): a recorded session holding an `actions` step, kept, read back through Check::from_bytes and replayed through Check::replay on a fresh session, holding; a differing `actions` answer names the step and the key. v030-06 reads verified once it passes.
Reads the UI graph, the snapshot, the stable-id grammar and the `actions` verb without changing their shapes; the coverage read is read-only over the screen.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=4 path=packages/escher-driver/src/replay.rs
@task_file n=5 path=packages/escher-driver/src/recording_tests.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/command.rs
@task_file n=8 path=packages/escher-driver/src/execute.rs
@task_file n=9 path=packages/escher-driver/src/describe.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/recording.rs
@task_file n=12 path=packages/escher-driver/src/mcp.rs
@task_file n=13 path=packages/escher-driver/src/mcp/listing.rs
@task_file n=14 path=packages/escher-driver/src/cli.rs
@task_file n=15 path=packages/escher-driver/src/schema_tests.rs
@task_file n=16 path=packages/escher-driver/src/cli_tests.rs
@task_file n=17 path=.github/scripts/test_capability_statement.py
@task_file n=18 path=docs/capabilities.md
@task_file n=19 path=README.md
@task_item n=1 part=needs
The handler-coverage command itself: its answer lists (graph node id with optional owner, handler event name) pairs that no replayed kept check exercised, keyed on the graph's node id, never on a NodeId.
@task_item n=2 part=needs
The UI graph as the `graph` verb carries it on the cli surface (cli-graph): GraphNode.handlers and GraphHandler.event, node ids with their optional owner; the command reads this shape and leaves it unchanged.
@task_item n=3 part=needs
The `actions` verb's answer on the cli surface (cli-actions): the owed test keeps an `actions` step's answer whole and compares it on replay; a differing answer names the step and the differing key.
@task_item n=4 part=needs
The pending unpaged graph and unpaged actions items (cli-unpaged-graph, cli-unpaged-actions): the coverage count reads the graph's handler set whole and the kept `actions` answer is compared whole, so any page bound on those answers reaches the count and the test.
@task_item n=5 part=needs
The canvas limit: one DOM listener a canvas or custom widget holds (desktop-native-custom-widget) counts once per (node, event); the handlers it dispatches internally are not counted, and the acceptance says so, leaving 0.4.0's virtual-node extension open.
@task_item n=6 part=needs
The record of which (element, event) handlers a run reached: the graph's confirmed flag and the signal registry's dispatched events hold only where an app registered a state, and the count must hold on every app, registry or not.
@task_item n=7 part=needs
The check replay path: Check::from_bytes then Check::replay, each instance-verb step run through Session::run on a fresh session of the same app; the coverage command and the owed test both ride it.
@task_item n=8 part=needs
The read-only boundary over the coverage answer: it runs no step of its own and carries no control value, typed text, page text beyond ids, component and event names, or NodeId.
@task_item n=9 part=needs
The driver's verb count, if the command is a new verb: VERBS fourteen to fifteen and every pin holding that count (capability statement test, docs/capabilities.md, README tool count, schema_tests, cli_tests, stand_mcp_tools); the verb is named to the Cold-agent bar, not counted here.
@task_item n=10 part=needs
The ledger states: v030-07 claimed by this chunk with its integration acceptance; v030-06 moves from implemented to verified only once the owed replay test passes.
@task_standing n=1
convention: every answer, refusal or session error a driver command emits on the command line or its mcp role is JSON only on stdout, one line per command or newline-delimited JSON-RPC 2.0, never colour or an ESC byte, the same piped or not, with the exit status carrying the class (0 accepted, 1 refused, 2 usage, 3 session error; an mcp Err ending exits 3)
home: surface (Surface: cli, Component Patterns, cli-output)
@task_item n=1 part=not_needed
Color Palette, its token sets and the select list's engine-painted colours: the coverage command and the replay test paint and compare no colour.
@task_item n=2 part=not_needed
Typography, font loading and the screenshot's fonts-unavailable refusal: the chunk takes no screenshot and measures no text.
@task_item n=3 part=not_needed
Spacing: no layout value is set or read by the coverage count or the owed test.
@task_item n=4 part=not_needed
Depth Strategy and its hit-testing clip rules: the coverage read dispatches no pointer and changes nothing on the screen.
@task_item n=5 part=not_needed
Border Radius: no radius is drawn or measured.
@task_item n=6 part=not_needed
Motion's into-view scroll rule and reduced-motion absence: the chunk adds no scrolling and no animation.
@task_item n=7 part=not_needed
Iconography, cursors and favicons: the command has no visual output.
@task_item n=8 part=not_needed
Desktop-native window, titlebar, devtools overlay and stack: the chunk builds no shell or window change.
@task_item n=9 part=not_needed
Mobile-native surfaces (url bar, back button, capture, stack, tokens): the chunk touches no mobile build.
@task_item n=10 part=not_needed
Web-spa surface and its WASM host canvas: that canvas is the page the native app renders into, not the in-app canvas whose internal handlers the acceptance leaves uncounted; no canvas renderer is built.
@task_item n=11 part=not_needed
Cli tokens of the WPT runner and the paint_bench and screenshot example binaries (owo-colors terminal output): the coverage command is a driver surface, not those binaries.
