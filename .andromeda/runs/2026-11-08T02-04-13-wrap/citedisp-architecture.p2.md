@disposition n=1
row: changed fct-t7zq5feb@c22614 packages/escher-driver/src/schema.rs:197-202 → ? — the end rewritten «help: "the screen as…»
disposition: claim false — raised at Validate (the quoted help of `snapshot`'s `text` no longer reads so: its state words now end «invalid, focused and clickable», and `changed`'s help now adds «`clickable` among its state», schema.rs:159-163 and 203-208)
@disposition n=2
row: stretched fct-t7zq5feb packages/escher-driver/src/json.rs:115-134 → 116-136 — changed inside
disposition: claim false — raised at Validate (a node now also always writes `clickable`, after `focused`, and `differs` reads `clickable` as a node key: the always-written list «id, role, name, invalid, focused and bounds» is short of it)
@disposition n=3
row: stretched fct-t7zq5feb packages/escher-driver/src/json.rs:317-373 → 400-462 — changed inside
disposition: holds (the outcomes it states read so; the added `Graphed` arm, `{"nodes":[…],"states":[…]}`, is not stated here)
@disposition n=4
row: stretched fct-t7zq5feb packages/escher-driver/src/execute.rs:27-93 → 27-98 — changed inside
disposition: holds (the added `Outcome::Graphed` holds a `UiGraph` of string ids, no `NodeId`)
@disposition n=5
row: stretched fct-t7zq5feb packages/escher-driver/src/execute.rs:114-309 → 119-322 — changed inside
disposition: holds (the added `Command::Graph` arm reads the document in process and returns no `NodeId`)
@disposition n=6
row: stretched fct-t7zq5feb packages/escher-driver/src/execute.rs:313-361 → 326-375 — changed inside
disposition: holds (`Outcome::Graphed` joins the arms that record `verb` alone)
@disposition n=7
row: stretched fct-t7zq5feb packages/escher-driver/src/mcp.rs:44-112 → 44-112 — changed inside
disposition: holds (only the doc comment changed, eight to nine instance verbs)
@disposition n=8
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/element_id.rs:186-282 → 269-408 — changed inside
disposition: holds (`element_ids` still computes on demand, writing to neither tree; a reserved fifth form, `virtual_node_id`, produced by nothing, is unstated)
@disposition n=9
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/element_id.rs:78-161 → 109-223 — changed inside
disposition: holds (the walk now also records owner scopes and listeners; the four tiers and the anchor rule are unchanged)
@disposition n=10
row: changed fct-kllkxz3d packages/dioxus-native-dom/src/element_id.rs:208-214 → ? — the end rewritten «// A keyed element …»
disposition: holds — re-pointed by hand to 334-340
@disposition n=11
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/snapshot.rs:11-77 → 11-81 — changed inside
disposition: holds (`NodeState` gained `clickable`; the one serialized form stands)
@disposition n=12
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/snapshot.rs:96-115 → 100-124 — changed inside
disposition: holds
@disposition n=13
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/snapshot.rs:127-293 → 138-305 — changed inside
disposition: holds
@disposition n=14
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/lib.rs:35-39 → 41-48 — changed inside
disposition: holds (the range now also holds the `graph` and `signal_registry` re-exports beside `snapshot`'s)
@disposition n=15
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/snapshot_diff.rs:32-74 → 32-77 — changed inside
disposition: claim false — raised at Validate (`Reading` now has twelve variants, `Clickable` between `Focused` and `Bounds`, and `Reading::ALL` is `[Reading; 12]`, not eleven)
@disposition n=16
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/snapshot_diff.rs:174-209 → 177-214 — changed inside
disposition: claim false — raised at Validate (`differing` now also compares `clickable`, so the readings it fills are twelve, not the eleven the fact names)
@disposition n=17
row: stretched fct-jelmr2gx .github/scripts/test_cold_agent.py:94-168 → 94-170 — changed inside
disposition: claim false — raised at Validate (the pipe and its fake now carry thirteen verbs, `graph` among them, cold-agent.sh:53 and test_cold_agent.py:23; the fact's «twelve tools» and «twelve `mcp__escher__{verb}` tools» are false)
@disposition n=18
row: changed fct-jelmr2gx README.md:49 → ? — rewritten «An MCP client can drive the same apps: it spawns `target/debug/esch…»
disposition: holds — re-pointed by hand to 49 (the line now reads «the thirteen driver commands as tools», the form the pin reads)
