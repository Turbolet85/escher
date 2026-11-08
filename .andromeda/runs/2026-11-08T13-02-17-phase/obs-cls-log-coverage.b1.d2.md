@answer n=1 entity=log-coverage#1.1 v=yes
why: the new command span's line form, its verb and cause fields, and the span-field scrub that item 5 names are what this rule sets
@answer n=2 entity=log-coverage#2.1 v=no
why: the cold-agent verdict fields belong to scripts/cold-agent.sh, which the chunk does not reach
@answer n=3 entity=log-coverage#2.2 v=no
why: the pipe's own VERBS list and graph's verdict treatment live in scripts/cold-agent.sh, which the chunk does not reach
@answer n=4 entity=log-coverage#3.1 v=no
why: span_counts reads only the pipe's three counted verbs from server.log, and the new read is not one of them
@answer n=5 entity=log-coverage#3.2 v=yes
why: the recording module writes each answered call into the kept check, and this rule states the one-check-per-accepted-start record directory those checks sit in
@answer n=6 entity=log-coverage#3.3 v=no
why: tasks_met and the pipe's paged expect match are pipe-side, and the new read is not a snapshot answer
@answer n=7 entity=log-coverage#4.1 v=no
why: apps/browser's logging is not reached by the read, the fixtures or the recording change
@answer n=8 entity=log-coverage#5.1 v=yes
why: the escher-session accepted and refused stdout JSON line, the mcp role's stdout-only protocol lines and describe's help line are what items 7 and 8 change in cli.rs and describe.rs
@answer n=9 entity=log-coverage#6.1 v=yes
why: the command and tool_call span lines the new read and verb add, the verb table's thirteen-word count the new verb changes, and the host_log and mcp_log needle checks
@answer n=10 entity=log-coverage#7.1 v=yes
why: docs/capabilities.md is reached and states the command and tool_call span claims and scrub limits the new spans must keep true, which the capability pin does not check
@answer n=11 entity=log-coverage#8.1 v=no
why: blitz-dom log sites the read only uses, unchanged, with no failure path the read reaches, since its hit test runs on a live DOM and the no-DOM warn never fires
@answer n=12 entity=log-coverage#8.2 v=no
why: blitz-dom event and util log sites (IME, pointer links, WOFF, cursor icon) are not on the read's or the fixtures' path
@answer n=13 entity=log-coverage#8.3 v=no
why: blitz-dom layout log sites (text, SVG, image, table) are not reached by the minimal covered, off-screen and owner-served fixtures
@answer n=14 entity=log-coverage#8.4 v=no
why: blitz-dom node and property log sites and print_tree are not reached by the read or the fixtures
@answer n=15 entity=log-coverage#8.5 v=no
why: blitz-net fetch and cache log sites are not reached, since the chunk fetches nothing
@answer n=16 entity=log-coverage#8.6 v=no
why: blitz-html parse-error logs need a parse error that the minimal fixtures are not stated to carry
@answer n=17 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint convert-event and paint warnings are not reached by the read or the fixtures
@answer n=18 entity=log-coverage#8.8 v=no
why: launch_url and blitz-vibey-script's JS error logs are not reached by the read or the fixtures
@answer n=19 entity=log-coverage#8.9 v=no
why: the mutation writer's trace lines are not changed or read by the chunk, and host_log's needle check reads id and name text these lines do not state
@answer n=20 entity=log-coverage#8.10 v=no
why: mutation_buffer is a render component the chunk uses unchanged, and no scope item names its silence
@answer n=21 entity=log-coverage#8.11 v=yes
why: the new Outcome variant must be written by to_json alone and never logged, as Outcome::Graphed is, and graph.rs must stay silent with the 2 info_span sites (items 4 and 10)
@answer n=22 entity=log-coverage#8.12 v=no
why: the bridge's dispatch path is not reached, since the read dispatches nothing
@answer n=23 entity=log-coverage#8.13 v=no
why: blitz-tests diagnostics are not reached by the chunk's code or checks
@answer n=24 entity=log-coverage#8.14 v=no
why: the WPT runner's logging is not reached by the chunk
@answer n=25 entity=log-coverage#8.15 v=no
why: CI publish-build logging is not reached by the chunk
