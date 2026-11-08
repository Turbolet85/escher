@answer n=1 entity=conventions#1.1 v=no
why: the session adds no lint allowance or CI gate, and the lint, CI and rustdoc files this entity cites are not among the files the change reaches
@answer n=2 entity=conventions#2.1 v=no
why: no manifest, lockfile or dependency moved, and the Cargo.toml files this entity cites are not reached
@answer n=3 entity=conventions#3.1 v=no
why: the session follows the documentation rule without changing it, and no file this entity cites is reached
@answer n=4 entity=conventions#4.1 v=no
why: the session adds no TODO or FIXME marker, and the marked lines this entity cites sit in files the change does not reach
@answer n=5 entity=conventions#5.1 v=no
why: the new internals keep the pub(crate) and private visibility the rule states, and the files this entity cites are not reached
@answer n=6 entity=conventions#6.1 v=yes mode=stale
why: the span citations at execute.rs:109-124 and mcp.rs:118-121 and the trace macro citations at dioxus-native-dom lib.rs:67-90 sit in files the session reaches whole, so those cited line ranges can move
@answer n=7 entity=conventions#7.1 v=no
why: the cited unsafe and borrow sites sit in dioxus_document.rs and blitz-dom, which the change does not reach, and actionable_nodes adds a reader without changing the bridge render path
@answer n=8 entity=conventions#8.1 v=yes mode=stale
why: the session adds stand_actions.rs, a Session-holding check that declares mod session_common (declarers 33 to 34), so the list of Session-holding checks is stale, and the citations at schema.rs:628-630 and mcp.rs:291-293 sit in reached files
@answer n=9 entity=conventions#9.1 v=no
why: the session moves no test module and renames only the schema and MCP test names, so cli_tests.rs and recording_tests.rs keep their own-file modules and test paths
@answer n=10 entity=conventions#9.2 v=no
why: fixtures/mod.rs is not reached and states no declarer count, so a new fixtures declarer does not move the shared-fixture claim it holds
@answer n=11 entity=conventions#9.3 v=yes mode=stale
why: mcp_tests.rs and stand_requirement_sweep.rs are reached whole, so the cited line ranges 421-502 and 1050-1093 can move with the session's count and name changes
@answer n=12 entity=conventions#10.1 v=no
why: the binary and example rules cite apps and examples src files that the change does not reach
@answer n=13 entity=conventions#11.1 v=no
why: the seven_guis signal rule cites examples/seven_guis/src/tasks files, which the change does not reach
@answer n=14 entity=conventions#12.1 v=no
why: the HTML fixture rule cites examples/assets files, which the change does not reach
