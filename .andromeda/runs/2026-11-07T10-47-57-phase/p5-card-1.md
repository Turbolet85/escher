# P5 review — 2026-11-07-mounteddata-buffered-render · «MountedData buffered render»

**The plan:** `.andromeda/cache/docs/2026-11-07-mounteddata-buffered-render/plan.md` (rev 2, 387 lines) — open it in an
editor. No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-07-mounteddata-buffered-render` is
a diff of the chunk's records.

## Goal
Every `DioxusDocument` renders with the document unborrowed. `initial_build` and `poll` run the vdom into a
crate-private buffer of Dioxus 0.7's `WriteMutations` calls, then take one borrow, replay the buffer in order into the
existing `MutationWriter`, and flush the mounted events as now. So a task polled during render can call `MountedData`
(`set_focus`, `get_client_rect`, `scroll_to`, …) without a `RefCell already borrowed` panic. This is an engine change for
every Dioxus document, on the founder's «Порт, отдельный чанк (Рекомендую)».

## Touchpoints
- new `packages/dioxus-native-dom/src/mutation_buffer.rs` — the recorder of the 15 calls and its replay
- `packages/dioxus-native-dom/src/dioxus_document.rs` — a `pub(crate)` buffer field, the two render sites, upstream's
  `task_polled_during_render_can_access_document` ported into `mod tests`
- `packages/dioxus-native-dom/src/lib.rs` — `mod mutation_buffer;` (`keyboard_compat` stays)
- `tests/blitz-tests/tests/dioxus_runtime_probe.rs` — the two `should_panic` pins become
  `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, and the module text is re-said
- untouched: `mutation_writer.rs` (byte-identical to `434e0661`), every manifest and the lock, Cells' synchronous handles

## Top acceptance criteria
1. The ported witness passes: a task spawned during a re-render calls `set_focus` and `get_client_rect` with no panic,
   and focus lands on its input.
2. The probe passes 10 tests with no `should_panic` left. In both layout modes the spawned task's `set_focus` moves the
   engine's focus, the snapshot's one `focused` node and the tree's focus to `focus-target`, precondition first.
3. `mutation_writer.rs` is byte-identical to `434e0661`. No manifest, lock or `deny.toml` moves; Dioxus stays at `"0.7.3"`.
4. The buffer module holds no log site, print, env read, listener or panic path, and the edited bridge lines add none.
5. `run stand` exits 0 (237 · 0 · 7 at the base). `fast`, `doc` and `audit` are green. The operator's CI run reads
   green, which covers the a11y, coverage (dioxus-native-dom ≥ 83 %) and stand-repeat legs.

## Test commands
19 `[[gate]]` entries:
- three test runs (dioxus-native-dom's unit tests, the witness by name, the probe by name)
- seven probes (no `should_panic`, 10 tests, the writer's byte-identity, no manifest move, two silence censuses, the
  probe file's silence)
- agent-run `boot` · `run dioxus_runtime_probe` · `run stand`
- `ci-leg.sh fast` · `doc` · `audit`
- the operator's hygiene, push and CI read

Baselines are written on all 11 `new` entries. Each guard entry's known-positive control was fired as the entry in a
scratch clone (`.andromeda/runs/2026-11-07T10-47-57-phase/controls.md`): every one read as wanted.

## Leaned, not asked (each settled by an artifact)
- The buffer lives on `DioxusDocument`, not upstream's `DioxusState`. That keeps `mutation_writer.rs` byte-identical,
  and the borrows are disjoint.
- The witness goes in `mod tests`, where upstream places it, not in `bridge_tests`.
- The inverted pins assert the three focus readings (a11y standing S1). The target `div` is a `GenericContainer` tree
  node.
- Upstream's `element_removed_before_its_mounted_event_is_flushed` stays out. It fails on Dioxus 0.7's scheduling,
  which the buffer does not move (dioxus-core 0.7.10 `virtual_dom.rs:599–626`, read, not run).
- The escalate patterns (`rul-izni57qv` · `rul-3ndrhilc` · `rul-72xhfjoz`) were read. Nothing crosses a hardened boundary,
  and the engine change is the founder's ruled choice, so no fork was asked.

## A limit the report will state
A task polled during render acts on the document as the previous render left it. A node created in the same render is
not in the document yet, so `set_focus_to` returns `false` for it.

## Capabilities
None claimed. The eleven planned 0.3.0 capabilities (`v030-05` … `v030-15`) name the UI graph, exploration,
explanation and ship surfaces. This chunk builds none of them: it is their precondition.

## Validation
- Mechanical checks 1–12 pass: `planlint` 0 hits, `gate.py run --dry-run` parses 19 entries, `scope lists: parsed 4
  items`, `inputs.py verify` reads 1 cited entry with no drift. The 7 «unchanged» claims `planlint` listed were each read
  against the steps, and none is contradicted.
- Validation-1: aligned with `doc:2026-11-07-mounteddata-buffered-render/scope` (rev 4) and the working entry.
- P2 extracts: one `@none` (design), so aggregate B passes.
- sampler: off

## Beyond the plan
| # | Proposal | Kind | Where |
|---|---|---|---|
| 1 | Upstream's `element_removed_before_its_mounted_event_is_flushed` stays out until the Dioxus 0.8 move, where it was recorded as returning with `286874cc1`. Its owner is the store's `residual` row for Dioxus 0.8.0-alpha.3 (target: next), the row architecture §Project Intent names (fct-4ncs7geh). | deferral | test-plan §1 Test Scope Summary (fct-ybmlkzsq), re-said by the wrap |

Apply? (yes / review / cancel)
