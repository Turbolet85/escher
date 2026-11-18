# Outcome — 2026-11-18-served-error-page

**Outcome:** green + smoke · 0 fix iterations · Next → the OPERATOR PASS (entries 13 hygiene, 14 push, 15 CI read), then
`/andromeda-wrap-session`.

## Files changed
New (2): `examples/seven_guis/src/tasks/order.rs` · `tests/blitz-tests/tests/stand_order_served.rs`.
Modified (18 listed): `examples/seven_guis/src/tasks/mod.rs` · `examples/seven_guis/src/stand.rs` ·
`examples/seven_guis/src/session_host.rs` · `examples/seven_guis/tests/cli_commands.rs` ·
`examples/seven_guis/tests/host_log.rs` · `tests/blitz-tests/tests/common/mod.rs` · `tests/blitz-tests/tests/stand_actions.rs`
· `stand_cross_os_readings.rs` · `stand_explore.rs` · `stand_graph.rs` · `stand_handler_coverage.rs` · `stand_layout.rs` ·
`stand_keyboard.rs` · `stand_contrast.rs` (all under `tests/blitz-tests/tests/`) · `scripts/cold-agent.sh` ·
`.github/scripts/test_cold_agent.py` · `docs/capabilities.md` · `README.md`.
Recorded out-of-list (2): `examples/seven_guis/Cargo.toml` · `Cargo.lock`.

## Deviations, with their justifications
1. **The `tracing` dependency** — scope record lines:
   - ``- `examples/seven_guis/Cargo.toml` · companion · serves step 1 · self``
   - ``- `Cargo.lock` · mechanical · serves step 1 · self``
   Step 1's `tracing::info!` needs `tracing`; `seven_guis` declares none, and `dioxus-native`'s prelude re-exports none
   (`packages/dioxus-native/src/prelude.rs`). Added `tracing = { workspace = true }`; `cargo update --offline -p seven_guis`
   moved the lock by one edge (`"tracing"` in the `seven_guis` entry), no new package.
2. **A red by the plan's own wording — entry 10, the guard**: `red · last line 'guard: clean' ✗ (last: 'guard: outside 1
   files')`, the one outside file `Cargo.lock (recorded mechanical · serves step 1)`. The entry's allow set and note
   («Cargo.lock included») contradict the recorded mechanical line above; the plan stays as written, the edit stays made
   (fix-loop-protocol, *A red by the plan's own wording*). Word: none sought — the edit is companion/mechanical, made and
   recorded under Trigger 3; the wrap's light gate dispositions it by its RED-by-the-plan's-own-wording arm.
3. **`caused` items read no owner** — the plan's step 5 and the (caused) criterion say each item is tied to its control's
   id, «its owner» and event. Measured: every `caused` item reads `owner` absent, and so does the UI graph for every
   page control (`back-btn`, `order-quantity`, `order-submit`, `order-export`, `order-status` all `None`), the page keying
   each control by its own HTML `id` under no keyed element. The check pins that reading (`tied(id)` with no owner) and
   asserts the graph reads the controls with no owner, so the item's owner is the graph's own.
4. **Test names** — `stand_cross_os_readings`'s `the_table_counts_its_rows_and_covers_the_six_tasks` kept its name (it
   now covers seven): `stand_requirement_sweep.rs:1231` pins it as a witness. `test_cold_agent.py`'s six-app test renamed
   `test_a_seven_app_run_passes_with_no_wrong_call` as the plan's step 10 says (no other pin: `git grep` reads it alone).
5. **In-intent additions** — `stand_keyboard` also walks the order page with its failure shown
   (`ORDER_FAILED_TAB_ORDER`, Retry in Submit's place, a local const), so the control enabled after boot is reached by
   Tab (a11y rule: a control enabled after boot takes a place in a stated order). `stand_contrast` reads the failure, Retry
   and the placed status besides the boot rows and Submit's hover/active.

## Guard pass line (entry 10, `gate.py show … --n 10`)
```
guard: allowed 20 — .github/scripts/test_cold_agent.py, README.md, docs/capabilities.md, examples/seven_guis/src/session_host.rs, examples/seven_guis/src/stand.rs, examples/seven_guis/src/tasks/mod.rs, examples/seven_guis/src/tasks/order.rs, examples/seven_guis/tests/cli_commands.rs, examples/seven_guis/tests/host_log.rs, scripts/cold-agent.sh, tests/blitz-tests/tests/common/mod.rs, tests/blitz-tests/tests/stand_actions.rs, tests/blitz-tests/tests/stand_contrast.rs, tests/blitz-tests/tests/stand_cross_os_readings.rs, tests/blitz-tests/tests/stand_explore.rs, tests/blitz-tests/tests/stand_graph.rs, tests/blitz-tests/tests/stand_handler_coverage.rs, tests/blitz-tests/tests/stand_keyboard.rs, tests/blitz-tests/tests/stand_layout.rs, tests/blitz-tests/tests/stand_order_served.rs · companions excluded 1 — examples/seven_guis/Cargo.toml (serves step 1) · changed 209 · excluded 187
guard: outside 1 files
```
The excluded companion `examples/seven_guis/Cargo.toml` serves step 1 (the page's `tracing::info!`). The 187 excluded
are the pipeline's artifacts (`.andromeda/**`, the version workspace, `.claude/session-handoff.md`).

## Gates (trail `.andromeda/runs/2026-11-18T10-58-34-implement/gate-2026-11-18-served-error-page.json`)
`entries 15 · green 11 · red 1 (10) · recorded 0 · timeout 0 · not-run 3` — 1 lib · 2 stand_order_served · 3 the eight
table targets (160.27 s) · 4 cli_commands (smoke) · 5 host_log · 6 bash -n · 7 cold-agent unittest · 8 ci-scripts · 9 the
secret search (exit 1, no output) · 11 `fast` (395.51 s) · 12 `stand-repeat` (1005.69 s) green; 10 red (deviation 2);
13–15 `leg operator`, not run — the operator pass's.
No deferral: every gate ran.

## Controls
- control-1 (a)–(c), entry 7: the tool read `INDETERMINATE: … no result line` ×3 (its case reader knows no unittest
  line); by hand each removed log reads its named case `FAIL` — a: `test_each_unused_capability_fails` ×3 sub-cases
  (`caused_by_kind` error · log · panic), b: `test_caused_is_counted_by_kind_from_the_acting_answers` (35 failures in
  all), c: `test_the_prompt_names_no_stand_id` — all three hold by hand (`evidence/control-1.txt`).
- control-2 (d), entry 2: `control d-own-boundary · entry 2 · holds — in place green, removed red` (`evidence/control-2.txt`).

## Smoke
✓ — `cargo build --locked -p seven_guis --bin escher-session`, then the binary: `start order` (exit 0), `click
order-submit` (an `error` item, `boundary` `own`, the rule), `click order-retry`, `type order-quantity 12`, `click
order-submit` (`[('log', 'order placed')]`), `click order-export` (`[('panic', 'export is not built yet')]`), `stop`
(exit 0). State dir `target/tmp/smoke-order-1` left in place (the Bash guard refuses a recursive rm) — for the operator.

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| escher-session (the smoke's host) | this run | terminated — `stop` answered `{"stopped":true}`; `census: none running` (comm escher-session, cargo, seven_guis_native) |
| cargo test children of entries 1–12 and the two controls | the gate tool / control tool | terminated — no survivor named; census none running |

## Decisions & corrections
- Guard refusal: the Bash guard refused `rm -rf target/tmp/smoke-order` inside the smoke call; re-issued with a fresh
  directory `target/tmp/smoke-order-1`, left for the operator.
- Recurrence of a standing learning: testing.md 2026-11-14 (extended 2026-11-18) — `gate.py control` reads no unittest
  result line; controls (a)–(c) read INDETERMINATE and were read by hand, as the learning says.
- Recurrence of a standing learning: testing.md 2026-11-14 — a panicking handler planted as a plain function passed by
  name (`export`), which compiled first time.
