@task_prose part=chunk
Builds a probe, in test or example code only, of the four links of the UI graph (node to component, node to handlers, handler to state it writes, state to the nodes that read it), each read through what Dioxus 0.7.3 and the dioxus-native-dom bridge already expose, with no Dioxus crate change and no engine change for every document.
Each link gets one of three verdicts (exposed, exposed with a cost, not exposed without a fork-level change), measured by runs on the stand: a lean stateful task (Counter or CRUD) and one served task, headless, in both layout modes where layout matters.
Answers the "reacts to a click" reading: is a node's click listener readable from the listener map today.
Re-verifies at HEAD the MountedData hazard: a task spawned from a key handler is first polled inside render_immediate, and a MountedData call panics RefCell already borrowed at events.rs:162. The chunk names it and brings its fix to the founder as a fork with options; it does not make the fix.
Writes one plain-words reading for the founder (each verdict with its evidence, the cost class, the forks) before the UI graph chunk is planned.
@task_file n=1 path=tests/blitz-tests/
@task_file n=2 path=examples/seven_guis/
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: the poll mechanism, the task first polled inside render_immediate while it holds the document's RefCell, re-verified at HEAD
@task_file n=4 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the MountedData RefCell already borrowed panic coordinate events.rs:162, re-verified at HEAD after the upstream sync
@task_file n=5 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: the hazard measured beside step 15, the headless measurement of the MountedData panic through the harness's poll(None)
@task_item n=1 part=needs
The dioxus-native-dom bridge as the probe's read surface: DioxusDocument, its element-id reads (data-dioxus-id, element_to_node_id and try_element_to_node_id in mutation_writer.rs) and its listener map and event names, read only, never changed.
@task_item n=2 part=needs
The stable element id the probe reads for node to component: an HTML id author key, else an anchored, component or document path. It is computed on demand and written by escher's code to no log, and no NodeId crosses any carrier.
@task_item n=3 part=needs
Nothing crosses: the probe adds no verb, argument, node key, result field, cause, session error, exit, carrier, listener, port, env var, workspace crate, log field or span. So the driver socket's closed grammar, the twelve-verb command schema and the widening rule (a new field, carrier or exit is a boundary widening) stay untouched.
@task_item n=4 part=needs
Values masked, never logged: nothing the probe reads (a component name, a handler, a signal's value) is printed, logged or put in a field, in the probe's test output and in the founder's reading alike. A password's or file input's value stays MASKED_VALUE on every carrier the probe touches.
@task_item n=5 part=needs
The MountedData hazard's components in events.rs: the MountedData call, try_doc returning None when the document is mutably borrowed, NodeHandle::node panicking with Node does not exist, and MountedError::OperationFailed. The probe measures the hazard's RefCell borrow panic path and does not repair it, because the fix is an engine change for every document, the founder's fork.
@task_item n=6 part=needs
The Dioxus nodes panic paths a handler-reading probe can meet: supported event-data conversions unwrap the platform-data downcast, and twelve dioxus event-data conversions reach todo!, unimplemented! or panic!. Link 2's click reading goes through the listener map, never through a conversion that panics.
@task_item n=7 part=needs
No fork-level change in the dependency graph: Dioxus stays 0.7.3, with no patch, vendored copy or pin move, and no package enters Cargo.lock. Builds still pass --locked and the cargo-deny audit leg stays green over the resolved graph.
@task_item n=8 part=needs
The headless stand harness the probe runs on: the harness's poll(None), settles that never wait on a load in flight, and query helpers that panic on an invalid selector (the probe passes only valid fixed selectors).
@task_item n=9 part=needs
If the served task runs through the driver session: the state directory is created 0700 (a group- or other-open one refused), the socket is 0600, the path is a parameter (never an env read or a cwd-relative default), and the label is 1 to 32 bytes of a-z, 0-9 and dash. Every call passes validate inside Session::run.
@task_item n=10 part=needs
The probe reads no environment variable beyond the registered test-time ones (CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR, CARGO_BIN_EXE_escher-session). The escher-driver library and escher-session still read none but RUST_LOG.
@task_item n=11 part=needs
escher's own log sink as the probe meets it: escher_telemetry::init's target allowlist redacts engine-target fields and escher content-named fields and drops other targets. The probe adds no log field or span, so the allowlist's lists do not change.
@task_item n=12 part=needs
The windowed shell's poll path is read from the code, not run, for the hazard's windowed half. The reading says so, unless a verdict needs a windowed run.
@task_standing n=1
convention: each new test reads no env var but the build-time CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR or CARGO_BIN_EXE ones already registered; a new test-only read is registered as test-only before it lands
home: secret-management
@task_standing n=2
convention: each new test that makes a state or recording directory creates it 0700 under the test target directory, with its files 0600 and its path passed as a parameter, and removes only what it created, non-recursively
home: input-validation
@task_standing n=3
convention: each new log line or span carries fixed words, bools and counts only, never read content, and passes escher-telemetry's target allowlist with its content-named fields redacted
home: logging-monitoring
@task_standing n=4
convention: each new test's or example's dependency is inherited with workspace = true, no package enters Cargo.lock, and every cargo leg still passes --locked under the cargo-deny audit
home: dependency-security
@task_standing n=5
convention: each committed piece of evidence holds no host path (masked), no typed text, no secret, token or credential-file name, and no kept check beyond the two committed stand-Counter checks
home: data-protection
@task_standing n=6
convention: each test diagnostic or debug dump on stdout or stderr holds nothing a caller typed or a document carries beyond the masked values
home: Logging & Monitoring
@task_item n=1 part=not_needed
The browser app's at-rest data (the sqlite history store, the HTTP cache on disk, Clear history and Clear Cache, cookies), which the probe neither reads nor writes.
@task_item n=2 part=not_needed
The remote web content boundary and the net provider's per-input checks (file: and data: URLs, fetched stylesheets, iframes, images, favicons, fonts, @font-face, SVG data): the probe fetches nothing remote.
@task_item n=3 part=not_needed
Script execution in blitz-vibey-script (the ScriptFetcher, JS fetch, module imports, script types, JS API argument checks, JS console logging): the probe runs Dioxus apps, not document scripts.
@task_item n=4 part=not_needed
The upstream-only publish-browser workflow, the Signed Builds environment, the macOS and Android signing material, and the wpt-post-results workflow with its trusted-scripts checkout and PR comment splice.
@task_item n=5 part=not_needed
The WPT runner: its local file reads, quarantine and expectation checks, env_logger, WptNetProviderError, and per-test panic catching.
@task_item n=6 part=not_needed
The upstream CLI examples and apps (screenshot, paint_bench, preact_script, multi_doc and its MULTI_DOC env reads, rdme and its comrak unsafe markdown), with their argument parsing and panic paths.
@task_item n=7 part=not_needed
The cold-agent pipe and its agent-written check, the operator's Claude Code login, and the MCP live trial: the probe runs no model session.
@task_item n=8 part=not_needed
Browser navigation surfaces: urlbar parsing, navigation to the external browser, link href resolution, URL fragments, and form submission with its method, enctype and file-path entries.
@task_item n=9 part=not_needed
The engine's layout and paint value guards and panic paths (table roots, unreachable! layout states, degenerate aspect ratios, non-finite geometry, radii and gradient clamps), and the unsafe calc-pointer blocks in stylo_taffy and blitz-dom layout. The probe changes no engine code.
@task_item n=10 part=not_needed
CSS, selector, style element and legacy-attribute parsing, and the markup numeric attributes (rowspan, select size, ol start), which the probe neither changes nor reads as a verdict.
@task_item n=11 part=not_needed
The fixture pages' own requests and subresource integrity (graphite, google, servo, gosub) and the vendored Preact copy.
@task_item n=12 part=not_needed
CI token scoping and the bootstrap phases' records (auth scaffolding, dep-audit tooling install, the logging redaction wire's upstream fmt::init remainder), which this chunk does not change.
