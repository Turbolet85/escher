@proposal n=1 detector=D-cascade severity=warning file=test-strategy anchor=fct-5d4ttzku basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## 2. Test Strategy
change: append: since 2026-11-08-handler-coverage `Check::replay` runs through the crate-private `Check::replay_each` with a no-op hook, its behaviour unchanged, and `handler_coverage` is its second caller — it replays kept checks, each on a fresh session, and reads the UI graph at boot and after each replayed step, listing the handlers no check exercised (`escher-session coverage <app> <check>...`), so a recorded check is also a measure of what the checks leave unexercised (packages/escher-driver/src/replay.rs:47-55; packages/escher-driver/src/coverage.rs:64-102)
sidecar: test-strategy recorded-check replay: kept checks now also read by handler_coverage through replay_each (2026-11-08-handler-coverage)
rationale: report Changes Symbols / APIs states replay_each (replay.rs:47-55 · 67) with Check::replay calling it unchanged and handler_coverage the other caller (coverage.rs:64-102)
@proposal n=2 detector=D-cascade severity=warning file=test-strategy anchor=fct-krjmr356 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-nu73ltec
section: ## 2. Test Strategy
change: append: since 2026-11-08-handler-coverage the statement states handler coverage and its three limits — handlers inside a canvas not counted, a handler on no screen a replay reached not listed, a bubbling dispatch counting every ancestor listener it passed — (docs/capabilities.md:273-281), the claim pinned by `stand_handler_coverage` and `cli_commands`, its schema-held names and counts unmoved
sidecar: test-strategy claims mapped to checks: the handler-coverage claim and its three limits join docs/capabilities.md (2026-11-08-handler-coverage)
rationale: report New text lists docs/capabilities.md 273-281 as the three limit bullets and Outcome v030-07 states the canvas limit there; test_capability_statement.py is byte-identical (Unchanged)
@proposal n=3 detector=D-cascade severity=warning file=test-strategy anchor=fct-7m7lxsmy basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 2. Test Strategy
change: `tests/common/mod.rs` read through `mod common;` by 24 `stand_*.rs` checks (23 before `stand_handler_coverage.rs`), `tests/session_common/mod.rs` read through `mod session_common;` by 35 checks, `stand_handler_coverage.rs` the latest (34 before it), and 161 `.rs` files, 160 test targets and the unbuilt `all.rs`, `stand_handler_coverage.rs` the latest, its own auto-discovered target with no manifest line (160 and 159 before it) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: test-strategy directory pattern: mod common declarers 23 to 24, session_common readers 34 to 35, blitz-tests files and targets 160/159 to 161/160 with stand_handler_coverage.rs (2026-11-08-handler-coverage)
rationale: report Counts: mod common declarers 23 to 24 naming fct-7m7lxsmy, mod session_common declarers 34 to 35, stand_*.rs 53 to 54; Files lists tests/blitz-tests/tests/stand_handler_coverage.rs as the one new blitz-tests file and no manifest moved
@proposal n=4 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-klxgwgcy basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 4. Unit Test Strategy
change: cli_tests.rs pins a line read by its rules in order (109 rows) and the schema is driven through the `escher-session` binary by seven_guis' `cli_commands` (18), the rest unchanged
sidecar: unit-test-strategy escher-driver: cli_commands 17 to 18 and the line table to 109 rows (2026-11-08-handler-coverage)
rationale: report Counts: seven_guis cli_commands on linux 17 to 18 naming fct-klxgwgcy, and a_line_is_read_by_its_rules_in_order 102 to 109 rows; the entity's 61 rows is that table
@proposal n=5 detector=D-new-detail severity=warning file=unit-test-strategy anchor=fct-klxgwgcy fact=F8 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-sstfl5zh
section: ## 4. Unit Test Strategy
change: - **escher-driver (the coverage line and role):** json.rs's `a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner` pins the coverage line's form, `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` with `owner` left out when absent, and cli_tests.rs's `the_coverage_role_reads_its_checks_refusing_any_it_cannot_read` pins a missing file, a non-check, one past the bound and another app's check each refused `malformed`, a failing boot, and an accepted two-check run, its scratch files under the workspace `target/tmp` (packages/escher-driver/src/json.rs:1245-1274; packages/escher-driver/src/cli_tests.rs:614-674) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: unit-test-strategy: new escher-driver detail for the coverage line and the coverage role's unit tests (2026-11-08-handler-coverage)
rationale: report Deviations 5 names both unit tests and their rows; Symbols / APIs states Coverage::to_json's form; New text lists json.rs 1245-1274 and cli_tests.rs 614-674
@proposal n=6 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-fwtnmbsl basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 4. Unit Test Strategy
change: escher-driver's unit tests read 104 — cli_tests.rs 6 and json.rs 8 with 2026-11-08-handler-coverage's two coverage tests (102 before it)
sidecar: unit-test-strategy escher-driver actions read: unit-test count 102 to 104 (2026-11-08-handler-coverage)
rationale: report Counts: escher-driver cli_tests.rs 5 to 6 and json.rs 7 to 8, no other escher-driver test added (coverage.rs and replay.rs gain none in New text)
@proposal n=7 detector=D-new-detail severity=warning file=unit-test-strategy anchor=fct-mi2j5ztx fact=F7 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-uzhqu5vc
section: ## 4. Unit Test Strategy
change: - **dioxus-native-dom (the reached record):** `reached::tests` (4) pin the reached record — a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, and the record's `Debug` printing its count alone — and `bridge_tests::the_reached_record_holds_on_a_page_with_no_registered_state` pins it where no state is registered (packages/dioxus-native-dom/src/reached.rs:108-210; packages/dioxus-native-dom/src/dioxus_document_tests.rs:324-354) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: unit-test-strategy: new dioxus-native-dom detail for the reached record's unit tests (2026-11-08-handler-coverage)
rationale: report Coverage of new surfaces: the reached record tests unit (reached::tests 4, bridge_tests +1); Counts: bridge_tests 8 to 9; New text lists reached.rs 108-210 and dioxus_document_tests.rs 324-354
@remark
F1-F4 and F6 (integration-test-strategy), F5 (e2e-test-strategy, cli_commands' coverage test beside test-strategy#2.2) and F9 (test-data-fixtures, the new common/mod.rs tables beside test-strategy#10.1) are other categories' facts; detectors D-tests-coverage, D-tests-framework and D-tests-obs-harness find no drift (unit + integ tests on each new surface, standard cargo test, harness unchanged)
