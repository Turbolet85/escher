@task_prose part=session
The session added one blitz-tests test target, dioxus_runtime_probe (10 tests, both layout modes), which reads public Dioxus 0.7.10 API only to measure what Dioxus exposes of component, handlers, written state and readers; no crate, symbol, manifest or file under packages or examples changed.
It re-measured the MountedData hazard at HEAD and pinned it with two should_panic tests: a task a key handler spawned is first polled inside DioxusDocument::poll's held borrow, and its set_focus panics RefCell already borrowed; the fix is routed to a chunk of its own (port of upstream 286874cc1's buffered render) on the founder's word.
An operator fix commit bumped pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock alone (fixing RUSTSEC-2026-0433) and added RUSTSEC-2026-0429 (zip 2.4.2) to the deny.toml advisories ignore, now eight IDs, the zip IDs four; the zip drop residual res-h7acz56c was revised to rev 2.
The audit leg reads advisories ok, licenses ok locally and in CI#41914044942, green 17/17; run all moved to 743 passed, 0 failed, 12 ignored, and blitz-tests to 158 .rs files and 157 test targets.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
Dependency Security, the zip and pulldown-cmark ignore and resolution text (dependency-security fct-a4qh6ioi): the zip 2.4.2 ignores go from three IDs to four (RUSTSEC-2026-0367, 0368, 0369 and now 0429), and RUSTSEC-2026-0414's pulldown-cmark 0.13.5 resolution becomes 0.13.6, which also fixes RUSTSEC-2026-0433.
@task_item n=2 part=touches
Dependency Security, the Audit tool: the deny.toml advisories ignore list now holds eight IDs, cited at deny.toml:20-30, the new line at 26; any count of ignores or deny.toml line citation moves.
@task_item n=3 part=touches
Dependency Security, the zip ignores' written reason: re-read for RUSTSEC-2026-0429. zip 2.4.2 reaches the graph only through anyrender_serialize 0.8.0 (the browser capture feature), whose deserialize opens a ZipArchive and reads entries by_name (anyrender_serialize-0.8.0/src/lib.rs:562-614) and never calls extract. No fixed release is reachable, since anyrender_serialize 0.8.0 is the latest and requires zip ^2.1.
@task_item n=4 part=touches
Dependency Security, pulldown-cmark: the update was a lock-only, semver-compatible cargo update --precise 0.13.6 with no coupled pin, reached only through rdme. RUSTSEC-2026-0433 is quadratic parsing under ENABLE_WIKILINKS, an option rdme never sets, so no path the repo uses was affected. Any pulldown-cmark version or advisory text moves.
@task_item n=5 part=touches
Dependency Security, the zip drop residual: res-h7acz56c is revised rev 1 to rev 2 and names the fourth ID, its status re-carried and disposition 0.3.0 unchanged. The residuals now live as residual rows in the store, and no .andromeda/residuals.md stands, so any text pointing to a residuals.md line for it is stale.
@task_item n=6 part=touches
Dependency Security, CI integration and update policy: the CI Dependency audit went red on CI#41911351081 for two advisories published after the base's green audit, with Cargo.lock and deny.toml unchanged. After the fix it went green on CI#41914044942. Text claiming the audit state holds between lock changes is affected.
@task_item n=7 part=touches
Dependency Security, the upstream 286874cc1 left-out note (dependency-security fct-f5jtvf4a): still true, but its buffered render is now routed to a port chunk of its own onto the 0.7 bridge. That port writes its own buffer of 0.7 WriteMutations calls and replays it into the existing MutationWriter, which keeps mutation_writer.rs byte-identical to upstream at 434e0661.
@task_item n=8 part=touches
Error Handling, the MountedData record (error-handling fct-7ni72vcj): re-measured at HEAD. inner.borrow_mut() is held at packages/dioxus-native-dom/src/dioxus_document.rs:295 across vdom.render_immediate at :297. dioxus-core 0.7.10 polls the queued task through handle_task_wakeup (virtual_dom.rs:611). set_focus is at packages/dioxus-native-dom/src/events.rs:295 and borrows through doc_mut at events.rs:162. The coordinates and the routing to the port chunk move.
@task_item n=9 part=touches
Error Handling, the MountedData contrast: set_focus called at once in a key handler, with no task, moves focus and does not panic (measured in both modes). get_scroll_offset and get_scroll_size take a shared borrow through doc at events.rs:152, read from the code and not run. The windowed path was read, not run.
@task_item n=10 part=touches
Error Handling, Panic paths (the Dioxus nodes entry) and Graceful degradation (the try_doc entry, events.rs:155-177): a measured panic path, RefCell already borrowed through doc_mut in a task polled during render, now stands beside them, and its coordinates fall inside those citations.
@task_item n=11 part=touches
Error Handling, the test inventory of panic guards: two should_panic pins (mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental and _incremental) and the contrast test mounted_data_asked_at_once_in_a_key_handler_acts join the tests that bear on panics. Each pin asserts its precondition first: the handle is held and the key target is focused by Tab.
@task_item n=12 part=touches
Any count security-plan carries of blitz-tests files or targets, or of run all totals: blitz-tests now has 158 .rs files and 157 test targets (was 157 and 156), and run all is 743 passed, 0 failed, 12 ignored (was 733). The new target is not in run stand (237 tests over 51 files, unchanged), stand-repeat or the a11y leg.
@task_item n=13 part=touches
Dependency Security, Dioxus versions: Dioxus stays at the 0.7.3 requirement, with dioxus-core, dioxus-signals and dioxus-html 0.7.10 in the lock. A version claim is re-confirmed, not moved.
@task_fact n=1 slug=dependency-security
fact: A CI Dependency audit can go red on advisories published after the base's green audit even with Cargo.lock and deny.toml unchanged; the local fast leg runs no audit, and bash .github/scripts/ci-leg.sh audit is the local reproduction to run before a push.
@task_fact n=2 slug=dependency-security
fact: RUSTSEC-2026-0429 (zip 2.4.2: ZipArchive::extract aborts the process on a symlink entry with a large declared size, fixed only in zip 9.0) is ignored by ID with the zip IDs' reason, because anyrender_serialize 0.8.0 never calls extract.
@task_fact n=3 slug=error-handling
fact: The MountedData RefCell hazard is pinned at HEAD by dioxus_runtime_probe's two should_panic tests, one per layout mode, with no fix applied; the port chunk MountedData buffered render, the founder's answer Port as a separate chunk, inverts the pin, with upstream's test task_polled_during_render_can_access_document as its witness.
@task_item n=1 part=untouched
Threat Model Summary vectors (remote content, script execution, CLI input, markdown rendering, WPT local reads, CI workflow, session socket, MCP stdio): the new test target has no runtime surface, external input or listener, so no vector or trust boundary changes; the security tier stays 0.
@task_item n=2 part=untouched
Authentication & Authorization: no auth code, token, handshake or peer check is added or removed.
@task_item n=3 part=untouched
Input Validation, the driver socket, MCP server, command schema, session label, recording and kept-check rules: no driver code moved, VERBS stays twelve, CAUSES eight, SessionError fifteen.
@task_item n=4 part=untouched
Input Validation, Dioxus mutations: data-dioxus-id parsed as usize (fct-qk7naa6l, input-validation fct-omhuceqe) is still true. The finding that the listener mark over-includes an element whose listener was removed is an a11y-plan concern, not a parsing rule.
@task_item n=5 part=untouched
Data Protection, masking (MASKED_VALUE, file-input path) and kept or committed checks: no snapshot field is added (NODE_FIELDS stays thirteen), and the new test carries no value, signal value or node id in its failure messages.
@task_item n=6 part=untouched
Secret Management, environment values read: the new test file reads no env var (silence census 0), and no secret source is added.
@task_item n=7 part=untouched
Logging & Monitoring, escher_telemetry's sink, the agent-run and cold-agent logs, stdout output: the new test prints nothing and installs no subscriber, and no log site or event shape moves.
@task_item n=8 part=untouched
API Security, outbound requests, the session socket and navigation: no listener, request or bound changes.
@task_item n=9 part=untouched
Error Handling, the downcast_doc_mut entry (error-handling fct-6rk5sgno): it is a different symbol that only shares the coordinates token, and it is unchanged.
@task_item n=10 part=untouched
Error Handling, escher-driver's typed Refusal and SessionError tables and the MCP protocol errors: unmoved.
@task_item n=11 part=untouched
Dependency Security, Unsafe code, Vendored JS, CI tooling pins and the Signed artifacts of Supply chain integrity: no tool, action pin, unsafe block or vendored file moved; rustc stays 1.99.0.
