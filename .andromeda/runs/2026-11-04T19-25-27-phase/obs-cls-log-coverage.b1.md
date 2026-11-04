@answer n=1 entity=log-coverage#1.1 v=no
why: the closed-span line format of escher-driver and escher-telemetry; the chunk adds no span and reads no log line, so its evidence comes from assertions
@answer n=2 entity=log-coverage#2.1 v=no
why: cold-agent verdict fields in the agent pipe; the probe does not reach scripts/cold-agent.sh
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts over the cold-agent server.log belongs to the agent pipe, not to the probe
@answer n=4 entity=log-coverage#3.2 v=no
why: the cold-agent recording directory and its kept checks, outside the chunk's files
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met reads cold-agent snapshot answers, which the chunk never reads
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser emitters are not_needed for this chunk, and the probe runs no browser
@answer n=7 entity=log-coverage#5.1 v=no
why: example prints and escher-session's fixed endings; the probe adds no verb, tool or answer and prints nothing by its own rule
@answer n=8 entity=log-coverage#6.1 v=yes
why: host_log and mcp_log must still pass under both builds with 0 needles at trace (task item 10), and this entity records those drives and the command span's eight fields
@answer n=9 entity=log-coverage#7.1 v=no
why: docs/capabilities.md restates emitter reach; the chunk changes no span set and no scrub limit, and touches no capability statement
@answer n=10 entity=log-coverage#8.1 v=yes
why: the probe must give the same verdicts under both builds and avoid calling debug_log_node, which prints, and this entity names the workspace-build-only focus trace and that print
@answer n=11 entity=log-coverage#8.2 v=no
why: IME, link-href, WOFF and cursor logs sit on paths the Counter, CRUD and served clicks do not reach, and the scope names no link or cursor
@answer n=12 entity=log-coverage#8.3 v=no
why: the layout log sites emit only under a subscriber the probe never installs, and no call of the probe must act on them
@answer n=13 entity=log-coverage#8.4 v=yes
why: Node::print_tree writes the tree to stdout with println!, a call the probe must avoid since it prints nothing
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net fetch logs fire only on fetches, and the probe starts none (the examples probe holds no blitz_net, task item 11)
@answer n=15 entity=log-coverage#8.6 v=no
why: HTML parse-error logs need an HTML parse and a subscriber, and the probe runs neither
@answer n=16 entity=log-coverage#8.7 v=no
why: convert_events and paint warnings sit on windowed-shell and paint paths the headless probe does not run, and the chunk adds no packages line for the event.rs census caveat to reach
@answer n=17 entity=log-coverage#8.8 v=no
why: launch_url, vibey-script JS logs and the JS console mapping belong to the windowed launch and the JS runtime, which the probe does not use
@answer n=18 entity=log-coverage#8.9 v=yes
why: the probe reads element ids and listeners and must not depend on the mutation_writer debug trace! lines that carry text contents and attribute values (task item 4)
@answer n=19 entity=log-coverage#8.10 v=no
why: the println! and log-phase-times prints belong to existing tests; the probe enables no feature and prints nothing by its own rule
@answer n=20 entity=log-coverage#8.11 v=no
why: wpt runner logging and its panic hook are not_needed for this chunk
@answer n=21 entity=log-coverage#8.12 v=no
why: CI publish builds at CARGO_LOG info are not_needed for this chunk
@answer n=22 entity=log-coverage#9.1 v=no
why: a console-call census over the earlier s02 files, which this chunk does not change
@answer n=23 entity=log-coverage#9.2 v=yes
why: records the tracing, log and env_logger silence over the examples slice, which carries the served-stand census over stand.rs and the shared modules that the chunk re-measures (task item 11)
@answer n=24 entity=log-coverage#9.3 v=yes
why: records the silence census over the shared fixtures module, stand_act_spans and the seven_guis host_log tests, the census the chunk re-measures over stand.rs and the shared modules (task item 11)
@answer n=25 entity=log-coverage#9.4 v=no
why: a sink census over key_press_after_container_lost_its_box and url_search_params, existing tests the chunk does not re-measure
