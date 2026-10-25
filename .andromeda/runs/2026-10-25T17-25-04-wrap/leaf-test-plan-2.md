group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «`stand_snapshot_parts` (29 readers, `stand_snapshot_parts` the twenty-ninth;»
  source: .andromeda/test-plan.md:149 «(31 readers with `stand_cells_served`, which reads the module's call builders, `Acted` and `act`, and `stand_load_page`, which reads `act`, `click`, `lint`, `linted` and `snapshot`; 29 with»
  replace-with: «`stand_snapshot_parts` and the two served-page checks `stand_cells_served` and `stand_load_page` (31 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first;»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «`found`, `occurrences` and the unix-only `recorded_needles` — called by `host_log` and `mcp_log` alone»
  source: .andromeda/test-plan-detail/test-scope-summary.md:14 «`found`, `occurrences`, the unix-only `recorded_needles` and `cells_needles(part)` (part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds)»
  replace-with: «`found`, `occurrences`, the unix-only `recorded_needles` and `cells_needles(part)` (part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds) — called by `host_log` and `mcp_log` alone»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «whose `fetch` answers each request from its own thread after a 200 ms sleep; `RecordingShell`»
  source: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:16 «`LoadNet`, the load page's network stand-in, private to `examples/seven_guis/src/stand.rs` — a third hand-written network stand-in beside `HeldNet` and `LateNet`: `stand::boot_load` replaces»
  replace-with: «whose `fetch` answers each request from its own thread after a 200 ms sleep; the stand's own `LoadNet`, private to `examples/seven_guis/src/stand.rs`, which `stand::boot_load` puts in its options' `net_provider` and which answers the fixed `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), `is_noop` not overridden; `RecordingShell`»

- leaf: .claude/rules/testing.md:30   class: stale
  leaf-text: «Stand checks boot through `seven_guis::stand::boot(task, stand::options(incremental))` — pinned 800×600 · scale 1 · Light, the bundled DejaVu Sans with system fonts off, offline —»
  source: .andromeda/test-plan.md:130 «and the load page through `stand::boot_load(HarnessOptions)` (TaskShell titled "Load", its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept)»
  replace-with: «Stand checks boot through `seven_guis::stand::boot(task, stand::options(incremental))` — the served pages through `stand::boot_cells` and `stand::boot_load` — pinned 800×600 · scale 1 · Light, the bundled DejaVu Sans with system fonts off, offline but for the load page, whose `net_provider` is the stand's `LoadNet`, every other field kept —»

- leaf: .claude/rules/testing.md:30   class: omission
  leaf-text: «whose in-file `LateNet` answers from its own thread after a 200 ms sleep), no `dispatch_recorded`.»
  source: .andromeda/test-plan.md:262 «the 200 ms sleep of the stand's own `LoadNet` (examples/seven_guis/src/stand.rs:171) is the stand's, not a check's, `stand_load_page` waiting it out through the driver's load wait»
  replace-with: «whose in-file `LateNet` answers from its own thread after a 200 ms sleep; the 200 ms sleep of the stand's own `LoadNet` is the stand's, not a check's, `stand_load_page` waiting it out through the driver's load wait with no sleep or clock read of its own), no `dispatch_recorded`.»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «and read again with the stand checks' named list it reads 4918/5019 = 97.99 %»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read again with the served Cells and load page, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %»
  replace-with: «and read again with the served Cells and load page it reads 4918/5019 = 97.99 % (dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %)»

- leaf: .claude/rules/verification-harness.md:25   class: stale
  leaf-text: «with a fixed prompt asking for the four lean apps one at a time»
  source: .andromeda/test-plan.md:182 «The fixed prompt `STAND_PROMPT` asks for six apps one at a time — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells (`A0` 5, `B1`»
  replace-with: «with a fixed prompt asking for six apps one at a time — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells (plus one visual lint) and the load page —»

- leaf: .claude/rules/verification-harness.md:25   class: omission
  leaf-text: «the driver's own `escher-session mcp` (the twelve verbs; its stderr at `RUST_LOG=info` in `server.log`, counted by `tool_call` span lines, never printed or copied)»
  source: .andromeda/test-plan.md:183 «read only to count ` span="tool_call" ` lines, their ` ending="refused" ` / ` ending="error" ` and the accepted lines of each counted verb, `lint` · `screenshot` · `advance` (`span_counts`)»
  replace-with: «the driver's own `escher-session mcp --record` into the owner-only `target/cold-agent/record/` (`mkdir -m 700`) (the twelve verbs; its stderr at `RUST_LOG=info` in `server.log`, counted by `tool_call` span lines, their `refused` / `error` endings and the accepted lines of each counted verb, `lint` · `screenshot` · `advance`, never printed or copied)»

- leaf: .claude/rules/verification-harness.md:26   class: stale
  leaf-text: «all four lean apps in `tasks_met` (read from a `snapshot` answer while the app is the latest accepted `start`'s, never the agent's reply)»
  source: .andromeda/test-plan.md:184 «all six apps in `tasks_met` (an app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`»
  replace-with: «all six apps in `tasks_met` (read from an accepted `snapshot` answer while the app is the latest accepted `start`'s, a paged screen read as the latest answer of each of its parts joined, never the agent's reply), each of `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` above 0, `recording_kept` (each accepted `start` left one kept check), `wrong_calls × 100 ≤ 5 × tool_calls`»

- leaf: .claude/rules/verification-harness.md:26   class: retired
  leaf-text: «`wrong_calls` is recorded, never deciding.»
  source: .andromeda/test-plan.md:185 «`wrong_calls` = refused + error + foreign `tool_use` blocks + `permission_denials`, deciding the outcome against `wrong_call_bar_pct` (5)»
  replace-with: «`wrong_calls` decides the outcome against `wrong_call_bar_pct` (5 %), a run over it failing `wrong-calls-over-bar`, beside the reasons `capability-unused` and `recording-not-kept`.»

- leaf: .claude/rules/verification-harness.md:27   class: stale
  leaf-text: «State lives in `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}` only»
  source: .andromeda/test-plan.md:187 «`target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` only — `record/` (`mkdir -m 700`) the live server child's kept checks»
  replace-with: «State lives in `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` only (`record/`, `mkdir -m 700`, the live server child's kept checks, never printed, copied or committed)»

- leaf: .claude/rules/verification-harness.md:36   class: omission
  leaf-text: «The headless stand boots through `seven_guis::stand::{boot, boot_timer}` over `from_vdom` with `stand::options(incremental)` (bundled DejaVu Sans, system fonts off, offline);»
  source: .andromeda/test-plan.md:130 «and the load page through `stand::boot_load(HarnessOptions)` (TaskShell titled "Load", its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept)»
  replace-with: «The headless stand boots through `seven_guis::stand::{boot, boot_timer}` over `from_vdom` with `stand::options(incremental)` (bundled DejaVu Sans, system fonts off, offline), and beside the lean tasks Cells through `stand::boot_cells` and the load page through `stand::boot_load`, its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept;»
