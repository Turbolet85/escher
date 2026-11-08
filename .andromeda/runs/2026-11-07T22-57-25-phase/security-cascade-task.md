@task_prose part=chunk
Builds the UI graph (v030-05): one read that gives, for the current screen of a held instance, each node, the component that rendered it, its handlers, the state they write and the nodes that read that state.
Links 1 and 2 are read by the public dioxus-core vdom walk that the stable ids already run. Links 3 and 4 come from a new escher-side signal registry in dioxus-native-dom, and the seven_guis stand's signals move onto it. There is no Dioxus fork and no blitz-dom engine change.
Adds the reacts-to-a-click reading as a closed boolean snapshot field, which moves NODE_FIELDS (thirteen today), the schema help texts, describe() and every pin that counts them. It absorbs residual res-gvelrxja.
The read crosses as a new escher-driver verb (VERBS holds twelve today) with a new outcome, its written form, its MCP tool and its recorded step. Values the graph carries read masked, and no log site carries graph content.
Reserves a virtual-node id form that collides with none of the four stable id forms, and keys the graph on the agent-visible node with an optional owner element. A contract or boundary change this brings is the founder's fork, asked at P4.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_item n=1 part=needs
The driver command schema: the new verb joins the closed static VERBS table, and its arguments use bounded argument kinds. Every call passes validate or validate_session, a typed Command or a Refusal comes back, and nothing public runs a Command that did not come from validate.
@task_item n=2 part=needs
The widening rule for the ratified carriers. A new closed-value field (the reacts-to-a-click boolean) is not a boundary widening. A new verb's carrier content that holds an open value set (component names, handler event names, registered state names) is a widening, and so the founder's fork.
@task_item n=3 part=needs
The stable element id and its four forms: author key, anchored path, component path and document path. The reserved virtual-node form needs a separator no current id can hold, so no id can make two ids equal. The id is computed on demand and written to no log, and no NodeId crosses a carrier or is kept between calls.
@task_item n=4 part=needs
Value masking: a password's or a file input's value reads as MASKED_VALUE on every carrier, the snapshot, its text and diffs included. This covers any state or control value the graph carries, never the text, its length or a file's host path.
@task_item n=5 part=needs
Logging of the graph's content: no log site, span field or print carries a node, a component name, a handler, a state or a value. The one command span per call holds fixed words, bools and counts only.
@task_item n=6 part=needs
The escher_telemetry sink allowlist: escher targets have their content-named fields redacted, engine targets keep only the allowlisted fields, and records from other targets are dropped. Any new escher-target field the graph reader or the new verb emits has to fit this rule.
@task_item n=7 part=needs
The driver session socket's bounds: one request line under a size bound, a closed grammar, bounded replies holding ids, names, masked control values, snapshot text and diffs, and no NodeId. The new verb's answer and outcome travel on this carrier.
@task_item n=8 part=needs
The MCP server (serve_mcp): the new verb's MCP tool has typed arguments, then validate. Results are bounded, protocol errors use fixed codes with no data, and there is no print, unwrap, expect or panic outside tests.
@task_item n=9 part=needs
The escher-session command line: the new verb is read through the closed command_line reader, with the four fixed endings and nothing the caller typed written back. It installs no subscriber and reads no environment variable.
@task_item n=10 part=needs
Recorded and kept checks: the new verb's recorded step is read back only through the strict Check::from_bytes, with closed keys and schema-named arguments. Each kept check holds the masked answer. The two committed stand-Counter checks hold no name, typed text or host path.
@task_item n=11 part=needs
The typed Refusal: a closed set of eight causes with fixed names and no String field. A refusal or outcome the new verb adds stays inside this closed, fixed-text form, and only the cause's fixed name reaches a log.
@task_item n=12 part=needs
The panic paths in dioxus-native-dom: element_to_node_id unwraps, NodeHandle::node panics on a missing node, and event-data downcasts unwrap. The graph reader and the registry must take the option-returning paths (try_element_to_node_id, try_doc) and must not panic on a stale or missing id.
@task_item n=13 part=needs
Driver input-handling bounds: no unwrap, expect or panic on anything a call supplies, and no clock read in the schema or its validation. An id that names no element, or an element that cannot take the call, is refused with nothing run.
@task_item n=14 part=needs
The snapshot and diff carriers (DioxusDocument::snapshot, Snapshot::to_text, Snapshot::diff) gain the reacts-to-a-click field. A diff takes its values only from the snapshots, so masked values stay masked.
@task_item n=15 part=needs
The trust boundaries of the local session socket and the MCP stdio server: owner-only and with no authentication. The client receives whatever the graph read carries out (component, handler and state names), and what the client does with it is the client's.
@task_item n=16 part=needs
Dependency security: the graph uses only dioxus-core's public API (vdom walk, dynamic_attrs, Readable::subscribers, ReactiveContext::origin_scope), with no Dioxus fork and no new package in Cargo.lock. The cargo-deny audit leg runs over the resolved graph under --locked.
@task_item n=17 part=needs
The engine-change carve-out: a change to blitz-dom for every document would be a boundary widening. The scope expects none, and the registry graphs only the signals an app registers.
@task_standing n=1
convention: Each new test's environment read (CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR, TMPDIR) stays test-only and is recorded in the plan's list of environment values read. The library and the escher-session binary read no env var beyond RUST_LOG.
home: secret-management
@task_standing n=2
convention: Each new log line or span field on an escher target holds fixed words, bools and counts only, never content, and passes the escher_telemetry allowlist. The crate installs no subscriber.
home: logging-monitoring
@task_standing n=3
convention: Each new refusal or session error is written from a fixed table with no path and no caller-supplied text.
home: error-handling
@task_standing n=4
convention: Each new verb, argument kind or MCP tool goes through validate, with typed arguments and bounded requests and results, on every carrier: the socket, the command line and MCP.
home: input-validation
@task_standing n=5
convention: Each new test fixture or committed check holds no typed text, host path, name or secret. A check of any session other than the committed stand-Counter sessions stays owner-only and is never committed.
home: data-protection
@task_item n=1 part=not_needed
The browser app's urlbar, navigation to the external browser, link href resolution, history persistence (sqlite) and the HTTP cache. The UI graph does not touch the browser app.
@task_item n=2 part=not_needed
The net provider's file: and data: URL handling, TLS and outbound request controls. The graph reads the vdom and the registered signals and fetches nothing.
@task_item n=3 part=not_needed
Script execution in blitz-vibey-script: the ScriptFetcher, JS API arguments, script types and JS console logging. The stand is a Dioxus app and runs no document scripts.
@task_item n=4 part=not_needed
The WPT runner: its local file reads, net errors, env_logger and panic catching.
@task_item n=5 part=not_needed
rdme markdown rendering (comrak unsafe) and the CLI examples: screenshot, paint_bench, preact_script and multi_doc, with their env reads.
@task_item n=6 part=not_needed
CI secrets and supply chain: signing material, the Signed Builds environment, the publish-browser and wpt-post-results workflows, and the pinning of CI actions.
@task_item n=7 part=not_needed
CSS, SVG, font, image and stylesheet parsing and fetch validation, and the layout and paint value clamps. The chunk changes no engine parsing or rendering.
@task_item n=8 part=not_needed
Form submission, the file dialog, the clipboard, and NativeFormData validity in dioxus-native.
@task_item n=9 part=not_needed
The panic paths in the upstream engine: blitz-dom layout and table panics, blitz-net client setup, html_sink parse unwraps, and shell and runtime setup unwraps.
@task_item n=10 part=not_needed
CLI argument checks of the helper scripts apt-install.sh and coverage_floor.py.
@task_item n=11 part=not_needed
The fixtures' subresource integrity, the fixture tokens and the fixture-page requests (graphite, servo, google).
