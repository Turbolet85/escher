# /implement outcome — 2026-11-04-dioxus-runtime-probe

**Outcome: green** (smoke skipped: no boot-path or UI-surface change). Next → the OPERATOR PASS for the plan's
`operator` entries **8** (hygiene), **9** (push) and **10** (CI read), then `/andromeda-wrap-session`.

## Files changed
- New: `tests/blitz-tests/tests/dioxus_runtime_probe.rs` — 10 tests: links 1 to 4 and the MountedData pin, each in
  both layout modes, three fixture pages in the file.
- Modified: none. Nothing under `packages/`, `examples/`, `Cargo.toml`, `Cargo.lock` or `deny.toml` (gate 3 green).
- Chunk evidence (in `.andromeda/cache/chunks/2026-11-04-dioxus-runtime-probe/evidence/`): `probe-reading.md` (the
  founder's reading) and `counts.md`.

## Gates
Trail: `.andromeda/runs/2026-11-04T20-29-30-implement/gate-2026-11-04-dioxus-runtime-probe.json`. Two firings of the
block, both `entries 10 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 3` (8–10 `not run — leg operator`). The
second firing is the reading: it ran on the check as it stands. Entry 2 (silence census) read `exit 1`, last line `0`.
Entry 7 (doc) rebuilt nothing, since no crate source changed. No deferral; no `gate.py guard` entry in the block.

Counts (`counts.md`): 158 `.rs` files and 157 targets under `tests/blitz-tests/tests/` (predicted 158 and 157), 51
`stand_` files, 10 tests in the check, `run all` 743 passed · 0 failed · 12 ignored (733 + 10).

## Verdicts measured (detail in `probe-reading.md`)
- Link 1 **exposed** — the public vdom walk names Counter's and CRUD's own component for the named ids and a
  component for every task-body element; a deleted CRUD row resolves to nothing.
- Link 2 **exposed** — the bridge's listener attributes (increment `click`, value none, CRUD list `keydown`, Cells
  676 `click` + 1 `keydown` = the pinned 677) over-report: a removed listener's attribute stays and no longer acts,
  and `mounted` writes none. The vdom walk's dynamic attributes read the listeners exactly, `mounted` included.
- Link 3 **exposed by effect** (Counter's click diff names the value; the writer's click changes exactly the two
  readers' nodes); **by state identity, not exposed without a fork-level change** for hook-local state (read, not run).
- Link 4 **exposed for a held signal** (`subscribers()` → `origin_scope()` = exactly the two readers' scopes);
  **not exposed** for hook-local state (read, not run).
- MountedData hazard **pinned** in both layout modes (precondition asserted, then a key press panics
  `RefCell already borrowed`); the synchronous contrast acts. The run's stack reads the plan's coordinates:
  `events.rs:162` ← `set_focus` `events.rs:295` ← the spawned task ← dioxus-core `render_immediate`
  (`virtual_dom.rs:611`) ← `DioxusDocument::poll` `dioxus_document.rs:297`. No fix applied.

No capability claimed (plan: none); the ledger holds no `chunk == 2026-11-04-dioxus-runtime-probe` row, so no
matrix write.

## Deviations from the plan, with justifications
Scope record: none needed (`gate.py scope`: `scope: clean — changed 1 · listed 1 · recorded 0`).
1. **Link 2 gained a second reader, the vdom walk's dynamic attributes** (`VNode.dynamic_attrs`,
   `VNode::mounted_dynamic_attribute`, public dioxus-core API), asserted beside the attribute read in three tests.
   Step 4 asked only for the attribute read. Found while drafting the reading; it measures link 2 exactly with no
   fork-level change, and so changes the founder's link-2 fork (now: which reader, recommended the walk). In-intent,
   in the listed file; no new surface.
2. **`mounted_dynamic_node`, listed in step 3, was not used** — an element a component renders is always under one of
   its template roots, so `mounted_root` suffices.
3. **The toggled-listener page spreads the listener in and out** (`..spread` of a `Vec<Attribute>` built with
   `dioxus::html::onclick`). Read from `dioxus-core-0.7.10/src/diff/node.rs`: only an attribute leaving the list
   reaches `remove_event_listener`; a listener value turning to none is written as an `on{event}` attribute instead.
   The spread is the form that exercises the path step 4 predicts.
4. **The shared-state test clicks the writer once before the measured step**, so the step's diff carries no focus
   move onto the writer button; the diff then names exactly the two readers, as step 5 asserts.
5. **`hazard_ready` pumps once more after boot** before asserting the held handle, so the mounted event's signal
   write has rendered.
6. **The gate block was fired twice**: the check gained deviation 1 while the first firing ran; the second firing
   is the reading. Both were green.

## Surfaced / stuck
Nothing surfaced, nothing stuck. Expected wrap amendments as the plan lists them; link 2's recorded limit should read
that the attributes over-report and the vdom walk is exact.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (gate entries 1, 4–7; the direct run; `run all`) | this run | terminated — `gate.py census --comm cargo --comm rustc --comm dioxus_runtime_probe --comm seven_guis_native --comm escher-session`: `census: none running` |
| `dioxus_runtime_probe` test binary (direct run, backtrace read) | this run | terminated (same census) |

## Decisions & corrections
- Guard refusal (bash-guard, `cd` moving the session cwd), twice — re-issued with absolute paths.
- Guard refusal (bash-guard, a `cat` heredoc with a file target) once — the edit went through the Edit tool instead.
- Standing learning met: 2026-11-01 «assert the fixture's precondition first» — applied in `hazard_ready` (handle
  held; focus by Tab).
- A captured backtrace was read in the session scratchpad to confirm the panic site; only its coordinates were
  carried into the reading, no payload or stderr.
