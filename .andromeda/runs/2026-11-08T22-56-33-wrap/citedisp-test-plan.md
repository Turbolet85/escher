@disposition n=1
row: stretched fct-xxpbx2ay examples/seven_guis/tests/host_log.rs:35-641 → 37-730 — changed inside
disposition: holds (host_log still runs the host at trace reading no id, no name and no typed text on stderr; the claim states no test count)
@disposition n=2
row: held fct-dp2mr6q3 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…»
disposition: holds — stands (lines 43-49 are the step that posts the WPT results into the PR description)
@disposition n=3
row: stretched fct-csryo2zo examples/seven_guis/tests/cli_commands.rs:136-716 → 140-774 — changed inside
disposition: claim false — raised at Validate (cli_commands now holds 18 tests on a platform, not 17, since coverage_replays_a_kept_check_and_lists_the_handlers_it_left was added)
@disposition n=4
row: stretched fct-csryo2zo examples/seven_guis/tests/cli_commands.rs:680-690 → 737-748 — changed inside
disposition: holds (the usage-error test still reads the usage line naming --record on start, the host role and the mcp role)
@disposition n=5
row: moved fct-kgita7a3 dioxus_document.rs:318 → 319 — listed, not written: suffix
disposition: holds — re-pointed by hand to 319 (the borrow_mut of inner in poll now stands at 319)
@disposition n=6
row: moved fct-kgita7a3 dioxus_document.rs:317 → 318 — listed, not written: naked
disposition: holds — re-pointed by hand to 318 (vdom.render_immediate now stands at 318)
@disposition n=7
row: stretched fct-adkwyvky examples/seven_guis/tests/host_log.rs:35-641 → 37-730 — changed inside
disposition: claim false — raised at Validate (host_log now holds 6 tests, not 5, a coverage-run test added, and the Cells part read is its fifth test, not its fourth)
@disposition n=8
row: stretched fct-k5y3m6cu@c1600 examples/seven_guis/tests/common/mod.rs:371-597 → 371-624 — changed inside
disposition: holds (the listed helpers stand in the range, the new coverage_needles among them, called by host_log alone)
@disposition n=9
row: stretched fct-k5y3m6cu examples/seven_guis/tests/host_log.rs:35-641 → 37-730 — changed inside
disposition: claim false — raised at Validate (tests/host_log.rs now holds 6 tests, not 5, since a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace was added)
@disposition n=10
row: stretched fct-k5y3m6cu examples/seven_guis/tests/cli_commands.rs:136-716 → 140-774 — changed inside
disposition: claim false — raised at Validate (tests/cli_commands.rs now holds 18 tests on a platform, not 17, since coverage_replays_a_kept_check_and_lists_the_handlers_it_left was added)
@disposition n=11
row: stretched fct-k5y3m6cu examples/seven_guis/tests/common/mod.rs:1-597 → 1-624 — changed inside
disposition: holds (the shared module is still no target and holds the guards, the client run and the log needles it lists)
@disposition n=12
row: stretched fct-nks3g3e7@c11444 packages/escher-driver/src/json.rs:552-1219 → 577-1275 — changed inside
disposition: claim false — raised at Validate (json.rs now holds 8 unit tests, not 7, a coverage-line test added, so the crate total of 102 no longer holds)
@disposition n=13
row: stretched fct-nks3g3e7@c11489 packages/escher-driver/src/cli_tests.rs:1-582 → 1-603 — changed inside
disposition: claim false — raised at Validate (cli_tests.rs now holds 6 tests, not 5, a coverage-role test added, and its line table asserts 109 rows, not 102)
