# Implement outcome — 2026-11-08-what-can-i-do-here

**Outcome: green** (no smoke: no boot-path or UI-surface change; the plan lists no smoke entry). Next → the OPERATOR
PASS: entries 17 (hygiene), 18 (the push) and 19 (the CI read), then `/andromeda-wrap-session`.

## Gates
The final full firing was 16 green, 0 red, and 3 not run (the operator legs 17, 18 and 19). After a later
test-only edit to `stand_actions.rs`, entries 4, 10, 13 and 14 were fired again and read green
(trail `.andromeda/runs/2026-11-08T14-09-46-implement/gate-2026-11-08-what-can-i-do-here.json`). There were three
iterations, and no deferral: Rust, Python, shell and docs all carry delta. There are no `gate.py guard` entries.
- Entry 13 reads 253 passed · 0 failed · 7 ignored over the stand files, 53 of them now. The earlier reading was
  244 · 0 · 7 over 52. The difference is `stand_actions`'s 8 tests and the new `stand_mcp_tools` `actions` test.
- Matrix: `v030-06` is planned → implemented, with ref
  `tests/blitz-tests/tests/stand_actions.rs::every_listed_cause_on_the_lean_tasks_is_what_the_call_returns`.

## Files changed
31 files changed in total: 29 from the Files-to-modify list, 2 new, and 1 recorded companion. `fixtures/mod.rs` and
`test_capability_statement.py` needed no edit.
- **New:** `packages/escher-driver/src/actions.rs`, `tests/blitz-tests/tests/stand_actions.rs`.
- **dioxus-native-dom:** `actionable.rs`, `lib.rs`.
- **escher-driver:** `execute.rs`, `schema.rs`, `schema/spec.rs`, `schema_tests.rs`, `command.rs`, `json.rs`,
  `describe.rs`, `lib.rs`, `mcp.rs`, `mcp_tests.rs`, `cli_tests.rs`, `recording_tests.rs`.
- **blitz-tests:** `session_common/mod.rs`, `common/mod.rs`, `stand_act_spans.rs`, `stand_act_scroll.rs`,
  `stand_mcp_tools.rs`, plus the companion `stand_requirement_sweep.rs`.
- **seven_guis tests:** `cli_commands.rs`, `mcp_server.rs`, `host_log.rs`, `mcp_log.rs`, `common/mod.rs`.
- **Scripts and docs:** `scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`, `docs/capabilities.md`,
  `README.md`.

## Deviations, with justifications
1. **The scope record has one line:**
   ``- `tests/blitz-tests/tests/stand_requirement_sweep.rs` · companion · serves `packages/escher-driver/src/schema_tests.rs` · self``.
   Its `WITNESSES_020` row v020-01 cites the schema test I renamed. The rename was from
   `the_thirteen_verbs_are_named_in_order_each_at_its_level` to `the_fourteen_…`, so the citation moved with it.
2. **Two test names now state fourteen:** the schema test above and `mcp_tests`'
   `the_listing_is_the_fourteen_verbs_of_the_table`. Only the sweep cited either name.
3. **`stand_mcp_tools`:** the `graph` tool-call test's body became a shared helper. The test keeps its name, and
   `an_actions_tool_call_…` runs the same assertions.
4. **`schema_tests`:** its field-kind table now keys `nodes` per verb (`graph.nodes`, `actions.nodes`), because two
   verbs' `nodes` fields differ in kind. The `actions` help is pinned word for word among the `amended` rows, which
   now number 12.
5. **The shared check (step 2):** it is `check_target`, which returns `Cleared::{Point, Rect, Free}`.
   - `screenshot` with an id reads `can_paint_text` after the crop, before painting, in the order the arm met it.
   - The whole-screen path and the raster's error mapping are unchanged.
   - `Target` and the new `target_of` are crate-visible, so the read builds one snapshot and one id map per call.
6. **An in-tree witness instead of a control:** the removed-listener unit test also asserts that `data-dioxus-id`
   outlives the listener. That shows the old mark leg would still count it (testing.md 2026-11-07). The plan named no
   control, and a clone would rebuild the workspace.
7. **Coverage added beyond the plan's list:** `stand_actions` asserts the three `disabled` controls on the lean tasks
   at boot (the acceptance names them). The fixture page for `MASKED_VALUE` is `fixtures::password_labelled`, with
   the sentinel typed into it.
8. **Stale comments corrected in listed files:** the cold-agent fake said "twelve tools" (now fourteen), and
   `host_log` said "Seven calls" (now nine).
9. **Measured values:**
   - `ACTIONABLE_AT_BOOT` reads 2 · 7 · 3 · 11 · 678 · 2 for counter, flight-booker, timer, crud, cells and load.
     The forecast was "about 677" for Cells; 678 is 676 cells, the grid and the back button.
   - The Cells `actions` answer through the binary (`start cells`, `actions`, `stop`) is 106,088 bytes for 678 nodes.
     461 cells read `off-screen` for click, type and screenshot, and every node takes `scroll`.
   - `stand_actions` asserts the answer is more than 100,000 bytes and at most 1,048,576, in both modes.

## Surfaced / stuck
Nothing was surfaced or stuck. No spec↔reality gap was found.

## Process hygiene
| process | started by | final state |
|---|---|---|
| escher-session host (`start cells`, P1 measurement) | this run | terminated (`stop` exit 0) |
| cargo / rustc and test binaries of gate entries | a leg (gate tool) | terminated |

Census (`gate.py census --comm escher-session --comm cargo --comm rustc --args agent-run.sh --args stand_actions`):
`census: none running`.

## Decisions & corrections
- Guard refusals: the Bash cwd guard refused two `cd` calls (into `andromeda-implement/references` and
  `tests/blitz-tests/tests`). Each was re-issued with absolute paths.
- A standing learning recurred: testing.md 2026-10-10 (extended 2026-10-25) says to search `tests/` for what a check
  pins. Its extension names `stand_requirement_sweep`'s source table. I did not run that search before renaming a
  test, and the sweep read red at the first full firing. It is fixed by the companion line above.
