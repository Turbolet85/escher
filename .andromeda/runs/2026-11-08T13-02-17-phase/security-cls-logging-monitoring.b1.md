@answer n=1 entity=logging-monitoring#1.1 v=yes
why: the scope's command span rule (task item 6) names one command span per call holding verb and refused cause alone, and the read adds a tenth instance-level verb to the verb table whose count this entry records
@answer n=2 entity=logging-monitoring#1.2 v=yes
why: escher-session writes each command's answer as one stdout JSON line under fixed statuses, and the read's answer carrier in cli.rs and json.rs must keep caller-typed text out of that line
@answer n=3 entity=logging-monitoring#2.1 v=no
why: the failed-load line comes from the net path's ResourceLoadResponse and NetHandler::error, which the read never reaches since it loads nothing and net intake is not needed
@answer n=4 entity=logging-monitoring#3.1 v=no
why: the host_log and mcp_log select-list tests witness a popup the read neither opens nor changes, and the read's answer carries no option label
@answer n=5 entity=logging-monitoring#4.1 v=no
why: the select list's missing log site and span lives in blitz-dom and blitz-paint, outside the chunk's files, and the read's span carries verb and cause alone under the scope's own rule
@answer n=6 entity=logging-monitoring#5.1 v=no
why: the MutationBuffer recorder in dioxus-native-dom is not a file the chunk reaches, since the read touches only the graph, snapshot and actionable readers
@answer n=7 entity=logging-monitoring#6.1 v=yes
why: the graph verb's silence record is the pattern the new read's command span and its host_log and mcp_log drives must match, because the read's ids, component names and state names are the content kept out of logs
@answer n=8 entity=logging-monitoring#7.1 v=no
why: the cold-agent pipe script is not among the chunk's files and its counted verb list is not named by the scope
