# arch · cascade read · group 7 of 7

## Carried
- (.andromeda/architecture-detail/existing-scopes.md:118) The stand's tasks, its headless `stand` boot and the `escher-session` binary are the units this chunk edits: the widened task set (`cells` served, the flight booker's select) must be booted by `start` and the `mcp` role through the one place a task, the stand's boot and the timer's tick meet a driver `Session`, `flight.sh` must move with the flight booker among the flows `cli_flow` runs, and any dependency a stand-held load adds must be stated beside the crate's native dependencies; the widened stand must keep registering the one bundled font.
  > Tasks cells, circle_drawer, counter, crud, flight_booker, temp_converter, timer
  > (examples/seven_guis/src/tasks/mod.rs:1-7); native-only module `stand`, the headless stand booting the lean four in
  > TaskShell (examples/seven_guis/src/lib.rs:2-3; examples/seven_guis/src/stand.rs:1-3); a second binary,
  > `escher-session` — the driver's command line over the four lean tasks and, in its host role, the headless session
  > host and, since 2026-10-10-mcp-surface, in its `mcp` role an MCP server over its standard streams: the one place a
  > lean task, the stand's boot and the timer's tick handle meet a driver `Session`, its `main` handing argv and that
  > boot to `escher_driver::command_line` (examples/seven_guis/src/session_host.rs:1-5;
  > examples/seven_guis/Cargo.toml:45-47) — and the crate's eight integration-test targets, all spawning the built
  > `escher-session` binary — `tests/host_binary.rs`, `tests/host_log.rs`, since 2026-10-10-driver-cli
  > `tests/cli_commands.rs`, `tests/cli_flow.rs` and `tests/host_timer.rs`, and since 2026-10-10-mcp-surface
  > `tests/mcp_server.rs` and `tests/mcp_log.rs`, and `tests/recording.rs` — with their shared module
  > `tests/common/mod.rs`, which is no test target, and `tests/flows/`, a directory of three POSIX `sh` scripts —
  > `counter.sh` and `flight.sh`, which `cli_flow` runs, and `record_checks.sh`, which `recording` runs to record the
  > two committed checks and compare them byte for byte — and no target either
  > (examples/seven_guis/tests/recording.rs:205-262; examples/seven_guis/tests/host_binary.rs:1-4;
  > examples/seven_guis/tests/host_log.rs:1-15; examples/seven_guis/tests/common/mod.rs:1-8;
  > examples/seven_guis/tests/cli_flow.rs:31-74); native dependencies escher-telemetry, blitz-test-harness, blitz-traits
  > and escher-driver, `dioxus-native` with `system-fonts` + `woff` (examples/seven_guis/Cargo.toml:29-35)
  (.andromeda/architecture.md:246)
  > seven_guis' bytes are the crate-root `DEJAVU_SANS`, compiled on native too, where the headless stand registers the
  > same font
- (.andromeda/architecture-detail/existing-scopes.md:85) The chunk exercises the driver end to end over a widened stand and must leave it the one definition of its verbs and refusals, with replay through `Session::run` the acceptance's «passes on replay» half and the driver naming no app — every stand change stays in seven_guis.
  > The driver's session, its command and refusal schema and its command line — one headless app instance held by one
  > process, with a start, attach and stop lifecycle, the one definition of the driver's verbs and of how it refuses a
  > call,
  (.andromeda/architecture-detail/existing-scopes.md:87)
  > and replay (`Check::replay`, `Replay` and `Change`: a kept check's calls run again through `Session::run`, stopping
  > at the first step whose acceptance or kept answer differs) and lint (`Session::lint`, `Finding`, `Defect`, `Reason`
  > and `Bounds`, the visual lint, carried by the `lint` verb — execute holding `Outcome::Linted`, json the finding's
  > written form) — re-exported at the crate root beside blitz-test-harness's `Busy` and `Settled`; it depends on
  > blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` (each with no feature), names one feature —
  > dioxus-native-dom's `accessibility` — and no app (packages/escher-driver/src/lib.rs:122-159;
  > packages/escher-driver/Cargo.toml:13-18)
- (.andromeda/architecture-detail/existing-scopes.md:105) The stand checks' boot claims — offline, bundled-font text, fresh-boot and incremental identity, one stable element id per element — must keep holding for every task the widened stand boots, `stand_element_ids` among the files the chunk re-pins.
  > the seven_guis headless stand — mount, bundled-font text, fresh-boot and incremental identity, offline, one driven
  > interaction per lean task, timer ticks, one stable element id per element
  > (tests/blitz-tests/tests/stand_boot.rs:1-3; tests/blitz-tests/tests/stand_counter.rs:1-2;
  > tests/blitz-tests/tests/stand_flight_booker.rs:1-3; tests/blitz-tests/tests/stand_timer.rs:1-3;
  > tests/blitz-tests/tests/stand_crud.rs:1-3; tests/blitz-tests/tests/stand_element_ids.rs:1-5);
- (.andromeda/architecture-detail/existing-scopes.md:106) The chunk edits `tab_order` and `ENABLED_TAB_ORDER` and the Enter/Space rows: the flight booker's select must be restated once in `tests/common/mod.rs`'s per-task tables, and `stand_keyboard`'s walk and its 20 activation rows re-pinned to it in both layout modes.
  > the stand checks' shared module `tests/common/mod.rs` — no test target and no test in it, the one statement of the
  > per-task tables `controls`, `tab_order`, `INPUT_NAMES` and `rendered` — joined at 2026-10-14-stand-a11y-assertions
  > by `ENABLED_TAB_ORDER` (the flight booker after `flight-return`, CRUD after a row is selected) and `REGION_NAMES`
  > (`crud-list` → `People`) (tests/blitz-tests/tests/common/mod.rs:72-99;
  > tests/blitz-tests/tests/common/mod.rs:133-136) — and of seven helpers — `tab_order` (CRUD six stops, `crud-list`
  > third) and the helper `focused` moved in from `stand_accessibility_ids` and `stand_snapshot_state` since
  > 2026-10-13-stand-keyboard-harness — read through `mod common;` by 21 stand checks (`stand_contrast` the eighteenth,
  > since 2026-10-13-stand-contrast-harness, `stand_keyboard` the nineteenth, since 2026-10-13-stand-keyboard-harness,
  > and `stand_requirement_sweep` and `stand_non_lean_tasks` the twentieth and twenty-first, since
  > 2026-10-14-stand-requirement-sweep) (tests/blitz-tests/tests/common/mod.rs:1-6;
  > tests/blitz-tests/tests/common/mod.rs:48-70; tests/blitz-tests/tests/common/mod.rs:101-121); since
  > 2026-10-13-stand-keyboard-harness the stand keyboard check `stand_keyboard`, 14 tests in both layout modes (11 until
  > 2026-10-14-stand-a11y-assertions, which added the last three and put it in the `a11y` CI leg) — Tab and Shift+Tab
  > walking each lean task's stated sequence with wrap, the engine's focus, the snapshot's `focused` and the tree's
  > focus agreeing at every step; Enter and Space on each focused stand button read as its click, 20 rows;
- (.andromeda/architecture-detail/existing-scopes.md:107) The flight booker's state after `flight-return` is reached by keyboard and walked in `ENABLED_TAB_ORDER`; once the one-way / return choice is a select, that enabled state and its six-stop walk must be re-reached by the select's keys and re-pinned, every `controls(task)` id still reached by Tab.
  > the walk beyond boot — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight
  > stops), each state reached by keyboard alone and walked in `ENABLED_TAB_ORDER` — every `controls(task)` id reached
  > by Tab (an empty set difference against `tab_order` and `ENABLED_TAB_ORDER` over every lean task), and Enter and
  > Space on the later-enabled `crud-update` and `crud-delete` each read as its click
- (.andromeda/architecture-detail/existing-scopes.md:108) The acceptance's recorded run that passes on replay maps onto the recording and replay checks, and the load wait the chunk brings to the stand is today witnessed only by an in-file fixture's load in flight read as not settled — a stand-held load must be read the same way; the committed kept checks stay the two Counter ones `stand_replay` replays.
  > a recorded session — `stand_session_recording`: the four lean tasks recorded, each record ending with its kept
  > answer (a shot's `{"width","height"}`, no `png` and no shot's first 64 base64 characters), and replayed through
  > `Check::replay` in both layout modes — each script ending `snapshot` · `screenshot`, so the shot path's kept form is
  > covered in process on each unix OS CI runs (linux, macOS; the file is unix only, as recording is) — a typed password
  > kept as the stand-in, a recording directory open to its group refused with nothing written
  > (tests/blitz-tests/tests/stand_session_recording.rs:108-286); the committed checks replayed — `stand_replay`, 3
  > tests in both layout modes, not unix-gated: each committed check holding on a fresh stand Counter, each red at its
  > first activation step (1 for the command line's check, 3 for the MCP role's) on a test-local variant `Counter` that
  > adds 2, every earlier step held and `changed` among its keys, and a check on a session of another app running
  > nothing, its failure messages holding the mode, the file and a step index alone
  > (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:198-234;
  > tests/blitz-tests/tests/stand_replay.rs:236-258); settle on a held instance — a Timer step returning with its
  > delayed update present and no sleep, an idle settle changing nothing, and on in-file fixtures a write made while
  > mounting, a load in flight read as not settled until it is answered, a running animation reported and not waited on,
  > a hover that moves with layout, and two instances that never go quiet ending at the bound, in both layout modes
  > (tests/blitz-tests/tests/stand_settle.rs:1-9);
  (.andromeda/architecture.md:238)
  > tests/blitz-tests/ checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture-detail/existing-scopes.md:111) `stand_act_select` is reached and proves the select only on an in-file fixture; the flight booker's real select must be driven by id the same way — `ComboBox` node, options as its children, `type` by label, the Tab walk — and the check stays in both the `a11y` and `stand-repeat` legs.
  > a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and
  > its options as its children, `type` with a label choosing that option, a click then a key, the Tab walk skipping a
  > disabled select, a disabled select and an unchosen option refused — 5 tests in both layout modes
  > (tests/blitz-tests/tests/stand_act_select.rs:79-256), run in both the `a11y` CI leg and the `stand-repeat` leg (by
  > its `stand_` prefix),
  (.andromeda/architecture.md:247)
  > stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the
  > test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no
  > job-level permission; .github/workflows/ci.yml:300-323)
- (.andromeda/architecture-detail/existing-scopes.md:112) The chunk edits the contrast rows: `flight-one-way` is one of the 18 `controls(task)` rows and its pass/exempt record, the `TEXT_ROWS` and `STATE_ROWS` must be re-measured through `Harness::text_contrast` for the select that replaces it, with SC 1.4.3 kept on every non-exempt row.
  > since 2026-10-13-stand-contrast-harness the stand contrast check `stand_contrast`, 5 tests in both layout modes (2
  > until 2026-10-14-stand-a11y-assertions) — the 18 `controls(task)` rows over the four lean tasks at boot each read
  > through `Harness::text_contrast` against a stated record, pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since the
  > stand's accent moved one step darker at 2026-10-14-stand-a11y-assertions (pass 9 · fail 5 until then; the five
  > former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` now `#ffffff` on
  > `#3a5ce5` at 5.47), exempt exactly where the snapshot's `enabled` reads `Some(false)`, and the painted background
  > matching the measured one within 1 per channel; since that chunk it also asserts every non-exempt control row, the
  > CRUD list rows and the text outside the controls (`TEXT_ROWS`, 14, all pass) and the colour states it reaches, hover
  > and active included (`STATE_ROWS`, 19: 18 pass, 1 exempt), one shared reader classifying all three tables, and runs
  > in the `a11y` CI leg (tests/blitz-tests/tests/stand_contrast.rs:49-137;
  > tests/blitz-tests/tests/stand_contrast.rs:481-512; tests/blitz-tests/tests/stand_contrast.rs:514-564;
  > tests/blitz-tests/tests/stand_contrast.rs:577-605; tests/blitz-tests/tests/stand_contrast.rs:607-635;
  > tests/blitz-tests/tests/stand_contrast.rs:637-661);
- (.andromeda/architecture-detail/existing-scopes.md:113) The sweep holds every lean task's screen inside `SNAPSHOT_TEXT_BUDGET` through `Session::run`; serving `cells` — the stand's one screen over the budget — must keep that per-lean-task claim true or re-pin it, never let an over-budget task into the lean set unread.
  > since 2026-10-14-stand-requirement-sweep, three targets, each auto-discovered with no `[[test]]` table —
  > `stand_requirement_sweep`, 3 tests in both layout modes: every requirement held through `Session::run` alone on each
  > lean task (the screen inside `SNAPSHOT_TEXT_BUDGET`, one distinct id per node line, each an accessibility node's
  > `author_id`, one id-addressed acting call settled with the diff the check reads itself, an empty diff for a click on
  > `task-title`, `press tab` focusing the first Tab stop, two byte-equal 800 × 600 shots, an unread id refused
  > `not-found`), and `WITNESSES`, 16 rows in requirement order naming each requirement's witness tests, each found in
  > its file through `include_str!` (tests/blitz-tests/tests/stand_requirement_sweep.rs:90-216;
  > tests/blitz-tests/tests/stand_requirement_sweep.rs:241-992)
- (.andromeda/architecture-detail/existing-scopes.md:114) `WITNESSES_020` is the 0.2.0 record a new stand witness of v020-07…v020-09 joins, every witness found in `stand-repeat`; and `stand_non_lean_tasks` pins Cells at 676 click-reacting elements with no role — serving `cells` to the driver must reckon with that pinned count and key every actionable element.
  > — and `WITNESSES_020`, the 0.2.0 record, nine rows and 46 witnesses, a `Unit` witness naming a crate's unit test,
  > run by the workspace `test` leg and outside `stand-repeat` by construction, read by its third test, every other
  > witness found inside `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text
  > (tests/blitz-tests/tests/stand_requirement_sweep.rs:227-229;
  > tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818;
  > tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069); `stand_non_lean_tasks`, 3 tests in both layout modes —
  > the per-task count of elements that react to a click and state no role (0 on the four lean tasks, Home and
  > Temperature Converter, Circle Drawer 1, Cells 676, measured and pinned), the Circle Drawer dialog rule applying, and
  > its open dialog read keyed (tests/blitz-tests/tests/stand_non_lean_tasks.rs:93-148);
- (.andromeda/architecture-detail/existing-scopes.md:76) The chunk re-asserts Cells and the new select through dioxus-native-dom's readers: every actionable element keyed by `unkeyed_actionable`, stable ids, the snapshot and its diff, and the over-budget screen read in parts through `Snapshot::text_parts` under `SNAPSHOT_TEXT_BUDGET` — all compiled under `accessibility` and left unchanged.
  > Modules actionable (crate-private, compiled only under `accessibility`: the actionable-key check behind
  > `DioxusDocument::unkeyed_actionable`, its `UnkeyedActionable` type re-exported), dioxus_document
  (.andromeda/architecture-detail/existing-scopes.md:76)
  > element_id (crate-private: the stable element id derivation behind `DioxusDocument::element_id` / `element_ids`),
  > events, mutation_writer, snapshot (crate-private, compiled only under `accessibility`: the snapshot model behind
  > `DioxusDocument::snapshot`, its three types, `is_masked_input` and the `MASKED_VALUE` constant re-exported),
  > snapshot_diff (crate-private, compiled only under `accessibility`: the snapshot's diff `Snapshot::diff`, its
  > `SnapshotDiff` and `DiffNode` types and, since 2026-10-12-self-description, the `Reading` enum re-exported),
  > snapshot_text (crate-private, compiled only under `accessibility`: the snapshot's text form `Snapshot::to_text` and
  > its parts reader `Snapshot::text_parts`, its `SNAPSHOT_TEXT_BUDGET` constant and `TextPart` type re-exported)
- (.andromeda/architecture-detail/existing-scopes.md:79) The re-measured contrast rows read `Harness::text_contrast` and the stand-held load is judged by `settle`, so both must be read through the harness as it stands, the harness unchanged.
  > Modules harness, input, inspect, settle (packages/blitz-test-harness/src/lib.rs:22-26) and, since
  > 2026-10-12-headless-screenshot, the private screenshot (`Harness::screenshot`, `screenshot_rect`, `PixelRect`,
  > `Raster`, `ScreenshotError`, the three types re-exported from the crate root;
  > packages/blitz-test-harness/src/screenshot.rs:37-103) and, since 2026-10-13-stand-contrast-harness, the private
  > contrast (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the last two re-exported from the crate root;
  > packages/blitz-test-harness/src/lib.rs:21; packages/blitz-test-harness/src/contrast.rs:13-91) — six modules in all
- (.andromeda/architecture-detail/existing-scopes.md:82) The live run counts wrong calls from the `tool_call` span lines printed through escher-telemetry's span field formatter, so that count must be read from those lines as the sink prints them, with no log field or span added.
  > Telemetry bootstrap: modules format (the event formatter with the target allowlist — its drop of every
  > outside-target record and its field scrub — and, since 2026-10-07-driver-command-spans, the crate-private span field
  > formatter `SpanFields`, through which a closed span's line is printed under the same rule) and panic (the chaining
  > panic hook) under the `init` entry point (packages/escher-telemetry/src/lib.rs:26-27;
  > packages/escher-telemetry/src/lib.rs:102-163)
- (.andromeda/architecture-detail/existing-scopes.md:16) A load in flight on the stand must be carried through blitz-traits' `net` provider seam — already a seven_guis dependency — the way the fixtures and tests dispatch loads through a pluggable `NetProvider`, never by naming `blitz_net` in the stand.
  > Modules devtools, events, navigation, net, node_id, shell (packages/blitz-traits/src/lib.rs:4-9)
  (.andromeda/architecture.md:250)
  > fetches go through `NetProvider::fetch` with a `ResourceHandler` that sends on the channel then `request_redraw`
  > (packages/blitz-dom/src/net.rs:116-125)
  (.andromeda/architecture.md:255)
  > tests dispatch loads through a pluggable `NetProvider` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24;
  > tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- (.andromeda/architecture-detail/existing-scopes.md:134) The chunk reaches `tests/fixtures/mod.rs` and `tests/common/mod.rs`: `HeldNet`, the held network stand-in, is stated once in the fixtures module and the per-task tables once in common, and any re-use of them for the stand must keep each stated in its one module, their reader counts re-measured.
  > blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21
  > checks, `tests/session_common/mod.rs`, by 29, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared
  > fixture pages and the held network stand-in `HeldNet`, by 24 (22 `stand_*` files, `font_face_sniffed_source` and
  > `select_interaction`); what each holds is test-plan §3's (tests/blitz-tests/tests/fixtures/mod.rs:17-130) — as
  > measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md.
- (.andromeda/architecture.md:241) The chunk edits `scripts/cold-agent.sh`'s task set and goals; the pipe stands as a `.sh` and `.ps1` pair, so the new goals over 0.2.0's capabilities must reach both or the PowerShell twin's divergence must be stated.
  > scripts/ agent-run.{sh,ps1} (test contract) · cold-agent.{sh,ps1} (cold-agent pipe) · code-graph tooling
- (.andromeda/architecture.md:326) The agent-written check is replayed on fresh children and possibly elsewhere: a check compared byte for byte across OSes must hold no `screenshot` step, even as the new goals exercise the screenshot.
  > A shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes
  > holds no `screenshot` step.

## Bindings
- **existing-scopes (seven_guis) ↔ standard-contracts (headless stand · CLIs)**: the stand module that boots the lean four and the `escher-session` binary over the four lean tasks are the scope the stand contract's terms (offline, pinned viewport, one bundled font, no `blitz_net`) and the command-line contract govern; widening the boot set moves both sides together (.andromeda/architecture-detail/existing-scopes.md:118), (.andromeda/architecture.md:246)
- **existing-scopes (blitz-tests stand checks) ↔ conventions (tests) and contract-ci-contracts**: every re-pinned stand check runs in both layout modes and sits in the `stand-repeat` leg by its `stand_` prefix or in the `a11y` leg's named list, so a renamed or new stand check moves the leg's set (.andromeda/architecture-detail/existing-scopes.md:111), (.andromeda/architecture.md:247)
- **existing-scopes (blitz-tests shared modules, blitz-traits) ↔ occupied-resources (network ports, outbound hosts) and established-decisions (resource loading)**: a stand-held load reuses the fixtures' `HeldNet` shape through a pluggable `NetProvider` and must open no listener or port (.andromeda/architecture-detail/existing-scopes.md:134), (.andromeda/architecture-detail/existing-scopes.md:16), (.andromeda/architecture.md:255)
- **existing-scopes (escher-driver, escher-telemetry) ↔ established-decisions (driver session) and the security plan's crossings**: the driver the run exercises names no app and adds no verb or span; the screens the live run carries (Cells, the select) cross only through its ratified carriers, and wrong calls are counted from the `tool_call` lines the sink prints (.andromeda/architecture-detail/existing-scopes.md:87), (.andromeda/architecture-detail/existing-scopes.md:82)
- **existing-scopes (dioxus-native-dom) ↔ standard-contracts (Dioxus DOM bridge · snapshot in parts)**: Cells reached by `snapshot`'s `part` reads `Snapshot::text_parts` under `SNAPSHOT_TEXT_BUDGET`, and the per-lean-task in-budget claim of the sweep moves with it (.andromeda/architecture-detail/existing-scopes.md:76), (.andromeda/architecture-detail/existing-scopes.md:113)
- **existing-scopes (scripts) ↔ occupied-resources (names `scripts/cold-agent.sh`, `mcp__escher`)**: the pipe the chunk re-goals is a registered name, its `.sh`/`.ps1` pair moving together (.andromeda/architecture.md:241)
