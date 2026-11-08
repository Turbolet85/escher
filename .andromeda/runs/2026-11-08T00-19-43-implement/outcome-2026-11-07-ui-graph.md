# Implement outcome — 2026-11-07-ui-graph

**Outcome: green + smoke.** The plan's 19 non-operator gate entries are green at the second full block firing
(`gate-2026-11-07-ui-graph.json`, summary `entries 22 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 3`).
Three later targeted re-fires after test-only edits are also green: entries 4 and 11, 16 (stand run 244 · 0 · 7),
and 17 (`fast`, 17.4.log). After the operator's reader fix, entries 1, 3, 4, 10, 11, 16, 17 and 18 were re-fired:
`entries 22 · green 8 · red 0`, stand run 244 · 0 · 7 (16.4.log), `fast` 17.5.log, `doc` 18.3.log. v030-05 is
`implemented`, its ref written by `matrix.py implement`. After the writers change the same eight entries were
re-fired: `entries 22 · green 8 · red 0` (16.5.log, 17.6.log, 18.4.log).

Next → **the operator pass**: entries 20 (hygiene), 21 (the push) and 22 (the CI read) are `leg = 'operator'` and
this skill fires none of them. Then `/andromeda-wrap-session`.

## What was built
- **dioxus-native-dom**
  - `element_id.rs`: the reserved fifth id form, `VIRTUAL_KEY_SEPARATOR` (`///`) and `virtual_node_id`, which no walk
    calls. The walk also yields each element's owning scope and component and its current listeners
    (`element_facts`, `element_listeners`).
  - `signal_registry.rs`: the new `use_graph_state(name, writers, Subscribers)`, `DeclaredWrite`, `SignalRegistry` and
    the write record.
  - `dioxus_document.rs`: the dispatch is set around `runtime.handle_event` only when a registry exists.
  - `graph.rs`: the new `DioxusDocument::graph` → `UiGraph`.
  - `NodeState.clickable` and the ` clickable` token. `Reading::Clickable` makes `Reading::ALL` twelve.
- **escher-driver**: `GRAPH` (`VERBS` thirteen, nine instance verbs). Two new `FieldKind`s (`graph-nodes` and
  `graph-states`) make twelve. `NODE_FIELDS` is fourteen with `clickable`. Also `Command::Graph`,
  `Outcome::Graphed` and the hand-written JSON form, and describe's row `[13, 14, 12, 3, 8, 4, 15]`.
- **seven_guis**: all seven tasks register their signals through the stand's `use_graph_signal`, each with a
  declaration read off its code. Markup, ids and order are unchanged.
- **Tests**
  - New `stand_graph`, with 6 tests in both layout modes.
  - `stand_mcp_tools` gains the graph tool call's parenting test.
  - `stand_non_lean_tasks` gains `CLICKABLE`.
  - seven_guis: `cli_commands` and `mcp_server` each gain a graph test, and `host_log` and `mcp_log` a graph call
    with graph needles.
  - Counts moved in the schema, json, command, cli, mcp and recording unit tests.
  - The two committed checks were re-recorded by entry 8. They hold no host path, `png` or `screenshot` step.
- **Docs and scripts**: `docs/capabilities.md` (the verb, fourteen node keys, a UI-graph section, its limits, the MCP
  carried set, its proof), `README.md` («the thirteen driver commands as tools»), and `VERBS` in `scripts/cold-agent.sh`
  and `.github/scripts/test_cold_agent.py` (plus the fake's `graph`). The `capability-unused` set is unchanged.

## Deviations from the plan, with justifications
1. **Step 3's hook signature.** The plan wanted `use_graph_signal(name, writers, init) -> Signal<T>` in
   dioxus-native-dom. That crate names only dioxus-core and dioxus-html, neither of which re-exports `Signal`, and
   entry 13 forbids a manifest or lock change. So dioxus-native-dom offers `use_graph_state(name, writers,
   subscribers: Subscribers)`, using dioxus-core types alone. The `Signal`-returning `use_graph_signal` the stand
   uses lives in `examples/seven_guis/src/tasks/mod.rs`, a scope-record line:
   `- examples/seven_guis/src/tasks/mod.rs · in-intent · serves step 8 · self`. A non-stand app registers with
   `use_graph_state(name, writers, signal.subscribers())`.
2. **No re-subscribe.** Step 3 has the watcher's callback re-subscribe after each write. dioxus-signals 0.7.10's
   `update_subscribers` keeps every context whose `mark_dirty` returned true (`signal.rs:264-267`), so the
   subscription persists and the callback records only.
3. **`resolve_write` stops at a gone node.** Step 5's rule (first listening element of the chain, the writing scope
   first) handed a write by the since-removed Cells editor input to `cells-grid`, read `undeclared`. A record whose
   chain holds a node that no longer resolves before its candidate now resolves to nothing. This matches step 5's own
   «a record whose element no longer resolves is skipped». Pinned by the new
   `graph::tests::a_write_by_an_element_gone_since_is_not_handed_to_its_ancestor`.
4. **Circle Drawer's canvas declarations.** At first I left the async canvas handlers undeclared, expecting their
   writes to run in a later task. `stand_graph` read them `undeclared`: the spawned task runs inside the click's own
   dispatch. The canvas `click` is now declared for `circles`, `undo_stack` and `redo_stack`, and `dblclick` for
   `dialog_idx` and `dialog_r`. The double click's event name is `dblclick`.
5. **The stand's drive table** (`common::GRAPH_DRIVES`, driver calls per task) drives each conditional write once.
   Cells clicks A0, C0 and A0 so each click commits the other's editor, and leaves the keyed editor once by Enter and
   once by Tab.
6. **Listed files left unedited** (no edit needed): `mcp/listing.rs`, `mcp/result.rs` and `recording.rs` route every
   verb through the table and `to_json`. `stand_snapshot_text` kept its ceilings, and `stand_lint` and
   `stand_virtual_clock` count nothing the change moved.
7. **Companion edits outside research's lists**, in the scope record (gate.py scope: `clean`):
   - `- packages/escher-driver/src/mcp.rs · companion · serves step 7 · self` (its doc's «nine instance verbs»)
   - `- tests/blitz-tests/tests/stand_act_scroll.rs · companion · serves step 7 · self` (`VERBS.len()` 13)
   - `- tests/blitz-tests/tests/stand_cells_served.rs · companion · serves step 6 · self` (`PARTS` 5 → 6)
   - `- tests/blitz-tests/tests/stand_snapshot_parts.rs · companion · serves step 6 · self` (the focused line now
     reads ` focused clickable @`)

   `stand_requirement_sweep` (listed) also had its 0.2.0 witness renamed to
   `the_thirteen_verbs_are_named_in_order_each_at_its_level`.

## Measurements (`.andromeda/cache/chunks/2026-11-07-ui-graph/evidence/measurements.md`)
- **The Cells `graph` answer is 293,869 bytes** at boot (the first build read 547,685), under the 1,048,576 bound;
  the plan forecast about 200 KB. Two operator fixes brought it down:
  - (inputs#I7) a state's `readers` names its reading components, each once, joined to nodes by their `component`.
    Cells' states read `["Cells"]` or `[]`, and the answer went to 522,692.
  - (inputs#I8) a state's `writers` lists only the writes a run confirmed or flagged, `declared` keeping the
    declaration. The answer went to 293,869: `nodes` 292,790, `states` 1,058.

  The remainder is each cell node's declared `click` writes, kept as the plan has them. It is recorded as the stated
  limit «An unpaged graph» in `docs/capabilities.md` and residual `res-7cc5bi4k` (target next). `stand_graph` holds
  the reader join on every served screen, asserts no writer before a run confirms one and one after, and holds the
  Cells answer at more than 100,000 bytes and at most the bound.
- **The Cells screen reads in 6 parts** (5 at the base).
- **`CLICKABLE`** reads Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1 and
  Cells 676. The mark's count stays 677.
- **`run stand`** reads 244 · 0 · 7 (237 · 0 · 7 at the base).
- **dioxus-native-dom** has 100 unit tests passing (83 at the base; 1.2.log), and **escher-driver** 100 (the base's 100 tests, rows
  extended).

## Acceptance against the matrix row
v030-05's text is met as written:
- the graph through `Session::run`, the command line, MCP and in process (`stand_graph` holds the verb equal to
  `DioxusDocument::graph`);
- Counter's `count` declared at boot, confirmed after the click and read by `counter-value`;
- no flag on any stand task once its handlers have run;
- both flags on a page whose declaration drifts (graph unit test);
- `clickable` reads true on `counter-increment` and the 676 cells, and false on `counter-value` and `cells-grid`.

No premise disproof.

## Smoke
Fired on the boot path: the stand's tasks feed both binaries. `seven_guis_native` booted windowed under `timeout 12`,
exited 124 (killed by the timeout), with no panic and empty stderr. It opens on Home, which registers no state, so
this proves the boot alone. The registry ran in the `escher-session` binary through entry 10 and a manual
`start cells` · `graph` · `stop`.

## Guard entries
None in the plan.

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `seven_guis_native` (windowed smoke) | this run | terminated (timeout SIGTERM, exit 124) |
| `escher-session` host (`start cells`, manual measurement) | this run | terminated (`stop` answered `{"stopped":true}`) |
| test hosts and children of entries 10 and 16 | a leg | terminated |

Census: `gate.py census --comm seven_guis_native --comm escher-session` → `census: none running`.

## Decisions & corrections
- Operator correction (2026-11-08, after the first report): «Make each state list its reading COMPONENTS, not every
  element of them … Add no argument and no new field (a part argument would widen the crossing).» Applied in
  `graph.rs`, the `states` field's help, the json unit test, `stand_graph` (the join asserted) and
  `docs/capabilities.md`. No argument or field added.
- Operator correction (2026-11-08, after the reader fix, inputs#I8): «Make the writers change: a state's writers lists
  only the writes a run confirmed or flagged … record the remaining size as a stated limit and a residual row, target
  next.» Applied in `graph.rs`, the `states` help, `stand_graph` and `docs/capabilities.md`. Residual `res-7cc5bi4k`
  was minted (`rec.py put`, tx 288).
- Guard refusal: the Bash guard refused a `cd` into the skill's references dir. Re-issued with absolute paths.
- Guard refusal: the Bash guard refused a `cat` heredoc to a scratch script. Edits went through the Edit tool instead.
- Recurrence of the standing learning (2026-10-10: search `tests/` for a table's readers before calling a check
  unaffected). Four pins outside research's lists turned red at the first full blitz-tests pass: `stand_act_scroll`,
  `stand_cells_served`, `stand_snapshot_parts`, and `stand_requirement_sweep`'s pinned test name. Each was fixed as a
  companion edit.
- Companion scope-record lines were written at P2's scope read, not at each edit.
