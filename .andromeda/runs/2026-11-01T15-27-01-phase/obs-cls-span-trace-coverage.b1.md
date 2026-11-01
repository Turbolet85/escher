# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the examples slice holds the conflict surface examples/seven_guis/src/tasks/cells.rs, whose resolution could add a span the absence forbids; the host_log and mcp_log re-reads sit in that slice too
span-trace-coverage#1.2 · no · the apps slice holds no reached file, and the upstream apps' logged events are Not needed
span-trace-coverage#1.3 · no · the 15 s05 files hold no reached file or need the task names
span-trace-coverage#1.4 · no · the 17 listed s06 files hold no reached file or need the task names
span-trace-coverage#1.5 · yes · the layout slice holds the conflict surfaces layout/construct.rs, layout/inline/build.rs and layout/writing_mode.rs, where a merge could add a span the absence forbids; layout/damage.rs is named in the needs
span-trace-coverage#1.6 · no · the 16 listed files hold no reached file or need the task names; the one match is a doc comment, not a span
span-trace-coverage#1.7 · no · the 32 listed s09 files hold no reached file or need the task names
span-trace-coverage#1.8 · yes · the blitz-vibey-script runtime and element trees hold the conflict surfaces runtime/bootstrap.rs and runtime/events.rs, which the chunk resolves and whose script error surface the needs name
span-trace-coverage#1.9 · no · the 21 listed s11 files hold no reached file or need the task names
span-trace-coverage#1.10 · yes · the take carries upstream's test target pseudo_element_flow_classification (a conflict surface) and edits select_popup.rs; these added instances must be recorded in the test-crate silence census as installing no tracing or log subscriber
span-trace-coverage#1.11 · no · the 12 listed WPT runner files hold no reached file, and WPT runner statistics are Not needed
span-trace-coverage#2.1 · yes · the needs name the escher-driver span surface as a read-back of the take: 2 info_span! sites, command's eight fields and stand_act_spans' children; host_log counts those command lines
span-trace-coverage#2.2 · yes · host_log's fourth test, a re-read standing capture check, counts the hosted command span lines and reads 0 id and 0 name needles under both builds, which the take must keep
span-trace-coverage#2.3 · yes · the needs name the select list's silence record (node::select_list, render::select_list, stroke_text_as); host_log's and mcp_log's select-list tests at trace are re-read at 0 needles, and select_popup.rs is edited
span-trace-coverage#3.1 · yes · the needs name tool_call's two fields, tool and ending, and its span shape; the take must keep them
span-trace-coverage#3.2 · yes · the span keeps its two fields, tool and ending, with none added, which the needs name as tool_call's two; the cold-agent pipe's counting is Not needed, but the field rule binds
