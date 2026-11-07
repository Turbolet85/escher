@answer n=1 entity=log-coverage#1.1 v=yes
why: stand_act_spans re-asserts the command span's own fields and close line, and host_log and mcp_log read those spans at trace, all named by the scope
@answer n=2 entity=log-coverage#2.1 v=no
why: the cold-agent verdict fields belong to the cold-agent pipe, which the chunk does not touch
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts reads the cold-agent server.log, which the chunk does not touch
@answer n=4 entity=log-coverage#3.2 v=no
why: the cold-agent live server's recording directory is outside the chunk's reach
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met and the replay expect match belong to the cold-agent pipe, which the chunk does not touch
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser document loader, toolbar and persistence logs are not reached, and apps/browser is not needed
@answer n=7 entity=log-coverage#5.1 v=yes
why: the render path reaches the escher-session host and mcp role, whose endings and silence the scope checks through host_log and mcp_log, while the example println sites are not reached
@answer n=8 entity=log-coverage#6.1 v=yes
why: escher-driver's two info_span sites, stand_act_spans' eight fields and host_log and mcp_log's zero id and name needles at trace are all named by the scope
@answer n=9 entity=log-coverage#7.1 v=no
why: it restates docs/capabilities.md, which the chunk does not reach, and the span set and scrub limits it states stay true since the chunk adds no span, field, driver line or panic path
@answer n=10 entity=log-coverage#8.1 v=yes
why: the focus-move info line under the workspace build is carried by set_focus, which the inverted tests drive, and the mounted-event guard sits in the flush path the chunk changes
@answer n=11 entity=log-coverage#8.2 v=no
why: blitz-dom IME, link, pointer, WOFF and cursor-icon sites are off the render path and the scope names no pointer, IME or font behaviour
@answer n=12 entity=log-coverage#8.3 v=no
why: the layout sites fire only on text-node, SVG, image and table content the scope does not name, though get_client_rect carries layout
@answer n=13 entity=log-coverage#8.4 v=no
why: the blitz-dom element style warns fire on style properties the scope does not name, and the replay changes nothing passed to blitz-dom
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net fetch logging is outside the chunk's reach, and the task lists it as not needed
@answer n=15 entity=log-coverage#8.6 v=no
why: blitz-html parse-error logging is on sink finish, and the render path parses no HTML
@answer n=16 entity=log-coverage#8.7 v=no
why: blitz-shell event conversion and blitz-paint background and mask warns are not on the render path, and no paint or census over packages is in scope
@answer n=17 entity=log-coverage#8.8 v=no
why: blitz launch_url and blitz-vibey-script JS logs are not used by the chunk, and the JS error surface is not needed
@answer n=18 entity=log-coverage#8.9 v=yes
why: mutation_writer.rs's trace sites fire on the replay through the same MutationWriter, and dioxus_application's document-provider injection reaches the windowed launch the scope names
@answer n=19 entity=log-coverage#8.10 v=no
why: the println diagnostics sit in other test files the chunk does not touch, and the probe's own silence is carried by its census item
@answer n=20 entity=log-coverage#8.11 v=no
why: wpt runner logging is not needed for this chunk
@answer n=21 entity=log-coverage#8.12 v=no
why: the CI publish-build CARGO_LOG setting is outside the chunk, which adds no CI job or artifact
@answer n=22 entity=log-coverage#9.1 v=no
why: console-call absence over the 21 s02 files, which the chunk does not touch and the scope does not name
@answer n=23 entity=log-coverage#9.2 v=no
why: structured-logging absence over the 32 examples slice files, none of which the chunk touches
@answer n=24 entity=log-coverage#9.3 v=yes
why: it records the silence census reading 0 over the stand_* test additions, which the scope re-asserts as witnesses and whose census readings must stay as they are
@answer n=25 entity=log-coverage#9.4 v=no standing=S1
why: it records an absent env read, print or subscriber in key_press_after_container_lost_its_box and url_search_params, a convention each new check owes and not a census the chunk adds to
