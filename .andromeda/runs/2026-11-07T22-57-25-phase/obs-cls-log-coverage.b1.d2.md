@answer n=1 entity=log-coverage#1.1 v=yes
why: the new verb's command span and the new tool's tool_call span are closed spans judged pair by pair by this scrub rule, and their string fields are Debug-quoted (items 4, 5, 10)
@answer n=2 entity=log-coverage#2.1 v=no
why: the cold-agent verdict fields in scripts/cold-agent.sh, which the chunk does not touch and whose counted verbs the scope does not change
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts reads the accepted tool_call lines of lint, screenshot and advance in cold-agent.sh, a script the chunk does not reach and a verb set it does not change
@answer n=4 entity=log-coverage#3.2 v=yes
why: item 8 writes each answered call into the kept check under this recording directory, so the one-check-per-accepted-start and never-printed-or-copied rule rules over what the chunk writes
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met reads snapshot answers and paged parts in cold-agent.sh, and the new graph read is neither a snapshot nor a paged answer
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser is not touched (not_needed item 1)
@answer n=7 entity=log-coverage#5.1 v=yes
why: escher-session's command line writes the read's answer as one line of JSON on stdout with stderr empty, and its usage line is built from VERBS, which gains the new verb (items 11, 13)
@answer n=8 entity=log-coverage#6.1 v=yes
why: the new verb opens one command span per call and the new MCP tool opens one tool_call span per call, in the host and mcp roles that item 9 reads at trace (items 4 to 6, 9)
@answer n=9 entity=log-coverage#7.1 v=no
why: the chunk adds no span and changes no scrub limit, so the by-hand rule on the span set does not fire, and docs/capabilities.md is not named
@answer n=10 entity=log-coverage#8.1 v=no
why: blitz-dom's focus, image, resolve and resource log sites are engine code the scope does not name and the chunk does not change (not_needed item 8), and the graph read moves no focus
@answer n=11 entity=log-coverage#8.2 v=no
why: blitz-dom event and util log sites are engine code the scope does not name
@answer n=12 entity=log-coverage#8.3 v=no
why: blitz-dom layout log sites are engine code the scope does not name
@answer n=13 entity=log-coverage#8.4 v=no
why: blitz-dom node property warnings are engine code the scope does not name, and the graph carries no CSS property value
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net fetch logs, and the graph reads no network (not_needed item 7)
@answer n=15 entity=log-coverage#8.6 v=no
why: blitz-html parse-error logs are engine code the scope does not name
@answer n=16 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint log sites are engine code the scope does not name
@answer n=17 entity=log-coverage#8.8 v=no
why: blitz and blitz-vibey-script log sites are not touched (not_needed item 3 and the launch path)
@answer n=18 entity=log-coverage#8.9 v=yes
why: item 2 names dioxus-native-dom's mutation_writer debug trace sites, which record text-node contents and attribute values, and the graph code must add no such site
@answer n=19 entity=log-coverage#8.10 v=no
why: mutation_buffer is an existing crate-private module the scope does not name or edit, and its silence is its own code rather than a rule the chunk's new code must meet
@answer n=20 entity=log-coverage#8.11 v=no
why: its println and eprintln bullets name paint_tree_bench.rs and br_trailing_line.rs, which the chunk does not edit, and the edited stand_non_lean_tasks.rs is held by the test silence convention
@answer n=21 entity=log-coverage#8.12 v=no
why: wpt runner logging is not touched (not_needed item 2)
@answer n=22 entity=log-coverage#8.13 v=no
why: CI publish build logging is not touched, and the chunk adds no CI job (not_needed item 5)
@answer n=23 entity=log-coverage#9.1 v=no
why: a past absence record over the s02 files, which the scope does not name, and earlier silence records are not re-run (not_needed item 10)
@answer n=24 entity=log-coverage#9.2 v=yes
why: it records the absence of tracing, log and env_logger across the examples slice, where the seven_guis stand and its tasks sit, and item 12 re-measures that silence
@answer n=25 entity=log-coverage#9.3 v=yes
why: it records zero log, env and listener sites in the test additions of stand_act_spans and seven_guis host_log, both of which the chunk extends (items 6, 9)
