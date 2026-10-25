# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The `ci-scripts` leg's count must move with the cold-agent pipe's contract tests: any case added to `test_cold_agent.py` for the new goals must be counted again in the leg's file and test totals.
  > 104 tests in eight test files (§4; 100 in seven until test_capability_statement.py added its 4; 94 until 2026-10-15-cold-agent-test, whose test_cold_agent.py went 27 → 33 as `ColdAgentStubTest` retired with the stub;
- (.andromeda/test-plan-detail/test-scope-summary.md:15) If a served task pages, `host_log`'s needle reading has to hold for it: no id, name or typed text on a client's stderr at `trace`, a part read included.
  > `tests/host_log.rs` (3 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`, and the third reading the clients of a recording session — `start crud --record` through `stop`, a part read and a `lint` among them — 0 needles of each class over their own stderr at `trace`, examples/seven_guis/tests/host_log.rs:40-130)
- (.andromeda/test-plan-detail/test-scope-summary.md:16) The flight booker's mode change moves `tests/flows/flight.sh`, and both shell flows must stay green under plain `sh`.
  > `tests/cli_flow.rs` (2, unix only: the two shell flows of `tests/flows/` under plain `sh`)
- (.andromeda/test-plan-detail/test-scope-summary.md:18) `cli_commands` must start, read and stop every task the binary's `start` serves, so the claim moves if `cells` (or another task) joins the four.
  > and `cli_commands` starts, reads and stops all four tasks
- (.andromeda/test-plan-detail/test-scope-summary.md:23) A stand `select` must stay inside what the select model covers (single choice, no `optgroup`), and its options must carry author keys or they show up in `unkeyed_actionable`.
  > its stated limits (a `multiple` select and an `optgroup`'s options not modelled, no popup painted, an option with no author key reading in `unkeyed_actionable`) asserted by no check
- (.andromeda/test-plan-detail/test-scope-summary.md:60) No verb, node key, cause, exit status or session error is added, so `describe.rs`'s count row must read the same after the chunk.
  > its count row reads `[12, 13, 12, 3, 8, 4, 15]`
- (.andromeda/test-plan-detail/test-scope-summary.md:93) A flight-mode `select` changes the stand's roles: the 15 controls' roles and names and the interactive-role count per task must be stated again in the tables.
  > the 15 controls' roles and names — and since 2026-10-14-stand-a11y-assertions every tree node of the four lean tasks in the nine interactive roles named, at least `controls(task).len()` of them per task
- (.andromeda/test-plan-detail/test-scope-summary.md:94) `stand_snapshot_state` pins the flight mode as a `pressed` reading, and a mode held as a `select` value has to restate that pin.
  > since 2026-10-10-snapshot-states exactly the active flight mode reading pressed and the reading swapping with the mode
- (.andromeda/test-plan-detail/test-scope-summary.md:95) The every-screen-under-budget pin covers the lean tasks. A paged `cells` screen must sit outside that claim or restate it, and every lean screen must stay under 10,000 bytes.
  > each screen under its in-file ceiling and under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes; the largest, Crud, reads 2049 since 2026-10-10-snapshot-states — 2034 before it)
- (.andromeda/test-plan-detail/test-scope-summary.md:96) A Cells screen served to the driver must keep `unkeyed_actionable()` at 0 in both layout modes, with every cell's key as pinned.
  > of all seven tasks — `unkeyed_actionable()` 0 · 0 · 0 on Temperature Converter · Circle Drawer · Cells in both layout modes (2 · 3 · 676 until then), Circle Drawer's dialog open included, the three tasks keyed by added `id:` lines alone (`temp-celsius`, `temp-fahrenheit`, `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter`, `circle-close`, `cell-{A..Z}{0..25}` on the 676 data cells, `cell-input`), no CSS selecting by them and no other markup line moved
- (.andromeda/test-plan-detail/test-scope-summary.md:102) The recorded-session replay runs on each lean task, so a changed flight booker must still record and replay through `Check::replay` in both layout modes. This is one candidate for the acceptance's «passes on replay».
  > and replays through `Check::replay` on each lean task in both layout modes, each script ending `snapshot` · `screenshot`, so the shot path's kept form is covered on each unix OS CI runs
- (.andromeda/test-plan-detail/test-scope-summary.md:104) `stand_act_ids` asserts its row counts over the four lean snapshots, so a `select` and its option nodes move the 77 / 70 / 7 counts, which must be measured again.
  > for each of the 77 nodes of the four lean tasks' snapshots, the 15 controls among them, the id the snapshot lists is the `author_id` of the one accessibility node carrying it, the element `element_ids()` resolves it to is that node's, and a driver `click` naming it is accepted on 70 and refused on the 7 the check states from the measured stand
- (.andromeda/test-plan-detail/test-scope-summary.md:106) `stand_act_disabled` reaches the return flight by a driver choice, and once that choice is a `select` it must still refuse the return date while one-way and accept it after.
  > a `type` into the Flight Booker's return date and a `click` on it are refused `disabled` while the flight is one-way, and the same `type` is accepted once the driver chooses the return flight
- (.andromeda/test-plan-detail/test-scope-summary.md:108) `stand_act_states` pins the flight mode as two pressed buttons, and the mode's move to a `select` must restate it.
  > a flight-mode click returns both mode buttons with their pressed reading
- (.andromeda/test-plan-detail/test-scope-summary.md:115) `stand_diff` pins the Flight Booker's Book diff node for node, and the restated flight booker must be read again there.
  > `stand_diff` gains the Flight Booker's Book naming eight nodes — seven that only moved, `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, `differs` = `[focused, bounds]`
- (.andromeda/test-plan-detail/test-scope-summary.md:115) The contrast record is one row per `controls(task)` row, so a flight booker `select` moves its rows and pass tally, and every non-exempt row must still pass.
  > every stand control reads its recorded text/background pair (18 rows, one per `controls(task)` row over the four lean tasks at boot, read through `Harness::text_contrast` from computed styles; pass 14 · fail 0 · exempt-inactive 3 · no-text 1
- (.andromeda/test-plan-detail/test-scope-summary.md:116) The colour-state rows include the flight booker's return-chosen state, which must still be reached and pass once the choice is a `select`.
  > (`STATE_ROWS`, 19 reached states — return-chosen 3, start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — 18 pass and 1 exempt, the lowest pass 5.11)
- (.andromeda/test-plan-detail/test-scope-summary.md:116) The boot Tab sequence of each lean task is pinned, and a `select` in the flight booker must be restated in `common::tab_order`.
  > Tab and Shift+Tab walk each lean task's stated sequence (`common::tab_order`) and wrap, the engine's focus, the snapshot's `focused` and the tree's focus agreeing at every step
- (.andromeda/test-plan-detail/test-scope-summary.md:117) The Enter/Space table names `flight-return` as a changing stand button, and that row must be restated if the button gives way to a `select`.
  > Enter and Space on a focused stand button read as its click (20 rows stated and counted, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing)
- (.andromeda/test-plan-detail/test-scope-summary.md:117) The enabled-state Tab order is reached through `flight-return`, and every control must lie in the union of the two tables, so a `select` control must enter one of them.
  > the Tab walk beyond boot in two enabled states each reached by keyboard alone — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops), stated in `common::ENABLED_TAB_ORDER` — Enter and Space on `crud-update` and `crud-delete` once enabled each reading as its click (4 rows), and every `controls(task)` id lying in the union of `tab_order` and `ENABLED_TAB_ORDER`, an empty set difference over `LeanTask::ALL`
- (.andromeda/test-plan-detail/test-scope-summary.md:118) The requirement sweep's flight booker acting call is `click flight-return`, and it must be restated to a call that still returns a non-empty diff.
  > one id-addressed acting call (`click counter-increment` · `click flight-return` · `advance 300` · `click crud-person-0`) returning settled with a non-empty diff equal to the check's own
- (.andromeda/test-plan-detail/test-scope-summary.md:119) The 0.2.0 witness record runs v020-01 to v020-09. A stand witness of v020-07, v020-08 or v020-09 added by this chunk must be named in it.
  > and `WITNESSES_020`, the 0.2.0 record: nine rows v020-01 to v020-09 in order, 46 witnesses
- (.andromeda/test-plan-detail/test-scope-summary.md:119) Every new non-unit witness must be a `stand_*` file or be named in the stand-repeat lists and `ci-leg.sh`.
  > every non-`Unit` blitz-tests witness a `stand_*` file or one of `STAND_REPEAT_NAMED` (10) and every `examples/seven_guis/tests/` witness one of `STAND_REPEAT_SEVEN_GUIS` (8), and `.github/scripts/ci-leg.sh`, read through `include_str!`, holding the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18
- (.andromeda/test-plan-detail/test-scope-summary.md:119) Cells holds 676 click-reactive elements that state no role, a measured limit pinned in `stand_non_lean_tasks`. Serving it to the driver must not add a snapshot field: `NODE_FIELDS` stays thirteen.
  > the elements that react to a click and state no role counted against a stated 7-row table (0 on the four lean tasks and Home, Circle Drawer 1, Cells 676
  >  — measured and pinned, the snapshot field for it deferred, `NODE_FIELDS` still thirteen)
- (.andromeda/test-plan-detail/test-scope-summary.md:124) The load wait is proven today only on a fixture behind `LateNet`/`HeldNet`. A load in flight on the stand must reproduce the settled-after-answer reading through `Session::run`.
  > `stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a body stylesheet: a load its in-file `LateNet` answers after 200 ms ends the `click` settled
- (.andromeda/test-plan-detail/test-scope-summary.md:125) The `select` driver reading is proven on a fixture: a ComboBox with options as children, and `type` choosing by label with `value` and `selected` in `differs`. The flight booker's `select` must read the same way on the stand.
  > `stand_act_select` 5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically: at boot `fx-pick` reads `ComboBox`, `Fruit`, value `b`, its three options under it named Apple · Banana · Cherry, Banana selected; `type --id fx-pick --text Cherry` accepted and settled, its diff naming `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`
- (.andromeda/test-plan-detail/test-scope-summary.md:126) The paged walk is proven on a fixture page, and a `cells` screen served to the driver must satisfy the same walk.
  > `stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: the walk in parts through `Session::run` — part 1 by default, each part within the budget, the parts concatenating to `to_text()`, the empty part past the last, an id read only in the last part addressable, a password masked in every part
- (.andromeda/test-plan.md:176) `run stand`'s baseline count must be re-counted once stand checks are added or restated. A new `stand_` file is picked up by its prefix.
  > `run stand`'s `run.end` reads passed 197 · failed 0 · ignored 7 over 48 files
- (.andromeda/test-plan.md:180) The pipe holds one task `stand` over the four lean apps, so widening its goals moves this statement and the usage allowlist.
  > the one task `stand` — the four lean apps in one client session (`TASKS=(stand)`; `counter` no task since 2026-10-15-cold-agent-test, so `run counter` is a usage error)
- (.andromeda/test-plan.md:182) The prompt's goals move to 0.2.0's capabilities, and the prompt must still name no element id and no app slug.
  > The fixed prompt `STAND_PROMPT` asks for the four lean apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped before the next, then the check as the reply's last fenced json block; it names no element id and no app slug
- (.andromeda/test-plan.md:183) The agent's check is replayed in full and ablated on fresh children under a 30 s reply bound, so a `cells` screen, a paged read and a load wait must each fit inside it.
  > `judge_check` replays it in full on a fresh `escher-session mcp` child, then ablated (its acting steps left out) on a second fresh child, each spawned directly with stderr to `/dev/null`, every reply under `REPLY_BOUND = 30` s
- (.andromeda/test-plan.md:184) `tasks_met` is defined over the four lean apps, so new goals need their own met rule or the definition must be restated.
  > all four lean apps in `tasks_met` (an app is met from a `snapshot` answer while it is the app of the latest accepted `start` — an accepted `stop` clears it — never from the reply)
- (.andromeda/test-plan.md:184) The wrong-call count that the founder's bar judges is defined here and today decides nothing. The bar set at P4 must say how it reads against this count.
  > `wrong_calls` = refused + error + foreign `tool_use` blocks + `permission_denials` — recorded, never deciding the outcome
- (.andromeda/test-plan.md:187) 0.1.0's baseline is the reference the founder's bar is set from.
  > 43 tool calls (start 4 · snapshot 8 · click 10 · type 6 · press 10 · advance 1 · stop 4), 0 wrong calls (`refused_calls` 0 · `error_calls` 0 · `spacer_calls` 0), 0.3197 USD
- (.andromeda/test-plan.md:187) The live run is recorded as not re-fired, so this chunk's one live run must be recorded as the pipe's new proof over twelve tools.
  > the pipe requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired
- (.andromeda/test-plan.md:217) The pipe's contract tests must cover each new verdict path with no live model, credential or built binary, and with no scrub-set key or planted marker in events or verdict.
  > test_cold_agent.py runs `scripts/cold-agent.sh` under a `claude` shim and a fake `target/debug/escher-session`, with no live model, credential or built binary — one case per verdict path, isolation breach, usage and precondition exit, and the events and verdict holding no scrub-set key and no planted marker
- (.andromeda/test-plan.md:261) A load in flight on the stand must not bring a sleep or wall-clock read into a check beyond the two named ones.
  > no sleep and no wall-clock read in the checks, except the two whose subject is wall time — `stand_virtual_clock`'s one 800 ms sleep, and `stand_load_wait`'s `Instant::now()` reads around its call with its `LateNet` stand-in's 200 ms sleep
- (.andromeda/test-plan.md:267) This owed measurement (target: next) is unblocked once the host serves a task that pages, so the chunk touches its residual.
  > a paged `part` read through the host's own sink is unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/test-plan.md:336) The stand's pinned options carry no net provider, and how a load in flight reaches the stand (the P4 fork) must be weighed against this literal.
  > `seven_guis::stand::options(incremental)` builds the stand's full `HarnessOptions` literal: 800×600, scale 1.0, Light, the bundled font, `net_provider: None`, `base_url: None`
- (.andromeda/test-plan.md:412) Every stand check added or restated is held to the zero-failure, three-round, no-retry budget.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets and seven_guis' process checks three times with no retry

## Bindings
- **Flight booker `select` ↔ a11y plan**: the Tab tables, Enter/Space rows and contrast rows this category pins are the a11y surface's SC 2.1.1 · 2.4.3 · 1.4.3 records. A restated control moves both sides together (.andromeda/test-plan-detail/test-scope-summary.md:117) (.andromeda/test-plan-detail/test-scope-summary.md:115)
- **`cells` served ↔ body §5 owed measurement**: a paged task on the host turns the residual's unmeasured part read into a `host_log`-shaped reading (.andromeda/test-plan.md:267) (.andromeda/test-plan-detail/test-scope-summary.md:15)
- **Load in flight on the stand ↔ body §7/§8 and arch §Occupied Resources**: the stand's options carry `net_provider: None` and the one held stand-in is `HeldNet`. A stand provider is the P4 fork and may be a resource registration (.andromeda/test-plan.md:336) (.andromeda/test-plan-detail/test-scope-summary.md:124)
- **New witnesses ↔ body §9/§10 stand-repeat**: `WITNESSES_020`'s placement rule and the zero-failure leg bind the same checks (.andromeda/test-plan-detail/test-scope-summary.md:119) (.andromeda/test-plan.md:412)
- **Live run ↔ security plan §Surfaces**: the pipe's proof records one ratified run, and a second carriage of new screens to the provider is the founder's word (.andromeda/test-plan.md:187)
- **Schema counts ↔ the chunk's Boundaries**: no verb, cause, session error or key added keeps `describe.rs`'s row (.andromeda/test-plan-detail/test-scope-summary.md:60)
