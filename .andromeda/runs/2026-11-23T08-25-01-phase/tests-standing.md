# tests · standing conventions

- S1 · each new integration test stands as its own blitz-tests target, one target per file · test-strategy
  decision/oversize-test-targets-kept · Oversize integration-test targets kept whole — Escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them. · why: Ratified; a split into several targets moves the pinned target layout the CI legs read.
- S2 · a regression test is named for the bug it fixes and documents it, here upstream issue 1578 · 2-test-strategy
  decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
  decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
- S3 · each new test is controlled red before green, with its red reading kept · unit-test-strategy
- S4 · each new stand check runs in both layout modes · test-scope-summary
- S5 · each new stand check runs under the bundled real font and never skips for want of one · 2-test-strategy · test-strategy
- S6 · each new pixel assertion reads a CPU buffer and compares frames whole or by pinned literal pixels · 2-test-strategy · test-strategy
- S7 · each new test pins its expected values as literals and its failures print no argument value · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
- S8 · each new test states its data in code, shared fixtures and stand tables kept in one module that is no target · 3-test-harness-contract
  decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file
  decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
  decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: A helper whose body or failure message differs from the shared one stays in its check; stand_element_ids keeps its own boot; task_keys, ceiling, component, the cards array and SECRET stay per file.
  decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
  decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.
- S9 · each new stand check joins the three-round stand repeat under a zero-failure budget · 9-ci-integration · ci-integration
  decision/joining-file-keeps-its-legs · A file joining a leg keeps its legs — A test file joining a CI leg keeps every leg it already ran in.
  decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds
  decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
