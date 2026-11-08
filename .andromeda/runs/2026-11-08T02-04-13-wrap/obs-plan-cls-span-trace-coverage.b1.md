@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the session adds no span, instrument or tracing call to the examples files it reaches, and no new example file joins the slice
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the session adds no span or instrument in the apps slice, and its two new dioxus-native-dom modules hold no log site or span
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the session adds no span, instrument or debug_span to the s05 files, its only span change being graph's word in the existing verb field
@answer n=4 entity=span-trace-coverage#1.4 v=no
why: the session adds no span or instrument to the listed s06 files, and graph touches only the existing command span
@answer n=5 entity=span-trace-coverage#1.5 v=no
why: no layout file is reached and the session adds no span or instrument to the layout slice
@answer n=6 entity=span-trace-coverage#1.6 v=no
why: the session adds no span or instrument in the two new dioxus-native-dom modules or the listed files, so the single doc-comment match stands
@answer n=7 entity=span-trace-coverage#1.7 v=no
why: the session adds no span or instrument to the listed s09 files, and the graph verb adds only a word to the existing command span
@answer n=8 entity=span-trace-coverage#1.8 v=no
why: the session changes no blitz-vibey-script or blitz engine source, no blitz-dom file and no dependency, and adds no span or instrument
@answer n=9 entity=span-trace-coverage#1.9 v=no
why: the session adds no span or instrument to the listed s11 files
@answer n=10 entity=span-trace-coverage#1.10 v=yes mode=stale
why: stand_graph.rs is a new file in the test crate, so the slice's file count moves up by one while its silence census still reads 0
@answer n=11 entity=span-trace-coverage#1.11 v=no
why: the session reaches no wpt runner file and adds no span, metric or tracing call
@answer n=12 entity=span-trace-coverage#2.1 v=yes mode=add-to
why: graph joins the verbs whose command span records verb alone (F2), while the eight fields and the 2 info_span! sites stay unchanged
@answer n=13 entity=span-trace-coverage#2.2 v=yes mode=stale
why: host_log now drives one graph call at trace, so its cited lines host_log.rs:138-281 can be off after the lines added
@answer n=14 entity=span-trace-coverage#2.3 v=yes mode=stale
why: host_log.rs and mcp_log.rs gained lines, so the cited ranges host_log.rs:283-394 and mcp_log.rs:70-115 can be off
@answer n=15 entity=span-trace-coverage#2.4 v=yes mode=stale
why: packages/escher-driver is no longer untouched, since the session edits its schema, command, execute, json and mcp files
@answer n=16 entity=span-trace-coverage#3.1 v=yes mode=stale
why: graph is the thirteenth verb-table word and joins the tool field, so the count of the verb table's words reads thirteen, not twelve
@answer n=17 entity=span-trace-coverage#3.2 v=no
why: the cold-agent counted verbs and the span_counts reader are unchanged, graph joins VERBS but is not counted, so the verdict fields do not move
