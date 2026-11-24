# arch · standing conventions

- S1 · each integration test file holds one behaviour and is its own blitz-tests target, joined to the target pin · conventions
- S2 · integration tests live in blitz-tests · design-philosophy
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · except: blitz-vibey-script keeps its own integration tests in its tests directory
  decision/validation-sources · Validation sources — Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.
