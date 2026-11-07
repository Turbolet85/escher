# Implement outcome — 2026-11-07-mounteddata-buffered-render

**Outcome: surfaced.** Every gate the skill fires is green: 16 of 16 non-operator entries on the first firing, with
no fix iterations (trail `.andromeda/runs/2026-11-07T11-47-25-implement/gate-2026-11-07-mounteddata-buffered-render.json`).
The buffered render is built and the hazard is closed. A mutation control found that upstream's ported test, which
the plan names the fix's witness, passes on Dioxus 0.7 without the fix. That premise is surfaced below; nothing was
authored. The skill never fires the operator entries (17 hygiene, 18 push, 19 CI read).

## Files changed
- `packages/dioxus-native-dom/src/mutation_buffer.rs` (new): a crate-private `MutationBuffer` that records dioxus-core
  0.7.10's 15 `WriteMutations` calls in order and replays them (`replay`), keeping its allocations. Its `Debug`
  prints the entry count alone.
- `packages/dioxus-native-dom/src/lib.rs`: adds `mod mutation_buffer;` outside every feature gate; `mod keyboard_compat;`
  stays.
- `packages/dioxus-native-dom/src/dioxus_document.rs`:
  - adds the `pub(crate) buffered_mutations` field, set in `new`;
  - reshapes `initial_build` and `poll`: render with the document unborrowed, then one borrow, one `MutationWriter`
    and the replay, then the drop, then `flush_queued_mounted_events()`;
  - ports `task_polled_during_render_can_access_document` into `mod tests` after
    `mounted_data_of_removed_element_reports_error`, its body as upstream wrote it.
- `tests/blitz-tests/tests/dioxus_runtime_probe.rs`:
  - the two `should_panic` pins become `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`.
    They share one helper, `spawned_focus_acts`, which asserts `#presses` reads `1` and that the engine's focus, the
    snapshot's one `focused` node and the accessibility tree's focus all name `focus-target`;
  - the module text is re-said;
  - the file keeps 10 `#[test]`s.
- `mutation_writer.rs` is untouched (entry 6 green). No manifest, lock or `deny.toml` moved (entry 7 green).

## Readings
- Entry 1: `test result: ok. 83 passed; 0 failed`. The dioxus-native-dom unit count is 83, as predicted (82 → 83).
- Entry 2: `test dioxus_document::tests::task_polled_during_render_can_access_document ... ok`.
- Entry 3: `test result: ok. 10 passed`. Both `…_spawned_acts_*` read `ok`.
- Entries 4 / 5: `should_panic` reads 0, `#[test]` reads 10.
- Entries 8 / 9 / 10: 0 (no log site, print, env read, listener or panic token added).
- Entry 13 (`run stand`): `passed 237 · failed 0 · ignored 7`, outcome passed, over 51 files. Unchanged.
- Entries 14 (`fast`), 15 (`doc`) and 16 (`audit`): green.
- Entry 11 (`agent-run.sh boot`) and entry 12 (`run dioxus_runtime_probe`): green.
- No `gate.py guard` entry in the block, so there are no pass lines to copy.
- Mutation control (`.andromeda/cache/chunks/2026-11-07-mounteddata-buffered-render/evidence/mutation-control.md`):
  `poll`'s borrowed render was restored in place, and the file's sha256 matched on restore. The two inverted pins read
  FAILED (`RefCell already borrowed`) and green with the fix. The ported witness reads `ok` either way.

## Surfaced
- **Value / contract:** upstream's `task_polled_during_render_can_access_document` is "the witness" of the buffered
  render. Sources: the plan's Goal, its first acceptance criterion and step 6; the scope's «Upstream's regression test
  as the witness»; the expected amendment to test-plan §1 Test Scope Summary (fct-e2zfwpza).
- **Implementation reality:** on dioxus-core 0.7.10 the ported test passes with or without the fix. Its task is spawned
  in the app's component body during `render_immediate`'s `RerunScope`. No `queue_events` follows that arm, so the next
  `poll`'s `wait_for_work` → `process_events` → `poll_tasks` (`virtual_dom.rs:445–519`) first polls the task, outside
  the borrow. The hazard needs the task to be polled inside `render_immediate` (`Work::PollTask`, `:610–613`). The
  probe's key handler gets there because it also dirties a scope. So on this bridge the discriminating guard is the
  two inverted probe pins, not upstream's test. The test still passes as the acceptance text states, and it stays
  ported as upstream wrote it (step 6). It guards the 0.8 scheduling it was written for and keeps the next sync's merge
  surface small.
- **Likely amendment (wrap's or the operator's call):** test-plan §1 / §6 and the architecture hazard record name the
  probe's two pins as the 0.7 witness of the fix, and record upstream's test as ported, passing, and insensitive to the
  borrow on 0.7. The other route is a re-plan that adds a 0.7-sensitive witness in `mod tests`, for example a task
  spawned from a handler that also writes a signal. No capability is affected: the chunk owns no coverage row, and no
  matrix ref was written.

## Deviations from plan, with justifications
- The text spans are `Range<usize>` into one string arena (upstream's `StrSpan` form), not `u32` spans. With them the
  buffer needs no `u32::try_from(…).expect(…)`, which entry 8 forbids.
- A recorded span is read by `strings.get(span).unwrap_or_default()`, not by slicing. That leaves no slicing panic path
  in the module (security S4); the spans are correct by construction.
- `#[track_caller]` is on the probe's new assertion helper (`.claude/rules/testing.md`, File and naming conventions).
- The field is named `buffered_mutations` (upstream's name, moved from `DioxusState` to `DioxusDocument` as the plan
  directs).
- Scope record: none needed. `gate.py scope` reads `clean — changed 4 · listed 4 · recorded 0`.

## Stated limits (from the plan, confirmed)
- A task polled during render sees the document as the previous render left it: a node created in the same render is
  not yet in it. The witness and the pins act on nodes mounted by an earlier render.
- Upstream's `element_removed_before_its_mounted_event_is_flushed` is left unported (step 9). On Dioxus 0.7 the app's
  re-run lands in the next `render_immediate`, after this render's mounted flush, so the test would read 1 where it
  asserts 0. The buffer moves no scheduling. Nothing is written for it.
- Smoke: skipped, because the plan has no smoke or self-verify entry. The render path ran headlessly under entries 13
  and 14; the windowed `dioxus-native` launch is read and not run.
- No deferral: the Rust delta is 4 files and every gate ran.

## Next
The operator pass, entries 17 (hygiene), 18 (push) and 19 (CI read); then `/andromeda-wrap-session`, which resolves
the surfaced witness premise with the operator. Alternatively, `/andromeda-phase` to re-plan a 0.7-sensitive witness
first.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started that outlived its entry: `census: none running` for comm escher-session · seven_guis_native · cargo · rustc and args agent-run.sh · dioxus_runtime_probe (census trail in the run dir) | — | — |

## Decisions & corrections
- Guard refusal: the Bash guard blocked `cd` into the skill's references directory ("moves the session cwd"). The call
  was re-issued once with absolute paths and ran.
- The mutation control was the agent's own choice, not a plan entry. Its reading is the surfaced premise above.
- The operator's ruling on the surfaced witness premise, 2026-11-07, verbatim: "The witness question goes to the wrap:
  amend the specs." So the wrap amends; no re-plan.
