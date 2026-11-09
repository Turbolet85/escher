@proposal n=1 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 1. Test Scope Summary
change: in the seven_guis row: `tests/cli_commands.rs` reads 18 tests (17 until 2026-11-08-handler-coverage), adding a `coverage` role run over a Counter check the binary kept that answers its coverage line, exit 0 (`coverage_replays_a_kept_check_and_lists_the_handlers_it_left`); `tests/host_log.rs` reads 6 tests (5 until that chunk), adding a `coverage crud` run over a CRUD session kept through the binary with a typed sentinel and a row click that reads 0 id, name, coverage, sentinel, path and recorded needles at `trace` under both builds; and the common module's needle readers gain `coverage_needles()`, the CRUD screen's needles read from its UI graph, called by `host_log` alone
sidecar: test-scope-summary seven_guis row: cli_commands 17 to 18 and host_log 5 to 6 with the coverage role's runs, common gains coverage_needles()
rationale: the report's Counts list cli_commands on linux 17 to 18, host_log 5 to 6 and coverage_needles() called by host_log alone (fct-xgydedcz), and its Outcome names both new tests (fct-nu73ltec, fct-n5joryov)
@proposal n=2 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 1. Test Scope Summary
change: escher-driver reads 104 unit tests in seventeen files, 67 inline and 37 in the `#[path]` files (102, 66 and 36 until 2026-11-08-handler-coverage); json.rs reads 8, adding a coverage line keyed per handler with an absent owner left out; cli_tests.rs reads 6, its line table 109 rows (102 until then) with the `coverage` role's among them, adding the coverage role reading its checks and refusing `malformed` a missing file, a non-check, one past the bound and another app's check, a failing boot and an accepted two-check run; and `handler_coverage` joins the readings held by an integration check, `stand_handler_coverage`, not by a unit test
sidecar: test-scope-summary escher-driver row: 102 to 104 unit tests, json.rs 7 to 8, cli_tests.rs 5 to 6, line table 102 to 109 rows, handler_coverage read by stand_handler_coverage
rationale: the report's Counts list cli_tests.rs 5 to 6, json.rs 7 to 8 and the line table 102 to 109, naming this fact as the site (fct-xgydedcz); deviation 5 names the two new unit tests (fct-sstfl5zh)
@proposal n=3 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 1. Test Scope Summary
change: dioxus-native-dom reads 109 unit tests in twelve files (104 in eleven until 2026-11-08-handler-coverage), adding four in `reached::tests`, under no feature gate, pinning the reached record: a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, and the record's `Debug` printing its count alone; and nine on the bridge (eight until then), adding `the_reached_record_holds_on_a_page_with_no_registered_state`
sidecar: test-scope-summary dioxus-native-dom row: 104 in eleven to 109 in twelve files, reached::tests 4 new, bridge_tests 8 to 9
rationale: the report's Counts list bridge_tests 8 to 9 and reached::tests 4 new with no master count found (fct-xgydedcz), and Coverage of new surfaces names reached::tests 4 and bridge_tests +1 (fct-uzhqu5vc)
@proposal n=4 detector=D-cascade severity=warning file=body anchor=fct-jydhmppz basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-uzhqu5vc
section: ## 1. Test Scope Summary
change: the dioxus-native-dom broad line names the reached record beside the bridge and its dispatch record: unit tests of the snapshot builder, ..., the bridge, its dispatch record and the reached record, the UI graph reader, ...
sidecar: test-plan body section 1 dioxus-native-dom line names the reached record among the unit-tested subjects
rationale: the reached record is a new dioxus-native-dom surface with its own unit tests, reached::tests 4 and bridge_tests +1 (fct-uzhqu5vc, fct-zcn6rmkk)
@proposal n=5 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-rjsmqegm basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-u4lswkp4
section: ## 1. Test Scope Summary
change: the parenthesis listing the dioxus-native-dom files that match the test search adds reached.rs since 2026-11-08-handler-coverage, after graph.rs and signal_registry.rs; stylo_taffy and dioxus-native still hold none
sidecar: test-scope-summary dioxus-native and stylo_taffy row: reached.rs joins the dioxus-native-dom files holding tests
rationale: the report's New text lists reached.rs as a new file holding `mod tests` at 108-210 (fct-u4lswkp4), new module dioxus_native_dom::reached (fct-z4cdnfit)
@remark
No task fact is of category test-scope-summary: F1-F4 and F6 are integration-test-strategy's, F5 e2e-test-strategy's, F7-F8 unit-test-strategy's, F9 test-data-fixtures', for the new-detail pass; F3's reached-record limits (canvas, no screen reached, bubbling) would sit beside the §1 dioxus-native-dom limit rows test-scope-summary#12.1 and #13.1 if placed there.
