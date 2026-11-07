@answer n=1 entity=span-trace-coverage#1.1 v=no
why: absence of span! and instrument over the examples slice; the entity names no file the chunk edits and no named check, and the chunk adds no span
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: absence over the apps slice, where apps/browser is not_needed; the entity names no edited file and no named check
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: absence over the 15 s05 files; the entity names no edited file and no named check
@answer n=4 entity=span-trace-coverage#1.4 v=no
why: absence over the 17 listed s06 files; the entity names no edited file and no named check
@answer n=5 entity=span-trace-coverage#1.5 v=no
why: absence over the layout slice; the chunk edits no layout file and adds no span
@answer n=6 entity=span-trace-coverage#1.6 v=no
why: absence over 16 listed files whose only match is a doc comment; the entity names no edited file and no named check
@answer n=7 entity=span-trace-coverage#1.7 v=no
why: absence over the 32 listed s09 files; the entity names no edited file and no named check
@answer n=8 entity=span-trace-coverage#1.8 v=no
why: absence over the blitz-vibey-script and blitz slice's moved runtime and element modules; the chunk edits no such file and the JS error surface is not_needed
@answer n=9 entity=span-trace-coverage#1.9 v=no
why: absence over the 21 listed s11 files; the entity names no edited file and no named check
@answer n=10 entity=span-trace-coverage#1.10 v=yes
why: the test crate slice holds dioxus_runtime_probe.rs, which the chunk edits, and the scope names the silence census reading 0 over that file
@answer n=11 entity=span-trace-coverage#1.11 v=no
why: absence over the wpt runner slice; the wpt runner is not_needed and the chunk edits no wpt file
@answer n=12 entity=span-trace-coverage#2.1 v=yes
why: the scope names escher-driver keeping its 2 info_span sites and stand_act_spans keeping its eight fields, which this entity's span rule states
@answer n=13 entity=span-trace-coverage#2.2 v=yes
why: the scope names host_log, which the render path reaches, and its 0 id and 0 name needles at trace under both builds
@answer n=14 entity=span-trace-coverage#2.3 v=yes
why: the scope names host_log and mcp_log reading 0 needles on an open select list at trace, and escher-driver's 2 info_span sites
@answer n=15 entity=span-trace-coverage#2.4 v=yes
why: the scope names the mounted-event guard in flush_queued_mounted_events and host_log and mcp_log reading 0 needles at trace
@answer n=16 entity=span-trace-coverage#3.1 v=yes
why: the tool_call span is escher-driver's second info_span site, which the scope names, and the render path reaches the escher-session mcp role
@answer n=17 entity=span-trace-coverage#3.2 v=no
why: the cold-agent pipe and its server.log span counting are not_needed, and the chunk does not touch scripts/cold-agent.sh
