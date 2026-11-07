@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: DioxusDocument's two render sites, initial_build (vdom.rebuild) and poll (vdom.render_immediate), run the vdom into an in-process buffer of 0.7 WriteMutations calls while the document is not borrowed, then take the document's borrow once, replay the buffer through the existing MutationWriter, and flush the queued mounted events as before.
The result is that user code the vdom runs during render (a component body, a task it polls) can call MountedData (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) without a RefCell already borrowed panic. This is an engine change for every Dioxus document: the stand, the windowed dioxus-native launch and any app on the bridge.
Ports upstream's regression test task_polled_during_render_can_access_document into dioxus_document.rs's tests, and turns the two should_panic hazard pins in dioxus_runtime_probe into tests that set_focus moves focus without a panic, in both layout modes, with their precondition still asserted first.
Dioxus stays at 0.7 (no crate added, patched, vendored or bumped). mutation_writer.rs stays byte-identical to upstream 434e0661 where the port allows. Nothing new crosses a boundary. Behaviour is otherwise unchanged, and the existing stand_*, stale_node_mapping, incremental_oracle and other dioxus_runtime_probe tests are re-run as the witness.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the two render sites initial_build and poll (their calls to flush_queued_mounted_events), and the mod tests that gets the ported test task_polled_during_render_can_access_document
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it as it does upstream; otherwise kept byte-identical to upstream 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic hazard pins (one per layout mode) inverted, and the module text re-said
@task_file n=6 path=tests/blitz-tests/tests/stale_node_mapping.rs mark=re-reads
what: the stale_node_mapping check, re-run as a witness that behaviour is unchanged
@task_file n=7 path=tests/blitz-tests/tests/incremental_oracle.rs mark=re-reads
what: the incremental_oracle check, re-run as a witness that incremental and non-incremental layout stay identical
@task_item n=1 part=needs
The Dioxus 0.7 bridge's render path in dioxus-native-dom (DioxusDocument initial_build and poll): the document's mutable borrow is no longer held across vdom.rebuild or render_immediate. It is taken once to replay the buffer, and the mounted events are flushed in the same order as before.
@task_item n=2 part=needs
The new mutation buffer (a new module in dioxus-native-dom): it holds element ids, tag and attribute names, attribute values and text in process only. It must never be printed, logged, put in a span or carried out by any carrier. This includes the values of password and file inputs, which the snapshot shows as MASKED_VALUE.
@task_item n=3 part=needs
MountedData operations on the bridge (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size, in dioxus-native-dom's events.rs): they now run during render. The plan records how they degrade: try_doc returns None while the document is mutably borrowed, a missing node gives MountedError::OperationFailed, and NodeHandle node/node_mut panics on a node that does not exist. The new path must keep these rules and must not add a new panic path.
@task_item n=4 part=needs
Dioxus mutations as an input-validation surface: data-dioxus-id is parsed as usize, unsupported attribute value types are ignored, falsy boolean attributes are removed, and dangerous_inner_html is applied with set_inner_html. Replaying the buffered calls into MutationWriter must keep all of these, in the original call order.
@task_item n=5 part=needs
MutationWriter's id mapping (element_to_node_id unwraps; try_element_to_node_id returns an Option). The replay must not reach the unwrap with an id that is not yet mapped or is stale. The recorded panic paths for Dioxus nodes stay as listed, unless the port changes one, and then it is named with its reason.
@task_item n=6 part=needs
The engine-change-for-every-Dioxus-document rule and the widening rule: the fix is ratified as the founder's choice and crosses no new boundary. No verb, argument, node key, result field, cause, session error, exit, carrier, listener, port, env var, workspace crate, log field or span is added.
@task_item n=7 part=needs
Values the snapshot carries through the platform adapter, the driver's answers, MCP results and kept checks stay identical: the stable element id (the HTML id or a path, with no NodeId crossing), the masked password and file-input values, the snapshot text and its diff.
@task_item n=8 part=needs
The dependency graph is unchanged: Dioxus stays at the 0.7.3 requirement (dioxus-core 0.7.10 in the lock), no 0.8 parts of upstream 286874cc1 are ported, and no package enters or moves in Cargo.lock. The cargo-deny audit leg and every cargo leg running with --locked must still pass over the changed code.
@task_item n=9 part=needs
dioxus-native-dom's tracing feature (turned on by dioxus-native's tracing feature): any log site the port adds on the render path goes through the escher_telemetry allowlist sink and must not carry mutation content. The scope allows no new log field or span.
@task_item n=10 part=needs
The hazard's mechanism as measured: poll holds inner.borrow_mut() across render_immediate, dioxus-core polls a queued task there, and set_focus borrows through doc_mut and panics. The inverted pins in dioxus_runtime_probe must show this is closed in both layout modes. The windowed dioxus-native path is read, not run.
@task_item n=11 part=needs
The panic-guard tests the plan lists under graceful degradation (stale_node_mapping's guards against invalid SlotMap key and similar panics) are re-run unchanged as witnesses of the buffered render.
@task_standing n=1
convention: a log line or span a change adds in escher's own code holds fixed words, bools and counts only, goes through escher_telemetry's target allowlist, and never carries document content, attribute text or a host path
home: logging-monitoring
@task_standing n=2
convention: every cargo build and test the change runs, its new tests included, passes --locked and the cargo-deny audit leg, and no package enters Cargo.lock without being read name by name
home: dependency-security
@task_standing n=3
convention: an env var read by a new test is registered as test-only and non-secret under Environment values read
home: Secret Management
@task_standing n=4
convention: a panic, unwrap or expect path a change adds or removes in engine or bridge code is recorded under the Panic paths or Graceful degradation entries of error handling
home: error-handling
@task_item n=1 part=not_needed
The driver session socket, its state directory and its recording directory (escher-driver serve, session.sock, 0700/0600 rules): the chunk adds no verb, carrier or listener and does not touch the driver crate.
@task_item n=2 part=not_needed
The MCP stdio server (escher-session mcp, JSON-RPC 2.0 framing, request and result bounds): untouched. No tool, result field or protocol error changes.
@task_item n=3 part=not_needed
The driver command schema and its typed refusals (validate, validate_session, Command, Refusal, SessionError): no verb, argument kind or cause is added or changed.
@task_item n=4 part=not_needed
Recorded and committed checks (Check::from_bytes, check-{n}.json, counter-cli.json, counter-mcp.json): the chunk writes, reads and changes no kept check.
@task_item n=5 part=not_needed
Remote content fetching and the net provider (file: and data: URLs, outbound request limits, TLS, the HTTP cache, iframe and stylesheet loads): the buffered render fetches nothing.
@task_item n=6 part=not_needed
Script execution in blitz-vibey-script (ScriptFetcher, script types, the JS API's argument checks, fetch GET/HEAD): the Dioxus bridge runs no document scripts.
@task_item n=7 part=not_needed
Parsers for CSS, selectors, SVG, fonts, images, favicons and legacy or numeric markup attributes in blitz-dom: the chunk edits no parser and changes no value they accept.
@task_item n=8 part=not_needed
The browser app's sqlite history persistence, its clipboard, the file dialog and the error and 404 pages: apps/browser is not reached.
@task_item n=9 part=not_needed
The CLI argument readers of the examples, agent-run.sh, cold-agent.sh, apt-install.sh, coverage_floor.py and escher-session's command_line: no argv grammar changes.
@task_item n=10 part=not_needed
CI workflow security (repository guards, the publish job's Signed Builds environment, signing material, WPT post-results trusted scripts, the token permissions on ci.yml): no workflow is edited.
@task_item n=11 part=not_needed
Authentication and authorization: none exists, and the chunk adds none.
@task_item n=12 part=not_needed
rdme's markdown rendering with comrak unsafe raw HTML, and the WPT runner's local file reads and net provider: neither is on the Dioxus bridge.
@task_item n=13 part=not_needed
Form submission and NativeFormData validity in dioxus-native: the chunk changes no form path.
