@task_prose part=session
The Dioxus 0.7 bridge now renders into a crate-private mutation buffer with the document unborrowed, then takes one borrow, replays into one MutationWriter and flushes once; ported from upstream 286874cc1 (blitz PR 1260) onto Dioxus 0.7.
This closes the MountedData hazard: a task polled during render can call set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset or get_scroll_size with no RefCell already borrowed panic, on every Dioxus document.
dioxus-native-dom gains the private module mutation_buffer; the dioxus_runtime_probe's two should_panic pins become two passing tests asserting the spawned task's focus acts; upstream's task_polled_during_render_can_access_document is ported.
Measured: upstream's ported test passes even with the borrowed render restored, so on Dioxus 0.7 it is insensitive to the borrow; the probe's two pins are the fix's discriminating witness.
No dependency, manifest, lock, CI step, leg, schema or master spec moved; CI#42371051720 green 17/17; dioxus-native-dom unit tests 82 to 83, coverage 2937/3387 = 86.71 %.
The wrap also re-says owner sites naming non-open residuals to their row status: res-llkgqygv re-carried at 0.3.0 (target next); res-3v42legm and res-6ssktw6x absorbed into v030-03, owned through the CARRY cry-mjwr45xr on Upstream sync ahead of determinism and exploration.
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
Established Decisions, the MountedData hazard record (fct-apdputpm): it states the hazard as standing, pinned by two should_panic tests, its fix owned by the route entry MountedData buffered render. Now closed: fixed by the buffered render on the 0.7 bridge, an engine change for every Dioxus document (the stand, the windowed dioxus-native launch, any app on the bridge), flagged not upstreamable (upstream holds it in 0.8 form), nothing sent, its merge surface the bridge document file (two render sites, the field, the ported test), the new buffer module and one mod line in the crate root.
@task_item n=2 part=touches
Any architecture row (hazard 3, buffer 2, title 2 rows) that names MountedData buffered render as the owner of a pending fix or says a MountedData call in a task polled inside render_immediate panics with RefCell already borrowed: that panic path is retired.
@task_item n=3 part=touches
Existing Scopes, dioxus-native-dom's module list (fct-kaaobbkf): gains the private module mutation_buffer, outside every accessibility cfg gate, no pub use; keyboard_compat stays.
@task_item n=4 part=touches
Existing Scopes, the dioxus_runtime_probe record (fct-ktfizprz, and probe rows naming the pins): the two should_panic pins mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental and _incremental and their helper press_with_spawned_focus are retired; they are now mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental via the track_caller helper spawned_focus_acts, asserting presses reads 1, the harness focus, the snapshot's one focused node and the accessibility tree focus all name focus-target; the file keeps 10 tests and 0 should_panic.
@task_item n=5 part=touches
Project Intent (fct-4ncs7geh) and other dioxus-08 rows that say 286874cc1 is to be ported: its buffered render is now ported onto Dioxus 0.7; the Dioxus 0.8 move itself still waits, Dioxus stays 0.7.3.
@task_item n=6 part=touches
Rows that say Cells calls its MountedData handles (scroll_to) synchronously because of the hazard: the calls stay synchronous and are kept, but are no longer needed for the hazard.
@task_item n=7 part=touches
Standard Contracts, Dioxus DOM bridge: DioxusDocument::initial_build and Document::poll for DioxusDocument keep their public signatures but now render through the buffer with the document unborrowed, then borrow the inner document once, build one MutationWriter, replay, drop it (the mutator's single flush), then flush queued mounted events; callers Harness::from_vdom and DioxusNativeApplication::can_create_surfaces unchanged.
@task_item n=8 part=touches
Upstream's mounted-event test element_removed_before_its_mounted_event_is_flushed: any architecture row saying it returns with 286874cc1 is wrong; it stays out on Dioxus 0.7 for 0.7's scheduling, not the borrow, and returns with the Dioxus 0.8 move, owned by residual res-6ssktw6x (absorbed into v030-03) through the CARRY cry-mjwr45xr.
@task_item n=9 part=touches
Architecture owner site fct-ivrm3he5 names res-llkgqygv (the stand's platform accessibility adapter and windowed AT witness) as an open owner: now re-said as owned by residual res-llkgqygv, re-carried at 0.3.0 (target next).
@task_item n=10 part=touches
Architecture owner sites fct-2fe67avo, fct-hff77mh6 and fct-j4ls6r2g name res-3v42legm (merge surfaces) or res-6ssktw6x (left-out upstream delta) as open owners: each now reads absorbed into v030-03, its re-read owned by the open CARRY cry-mjwr45xr on Upstream sync ahead of determinism and exploration; this chunk's merge-surface line goes on that CARRY.
@task_item n=11 part=touches
Line citations into the bridge document file and the dioxus-native-dom crate root may have shifted: 52 lines added in 8 ranges (7, 90-91, 155, 166-168, 171, 302-304, 307, 846-885) and one line at 21; e.g. Occupied Resources' dioxus_document citations at 112 and 141-148 sit after the added lines.
@task_item n=12 part=touches
Indirect: conventions on unsafe and borrowing and the cross-cutting Rc RefCell service wiring: the bridge's borrow conflict is now avoided by recording mutations and replaying them under one borrow, a new borrow-workaround shape beside taking a child list or layout out.
@task_item n=13 part=touches
Any architecture count of dioxus-native-dom's unit tests (82 in nine files) moves to 83 in nine files; the new module holds no test.
@task_fact n=1 slug=standard-contracts
fact: dioxus-native-dom's crate-private MutationBuffer records each of dioxus-core 0.7.10's 15 WriteMutations calls as one entry, attribute values cloned and the two texts copied into one String arena as Range<usize> spans read back by get().unwrap_or_default() with no slicing panic path; its Debug prints len alone; pub(crate) replay drains the entries in order into any WriteMutations and clears the arena keeping both allocations.
@task_fact n=2 slug=standard-contracts
fact: DioxusDocument holds a pub(crate) buffered_mutations MutationBuffer, set in DioxusDocument::new, its only constructor; vdom.rebuild and vdom.render_immediate write into it, never into the borrowed document.
@task_fact n=3 slug=established-decisions
fact: Limit by construction of the buffered render: a task polled during render acts on the document as the previous render left it, so a node created in the same render is not yet there and set_focus_to returns false for it; recorded as an open residual row (target next).
@task_fact n=4 slug=established-decisions
fact: On dioxus-core 0.7.10 a task spawned from a component body during render_immediate's RerunScope arm is first polled by the next poll's wait_for_work before the bridge borrows, while a task spawned from an event handler that also dirties a scope is first polled inside render_immediate's PollTask arm, the only path that met the borrow hazard.
@task_fact n=5 slug=existing-scopes
fact: Upstream's task_polled_during_render_can_access_document is ported into dioxus-native-dom's DioxusDocument tests as upstream wrote it and passes, but on Dioxus 0.7 it also passes with the borrowed render restored (insensitive to the borrow); the fix's discriminating witness on 0.7 is the dioxus_runtime_probe's two spawned_acts tests, which fail with RefCell already borrowed under that mutation.
@task_item n=1 part=untouched
Dependencies and versions: no manifest, lock or deny config moved; Dioxus stays 0.7.3 with dioxus-core 0.7.10, so Dependency pinning, Inherited Defaults' versions and the Stack's Dioxus 0.7 line stand.
@task_item n=2 part=untouched
The MutationWriter and DocumentMutator contracts: the writer is byte-identical and the mutator's flush-on-drop and redraw rules are unchanged; only how many times the bridge builds a writer per render changed.
@task_item n=3 part=untouched
keyboard_compat, its coverage exclusion, the two keyboard-types versions and the note that its retirement waits on the Dioxus 0.8 move all stand.
@task_item n=4 part=untouched
CI, legs and gates: no CI job, step, leg, cache or coverage floor changed; the probe stays out of run stand, stand-repeat and the a11y leg; the stand count 237 passed, 0 failed, 7 ignored over 51 files is unchanged.
@task_item n=5 part=untouched
Dated per-OS CI tallies read at CI#41333172440 in architecture remain true as dated and are not re-stated by this session.
@task_item n=6 part=untouched
Occupied Resources: no new listener, port, outbound host, env var, filesystem path, thread or process-wide state; the buffer holds data in process only.
@task_item n=7 part=untouched
Logging and telemetry: no new log site, print or span; the replay re-fires the writer's own trace calls, so the log stream is the one the direct write gave.
@task_item n=8 part=untouched
The Upstream sync record: merge base 23b9b26a9, upstream last read at e42ee1f25, the 36 taken and 21 left-out commits, and Dioxus 0.8.0-alpha.3 not taken, are unmoved.
@task_item n=9 part=untouched
The other direct engine change records (select list, focus fixup, password masking, key-press panic fix and the like) and their owner lines on Upstream sync ahead of determinism and exploration are unchanged.
@task_item n=10 part=untouched
The bridge's stable element ids, unkeyed_actionable, the snapshot and its diff, the masked-input and select tests, and the accessibility tree builder are unchanged.
@task_item n=11 part=untouched
The driver session, the headless stand, the test harness API and the CLIs are unchanged; Harness::from_vdom and the windowed launch call the bridge as before.
