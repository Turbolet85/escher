- line: .andromeda/obs-plan.md:70
old: «its `host` reads a clock for the idle expiry, and its MCP server one when a request arrives with a session held)»
new: «its `host` reads a clock for the idle expiry, its MCP server one when a request arrives with a session held, and its `session` one for the load wait and nothing else, `LOAD_WAIT` bounding it)»

- line: .andromeda/obs-plan.md:70
old: «and for the module twenty-six checks read — `stand_lint`,»
new: «and for the module twenty-seven checks read — `stand_load_wait`, `stand_lint`,»

- line: .andromeda/obs-plan.md:79
old: «its 20 readers (as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md)»
new: «its 21 readers (as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md)»

- line: .andromeda/obs-plan.md:119
old: «`passes` (settle's pass count, when settle answered quiet) and `added`»
new: «`passes` (settle's pass count, when settle answered quiet — on a step that waited on a load, the settle that went quiet) and `added`»

- line: .andromeda/obs-plan.md:119
old: «Still observed absent — a span, an event or a `tracing` dependency in the settle wait itself, `Harness::settle` in `packages/blitz-test-harness`,»
new: «Still observed absent — a span, an event or a `tracing` dependency in the settle wait itself, `Harness::settle` in `packages/blitz-test-harness`, and a span, an event or a field of its own in the driver's load wait over it (`settle_waiting`, bounded by `LOAD_WAIT`), which runs inside the call's `command` span and leaves its eight fields as they are,»
