@carried n=1 label=architecture/existing-scopes fact=fct-f647agr4
why: The read's covered cause must be decided through the same hit the executor reads, which answers a single select's open list first, so a row or a target under an open list is reported as a real call refuses it.
quote: Hit testing and `elementFromPoint`/`elementsFromPoint` (packages/blitz-dom/src/document.rs:2130-2191); a single select's open list, its readers and writer and its hit, answered first in `hit_with_scrollbar` (packages/blitz-dom/src/document.rs:772-1100; packages/blitz-dom/src/document.rs:2436-2438)
@carried n=2 label=architecture/existing-scopes fact=fct-fgeurl7w
why: The read's off-screen cause must be decided through the visible-region reader, the one the shared refusal path reads, never from a hit.
quote: into-view scrolling through nested scrolling boxes and the visible-region reader (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/scrolling.rs:655-796)
@carried n=3 label=architecture/existing-scopes fact=fct-cixr44np
why: The owner-served case is a select serving its options and groups, whose rules and open-list state stay crate-private in blitz-dom, so the read must reach them only through the public readers and leave the select checks holding.
quote: select (the crate-private rules of the select model, holding `SelectCursor`, `RowGesture` and `Gestured`; packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89), select_list (the crate-private state and geometry of a single select's open list, with 6 unit tests, its two public read types `SelectListView` and `SelectListRow` re-exported from the crate root; blitz-paint's `render::select_list` paints it, and `select_popup`, `select_popup_painted` and `stand_act_popup` are its checks;
@carried n=4 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The actionable-key check the read may fold in lives in this crate-private module compiled only under `accessibility`, so the read must reach it through `DioxusDocument::unkeyed_actionable` and its re-exported type.
quote: Modules actionable (crate-private, compiled only under `accessibility`: the actionable-key check behind `DioxusDocument::unkeyed_actionable`, its `UnkeyedActionable` type re-exported)
@carried n=5 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The listed node's id must be the stable element id of these four tiers, and its optional owner must stay open to the reserved `{owner}///{key}` form spelled by `virtual_node_id`.
quote: element_id (crate-private: the stable element id derivation behind `DioxusDocument::element_id` / `element_ids`, the walk's `element_facts` and `element_listeners`, and the reserved `{owner}///{key}` form's `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id`, both re-exported)
@carried n=6 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The read's node model must be the UI graph's `GraphNode` read through the read-only `DioxusDocument::graph`, its handlers and owners, under `accessibility`.
quote: graph (crate-private, compiled only under `accessibility`: the read-only reader `DioxusDocument::graph`, its `UiGraph`, `GraphNode`, `GraphHandler`, `GraphWrite`, `GraphState`, `GraphWriter` and `WriteFlag` types re-exported)
@carried n=7 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The read's actionable set reads the snapshot's `clickable` and the select tests `closed_select_of` and `is_select` from this module, read-only and with no `NodeId`.
quote: snapshot (crate-private, compiled only under `accessibility`: the snapshot model behind `DioxusDocument::snapshot`, its three types, `is_masked_input`, the `MASKED_VALUE` constant, `closed_select_of`, `is_multiple_select` and `is_select` re-exported)
@carried n=8 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The driver's module count must be restated if the read adds a module of its own.
quote: the MCP server that serves the verbs as tools over a caller's reader and writer: seventeen private modules
@carried n=9 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The new verb and its per-node verbs must be stated in and read from the one `'static` verb table `VERBS`, with no closed list of verb names elsewhere.
quote: schema (the `'static` verb table `VERBS` in its three levels — the tool level and its `help` row since 2026-10-12-self-description — with its argument and field specs, the kinds and their bounds, the key, busy-class and node-key lists, each node key with its help since that chunk, and the exit-status table `EXIT_STATUSES`;
@carried n=10 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: Any field the verb table gains to say which verbs act on a target or which refusals a target meets must be added to `VerbSpec` in the private child spec, re-exported, with its tests in `schema_tests.rs`.
quote: since 2026-10-16-file-splits its six shape types `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `Level` and `VerbSpec` in its private child spec, re-exported, and its tests in `schema_tests.rs`)
@carried n=11 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The read's refusal causes must come from the executor's own detection of the screen-level refusals, the one shared path, and the read must open no span beyond the one command span.
quote: execute (`Session::run` and `Outcome`, the executor — it resolves a call's id on the screen, detects the five screen-level refusals before anything runs, opens the one `tracing` span of a command, and carries two macOS `cfg` gates, the backward-delete arm of `press` and the select-all's modifier)
@carried n=12 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The new verb must enter through `Command`, `validate` and `call_of`, and its causes must be among the closed `Cause`/`CAUSES`.
quote: command (`Call`, `ArgValue`, `Key`, `Command`, `validate`, `SessionCommand`, `validate_session`, and the crate-private `call_of` that turns JSON arguments into a call), refusal (`Cause`, `CAUSES`, `Fault`, `Refusal`)
@carried n=13 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The read's answer must be written by hand in `json` and carried by `cli`'s `command_line` and its `help` ending.
quote: json (the hand-written forms `Outcome::to_json`, `Refusal::to_json` and `SessionError::to_json`) and cli (`command_line`, the whole command line of a binary that boots apps; its `help` ending since 2026-10-12-self-description)
@carried n=14 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The MCP listing and result of the new read must land in the `listing` and `result` children, each `pub(super)` and no further.
quote: with three private children, each widened to `pub(super)` and no further — framing (the request reader, the id, the protocol errors and the reply writers), listing (the handshake and the tool listing) and result (a tool call's result and its written form) — and its tests in `mcp_tests.rs`)
@carried n=15 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The self-description must state the new read from the tables alone, and a kept check must record and replay it through `Session::run`.
quote: describe (`describe`, the self-description written from the tables alone)
@carried n=15 run=2 label=architecture/existing-scopes fact=fct-mzrxx2a3
quote: replay (`Check::replay`, `Replay` and `Change`: a kept check's calls run again through `Session::run`, stopping at the first step whose acceptance or kept answer differs)
@carried n=16 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The new read must follow the read-only `lint` and `graph` verbs' pattern — a schema entry, a `Command` variant, an `Outcome` variant and a hand-written json form — re-exported at the crate root.
quote: lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the visual lint, carried by the `lint` verb — execute holding `Outcome::Linted`, json the finding's written form) and the `graph` verb — schema's `GRAPH` and its two `FieldKind`s, command's `Command::Graph`, execute holding `Outcome::Graphed`, json the graph's hand-written node and state forms — re-exported at the crate root beside blitz-test-harness's `Busy` and `Settled`;
@carried n=17 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The read must add no dependency or feature to escher-driver beyond dioxus-native-dom's `accessibility`, and name no app.
quote: it depends on blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` (each with no feature), names one feature — dioxus-native-dom's `accessibility` — and no app
@carried n=18 label=architecture/existing-scopes fact=fct-4zmppehn
why: These stand checks witness the disabled, covered and off-screen refusals a real call returns, which the read's per-verb causes must agree with.
quote: a control refused while it reads not enabled and acted on once it reads enabled, a covered and an out-of-view target refused on in-file fixtures
@carried n=19 label=architecture/existing-scopes fact=fct-4zmppehn
why: The owner-served select case the read lists is witnessed here — a disabled select and an unchosen option refused and a closed group's option refused `covered` — and the read must report the same causes.
quote: a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children, `type` with a label choosing that option, a click then a key, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused, a `multiple` select's name, role and rows each read with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` — 10 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:85-466)
@carried n=19 run=2 label=architecture/existing-scopes fact=fct-4zmppehn
quote: a closed option refused `covered`, a row past the scrollport off screen and brought in by `scroll`, a button under the list refused `covered`
@carried n=20 label=architecture/existing-scopes fact=fct-4zmppehn
why: The read's actionable set must be measured against the snapshot's `clickable` counts pinned per task, the P3 question whether one rule serves both reads.
quote: the per-task count of those role-less elements whose snapshot reads `clickable` (`CLICKABLE`: Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676)
@carried n=21 label=architecture/existing-scopes fact=fct-4zmppehn
why: The graph verb's check states the handlers and owners the read reads, no served id holding `///`, a read changing nothing and the Cells answer within the answer bound — the shape the new read's check must keep.
quote: `stand_graph`, 6 tests in both layout modes — the verb's answer equal to `DioxusDocument::graph`, Counter's declared write read at boot and confirmed after a click, every stand task's declarations agreeing with its code once its handlers have run, the served screens' handlers, owners and clicks, every graph node a snapshot node with no served id holding `///`, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound — in `stand-repeat` by its `stand_` prefix (tests/blitz-tests/tests/stand_graph.rs:106-394)
@carried n=21 run=2 label=architecture/existing-scopes fact=fct-4zmppehn
quote: and a `graph` tool call answering the readers' graph and parenting one `command` span that records its verb alone (tests/blitz-tests/tests/stand_mcp_tools.rs:771-843)
@carried n=22 label=architecture/existing-scopes fact=fct-4zmppehn
why: The new read's command span must record its verb and, refused, its cause alone, holding no id, name or typed text, as the span check reads from a sink capture.
quote: the one command span a call leaves, read from a sink capture in a re-run child — one `escher_driver` line per call holding its verb, a refused call's cause, an acting call's settle reading and the lengths of its diff, none at the default level, and no id, name or typed text anywhere in the capture
@carried n=23 label=architecture/existing-scopes fact=fct-4zmppehn
why: A builder and reader for the new read, and a new check reading this module, must be recorded in this module's list of readers and helpers.
quote: and what the checks that hold a session share, the module `tests/session_common/mod.rs` — no test target and no test in it, read through `mod session_common;` by five of the session checks, by `stand_settle`, by the sixteen `stand_act_*`, since 2026-10-10-mcp-surface, by `stand_mcp_tools` and, since 2026-10-12-headless-screenshot, by `stand_screenshot_repeat` and, since 2026-10-14-stand-requirement-sweep, by `stand_requirement_sweep` and by `stand_virtual_clock`, and by `stand_replay`, `stand_cells_served`, `stand_load_page` and `stand_graph` — the readers counted below —, and holding since 2026-10-07-act-by-id the driver's call builders, an outcome reader and a focus reader beside the session helpers, since 2026-10-07-refusal-detection the `scroll` builder and two refusal readers — the cause of a refused call, and that cause with whether the snapshot and both focus readings stayed equal —, since 2026-10-10-driver-cli the `snapshot` builder and since 2026-10-12-headless-screenshot the two `screenshot` builders and the `shot` reader, and `shot_prefix`, the first 64 base64 characters of a whole-screen shot as the driver writes it, and the `graph` builder and the `graphed` reader
@carried n=23 run=2 label=architecture/existing-scopes fact=fct-o6vmxcuj
quote: blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 22 checks, `tests/session_common/mod.rs`, by 33, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 31
@carried n=24 label=architecture/existing-scopes fact=fct-4zmppehn
why: A kept check recording the new read must keep its answer line as the recording check reads it, and replay it in both layout modes.
quote: a recorded session — `stand_session_recording`: the four lean tasks recorded, each record ending with its kept answer
@carried n=25 label=architecture/existing-scopes fact=fct-qkp2micb
why: The read must be proven on the headless stand and carried out by `escher-session`, whose argv reaches the edited `command_line` over the six served tasks.
quote: native-only module `stand`, the headless stand booting the lean four, Cells and the load page in TaskShell
@carried n=25 run=2 label=architecture/existing-scopes fact=fct-qkp2micb
quote: a second binary, `escher-session` — the driver's command line over six served tasks (the four lean ones, Cells and the load page) and, in its host role, the headless session host and, since 2026-10-10-mcp-surface, in its `mcp` role an MCP server over its standard streams: the one place a served task, the stand's boot and the timer's tick handle meet a driver `Session`, its `main` handing argv and that boot to `escher_driver::command_line`
@carried n=26 label=architecture/existing-scopes fact=fct-qkp2micb
why: A change to the recording's kept form must keep the two committed checks byte-equal to what `record_checks.sh` records.
quote: and `record_checks.sh`, which `recording` runs to record the two committed checks and compare them byte for byte
@binding n=1
**existing-scopes (escher-driver execute) ↔ existing-scopes (blitz-dom hit and visible region)**: the read's covered and off-screen causes must come from the executor's one refusal path, which reads the hit that answers an open select list first and the visible-region reader, so the list cannot name a cause a real call would not return (architecture/existing-scopes fct-mzrxx2a3) (architecture/existing-scopes fct-f647agr4) (architecture/existing-scopes fct-fgeurl7w)
@binding n=2
**existing-scopes (dioxus-native-dom graph, snapshot, actionable) ↔ existing-scopes (blitz-tests CLICKABLE and stand_graph)**: the read's actionable set draws on the graph's handlers and owners, the snapshot's `clickable` and `unkeyed_actionable`'s set, and must be measured against the per-task `CLICKABLE` counts and the graph check's owners (architecture/existing-scopes fct-kaaobbkf) (architecture/existing-scopes fct-4zmppehn)
@binding n=3
**existing-scopes (escher-driver schema and spec) ↔ existing-scopes (escher-driver json, cli, mcp, describe, recording, replay)**: a new verb or `VerbSpec` field stated once in `VERBS` must reach every carrier — the hand-written json form, the command line, the MCP listing and result, the self-description and a kept check — as the `graph` verb did (architecture/existing-scopes fct-mzrxx2a3)
@binding n=4
**existing-scopes (blitz-tests session_common) ↔ existing-scopes (blitz-tests shared-module counts)**: a builder for the new read and each new stand check reading `session_common` or `fixtures` must be recorded in the module's list and in the reader counts 33 and 31 (architecture/existing-scopes fct-4zmppehn) (architecture/existing-scopes fct-o6vmxcuj)
@binding n=5
**existing-scopes (seven_guis escher-session) ↔ existing-scopes (escher-driver cli and recording)**: the new verb reaches the stand's binary through `escher_driver::command_line`, and a kept-form change must keep the committed checks that `record_checks.sh` compares byte for byte (architecture/existing-scopes fct-qkp2micb) (architecture/existing-scopes fct-mzrxx2a3)
