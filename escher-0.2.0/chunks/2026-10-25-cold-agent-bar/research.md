# Codebase Research — 2026-10-25-cold-agent-bar

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 16 · **Graph queries:** 1 (rust) · **Probe runs:** 1
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its one Session Addition (a
  `timeout N` boot smoke exits 124, which the gate tool reads as its own bound) applied to the live leg's entry form;
  `.claude/rules/testing.md` and `.claude/rules/a11y.md` — read in full as they loaded on the stand files.
- **Platform issues consulted:** none — no runner-only bullet. Setup 5a read one sha, `e84dc8181`, green 17/17
  (CI#40193730816), so there is no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the founder's word on the cold agent's task reach: option 1, extend the stand so
  it covers all the checks, the wrong-call bar still open. `inputs#I2` — the operator's directive for this phase.

## Files inspected
- `scripts/cold-agent.sh` (full, 500 lines) — `TASKS=(stand)` (:14). `STAND_PROMPT` (:17-27) names the four lean apps
  by name, never by slug or id. `DONE` (:53-58) is one `contains`/`lacks` text pair per app. `tasks_met` (:165-181)
  reads the last `snapshot` answer's text while an app is the latest accepted `start`'s. `span_counts` (:148-157)
  counts ` span="tool_call" ` lines and their `refused` / `error` endings. `wrong_calls` (:352) is refused + errors +
  foreign + denials, and decides nothing. `judge_check` (:281-305) replays the check in full, then ablated, and
  requires `set(APPS)` started. `Server` (:207-246) runs `escher-session mcp` with no `--record`. The client is
  `--model claude-opus-5-5 --max-budget-usd 3` under `timeout 600` (:476-478).
- `.github/scripts/test_cold_agent.py` (575 lines, `ColdAgentPipeTest`, the `FAKE` server at :63) — one case per
  verdict path (:303-568), among them `test_the_prompt_names_no_stand_id` (:506) and the scrub-key and
  planted-marker cases (:539, :553).
- `examples/seven_guis/src/session_host.rs` (full) — `TASKS` holds four `(slug, LeanTask)` pairs (:13-18). Their slugs
  are what `command_line` receives as `apps` (:24). The Timer's boot carries `with_time(stand::timer_step)` (:40-43).
- `examples/seven_guis/src/stand.rs` (full) — `LeanTask` holds four variants with `ALL` (:15-40). `options` sets
  `net_provider: None` (:58-69), and its doc says to replace fields with `..` to inject one (:56-57).
  `boot_with_ticks` mounts `task_in_shell(task.task(), …)` (:104-122).
- `examples/seven_guis/src/app.rs` (:3-15, :69-71, :122-124) — `Task::Cells` already exists with slug `cells`, and
  `task_in_shell` already renders it inside `TaskShell`. Only the stand's `LeanTask` and the host's `TASKS` leave it out.
- `examples/seven_guis/src/tasks/flight_booker.rs` (full) — the type choice is two buttons, `flight-one-way` and
  `flight-return`, with `aria_pressed` and `type-btn` / `type-btn-active` (:66-81). A `String` signal is `"one-way"` /
  `"return"` (:45). The return date is `disabled: !is_return` (:95). The booked text reads «Booked return flight: … →
  …» (:105).
- `examples/seven_guis/src/tasks/cells.rs` (full) — a 26 × 26 grid. Each data cell is a `div` with `id:
  "cell-{col}{row}"` and an `onclick` that opens `input#cell-input` (:373-401). It commits on blur or Enter (:388-397).
  The grid sits in an `overflow: auto` root (:417-421). Formulas use the closed tokenizer (`#ERR`, `#CYCLE`).
- `tests/blitz-tests/tests/stand_load_wait.rs` (full) — the one load-wait witness, on a fixture: a click links a
  stylesheet, which `LateNet` (in the file, :48-60) answers after 200 ms from its own thread. It reads settled with
  the box's bounds in the diff, and `HeldNet` never answers.
- `tests/blitz-tests/tests/fixtures/mod.rs` (full) — `boot` / `hold` use `stand::options`, and `HeldNet` (:87-130) is
  a silent `NetProvider` that keeps each request.
- `tests/blitz-tests/tests/stand_act_select.rs` (full) — the fixture select read by the driver: role `ComboBox`, name
  from `<label for>` or `aria-label`, value the chosen option's, options `ListBoxOption` with `selected`. `type` chooses
  by label prefix, an unchosen option's `click` is refused `covered`, and a disabled select is skipped by Tab.
- `tests/blitz-tests/tests/select_interaction.rs` (:20-70, :315-340) — the controlled select: `oninput` writes the
  signal, each option's `selected:` reads it, and a re-seed from the app fires no event.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (:377-382) — `Input` and `Change` both reach the app as
  `NativeFormData { value }`, so `oninput: move |e| flight_type.set(e.value())` receives the chosen option's value.
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` (full) — Cells is reached through Home's card in process, and
  `unkeyed_actionable` reads 0 while it is open. It holds 676 click-reacting role-less cells (:95-103).
- `examples/seven_guis/tests/{cli_commands,host_binary}.rs` (:30-41; :105-150) — `cli_commands` states the four tasks
  in its own `TASKS` table (:36-41). `host_binary`'s usage row asserts the line names `counter` · `flight-booker` ·
  `timer` · `crud` (:139-148), and it holds with a fifth name added.
- `packages/escher-driver/src/cli.rs` (:91-196, :343-365) and `mcp/listing.rs` (:63-72, :118) — `start`'s app set is
  the `apps` the binary hands over. The usage line joins them, and the MCP `start` tool lists them as an `enum`, which
  is the program's own fixed text, so a new served app moves both and adds no carrier.
- `.andromeda/residuals.md:9` (the cold-agent wrong-call bar, `absorbed:v020-11`) and `:38` (a paged `part` read
  through the host's sink, unmeasured because the host serves no paged screen, target: next).
- `.andromeda/playbook.md` escalate patterns (:36, :48, :60) — Boundary widening, Provisional discharge, and Boundary
  widening by upstream merge.

## Graph impact
- **`LeanTask`** — 466 reference sites in 49 files (re-derived: `SELECT COUNT(DISTINCT file), COUNT(*) FROM refs
  WHERE callee_name = 'LeanTask' AND callee_kind = 'type' AND callee_file LIKE '%seven_guis/src/stand.rs'`, this run's
  `tree-query-2026-10-25-cold-agent-bar.json`). 22 `stand_*` files iterate `LeanTask::ALL` (the same trace's `ALL`
  rows), beside `common/mod.rs`, `session_common/mod.rs`, `cli_commands.rs` and `session_host.rs`. A fifth variant would
  enter every `ALL` loop and every exhaustive `match` over the four. Every table keyed by lean task (`tab_order`,
  `controls`, `rendered`, contrast rows, `WITNESSES`, `DONE`-like lists) would then need a Cells row it was never
  measured for. So Cells joins the stand through a boot of its own, beside `LeanTask` and not inside it.
- **`stand::options`** — 27 referencing file names in the same trace's `options` rows, among them non-stand checks (`text_extent`,
  `transformed_client_rect`, `virtual_clock_engine`, `password_caret_click`, `stand_load_wait`, …). Changing its
  `net_provider: None` default would put a provider under every one of them. Any load provider is injected per boot
  through `..stand::options(_)`, as `stand_load_wait.rs:66-70` already does.
- **`stand::boot` / `boot_timer`** — 15 and 8 caller files. These are unchanged if the new boots are added beside them.

## Patterns detected
- **A per-boot provider through struct update** (`stand_load_wait.rs:66-70`): `HarnessOptions { net_provider:
  Some(net), ..stand::options(incremental) }`. Only the boot that needs a load gets one, and the others stay offline.
- **A self-answering in-process stand-in** (`stand_load_wait.rs:48-60`): a `NetProvider` whose `fetch` answers from
  its own thread after a fixed delay. It uses no network, no `blitz_net`, no print and no env read.
- **A controlled select** (`select_interaction.rs:34-46`): `oninput` writes the signal, and each option's `selected:
  signal() == value`.
- **A served task's extra boot shape** (`session_host.rs:28-43`): a task that needs more than `stand::boot` gets it in
  the host's closure, as the Timer's step does.

## Conventions to follow
- **Author ids on every actionable element**: `{task}-{control}` (`flight-*`), with no CSS selecting by id
  (`flight_booker.rs:66-114`; layout-templates §Surface: desktop-native).
- **The stand's shared tables are stated once** in `tests/blitz-tests/tests/common/mod.rs`. The flight booker's rows
  sit at :25-26, :55-56, :79-80 and :151-152.
- **The pipe's events and verdict are encoded by python3 `json`**, and every field is a count, a bool or a closed
  word (`cold-agent.sh:36-37`, :341-393).

## Measured (P3 probe)
The equality the plan needs is "Cells, held as a driver session, fits the pipe's 30 s reply bound and pages through
`part`". It was measured by a throwaway probe (`.andromeda/runs/2026-10-25T12-59-18-phase/p3-probe-cells.rs.txt`,
log `p3-probe-cells.log`). The probe ran as a blitz-tests target, was removed after the run, and left no tree change.
Cells was opened through Home in both layout modes and held by `Session::start`.
- The whole snapshot text is **47,949 bytes, 740 lines, 5 parts** of `SNAPSHOT_TEXT_BUDGET`. Both `snapshot` and
  `snapshot part 1` answer `part 1/5`, 9,959 bytes.
- Each call's wall time (non-incremental / incremental): `snapshot` 9 / 9 ms · `click cell-A0` 110 / 67 · `type
  cell-input "5"` 157 / 68 · `press enter` 106 / 63 · `type "=A0*2"` 321 / 164 · `lint` 28 / 28 · `screenshot` 473 /
  445. Every call is under 0.5 s, far inside `REPLY_BOUND` 30 s.
- After the two commits the screen reads `GenericContainer "5" id="cell-A0"` and `GenericContainer "10" id="cell-B1"`.
  A cell's line names its displayed value, so a Cells goal is judged by snapshot text.
- **`lint` at boot reads 490 findings on Cells** in both modes: a grid wider and taller than the viewport inside a
  clipping root. The lean tasks read 0. Cells' boot reading is recorded as measured, never assumed (layouts extract).

## New files to create
- `tests/blitz-tests/tests/stand_cells_served.rs` — Cells booted through the stand's own boot and held by `Session::run`, both layout modes: the screen read in parts, a formula committed and read, a cell past the viewport scrolled to, filled and read in the last part
- `tests/blitz-tests/tests/stand_load_page.rs` — the load page (inputs#I3, F2) held by `Session::run`, both layout modes: a click whose load the stand's in-process stand-in answers ends settled with the message on screen, the session's clock unmoved

## Files to modify
- `examples/seven_guis/src/stand.rs` — a boot for Cells and a boot for the load page beside `LeanTask` (not new variants), the load page's markup and its in-process network stand-in
- `examples/seven_guis/src/session_host.rs` — `start` serves `cells` and `load` beside the four lean tasks
- `examples/seven_guis/src/tasks/flight_booker.rs` — the one-way / return choice as a controlled `select`
- `scripts/cold-agent.sh` — goals over 0.2.0's capabilities, the paged-answer reading, per-verb counts, `--record` on the live child, the founder's bar in the verdict
- `.github/scripts/test_cold_agent.py` — the fake server and one case per new or moved verdict path
- `tests/blitz-tests/tests/common/mod.rs` — the flight booker's `controls`, `rendered`, `tab_order`, `ENABLED_TAB_ORDER` rows
- `tests/blitz-tests/tests/session_common/mod.rs` — the flight booker's mode change in `change`
- `tests/blitz-tests/tests/stand_act_diff.rs` — the flight booker's diff pins
- `tests/blitz-tests/tests/stand_act_disabled.rs` — the return date enabled by a select choice
- `tests/blitz-tests/tests/stand_act_ids.rs` — the flight booker's control rows
- `tests/blitz-tests/tests/stand_act_states.rs` — the pressed mode buttons become the select's value and options
- `tests/blitz-tests/tests/stand_contrast.rs` — the accent row and the three return-chosen state rows re-measured
- `tests/blitz-tests/tests/stand_element_ids.rs` — the select and option ids
- `tests/blitz-tests/tests/stand_id_persistence.rs` — the flight booker's ids across re-render
- `tests/blitz-tests/tests/stand_keyboard.rs` — the flight booker's Tab and Enter/Space rows
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — the flight booker's witness call
- `tests/blitz-tests/tests/stand_session_recording.rs` — the flight booker's recorded call
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — the active mode read as the select's value
- `tests/blitz-tests/tests/stand_diff.rs` — the flight booker's Book diff re-read on the select layout
- `tests/blitz-tests/tests/stand_snapshot.rs` — the flight booker's snapshot nodes
- `examples/seven_guis/tests/cli_commands.rs` — the served task set and its usage line
- `examples/seven_guis/tests/host_log.rs` — a paged part read on `cells` through the host's own sink (residuals.md:38)
- `examples/seven_guis/tests/common/mod.rs` — the log checks' needles read from the served Cells screen

## Sweeps
- `flight-(one-way|return)(?!-date)` over every tracked file outside the masters and chunk folders (`git grep -P`):
  14 files hit. 13 test files will change: `common/mod.rs` 9, `session_common/mod.rs` 1, `stand_act_diff` 7,
  `stand_act_disabled` 1, `stand_act_ids` 2, `stand_act_states` 6, `stand_contrast` 7, `stand_element_ids` 2,
  `stand_id_persistence` 1, `stand_keyboard` 4, `stand_requirement_sweep` 1, `stand_session_recording` 1,
  `stand_snapshot_state` 9. The 14th file is `flight_booker.rs` itself. There are 0 hits in `examples/seven_guis/tests/`, `scripts/`,
  `.github/`, `docs/`, `README.md` and the committed checks. `flows/flight.sh` books one-way (:34-61) and touches no
  mode button, so it does not change.
- `"crud"` / the four-slug list over `examples/seven_guis/tests` (`git grep`): 6 hits. `cli_commands.rs:36-41` is a
  stated table and changes. `host_binary.rs:139-148`'s `all(contains)` holds with a fifth name, so no change. The
  `host_log` / `mcp_log` / `cli_commands` session starts name one task each, so no change.

## Open questions
- How the stand carries a load in flight (the founder's «все проверки», inputs#I1) → blocks: plan-decision (founder's
  fork at P4; the files `stand.rs`, `session_host.rs` and possibly a task file follow it).
- Whether the ratified carriage of the stand's screen to the model provider («Один запуск, до $3 (Рекомендую)»,
  2026-10-15, one run) covers a second run over the new screens → blocks: plan-decision (founder's fork at P4; a
  boundary widening by playbook :36 if he widens it).
- The wrong-call bar from 0.1.0's 0 of 43 (residuals.md:9) → blocks: plan-decision (founder's fork at P4; the
  verdict's new reason follows it).
