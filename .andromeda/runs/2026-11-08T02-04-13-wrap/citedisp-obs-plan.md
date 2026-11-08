@disposition n=1
row: held fct-byb3pmvg packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = ["n…»
disposition: holds — stands (line 14 still lists `tracing` among blitz's default features)
@disposition n=2
row: stretched fct-icom2cq7 examples/seven_guis/tests/mcp_log.rs:53-315 → 55-335 — changed inside
disposition: holds (the added `graph` call and graph needles are read into assertions only, 0 found, one `tool_call` line per call)
@disposition n=3
row: stretched fct-77ml32lr examples/seven_guis/tests/common/mod.rs:371-544 → 371-573 — changed inside
disposition: holds (the added `graph_needles` reader prints nothing and its messages name a count only, though the fact's list of readers does not name it)
@disposition n=4
row: stretched fct-ah5jnzn5 scripts/cold-agent.sh:42-536 → 42-536 — changed inside
disposition: holds (the one change adds `graph` to VERBS; the state area, the one server and COUNTED are unchanged)
@disposition n=5
row: stretched fct-7467zgun tests/blitz-tests/checks/counter-cli.json:1-8 → 1-8 — changed inside
disposition: holds (the re-recorded check adds the boolean `clickable` only; no typed text, path, secret or screenshot step)
@disposition n=6
row: stretched fct-7467zgun tests/blitz-tests/checks/counter-mcp.json:1-11 → 1-11 — changed inside
disposition: holds (the re-recorded check adds the boolean `clickable` only; no typed text, path, secret or screenshot step)
@disposition n=7
row: stretched fct-gwvelgmp packages/escher-driver/src/execute.rs:313-352 → 326-366 — changed inside
disposition: claim false — raised at Validate (the fact says the verb table holds twelve words, eight instance-level; with `graph` it now holds thirteen, nine instance-level, per schema::VERBS and its test)
@disposition n=8
row: stretched fct-zuu6oxcq packages/escher-driver/src/execute.rs:313-361 → 326-375 — changed inside
disposition: holds (`record_result` now also returns early on `Outcome::Graphed`, so an accepted `graph` records `verb` alone, which the fact does not list but does not contradict)
