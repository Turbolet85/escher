@disposition n=1
row: held fct-73ojmpsr apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = ["hyb…»
disposition: holds — stands (line 14 is the default feature list, holding `cookies` and `cache`)
@disposition n=2
row: held fct-4kkbivro packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…»
disposition: holds — stands (535-539 is the `let Some(display)` else-branch logging the info line and returning)
@disposition n=3
row: stretched fct-ghd6zkhy packages/escher-driver/src/mcp.rs:44-112 → 44-112 — changed inside
disposition: holds (serve_mcp still reads stdin and writes stdout, binding nothing; only the doc's instance-verb count changed)
@disposition n=4
row: changed fct-pbvi5fsv scripts/cold-agent.sh:53 → ? — rewritten «VERBS = ("snapshot", "click", "type", "press", "advance…»
disposition: claim false — raised at Validate (the pipe's `VERBS` at scripts/cold-agent.sh:53 now holds fourteen verbs, `actions` added after `graph`, not thirteen)
@disposition n=5
row: stretched fct-awfxp663 packages/escher-driver/src/execute.rs:119-322 → 126-334 — changed inside
disposition: claim false — raised at Validate (the verb table is no longer thirteen verbs: the executor now runs a fourteenth, `Command::Actions` answering `Outcome::Actions`, which the row does not name)
@disposition n=6
row: stretched fct-awfxp663 packages/escher-driver/src/execute.rs:326-366 → 338-379 — changed inside
disposition: holds (record_result still records only verb and cause for non-acting outcomes, `Outcome::Actions` among them)
@disposition n=7
row: stretched fct-awfxp663 packages/escher-driver/src/execute.rs:436-526 → 459-599 — changed inside
disposition: holds (lookup, stale/not-found and the disabled, off-screen, covered order stand, now routed through `check_target`)
@disposition n=8
row: stretched fct-f4eyvu3v packages/escher-driver/src/mcp.rs:44-112 → 44-112 — changed inside
disposition: claim false — raised at Validate (`tools/list` no longer lists thirteen verbs: `VERBS` now holds fourteen with `actions`, and the serve_mcp doc reads ten instance verbs)
@disposition n=9
row: held fct-d2yejoup tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (lines 1-5 are the module doc of the stand snapshot test)
@disposition n=10
row: stretched fct-d2yejoup packages/dioxus-native-dom/src/actionable.rs:15-44 → 18-47 — changed inside
disposition: holds (`UnkeyedActionable` still holds node, id, tag, role and the three readings, no text, name or value)
@disposition n=11
row: stretched fct-d2yejoup packages/dioxus-native-dom/src/actionable.rs:55-89 → 93-157 — changed inside
disposition: claim false — raised at Validate (the range now also holds `actionable_nodes`, a further reader of the id that crosses as the `actions` verb's answer, while the row says the actionable rule has no driver, CLI or MCP command yet and `VERBS` holds thirteen)
@disposition n=12
row: stretched fct-d2yejoup packages/escher-driver/src/execute.rs:408-453 → 421-476 — changed inside
disposition: holds (locate still does a linear search of `element_ids()` and get_node; resolve answers stale or not-found; only `target_of` was split out)
@disposition n=13
row: stretched fct-r5yt6pat examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds (mcp_log still asserts 0 sentinel, id and name needles on the mcp role's stderr, now with an `actions` call added)
@disposition n=14
row: stretched fct-d4potb3l packages/escher-driver/src/execute.rs:242-246 → 251-255 — changed inside
disposition: holds (a space-led text on a select still types nothing; only `&text` became `text`)
@disposition n=15
row: stretched fct-4hqe6pzr examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds (the needle checks stand; the line counts are dated to their measuring chunks)
@disposition n=16
row: stretched fct-tqdgjkao scripts/cold-agent.sh:42-536 → 42-536 — changed inside
disposition: holds (only the `VERBS` tuple changed; the pipe's events, verdict and scrub handling are unchanged)
@disposition n=17
row: stretched fct-gphdtjgv packages/escher-driver/src/mcp.rs:44-112 → 44-112 — changed inside
disposition: holds (serve_mcp still reads JSON-RPC from stdin and writes replies to stdout, with no listener, and holds its session in process)
