@task_prose part=session
The chunk MountedData buffered render ported upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: DioxusDocument::initial_build and Document::poll now render into a crate-private MutationBuffer with the document unborrowed, then take one borrow_mut, replay through one MutationWriter and flush mounted events unchanged.
It retired the RefCell already borrowed panic of a MountedData call (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) in a task polled inside render_immediate, on every Dioxus document; an engine change, flagged not upstreamable, nothing sent.
The probe's two should_panic pins became mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental (helper spawned_focus_acts), asserting the spawned task's set_focus acts; the probe keeps 10 tests and no should_panic.
Measured: upstream's ported test task_polled_during_render_can_access_document passes but is insensitive to the borrow on Dioxus 0.7 (dioxus-core 0.7.10 scheduling); the two probe pins are the fix's discriminating witness (they fail with the borrowed render restored).
No dependency, manifest, lock, deny.toml, schema, config, CI step or leg moved; dioxus-native-dom unit tests 82 to 83; CI#42371051720 green 17/17.
At the wrap resume the operator directed residual owner sites re-said to their rows' status, and the stated limit recorded as an open residual row (target next).
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
Error Handling's record of the spawned-task MountedData panic path (stated as standing, pinned by two should_panic tests, its fix owned by the route entry MountedData buffered render) is now closed: the RefCell already borrowed panic is removed on every Dioxus document by the buffered render; the mutation control shows it returns without the fix.
@task_item n=2 part=touches
Any security-plan text naming the probe's hazard pins mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental / _incremental or their helper press_with_spawned_focus: renamed and inverted to mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental / _incremental with the track_caller helper spawned_focus_acts; 0 should_panic remain and the probe file still holds 10 tests.
@task_item n=3 part=touches
Any text calling upstream's task_polled_during_render_can_access_document the witness of the fix: on Dioxus 0.7 it is ported as upstream wrote it, passes, and is insensitive to the borrow; the fix's witness on 0.7 is the probe's two spawned_acts pins.
@task_item n=4 part=touches
Any text stating upstream 286874cc1 is still to be ported: its buffered render is now ported onto 0.7; the Dioxus 0.8 move (0.8.0-alpha.3) is still not taken, and keyboard_compat.rs's retirement still waits on it.
@task_item n=5 part=touches
Graceful degradation's try_doc returning None when the document is mutably borrowed, and MountedError::OperationFailed for missing nodes: re-read, unchanged; a render no longer holds the borrow while user code runs, so a task polled during render now reaches the document instead of meeting the borrow.
@task_item n=6 part=touches
The mounted-event skip (upstream's element_removed_before_its_mounted_event_is_flushed left out, reading 1 mounted event on 0.7 where it asserts 0): still true; it stays out for Dioxus 0.7's scheduling, not the borrow, and returns with the Dioxus 0.8 move, owned by residual res-6ssktw6x (absorbed into v030-03) through the open CARRY cry-mjwr45xr on Upstream sync ahead of determinism and exploration.
@task_item n=7 part=touches
Owner sites naming residual res-llkgqygv (the stand's platform accessibility adapter and its windowed AT witness, security-plan fct-agnqdsgq, fct-d2yejoup, fct-q2xi4t6k) with no status: each re-said as owned by residual res-llkgqygv, re-carried at 0.3.0 (target next).
@task_item n=8 part=touches
Line citations into packages/dioxus-native-dom/src/dioxus_document.rs move: 52 lines added in 8 ranges (7, 90-91, 155, 166-168, 171, 302-304, 307, 846-885); citations into events.rs and mutation_writer.rs (try_element_to_node_id, element_to_node_id at 41-49) are unmoved, mutation_writer.rs byte-identical.
@task_item n=9 part=touches
Panic-path inventory for Dioxus nodes and the no-panic rule: the new module adds no unwrap, expect or panic!, and no slicing panic path (spans read by strings.get(span).unwrap_or_default()); the Dioxus nodes panic paths (event-data downcast unwraps, NodeHandle::node panics, element_to_node_id unwrap) stand unchanged.
@task_item n=10 part=touches
Dioxus mutations input validation (data-dioxus-id parse, unsupported attribute values ignored, falsy booleans removed, dangerous_inner_html via set_inner_html): the mutations are now recorded and replayed in order through the same MutationWriter, so these rules hold unchanged; the buffer clones attribute values and copies text into one arena, in process only.
@task_item n=11 part=touches
The engine-change record for every Dioxus document (the stand, the windowed dioxus-native launch, any app on the bridge): user code a render runs reaches the document through MountedData with no borrow panic; its merge surface (the two render sites, the field, the ported test, the new module, one mod line) is re-read at the next upstream sync under res-3v42legm, absorbed into v030-03, owned by the CARRY cry-mjwr45xr.
@task_item n=12 part=touches
The limit by construction: a task polled during render acts on the document as the previous render left it, so set_focus_to returns false for a node created in the same render; recorded as an open residual row (target next).
@task_item n=13 part=touches
Per-OS CI tallies dated at CI#41333172440: true as dated; the new reading is CI#42371051720 at e26f1a207, green 17/17 (linux 220 result lines, 1310 passed, 0 failed, 12 ignored); the stand 237 passed, 0 failed, 7 ignored over 51 files, unchanged.
@task_item n=14 part=touches
Dependency statements for the Dioxus bridge: unchanged and re-read; Dioxus stays 0.7.3 with dioxus-core 0.7.10 in the lock, no package entered or left, and the audit leg green.
@task_fact n=1 slug=error-handling
fact: dioxus-native-dom's crate-private MutationBuffer (its render-path recorder) holds no unwrap, expect or panic!, and reads each recorded text span by strings.get(span).unwrap_or_default(), so it has no slicing panic path; its spans are Range<usize> into one arena, needing no u32::try_from(...).expect(...).
@task_fact n=2 slug=error-handling
fact: initial_build and poll on DioxusDocument render into the MutationBuffer with the document unborrowed, then take one borrow_mut, replay through one MutationWriter, drop the borrow and flush mounted events; a task polled during render that calls MountedData acts on the document as the previous render left it, so set_focus_to returns false for a node created in that same render, an open residual (target next).
@task_fact n=3 slug=logging-monitoring
fact: dioxus-native-dom's MutationBuffer adds no log site, print, env read or listener; its Debug prints len alone, it holds ids, tag and attribute names, attribute values and text in process only, and its replay re-fires MutationWriter's own trace! calls, so the log stream is the one the direct write gave.
@task_item n=1 part=untouched
Threat Model Summary's attack surfaces: no vector, listener, socket, port, carrier or exit was added; the change is internal to the Dioxus render path, which dioxus-core alone calls.
@task_item n=2 part=untouched
Authentication & Authorization and Secret Management: no auth code, token, secret, CI secret or environment variable read was added or moved.
@task_item n=3 part=untouched
Dependency Security's audit tool, ignores, pinning and supply-chain rows: no manifest, lock or deny configuration was edited, and no dependency was added, removed or bumped.
@task_item n=4 part=untouched
escher-driver's input rules (session socket, state and recording directories, command schema and validate, labels, recorded checks, escher-session argv, MCP server): the driver, its carriers and its widening rule are not reached.
@task_item n=5 part=untouched
Data Protection (TLS, sqlite history, HTTP cache, kept and committed checks, MASKED_VALUE masking of password and file inputs, clipboard and file dialog): no stored medium or masking rule moved.
@task_item n=6 part=untouched
Upstream engine panic paths and degradations in blitz-dom, blitz-net, blitz-html, blitz-shell, stylo_taffy, the apps, examples and the WPT runner: none of those crates was touched.
@task_item n=7 part=untouched
Logging & Monitoring's escher_telemetry allowlist sink, the agent-run and cold-agent pipes' metadata-only logs, and the harness gates: no subscriber, leg, CI step or agent-run surface changed, and the probe stays out of run stand, stand-repeat and the a11y leg.
@task_item n=8 part=untouched
Cells' synchronous MountedData::scroll_to calls and keyboard_compat.rs with its coverage exclusion: both kept unchanged.
@task_item n=9 part=untouched
Bootstrap phases and the API Security outbound-request controls: no phase was discharged or reopened and no outbound request or navigation rule moved.
