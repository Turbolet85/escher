# arch · Project Intent — category read

## Carried
- (.andromeda/architecture.md:288) The sweep must prove every 0.2.0 capability it reaches on the internal 7GUIs stand (a minimal fixture beside it where the tasks lack the case), and must prove driver-carried capabilities through the framework's own driver, which is meant to be the harness.
  > escher is a fork of Blitz being turned into an agent-first UI framework — a headless driver (CLI + MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development Style agent-driven is the founder's word: the framework's own driver is meant to become the harness.
- (.andromeda/architecture.md:296) The stand whose tasks hold each requirement's case must stay the seven_guis demonstrator as its own description states it; a requirement the four lean tasks cannot hold takes a fixture beside it, not a new demonstrator app.
  > seven_guis "Seven benchmark tasks for GUI frameworks" (examples/seven_guis/src/app.rs:127-128)
- (.andromeda/architecture-detail/project-intent.md:33) Every check the sweep's record names, and every check it adds to fill a gap, must be a blitz-tests integration test (or a seven_guis process check), the project's stated headless validation source — not a WPT runner result.
  > blitz-tests is "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3), some of them Rust ports of script-driven WPT tests the runner cannot run (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)

## Bindings
- **stand proof ↔ the body's Headless stand contract**: the intent's "proven on an internal 7GUIs stand" (.andromeda/architecture.md:288) is held by `seven_guis::stand` booting the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font (.andromeda/architecture.md:163) — a sweep witness on the stand boots through it.
- **driver as harness ↔ the body's Driver session contract**: a capability the driver carries is proven through `Session::run` and its carriers (.andromeda/architecture.md:288; .andromeda/architecture.md:167), the stderr measurement of the new verbs on the `escher-session` host and its `mcp` role among them.
- **blitz-tests as validation source ↔ the body's Tests convention**: a check the sweep adds covers one behaviour per file, opens with a `//!` doc, runs in both layout modes and keeps one test target per file (.andromeda/architecture-detail/project-intent.md:33; .andromeda/architecture.md:121), the target list pinned by the tests plan's `test_blitz_tests_targets.py`.
