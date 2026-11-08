@carried n=1 label=architecture/existing-scopes fact=fct-7irbn6kv
why: The graph must be keyed on the agent-visible accesskit node this builder produces, read as it stands, and the builder must stay unedited since the chunk makes no blitz-dom change.
quote: Accessibility tree building (packages/blitz-dom/src/accessibility.rs:10-279)
@carried n=2 label=architecture/existing-scopes fact=fct-dac2ih77
why: seven_guis reaches dioxus-native-dom only through dioxus-native, so the plan must decide how the stand reaches the new registry: a direct dependency, or a re-export through dioxus-native's hooks or prelude.
quote: Modules assets, config, contexts, dioxus_application, dioxus_renderer, event_handlers, hooks, link_handler, prelude (packages/dioxus-native/src/lib.rs:12-22); depends on dioxus-native-dom (packages/dioxus-native/Cargo.toml:92)
@carried n=3 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The graph's link 1 must reuse the crate-private stable-id derivation, and the reserved virtual-node id form must keep what the actionable-key check reports.
quote: element_id (crate-private: the stable element id derivation behind `DioxusDocument::element_id` / `element_ids`)
@carried n=3 run=2 label=architecture/existing-scopes fact=fct-kaaobbkf
quote: Modules actionable (crate-private, compiled only under `accessibility`: the actionable-key check behind `DioxusDocument::unkeyed_actionable`, its `UnkeyedActionable` type re-exported)
@carried n=4 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The «reacts to a click» field must join the snapshot model under its `accessibility` gate, and every value the graph carries must read masked through the one `is_masked_input` test and `MASKED_VALUE`.
quote: snapshot (crate-private, compiled only under `accessibility`: the snapshot model behind `DioxusDocument::snapshot`, its three types, `is_masked_input`, the `MASKED_VALUE` constant, `closed_select_of`, `is_multiple_select` and `is_select` re-exported)
@carried n=4 run=2 label=architecture/existing-scopes fact=fct-kaaobbkf
quote: snapshot_diff (crate-private, compiled only under `accessibility`: the snapshot's diff `Snapshot::diff`, its `SnapshotDiff` and `DiffNode` types and, since 2026-10-12-self-description, the `Reading` enum re-exported), snapshot_text (crate-private, compiled only under `accessibility`: the snapshot's text form `Snapshot::to_text` and its parts reader `Snapshot::text_parts`, its `SNAPSHOT_TEXT_BUDGET` constant and `TextPart` type re-exported)
@carried n=5 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The graph read's new verb must join the one `'static` verb table and its node-key lists, and its outcome must be written by hand in json and described from the tables alone.
quote: schema (the `'static` verb table `VERBS` in its three levels — the tool level and its `help` row since 2026-10-12-self-description — with its argument and field specs, the kinds and their bounds, the key, busy-class and node-key lists, each node key with its help since that chunk, and the exit-status table `EXIT_STATUSES`;
@carried n=5 run=2 label=architecture/existing-scopes fact=fct-mzrxx2a3
quote: json (the hand-written forms `Outcome::to_json`, `Refusal::to_json` and `SessionError::to_json`)
@carried n=5 run=3 label=architecture/existing-scopes fact=fct-mzrxx2a3
quote: describe (`describe`, the self-description written from the tables alone)
@carried n=6 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: The new verb must run through the executor that opens the one command span, be served as an MCP tool through json's forms, and be kept by the recording as a replayable step.
quote: execute (`Session::run` and `Outcome`, the executor — it resolves a call's id on the screen, detects the five screen-level refusals before anything runs, opens the one `tracing` span of a command, and carries two macOS `cfg` gates, the backward-delete arm of `press` and the select-all's modifier)
@carried n=6 run=2 label=architecture/existing-scopes fact=fct-mzrxx2a3
quote: since 2026-10-10-mcp-surface mcp (`serve_mcp`, the MCP server holding one session, its replies written through json's forms, opening the `tool_call` span;
@carried n=6 run=3 label=architecture/existing-scopes fact=fct-mzrxx2a3
quote: recording (the crate-private writer of a kept check — each record's kept answer line `kept_line`, a shot's without its image
@carried n=7 label=architecture/existing-scopes fact=fct-mzrxx2a3
why: escher-driver must keep its dependency and feature set: the graph reader reaches it through dioxus-native-dom under `accessibility`, and no app or new crate is added.
quote: it depends on blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` (each with no feature), names one feature — dioxus-native-dom's `accessibility` — and no app
@carried n=8 label=architecture/existing-scopes fact=fct-4zmppehn
why: The «reacts to a click» field absorbs this reading, so the chunk must re-pin it, and the count stated here (Cells 677) must be reconciled with the scope's 676.
quote: the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid` with its `onkeydown`, measured and pinned), the Circle Drawer dialog rule applying, and its open dialog read keyed (tests/blitz-tests/tests/stand_non_lean_tasks.rs:93-149)
@carried n=9 label=architecture/existing-scopes fact=fct-4zmppehn
why: The graph read's call must leave the one command span this capture reads, holding no id, name or typed text, so no component, handler, state or value reaches it.
quote: the one command span a call leaves, read from a sink capture in a re-run child — one `escher_driver` line per call holding its verb, a refused call's cause, an acting call's settle reading and the lengths of its diff, none at the default level, and no id, name or typed text anywhere in the capture
@carried n=10 label=architecture/existing-scopes fact=fct-4zmppehn
why: A snapshot that gains a field can change a committed check's kept answer, so the two committed checks must still replay green on a fresh stand, or be re-recorded byte for byte.
quote: the committed checks replayed — `stand_replay`, 3 tests in both layout modes, not unix-gated: each committed check holding on a fresh stand Counter, each red at its first activation step (1 for the command line's check, 3 for the MCP role's)
@carried n=10 run=2 label=architecture/existing-scopes fact=fct-qkp2micb
quote: and `record_checks.sh`, which `recording` runs to record the two committed checks and compare them byte for byte
@carried n=10 run=3 label=architecture fact=fct-77tjcb3p
quote: checks/ — the two committed kept checks `stand_replay` replays
@carried n=11 label=architecture/existing-scopes fact=fct-4zmppehn
why: A state or control value the graph carries must read `MASKED_VALUE` through every carrier, as the MCP masked check pins for `type`.
quote: `stand_mcp_masked` — a password typed through an MCP `type` reads `MASKED_VALUE` and never the typed text
@carried n=12 label=architecture/existing-scopes fact=fct-qkp2micb
why: The stand's hook-local signals must move onto the registry, and the new verb must reach escher-session's command line and its spawned-binary tests over the six served tasks.
quote: a second binary, `escher-session` — the driver's command line over six served tasks (the four lean ones, Cells and the load page)
@carried n=12 run=2 label=architecture/existing-scopes fact=fct-qkp2micb
quote: native dependencies escher-telemetry, blitz-test-harness, blitz-traits and escher-driver, `dioxus-native` with `system-fonts` + `woff` (examples/seven_guis/Cargo.toml:29-35)
@carried n=13 label=architecture/existing-scopes fact=fct-okim3jaw
why: Cells' 676 data cells react to a click, so if the new field shows in the text form, this measured screen size and its part count must be re-measured.
quote: Cells booted through `stand::boot_cells` and held as a session, its screen 47,089 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET` through the driver
@carried n=14 label=architecture/existing-scopes fact=fct-o6vmxcuj
why: A new stand or session check of the chunk that reads a shared module must move the reader count recorded here.
quote: `tests/common/mod.rs`, read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 32, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 31
@carried n=15 label=architecture/existing-scopes fact=fct-ktfizprz
why: Links 1 to 4 must rest on what this probe measured through Dioxus' public API alone, and the probe must stay the witness of those mechanisms, outside the stand legs.
quote: reading Dioxus 0.7.10's public API alone: a node's owning component, its listeners and a held signal's readers; it holds no print, env read, subscriber or listener, and has no `stand_` prefix, so it is in neither `run stand`, `stand-repeat` nor the `a11y` leg, run by the workspace `test` leg and `run all` (tests/blitz-tests/tests/dioxus_runtime_probe.rs:198-631)
@binding n=1
**dioxus-native-dom's modules ↔ seven_guis and dioxus-native**: the signal registry and graph reader must be new crate-private modules of dioxus-native-dom, re-exported where public, and seven_guis, whose native dependencies do not name dioxus-native-dom, must reach the registry either by a direct dependency or by a re-export through dioxus-native. The plan must pick one (architecture/existing-scopes fct-kaaobbkf) (architecture/existing-scopes fct-qkp2micb) (architecture/existing-scopes fct-dac2ih77)
@binding n=2
**escher-driver's modules ↔ blitz-tests' driver checks**: the graph read must join `VERBS`, json's hand-written forms, `describe`, the MCP tools and the recording, and its call must leave the one command span the sink capture reads, with no id, name or typed text in it (architecture/existing-scopes fct-mzrxx2a3) (architecture/existing-scopes fct-4zmppehn)
@binding n=3
**the snapshot's new field ↔ the committed checks and Cells' measured screen**: the «reacts to a click» boolean in the snapshot model must leave `stand_replay`'s two committed checks green, or have them re-recorded through `record_checks.sh` byte for byte, and Cells' 47,089 bytes in 5 parts must be re-measured wherever the text form carries the field (architecture/existing-scopes fct-kaaobbkf) (architecture/existing-scopes fct-4zmppehn) (architecture/existing-scopes fct-qkp2micb) (architecture/existing-scopes fct-okim3jaw)
@binding n=4
**stand_non_lean_tasks' starting reading ↔ the scope**: the re-pin must start from the measured count, and the entry's Cells 677 (676 data cells plus `cells-grid`) must be reconciled with the scope's Cells 676 before the new field's count is pinned (architecture/existing-scopes fct-4zmppehn)
@binding n=5
**blitz-dom's accessibility builder ↔ the runtime probe**: the graph must be keyed on the accesskit node the unedited builder produces, and links 1 to 4 must be read only through the public Dioxus API mechanisms the probe measured (architecture/existing-scopes fct-7irbn6kv) (architecture/existing-scopes fct-ktfizprz)
@binding n=6
**blitz-tests' shared modules ↔ the chunk's new checks**: every new check that reads `tests/common`, `tests/session_common` or `tests/fixtures` must move that module's recorded reader count (architecture/existing-scopes fct-o6vmxcuj)
