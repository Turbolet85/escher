@answer n=1 entity=log-coverage#1.1 v=no
why: the closed-span line format of escher-telemetry's sink; the probe installs no sink (item 3), adds no span (item 6) and escher-telemetry stays unchanged (item 10)
@answer n=2 entity=log-coverage#2.1 v=no
why: cold-agent verdict fields in the agent pipe, not_needed 6; the probe reads no verdict field
@answer n=3 entity=log-coverage#3.1 v=no
why: cold-agent span_counts over server.log, the agent-run harness, not_needed 6
@answer n=4 entity=log-coverage#3.2 v=no
why: cold-agent recording directory and kept checks, the agent-run harness, not_needed 6 and 9
@answer n=5 entity=log-coverage#3.3 v=no
why: cold-agent tasks_met and the replay expect match, the agent-run harness, not_needed 6
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser emitters and the values they log as-is, not_needed 3
@answer n=7 entity=log-coverage#5.1 v=yes
why: names seven_guis_native and escher-session as the sink binaries that the item 7 reading must name, and item 11 bars escher_telemetry use in probe code; the other example prints and the escher-session and mcp surfaces are not reached (not_needed 9)
@answer n=8 entity=log-coverage#6.1 v=yes
why: the command span's eight fields and the host_log and mcp_log 0-needle lines at trace are what item 6 keeps and item 10 requires, so the probe adds no span or field; the tool_call and MCP-role lines are not_needed 9
@answer n=9 entity=log-coverage#7.1 v=no
why: docs/capabilities.md restates the span set and the scrub limits; the probe changes neither (items 6 and 7 add no span, hook or line) and the page is not reached
@answer n=10 entity=log-coverage#8.1 v=yes
why: the focus info line, present only under the workspace build, is an engine trace the probe's verdicts must not depend on (item 4), and the probe drives focus through the stand
@answer n=11 entity=log-coverage#8.2 v=no
why: pointer href, IME, WOFF and cursor logs carry content or sit off the probe's path, not_needed 8; a click on a link without href is not driven
@answer n=12 entity=log-coverage#8.3 v=no
why: layout error and info sites that carry node text or image URLs, not_needed 8; the probe's layout reads do not reach them
@answer n=13 entity=log-coverage#8.4 v=no
why: unsupported-property and invalid-value warnings and print_tree, the invalid CSS value warning and debug_log_node are not_needed 8; the probe's verdicts do not depend on them
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net request URLs and fetch errors, not_needed 8; the probe's verdicts do not depend on them
@answer n=15 entity=log-coverage#8.6 v=no
why: HTML parse-error lines written on sink finish; the probe's verdicts do not depend on them (item 4) and its scope parses no HTML
@answer n=16 entity=log-coverage#8.7 v=no
why: windowed event-conversion errors and paint warnings; the windowed shell is read, not run (item 8), and the headless probe converts no window events
@answer n=17 entity=log-coverage#8.8 v=no
why: launch_url, vibey-script JS error and console logs, not_needed 4 and windowed; the probe runs no page JS
@answer n=18 entity=log-coverage#8.9 v=yes
why: the mutation_writer trace lines carry text-node contents and attribute values under the workspace build, which the probe must not depend on (item 4), and dioxus-native-dom stays a read surface with its trace features untouched (item 5)
@answer n=19 entity=log-coverage#8.10 v=no
why: the blitz-tests phase-timing print is not_needed 1 and the test println diagnostics sit in files the chunk does not touch; the probe prints nothing (item 1) by its own census, not by this practice
@answer n=20 entity=log-coverage#8.11 v=no
why: wpt runner env_logger and free-form log calls, not_needed 5
@answer n=21 entity=log-coverage#8.12 v=no
why: CI publish builds at CARGO_LOG info, not_needed 2
@answer n=22 entity=log-coverage#9.1 v=no
why: JS console-call absence over the s02 files; the probe runs natively (not_needed 7) and the chunk touches none of those files
@answer n=23 entity=log-coverage#9.2 v=yes
why: the structured-logging absence over the examples slice covers the files item 11 keeps at 0 (stand.rs and the shared modules), and the probe's own file joins that census
@answer n=24 entity=log-coverage#9.3 v=yes
why: the silence census over the fixtures module and the stand check files is the census item 11 re-measures over stand.rs and the shared modules, which the probe's new readers must keep at 0
@answer n=25 entity=log-coverage#9.4 v=no standing=S1
why: the no-env-read, no-print and no-tracing silence each new check keeps, restated for a past chunk's test target that the chunk does not reach
