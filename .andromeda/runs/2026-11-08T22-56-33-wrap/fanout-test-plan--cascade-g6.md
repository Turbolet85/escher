@proposal n=1 detector=D-cascade severity=warning file=ci-integration anchor=fct-mrjxzany basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-x42u35ei
section: ## 9. CI Integration
change: In the stand-repeat clause read «54 files» where it reads «53 files», and after «`stand_graph` and `stand_actions`» add «and, since 2026-11-08-handler-coverage, `stand_handler_coverage` (8 tests), joining by its prefix with no change to ci-leg.sh, the workflows or the leg lists»; the rest of the row unchanged.
sidecar: ci-integration Legs: stand-repeat glob 53 -> 54 stand_*.rs files, stand_handler_coverage joining by its prefix (2026-11-08-handler-coverage).
rationale: The report's Harness / gate surface states the new stand_* target stand_handler_coverage (8 tests) is picked up by run stand and the stand-repeat leg by its prefix with no script or workflow change, and Counts moved reads stand_*.rs files 53 -> 54.
@proposal n=2 detector=D-cascade severity=warning file=ci-integration anchor=fct-prdavblg basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-n5joryov
section: ## 9. CI Integration
change: After the host_log fourth-test sentence add «and, since 2026-11-08-handler-coverage, `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` runs `coverage crud` over a kept CRUD check at `RUST_LOG=trace` and reads 0 id, name, coverage, sentinel, path and recorded needles, green under the `-p seven_guis` build and the workspace build».
sidecar: ci-integration Engine features by runner: host_log's coverage run read green under both builds (2026-11-08-handler-coverage).
rationale: The report's Outcome (obs) states host_log's new coverage test is green under the package build (entry 7) and the workspace build (entry 14), a further reading of the two runners this row records.
@proposal n=3 detector=D-cascade severity=warning file=quality-gates-coverage-targets anchor=fct-3ctmcebn basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-wbvne53q
section: ## 10. Quality Gates & Coverage Targets
change: Read «run the 54 `stand_*.rs` files» where it reads «53», and open the parenthesis with «with `stand_handler_coverage`, entering through the existing `stand_*.rs` glob, CI#42530780903 green 17/17, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md; 53 before it;» ahead of the `stand_actions` reading; the rest unchanged.
sidecar: quality-gates Flakiness budget: stand-repeat runs 54 stand_*.rs files with stand_handler_coverage, CI#42530780903 green 17/17 (2026-11-08-handler-coverage).
rationale: The report's Counts moved reads stand_*.rs 53 -> 54, its Harness / gate surface puts stand_handler_coverage in the stand-repeat leg by its prefix, and its Cross-project claims read CI#42530780903 green 17/17 at the pre-CI commit.
@remark
The stand_handler_coverage, host_log and cli_commands facts F1-F9 belong to integration/e2e/unit/fixtures categories (not this group); proposal 2 overlaps F6 (integration-test-strategy) on the host_log test, so keep the two consistent; the per-OS tallies and local baseline take no figure as the report records none.
