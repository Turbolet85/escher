@proposal n=1 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: In the module list add, since 2026-11-08-handler-coverage, reached (ungated, compiled with and without `accessibility`: the reached record, with `DioxusDocument::record_reached`, which provides a crate-private `ReachedRecord` at `ScopeId::ROOT` (a second call keeps the one record), and `DioxusDocument::reached_handlers`, the handlers in first-reached order deduplicated by (id, event) and empty when never switched on; written in `DioxusEventHandler::handle_event` before `runtime.handle_event`; its `ReachedHandler` type re-exported).
sidecar: existing-scopes dioxus-native-dom: the ungated reached module and its re-exported ReachedHandler joined the module list (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs and Crates / modules name the new ungated module `dioxus_native_dom::reached` (lib.rs:28) and `ReachedHandler` re-exported (lib.rs:45); its Counts name this fact as the drv-reexport site where the module is absent.
@proposal n=2 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: element_id's entry also names the ungated crate-private `element_listener_ids`, one vdom walk giving each element's stable id and its current listener events, the reached record's one reader.
sidecar: existing-scopes dioxus-native-dom: element_id gains the ungated crate-private element_listener_ids (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs list `element_listener_ids` at element_id.rs:304-320, crate-private and ungated, the record's one caller; deviation 1 says why it is used in place of the gated `element_facts`.
@proposal n=3 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Existing Scopes
change: dioxus_document's `cfg(test)` child module `bridge_tests` holds nine unit tests of the bridge (eight until 2026-11-08-handler-coverage), the ninth the reached record holding on a page with no registered state.
sidecar: existing-scopes dioxus-native-dom: bridge_tests moved from eight to nine unit tests (2026-11-08-handler-coverage).
rationale: The report's Counts give `dioxus_document_tests.rs` (`bridge_tests`) as moving from 8 to 9 tests; New text, by line lists `the_reached_record_holds_on_a_page_with_no_registered_state` at dioxus_document_tests.rs:324-354.
@proposal n=4 detector=D-cascade severity=warning file=existing-scopes anchor=fct-mzrxx2a3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: The driver now has nineteen private modules (eighteen until 2026-11-08-handler-coverage). Add, since that chunk, coverage (`handler_coverage`, `Coverage` and `HandlerRef`: a fresh session booted per check, a check of another app contributing nothing, the reached record switched on, and the UI graph read at boot and after each replayed step with `mounted` left out, deduplicated by (id, event); json the coverage line's hand-written form `Coverage::to_json`), re-exported at the crate root.
sidecar: existing-scopes escher-driver: the coverage module joined, making nineteen private modules, with json's Coverage::to_json (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs and Crates / modules name the new module `escher_driver::coverage` with `handler_coverage` (coverage.rs:64-102), `Coverage` and `HandlerRef`, re-exported (lib.rs:145), and `Coverage::to_json` (json.rs:419-442).
@proposal n=5 detector=D-cascade severity=warning file=existing-scopes anchor=fct-mzrxx2a3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: cli's entry adds, since 2026-11-08-handler-coverage, `command_line`'s `coverage` role, `coverage <app> <check>...`, which reads each named kept check through a bounded read and prints one coverage line.
sidecar: existing-scopes escher-driver: cli gains the coverage role (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs: `command_line` gains the `coverage` role (cli.rs:45-47 `COVERAGE_ROLE`, cli.rs:183-199, cli.rs:462-473, cli.rs:495-505 `read_check`), its one shape `coverage <app> <check>...`.
@proposal n=6 detector=D-cascade severity=warning file=existing-scopes anchor=fct-mzrxx2a3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: replay's entry adds the crate-private `Check::replay_each`, which runs a hook after each step. `Check::replay` now calls it with a no-op hook, and `handler_coverage` is its other caller.
sidecar: existing-scopes escher-driver: replay gains the crate-private Check::replay_each (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs: `Check::replay_each(&self, session, after_step)` (replay.rs:47-55, 67), crate-private; `Check::replay` calls it with a no-op hook and its behaviour is unchanged.
@proposal n=7 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-npklafdi
section: ## Existing Scopes
change: `stand_replay` holds 4 tests in both layout modes (3 until 2026-11-08-handler-coverage). The fourth: a recorded Counter session with two `actions` steps holds on a fresh Counter, all 4 steps run, and the copy with one `keyed` flipped stops at step 3, accepted then and now, its keys exactly `["nodes"]`.
sidecar: existing-scopes blitz-tests: stand_replay moved from 3 to 4 tests with the kept actions step replayed (2026-11-08-handler-coverage).
rationale: The report's Outcome (matrix#v030-06) names `stand_replay::a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes`, and its Counts give stand_replay tests as moving from 3 to 4.
@proposal n=8 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Existing Scopes
change: The shared `tests/common/mod.rs` also holds, since 2026-11-08-handler-coverage, `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` (the handlers per served task at boot, and those left after each task's `GRAPH_DRIVES`) and `grid_cells()`, which generates Cells' 676 grid cells. It is read through `mod common;` by 24 stand checks, `stand_handler_coverage` the twenty-fourth.
sidecar: existing-scopes blitz-tests: common/mod.rs gains the handler tables and grid_cells(), read by 24 stand checks (2026-11-08-handler-coverage).
rationale: The report's Counts give the blitz-tests `mod common;` declarers as moving from 23 to 24; New text, by line lists HANDLERS_AT_BOOT (common/mod.rs:336-380), UNEXERCISED_AFTER_DRIVES (382-401) and grid_cells (406-412); deviation 2 covers the generated Cells rows.
@proposal n=9 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-x42u35ei
section: ## Existing Scopes
change: Add, since 2026-11-08-handler-coverage, `stand_handler_coverage`, 8 tests in both layout modes. It holds handler coverage on the six served tasks against `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES`, two checks listing only what neither exercised, the load page's click read exercised, a refused step exercising nothing, a keyboard activation exercising its click, a handler whose step removes its element read exercised, a typed sentinel never on the line, and a check of another app contributing nothing. It is in `stand-repeat` by its `stand_` prefix and is a reader of `tests/session_common/mod.rs`.
sidecar: existing-scopes blitz-tests: the stand_handler_coverage target joined (2026-11-08-handler-coverage).
rationale: The report's Files name the new tests/blitz-tests/tests/stand_handler_coverage.rs; Harness / gate surface names the new stand_* target with 8 tests, picked up by stand-repeat by its prefix; the Outcome and New text, by line list its tests; the Counts give session_common declarers as moving from 34 to 35.
@proposal n=10 detector=D-cascade severity=warning file=existing-scopes anchor=fct-qkp2micb basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Existing Scopes
change: escher-session's roles add, since 2026-11-08-handler-coverage, the `coverage` role, `coverage <app> <check>...`: the handlers its kept checks left unexercised, with the same boot closure called once per check.
sidecar: existing-scopes seven_guis: escher-session gains its coverage role (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs: the `escher-session` binary's module doc names the role (session_host.rs:5-7); the boot closure and `command_line` call are unchanged, and the role calls the same closure once per check.
@proposal n=11 detector=D-cascade severity=warning file=existing-scopes anchor=fct-qkp2micb basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Existing Scopes
change: The shared `tests/common/mod.rs` holds `cells_needles(part)`, `graph_needles()`, `actions_needles()` and, since 2026-11-08-handler-coverage, `coverage_needles()` among its readers. `host_log` alone calls `coverage_needles()`, over a CRUD screen.
sidecar: existing-scopes seven_guis: tests/common/mod.rs gains coverage_needles() (2026-11-08-handler-coverage).
rationale: The report's Counts say the seven_guis common needle readers gain `coverage_needles()`, called by host_log alone, and name this fact as a needles site; New text, by line lists it at examples/seven_guis/tests/common/mod.rs:454-479; deviation 3 says it reads CRUD.
@proposal n=12 detector=D-cascade severity=warning file=existing-scopes anchor=fct-o6vmxcuj basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Existing Scopes
change: `tests/common/mod.rs` is read through `mod common;` by 24 checks and `tests/session_common/mod.rs` by 35, the 24 and the 35 as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md.
sidecar: existing-scopes: the blitz-tests mod common; readers moved from 23 to 24 and the session_common readers from 34 to 35 (2026-11-08-handler-coverage).
rationale: The report's Counts measure the blitz-tests `^mod common;` declarers moving from 23 to 24 (naming this fact's «by 23» as the site) and the `^mod session_common;` declarers moving from 34 to 35 by git grep over tests/blitz-tests/tests/*.rs.
@remark
Task facts F1-F4 and F6 (standard-contracts), F5 (established-decisions) and F7 (conventions) belong to those categories and none is an existing-scopes fact, so none is placed here. Body fct-gp7aqjse needs no edit because no crate was added.
