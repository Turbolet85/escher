@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the examples slice is not reached by the port and the chunk adds no span (task n=8), so the absence over that slice stands
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the apps slice is not reached by the port and the chunk adds no span (task n=8)
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the s05 files are a slice the port does not reach and the chunk adds no span (task n=8)
@answer n=4 entity=span-trace-coverage#1.4 v=no
why: the s06 files are a slice the port does not reach and the chunk adds no span (task n=8)
@answer n=5 entity=span-trace-coverage#1.5 v=no
why: the layout slice is a slice the port does not reach and the chunk adds no span (task n=8)
@answer n=6 entity=span-trace-coverage#1.6 v=no
why: the entity names no file the chunk reaches, the 16 listed files are not the port's units, and the chunk adds no span (task n=8)
@answer n=7 entity=span-trace-coverage#1.7 v=no
why: the s09 files are a slice the port does not reach and the chunk adds no span (task n=8)
@answer n=8 entity=span-trace-coverage#1.8 v=no
why: the blitz-vibey-script runtime and element bindings are code the port does not edit, and the moved code's absence is not changed by a dioxus-native-dom render change
@answer n=9 entity=span-trace-coverage#1.9 v=no
why: the s11 files are a slice the port does not reach and the chunk adds no span (task n=8)
@answer n=10 entity=span-trace-coverage#1.10 v=yes
why: the test crate slice holds tests/blitz-tests, where the probe's two inverted tests must stay silent with the silence census reading 0 over the file (task n=7), so the recorded absence binds
@answer n=11 entity=span-trace-coverage#1.11 v=no
why: the wpt runner slice is not reached by the port, and the wpt runner is not_needed (task item 6)
@answer n=12 entity=span-trace-coverage#2.1 v=yes
why: stand_act_spans keeps the command span's eight fields over the render path the port changes (task n=9), and the never-fielded rule over Command and Outcome is the buffer's no-Debug obligation (task n=2)
@answer n=13 entity=span-trace-coverage#2.2 v=yes
why: host_log is named by the scope (task n=9), and the hosted call's closed command span lines must still read 0 id and name needles under both builds
@answer n=14 entity=span-trace-coverage#2.3 v=yes
why: host_log's and mcp_log's select-list tests are checks the scope names as still passing at trace (task n=9), and the render path change reaches them
@answer n=15 entity=span-trace-coverage#2.4 v=yes
why: the mounted-event guard in flush_queued_mounted_events opens no span, and the port changes that function's two callers while keeping its order and silence (task n=5)
@answer n=16 entity=span-trace-coverage#3.1 v=yes
why: tool_call is escher-driver's second info_span site, which the scope keeps (task n=9), and the mcp role the render change reaches records it
@answer n=17 entity=span-trace-coverage#3.2 v=no
why: it states the cold-agent pipe's counting of tool_call fields, and the cold-agent pipe is not_needed (task item 2), with the two-field rule the chunk must keep carried by span-trace-coverage#3.1
