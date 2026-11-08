@answer n=1 entity=log-coverage#9.4 v=no
why: the session reaches neither the key_press_after_container_lost_its_box test file nor blitz-vibey-script, which moved no source
@answer n=2 entity=log-coverage#9.5 v=yes mode=stale
why: host_log.rs is reached and gained a graph call, so its cited lines 283-394 may have moved
@answer n=3 entity=log-coverage#9.6 v=yes mode=stale
why: mcp_log.rs is reached and gained lines, so its cited lines 70-115 may have moved
@answer n=4 entity=log-coverage#9.7 v=yes mode=stale
why: the shared seven_guis module gained graph_needles, so the cited lines 454-482 may have moved
@answer n=5 entity=log-coverage#9.8 v=yes mode=stale
why: tests/common/mod.rs is a reached shared module that gained Drive and GRAPH_DRIVES, so its silence reading may have moved from the 0.2.0 measure
@answer n=6 entity=log-coverage#9.9 v=yes mode=add-to
why: F1, the graph outcome written by to_json alone and never logged or fielded, while escher-driver keeps its 2 info_span! sites
@answer n=7 entity=log-coverage#9.10 v=no
why: blitz-vibey-script is untouched and its tests/dom.rs is not among the reached files
@answer n=8 entity=log-coverage#9.11 v=no
why: the session keeps the 2 info_span! sites and the eight span fields, and host_log and mcp_log still read 0 needles at trace under both builds
@answer n=9 entity=log-coverage#9.12 v=no
why: none of the five check files is among the session's reached files, and the census covers those files alone
@answer n=10 entity=log-coverage#9.13 v=no
why: the dioxus_runtime_probe file is not among the reached files, and its census covers that file alone
@answer n=11 entity=log-coverage#9.14 v=no
why: the two pins and their helper sit in dioxus_runtime_probe, which the session does not reach
@answer n=12 entity=log-coverage#9.15 v=yes mode=stale
why: stand_act_popup reads session_common, a reached shared module that gained graph and graphed, so the silence reading over the popup checks may have moved
