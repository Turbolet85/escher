@task_prose part=chunk
Adds one read-only driver read, beside snapshot, lint and graph, that lists each actionable node on the current screen with its stable id, its owner where its handler sits on another element, its verbs, and per verb whether a call would be accepted now or the one refusal cause (disabled, covered, off-screen) the call would return.
The node model is the UI graph's GraphNode with an optional owner (a select serving its options and groups), never keyed on a DOM NodeId; the verbs are read from the schema's verb table VERBS with no closed list of verb names in this code, which may need VerbSpec to say more (a contract change, the founder's fork).
The read and the executor decide a target's refusal through one shared path (today action_point in execute.rs: disabled, then off-screen, then covered), so the list agrees with a real call by construction.
The actionable-key check (residual L12) may fold in, each listed node saying whether its id is an author key or a positional path, if P4 finds it fits.
Proven on the stand in both layout modes, with a minimal fixture where the stand lacks a covered, an off-screen or an owner-served case; no engine change is expected.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the verb table VERBS and the command and refusal schema (a new verb, possibly new VerbSpec fields)
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: VerbSpec (what a verb states: name, level, help, arguments, result fields, possibly its targets and refusals)
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: action_point, the shared refusal path the read and the executor both call
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the written form of the new read's answer
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the command line's verb and usage
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the MCP tool listing
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the MCP result of the new read
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: recording the new read in a kept check
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: replaying the new read from a kept check
@task_file n=10 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capability statement and its count of driver commands
@task_file n=12 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability-statement pin over docs/capabilities.md and the README's command count
@task_file n=13 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=14 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=15 path=packages/dioxus-native-dom/src/actionable.rs
@task_item n=1 part=needs
The driver session contract: one Session running every call through Session::run after validate, the command and refusal schema stated once as static data (its count of verbs and closed causes), no refusal holding what a call supplied, carried by the v3 wire, the command line, MCP and the command and tool_call spans.
@task_item n=2 part=needs
The driver session boundary decision: ids, names, closed flag words and booleans may cross, never a NodeId; a new carrier or a new verb on every ratified carrier widens the boundary, which is the founder's fork.
@task_item n=3 part=needs
The UI graph and snapshot of the Dioxus DOM bridge: DioxusDocument::graph (UiGraph of nodes), the snapshot read read-only with no NodeId, and the snapshot's clickable reading an element's own current click listener on no disabled form control.
@task_item n=4 part=needs
Stable element ids in four tiers (author key, anchored path, component path, document path) and the reserved owner form spelled by virtual_node_id, which the node model's optional owner must stay open to.
@task_item n=5 part=needs
The actionable-key rule: every element an agent can act on reads an author key, and unkeyed_actionable lists, read-only, each actionable element whose stable id holds a slash, with its remedy; its actionable set (focusable, interactive role, listener) bounds which nodes the read lists.
@task_item n=6 part=needs
The refusal semantics the shared path must keep: the disabled reading through ElementData::is_disabled and option_disabled, seeing read through visible_region never from a hit, a hit stopping at clipping boxes and an open select list answered first, and an option of a closed single select staying refused covered.
@task_item n=7 part=needs
The select tests the driver reads (closed_select_of, is_select via locate, is_masked_input) and the select model's options as the select's children in the snapshot: the owner-served case today is a select serving its options and groups.
@task_item n=8 part=needs
Every driver answer written by hand in the crate-private json module, no serialization crate writing one, serde_json only reading MCP requests and kept checks, and no command's ending writing back what the caller typed.
@task_item n=9 part=needs
The command span records verb and, refused, cause alone; no log or event carries the read's content, its size, an id, a name or a value.
@task_item n=10 part=needs
The recording and replay of a kept check: answered calls kept owner-only in the launcher-named recording directory and replayed through Session::run; the two committed stand checks under tests/blitz-tests/checks are the one owner-only exception.
@task_item n=11 part=needs
The escher-session command line over the six served stand tasks, answering one line of JSON with exit codes 0 accepted, 1 refused, 2 usage, 3 session error, and the MCP server named escher listing the driver's commands as tools.
@task_item n=12 part=needs
The headless stand (seven_guis::stand: four lean tasks, Cells, the load page, pinned 800x600 Light viewport) on which the read is proven, in both layout modes.
@task_item n=13 part=needs
The front page README.md's count of driver commands an MCP client gets as tools, read by the capability-statement pin, which a new read changes.
@task_item n=14 part=needs
The engine-change rule: the engine's direct changes are ratified one by one and flagged upstreamable; any engine change this chunk finds it needs is the founder's fork, not taken here.
@task_item n=15 part=needs
The read changes nothing: like Session::lint and the graph, it runs no step, dispatches nothing, moves no virtual clock time and waits on no load.
@task_standing n=1
convention: each new test sits in a cfg(test) tests module at the end of its source file or a path child file; each integration test file covers one behavior, opens with a doc comment naming it, runs its scenario in both layout modes, and first asserts the fixture produces the condition
home: conventions
@task_standing n=2
convention: each new stand fixture or shared page is stated once in tests/blitz-tests/tests/fixtures/mod.rs, a page with one reader staying in its check's file, and stand checks share helpers through non-target modules under tests
home: conventions
@task_standing n=3
convention: each new module or file opens with a doc comment, public items carry doc comments, and constants state their rationale
home: conventions
@task_standing n=4
convention: each new line passes cargo fmt, clippy with -D warnings and rustdoc -D warnings through the ci-leg.sh legs, with expect_used or unwrap_used allowed only beside a justification
home: conventions
@task_standing n=5
convention: each new span or log line in escher-driver is built by hand with tracing ungated, no instrument attribute, carrying no id, name, value or read content
home: cross-cutting-patterns
@task_standing n=6
convention: each new crate-internal item is pub(crate) or pub(super), and each new error type is an enum with Display and From per wrapped error
home: conventions
@task_item n=1 part=not_needed
The browser app, its history store, document loader, URL suggester and about pages: the chunk reaches only the driver, the Dioxus bridge's reads and the stand.
@task_item n=2 part=not_needed
The WPT runner, its outputs, bucketing and CI WPT workflows: nothing in the chunk runs or changes them.
@task_item n=3 part=not_needed
JavaScript on Boa (blitz-vibey-script), its bootstrap, timers and module loader: the stand is Dioxus and the read touches no script.
@task_item n=4 part=not_needed
Networking (blitz-net), outbound hosts, the user agent and resource loading: the read makes no fetch.
@task_item n=5 part=not_needed
Painting, renderer backends, the password mask and screenshot rendering: the read paints nothing and takes no shot.
@task_item n=6 part=not_needed
Fonts, CSS approximations, unsupported CSS features and Taffy layout internals: the chunk changes no style or layout behaviour.
@task_item n=7 part=not_needed
Upstream sync bookkeeping and the merge surfaces of the ratified engine changes: the chunk merges nothing from upstream.
@task_item n=8 part=not_needed
Nix packaging, cross-compilation, the publish-browser deployment and release versioning: no packaging or release change.
@task_item n=9 part=not_needed
The shell loop, windowing, launch paths, hot reload and the accessibility adapter: the read runs headless through the driver session.
@task_item n=10 part=not_needed
The example fixture forms (google and graphite pages) and the HTML fixture pages under examples/assets: the chunk's fixtures are stand fixtures.
@task_item n=11 part=not_needed
Drag, virtual-node production, the canvas and two-target verbs: they stay in 0.4.0, the shapes only kept open.
@task_item n=12 part=not_needed
Handler coverage (v030-07) and auto-exploration (v030-09): their own entries.
