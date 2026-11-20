# tests · standing conventions

- S1 · a new shared test helper module is included by the checks as a module and is never a test target of its own · test-strategy
- S2 · test data a helper builds is stated in code, nothing seeded, and fixture pages are stated once in the shared fixtures module · 7-test-data-fixtures · test-data-fixtures
  decision/blitz-dom-fixtures-built-manually · blitz-dom fixtures built without the HTML parser — blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML. · why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
  decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.
- S3 · a helper's failure output prints no argument value and holds no id, name or typed text · unit-test-strategy
  decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
