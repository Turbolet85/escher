@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's mutation buffer, silent and held in process only
section: §6 Log Coverage (log-coverage.md: a new **dioxus-native-dom (mutation buffer)** item) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: a new item)
change:
  - §6 gains **dioxus-native-dom (mutation buffer)**: no site added. dioxus-native-dom's crate-private module `mutation_buffer` (`MutationBuffer`, recording dioxus-core 0.7.10's 15 `WriteMutations` calls and replaying them into `MutationWriter`) and the lines of `DioxusDocument::initial_build` and `Document::poll` that render through it hold no log site, print, env read, listener or panic token, both silence censuses reading 0. The replay re-fires `MutationWriter`'s own `trace!` calls, so the log stream is the one the direct write gives.
  - §8 gains the buffer's hold: `MutationBuffer` holds element ids, tag and attribute names, attribute values and text node contents in process memory only, from a render to its replay, which drains the entries and clears its text arena. Its `Debug` prints `len` alone, and nothing logs, fields or prints its contents, the silence census over the module reading 0.
why: the report's Coverage of new surfaces (`MutationBuffer`: instrumentation n/a, no new site, the replay re-firing `MutationWriter`'s `trace!` calls; PII redacted, `Debug` len only) and its Outcome (the obs criterion MET, both silence gates at 0). No obs-plan site named the buffer before (the `buffer` pattern read 0 obs-plan rows).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the probe's renamed spawned_acts pins, recorded silent
section: §6 Log Coverage (log-coverage.md: a new observed-absent row after the `dioxus_runtime_probe` row)
change:
  - §6 gains an observed absence: no print, env read, subscriber or listener in `dioxus_runtime_probe`'s two pins `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_acts_incremental` or in their `#[track_caller]` helper `spawned_focus_acts`. Searched with the silence census over the file: exit 1, reading 0.
why: the report renames and inverts the two `should_panic` pins into the `…_spawned_acts_*` tests on the helper `spawned_focus_acts`, and its gate on `dioxus_runtime_probe.rs` reads exit 1, last line 0.
kept: the standing `dioxus_runtime_probe` silence row (the report counts it no-change and still true: 10 tests).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the multiple select's form-entry citation, re-pointed
section: §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the `multiple` select item)
change:
  - The citation for `multiple_select_values` reads `dioxus_document.rs:32-46` (was 31-45). The claim is unchanged.
why: the chunk added line 7 to `dioxus_document.rs` (the report's New text, by line), which moves the cited block down one line.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the OTel export residual's status, at its owner site
section: Bootstrap phases (derive for route / setup-project) (contracts/bootstrap-phases-derive-for-route-setup-project: the **otel-sdk-install** item)
change:
  - The item's owner now reads «a cross-version residual, `res-fzitzzw2`, re-carried at 0.3.0 (target: next)» (was «a cross-version residual, `res-fzitzzw2`» with no status). The rest of the item is unchanged.
why: the operator's directive at this wrap's resume (`inputs#I4` item 1) to bring the master sites that name a non-open residual to their rows' status, the sites listed by overseer1 (`inputs#I5`). The `res-fzitzzw2` row reads `re-carried` · disposition `0.3.0` · target `next`.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
