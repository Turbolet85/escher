@answer n=1 entity=test-scope-summary#41.1 v=yes mode=stale
why: stand_requirement_sweep.rs is a reached file and the sweep test's helper names and cited line ranges sit in it, so they can have moved

@answer n=2 entity=test-scope-summary#42.1 v=no
why: the stand_ glob takes stand_graph with no script change and no named target added, so the leg's named targets and the containment asserts still hold

@answer n=3 entity=test-scope-summary#43.1 v=yes mode=stale
why: stand_cells_served.rs is reached and the Cells screen moved from 47,089 to 53,859 bytes and from 5 to 6 parts, its PARTS constant now reading 6

@answer n=4 entity=test-scope-summary#43.2 v=yes mode=stale
why: host_log.rs is reached and the host log checks now make a graph call, so the cited Cells part read at lines 138-281 can have moved

@answer n=5 entity=test-scope-summary#43.3 v=no
why: stand.rs is not among the session's files and the stand module's three unit tests and their five-passed count are unchanged

@answer n=6 entity=test-scope-summary#43.4 v=no
why: lint.rs and the lint's select-option overlap rule are not among the session's files and the graph verb does not change lint

@answer n=7 entity=test-scope-summary#44.1 v=yes mode=stale
why: cli_commands.rs, host_log.rs and mcp_log.rs are reached and the host and mcp-role log checks now make a graph call each, so the cited popup checks and their zero-needle reading can have moved

@answer n=8 entity=test-scope-summary#45.1 v=yes mode=stale
why: stand_cells_served.rs is reached whole and the Cells screen now splits into six parts, so the keyboard walk's first-part reading and cited lines can have moved

@answer n=9 entity=test-scope-summary#46.1 v=no
why: the WPT runner and its tests are unreached by the session, which changed no wpt file
