@answer n=1 entity=log-coverage#9.5 v=yes
why: host_log at trace reads 0 option needles, and the scope keeps host_log's zero id, name and typed-text needle reading over the render path change
@answer n=2 entity=log-coverage#9.6 v=yes
why: the mcp role's trace log reads 0 option needles, and the scope keeps mcp_log's zero needle reading after the render path reaches the mcp role
@answer n=3 entity=log-coverage#9.7 v=yes
why: option_needles is the needle value host_log and mcp_log read for their zero needle checks, which the scope re-asserts
@answer n=4 entity=log-coverage#9.8 v=yes
why: stand_act_select is one of the eight named checks and a stand check whose silence census the scope keeps; the other a11y files are untouched and not reached by the bridge
@answer n=5 entity=log-coverage#9.9 v=yes
why: the scope keeps escher-driver's 2 info_span sites, and stand_requirement_sweep.rs is a stand check whose silence census the scope keeps
@answer n=6 entity=log-coverage#9.10 v=no
why: blitz-vibey-script tests/dom.rs is untouched, no stand check, and not reached by the Dioxus render path
@answer n=7 entity=log-coverage#9.11 v=yes
why: the upstream take's census names escher-driver's 2 info_span sites, stand_act_spans' eight fields and host_log and mcp_log's zero needles at trace, all of which the scope keeps
@answer n=8 entity=log-coverage#9.12 v=no
why: the five named checks are untouched, none is a stand check, and the scope names none of them
@answer n=9 entity=log-coverage#9.13 v=yes
why: dioxus_runtime_probe.rs is a touched file whose silence census reads 0 and whose failure messages must name a layout mode and outcome only
@answer n=10 entity=log-coverage#9.14 v=yes
why: stand_act_popup is a stand check whose silence census the scope keeps as it is
