@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: the scope names examples/seven_guis probe code (task n=11), which must add no span! or instrument, so the examples slice census binds
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the apps slice is not reached, and apps/browser is named not needed (task not_needed n=3)
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the listed s05 files are not named by the scope and no reached file or named surface sits among them
@answer n=4 entity=span-trace-coverage#1.4 v=no
why: the listed s06 files are not named by the scope and no reached file or named surface sits among them
@answer n=5 entity=span-trace-coverage#1.5 v=no
why: the layout slice is not reached; the probe runs in both layout modes but edits no layout code
@answer n=6 entity=span-trace-coverage#1.6 v=yes
why: the probe reads dioxus-native-dom's element-id and listener surface, which task n=5 requires to add no span, so the listed-file absence binds
@answer n=7 entity=span-trace-coverage#1.7 v=no
why: the listed s09 files are not named by the scope and no reached file or named surface sits among them
@answer n=8 entity=span-trace-coverage#1.8 v=no
why: the blitz-vibey-script runtime and element bindings are not reached; the probe reads dioxus-native-dom only
@answer n=9 entity=span-trace-coverage#1.9 v=no
why: the listed s11 files are not named by the scope and no reached file or named surface sits among them
@answer n=10 entity=span-trace-coverage#1.10 v=yes
why: the new probe check lives in tests/blitz-tests, so the test-crate census must keep reading 0 with the new file counted (task n=1)
@answer n=11 entity=span-trace-coverage#1.11 v=no
why: the wpt runner is not reached and is named not needed (task not_needed n=5)
@answer n=12 entity=span-trace-coverage#2.1 v=yes
why: the scope names the command span's eight fields as untouched (task n=6), and its never-fielded Debug rule binds the probe's reads (task n=2)
@answer n=13 entity=span-trace-coverage#2.2 v=yes
why: task n=10 requires host_log to keep its closed command span count and 0 id and name needles at trace under both builds, which the probe must not disturb
@answer n=14 entity=span-trace-coverage#2.3 v=yes
why: task n=10 requires host_log and mcp_log to keep reading 0 needles at trace on an open select list
@answer n=15 entity=span-trace-coverage#2.4 v=yes
why: task n=10 requires host_log and mcp_log to keep reading 0 needles at trace under both builds, and the mounted-event guard in flush_queued_mounted_events is the events.rs surface the probe re-reads
@answer n=16 entity=span-trace-coverage#3.1 v=yes
why: the scope names the tool_call span with its tool and ending fields as untouched (task n=6), so the probe must add no field to it
@answer n=17 entity=span-trace-coverage#3.2 v=yes
why: task n=6 binds the tool_call's two fields with none added; its cold-agent server.log counting is itself not needed (task not_needed n=6) and is not carried
