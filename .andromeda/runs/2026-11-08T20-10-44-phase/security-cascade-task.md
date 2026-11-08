@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of kept checks (check-{n}.json, read by Check::from_bytes and run by Check::replay) has run on the stand, it lists exactly the UI graph's handlers (graph node id with its optional owner, handler event name) that none of those checks exercised.
Adds a record of which (element, event) handlers a run reached that holds on every Dioxus app, registry or not, read beside the bridge's event dispatch and the SignalRegistry; the coverage read is read-only, runs no step and carries no NodeId, control value or typed text.
States the canvas limit in the acceptance: only DOM listeners on agent-visible nodes count, one per (node, event); handlers a canvas dispatches inside one listener are not counted, and 0.4.0 extends the count to virtual nodes.
Writes the owed replay test (CARRY cry-sxrivdk3) for v030-06: a session holding an actions step is recorded, kept, read back through Check::from_bytes and replayed on a fresh session, the kept actions answer matching, and a differing answer naming the step and the differing key.
How checks reach the command (in-process replay or a new verb naming checks) is a P4 fork; a new verb would move the driver's verb table from fourteen to fifteen and every pin of that count.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=4 path=packages/escher-driver/src/replay.rs
@task_file n=5 path=packages/escher-driver/src/recording.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/command.rs
@task_file n=8 path=packages/escher-driver/src/execute.rs
@task_file n=9 path=packages/escher-driver/src/describe.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/mcp.rs
@task_file n=12 path=packages/escher-driver/src/mcp/listing.rs
@task_file n=13 path=.github/scripts/test_capability_statement.py
@task_file n=14 path=docs/capabilities.md
@task_item n=1 part=needs
The driver command schema: validate checks every call against a closed table of fourteen verbs and seven bounded argument kinds; a new coverage verb or argument kind must join that table, pass validate, and keep the fixed refusal order, and nothing public may run a Command that did not come from validate.
@task_item n=2 part=needs
The widening rule over the session socket and MCP carriers: a new field of a closed value set inside a ratified carrier is no widening, while a new verb, an argument naming a path on disk, an open value set or a new carrier is a boundary widening that escalates to the founder.
@task_item n=3 part=needs
What the socket and MCP replies may carry: ids, names, a UI graph's ids, component, state and handler event names, closed words and booleans, never an attribute's text and no NodeId; a coverage answer of graph node ids, owner ids and handler event names must stay inside that set.
@task_item n=4 part=needs
The stable element id grammar: the coverage list keys on the graph's stable id (with its optional owner), never a NodeId kept between calls, and the grammar keeps its shape.
@task_item n=5 part=needs
The recorded check reader Check::from_bytes: a kept check's bytes are untrusted, read strictly under byte and record bounds with closed keys, anything else refused malformed holding nothing of the file, no panic path; every replayed step still passes validate inside Session::run, which the owed actions replay test exercises.
@task_item n=6 part=needs
Kept check files and the recording directory: created 0600 in a 0700 directory, a group- or other-open directory refused, never replaced, no verb or result field naming the directory, nothing of it logged; the replay test's kept check lives so.
@task_item n=7 part=needs
Committed checks: only counter-cli.json and counter-mcp.json are committed; a check the new tests record (an actions step included) stays owner-only and is never committed.
@task_item n=8 part=needs
Masking of control values: a password's or file input's value reads as MASKED_VALUE in every carrier and kept check; the actions answer the replay compares and the coverage answer carry no control value or typed text.
@task_item n=9 part=needs
The Dioxus bridge dispatch: the event dispatch sets a registered SignalRegistry's dispatch around runtime.handle_event and clears it after, inert where no state is registered; a new reached-handler record beside it must take dioxus-core types alone, hold no state value, and have no log site, print, env read or unwrap on app input.
@task_item n=10 part=needs
The UI graph and the actions verb as carried on the socket and MCP (graph node ids, component, state and handler event names, actions answers' ids, owner ids, verb words, cause words, keyed booleans): the chunk reads them and must not change their shapes.
@task_item n=11 part=needs
Typed refusals and session errors: a refusal of a coverage call is one of the closed causes with a fixed name, meaning and remedy, no String field, nothing a call supplied; MCP protocol errors fixed codes with no data.
@task_item n=12 part=needs
The command span and escher's log sink: one command span per call holds fixed words, bools and counts only, never a call's content or the screen; a coverage count logged under an escher_ target passes the escher_telemetry allowlist, the crate installing no subscriber and reading no env var.
@task_item n=13 part=needs
The escher-session command line: a new verb is read by the closed command_line reader, validated before the session address is asked for, with the four fixed exit statuses and no ending echoing what the caller typed.
@task_item n=14 part=needs
The MCP server role: a new tool's arguments are typed then validated, results bounded, no print, unwrap, expect or panic outside tests, the request id the one echoed value.
@task_item n=15 part=needs
serde_json as the reader of a kept check's bytes and of the driver's answer line the replay compares (no preserve_order), writing nothing; the actions answer equality the test asserts reads through it.
@task_item n=16 part=needs
The driver's env reads: the escher-driver library and escher-session read no env var beyond RUST_LOG; the new tests reach their directories only through CARGO_TARGET_TMPDIR, CARGO_MANIFEST_DIR or the workspace target/tmp.
@task_standing n=1
convention: each new test that records or keeps a check writes it 0600 in a 0700 directory under the test target directory, never commits it, and reads it back only through Check::from_bytes
home: data-protection
@task_standing n=2
convention: each new test reaches its directories only through compile-time CARGO_TARGET_TMPDIR or CARGO_MANIFEST_DIR, never a new runtime env read
home: secret-management
@task_standing n=3
convention: each new log line or span field under an escher_ target holds fixed words, bools and counts only and passes the escher_telemetry allowlist
home: logging-monitoring
@task_standing n=4
convention: each new or changed CI cargo command passes --locked, and every ci.yml uses: stays pinned to a commit SHA
home: dependency-security
@task_item n=1 part=not_needed
Browser urlbar, external-browser navigation, link href resolution, and the net provider's file: and data: URL handling: the chunk adds no navigation or fetch.
@task_item n=2 part=not_needed
Script execution in blitz-vibey-script (script types, ScriptFetcher, JS API arguments, JS console logging): the chunk runs no document script.
@task_item n=3 part=not_needed
CSS, selector, SVG, font, image, favicon and iframe parsing and their fallbacks: the chunk parses no remote content.
@task_item n=4 part=not_needed
Browser history persistence in sqlite, the HTTP cache, clipboard and the native file dialog: the chunk stores none of them.
@task_item n=5 part=not_needed
TLS, outbound request limits, CORS and same-origin absence: the chunk makes no network request.
@task_item n=6 part=not_needed
CI signing material, the publish-browser workflow, the Signed Builds environment and WPT_GITHUB_TOKEN: the chunk touches no publish or WPT workflow.
@task_item n=7 part=not_needed
The WPT runner's local file reads, net errors, panics and env_logger: the chunk does not reach the WPT runner.
@task_item n=8 part=not_needed
rdme markdown rendering with raw HTML and the screenshot, paint_bench, preact_script and multi_doc examples' CLI arguments and panic paths: not reached.
@task_item n=9 part=not_needed
apt-install.sh and agent-run.sh argument checks: the chunk changes neither script.
@task_item n=10 part=not_needed
Engine panic paths in layout, paint, parsing and shell setup, and stylo_taffy's unsafe calc blocks: the chunk changes no engine layout or paint code.
@task_item n=11 part=not_needed
Fixture pages' remote requests, subresource integrity and origin-trial tokens: no fixture is touched.
@task_item n=12 part=not_needed
Form submission, NativeFormData validity and legacy markup attribute parsers: the chunk submits no form and reads no new attribute.
