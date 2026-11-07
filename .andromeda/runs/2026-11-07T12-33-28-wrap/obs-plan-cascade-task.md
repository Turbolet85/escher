@task_prose part=session
The Dioxus 0.7 bridge in dioxus-native-dom now renders into a new crate-private mutation buffer (MutationBuffer, private module mutation_buffer) with the document unborrowed, then replays it through one MutationWriter under a single borrow; DioxusDocument gains the crate-private field buffered_mutations.
This retires the RefCell already borrowed panic of a MountedData call (set_focus, get_client_rect, scroll_to and the rest) from a task polled during render_immediate, on every Dioxus document; ported from upstream 286874cc1, Dioxus 0.8 move still waiting.
The probe dioxus_runtime_probe's two should_panic pins became mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental (helper spawned_focus_acts), the file keeping 10 tests; upstream's task_polled_during_render_can_access_document was ported and measured insensitive to the borrow on Dioxus 0.7.
Obs measurement: the buffer module and the added bridge lines hold no log site, print, env read, listener or panic token (silence gates read 0); the replay re-fires MutationWriter's own trace! calls, so the log stream is the one the direct write gave; MutationBuffer's Debug prints len alone.
No dependency, feature, manifest, CI step, harness leg, telemetry crate or driver span moved; dioxus-native-dom unit tests 82 to 83; its coverage 2937/3387 = 86.71 % at CI#42371051720 (green 17/17).
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
Log coverage silence records: a new dioxus-native-dom render-path module (the mutation buffer) is added with no log site, print, env read, listener or panic token; a group recording silent modules and checks must carry it.
@task_item n=2 part=touches
Log coverage of dioxus-native-dom's emitters: the bridge now writes mutations through a buffer and replays them; the replay re-fires MutationWriter's existing trace! calls unchanged, so any statement about where dioxus-native-dom's debug lines fire (during render vs after) is re-read, the line set unchanged.
@task_item n=3 part=touches
PII: the buffer holds element ids, tag and attribute names, attribute values and text node contents in process memory only, between render and replay; its Debug prints a length alone; nothing logs, fields or prints its contents. A pii group stating dioxus-native's logged text-node contents and attribute values may need the in-process holding added.
@task_item n=4 part=touches
Any obs row on the dioxus_runtime_probe check (its silence row, its MountedData hazard, its should_panic pins, its 10-test count): the hazard is closed and the two pins are renamed and inverted to assert the spawned task's set_focus acts; the silence census over the probe still reads 0 and the 10-test count stands.
@task_item n=5 part=touches
Any obs row stating a panic path in a task polled during render (RefCell already borrowed reaching escher-telemetry's panic hook or std's stderr print): that panic path is retired on every Dioxus document.
@task_item n=6 part=touches
Any obs row listing dioxus-native-dom's modules or its private modules (keyboard_compat and the rest): mutation_buffer is added, private, outside every accessibility cfg gate, no pub use; keyboard_compat stays.
@task_item n=7 part=touches
CI integration readings: CI#42371051720 at e26f1a207 green 17/17; per-OS tallies linux 220 · 1310 · 0 · 12, macos and coverage 201 · 1307 · 0 · 12, windows 201 · 1259 · 0 · 9, a11y 14 · 114 · 0 · 0, stand repeat 228 · 1155 · 0 · 21; any obs row citing a dated CI run's tallies stays true as dated, a current-reading row moves.
@task_item n=8 part=touches
dioxus-native-dom's coverage figure, where an obs row cites it: 2937/3387 = 86.71 % (floor 83 %) at CI#42371051720; the coverage artifact upload itself is unchanged.
@task_item n=9 part=touches
Upstream-sync rows: 286874cc1's buffered render is now ported onto Dioxus 0.7, its Dioxus 0.8 form and the 0.8 move still waiting; upstream's mounted-event test stays out on 0.7; a row saying 286874cc1 is still to be ported is stale.
@task_item n=10 part=touches
Residual owner lines naming res-3v42legm or res-6ssktw6x (or res-fzitzzw2, opt-in OTel export) as open owners: each row reads absorbed into v030-03 (the first two) or re-carried at 0.3.0, target next (res-fzitzzw2); a site naming one with no status is re-said to its row's status.
@task_fact n=1 slug=log-coverage
fact: dioxus-native-dom's crate-private module mutation_buffer (MutationBuffer, recording dioxus-core 0.7.10's 15 WriteMutations calls and replaying them into MutationWriter) and the bridge lines added to the render sites hold no log site, print, env read, listener or panic token, the silence gates reading 0; the replay re-fires MutationWriter's own trace! calls, so the log stream is unchanged (as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md).
@task_fact n=2 slug=pii-scrubbing-compliance
fact: MutationBuffer holds ids, tag and attribute names, attribute values and text node contents in process memory only, between a render and its replay, its arena cleared at each replay; its Debug prints len alone and nothing logs, fields or prints its contents.
@task_fact n=3 slug=log-coverage
fact: The probe dioxus_runtime_probe's two renamed pins mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental and their helper spawned_focus_acts install no subscriber, read no env var, print nothing and bind no listener, the silence census reading 0 over the file.
@task_item n=1 part=untouched
escher-telemetry's subscriber, allowlist formatter, scrub sets, service identity and panic hook: no telemetry crate code, target prefix or field set moved.
@task_item n=2 part=untouched
escher-driver's command and tool_call spans, their fields, and the driver's command line and MCP output: the driver is unchanged.
@task_item n=3 part=untouched
The agent-run harness and cold-agent pipe log formats, event keys and state areas: no harness, leg or CI step changed, and the probe stays out of run stand, stand-repeat and the a11y leg.
@task_item n=4 part=untouched
Feature wiring of tracing, log-frame-times and log-phase-times across dioxus-native, dioxus-native-dom and the blitz crates: no manifest, lock, feature or dependency moved.
@task_item n=5 part=untouched
dioxus-native-dom's mutation writer and its logged text-node and attribute values, with their line citations: that writer is byte-identical to 434e0661, so its cited lines stand.
@task_item n=6 part=untouched
Metric coverage, error-reporting clients, OTel absence, blitz-vibey-script's script error surface and the upstream apps' stdout subscribers: none is reached.
@task_item n=7 part=untouched
The stand count 237 · 0 · 7 over 51 files and the silence rows of the stand checks and their shared modules: unchanged.
