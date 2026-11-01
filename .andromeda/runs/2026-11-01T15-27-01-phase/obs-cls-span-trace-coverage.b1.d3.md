# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the examples span census (32 files) holds the seven_guis host_log and mcp_log tests and the cells.rs conflict file, the baseline the delta read's new-span check runs against
span-trace-coverage#1.2 · no · the apps slice (86 files) is upstream-only apps code outside escher's sink and off the conflict list (Not needed: the upstream apps' logged events and stdout subscribers)
span-trace-coverage#1.3 · yes · the s05 span census (15 files) is the baseline the delta read compares each new span site against
span-trace-coverage#1.4 · yes · the s06 span census (17 listed files) is the baseline the delta read compares each new span site against
span-trace-coverage#1.5 · yes · the layout span census (8 files) covers the conflict files construct.rs, inline/build.rs and writing_mode.rs
span-trace-coverage#1.6 · yes · the 16-file span census is the baseline the delta read measures new spans against, its one match a node-id doc comment
span-trace-coverage#1.7 · yes · the s09 span census (32 listed files) is the baseline the delta read compares each new span site against
span-trace-coverage#1.8 · yes · the blitz-vibey-script and blitz span census covers the conflict files runtime/bootstrap.rs and runtime/events.rs, whose span absence the delta read re-measures
span-trace-coverage#1.9 · yes · the s11 span census (21 listed files) is the baseline the delta read compares each new span site against
span-trace-coverage#1.10 · yes · the test-crate census (61 files) covers tests/blitz-tests, the conflict file pseudo_element_flow_classification.rs and the stand_* re-reads
span-trace-coverage#1.11 · no · the WPT runner slice is upstream-only tooling outside escher's sink (Not needed: the WPT runner's statistics and report fields and the upstream-only workflows)
span-trace-coverage#2.1 · yes · the command span's eight fields and its eight refusal causes are the escher-driver span read-back the Needs name, witnessed by host_log and mcp_log
span-trace-coverage#2.2 · yes · the hosted part read: host_log's fourth test counts the closed command span lines and reads 0 id and 0 name needles, a standing capture check the chunk re-reads
span-trace-coverage#2.3 · yes · the select list's silence record (no span, field, event or log line added), witnessed by host_log's and mcp_log's select-list tests at 0 needles
span-trace-coverage#3.1 · yes · the tool_call span's two fields and its ending words are the escher-driver span read-back the Needs name, with stand_act_spans' children
span-trace-coverage#3.2 · yes · tool_call keeps its two fields, tool and ending, none added, which the read-back re-measures; the cold-agent pipe's verdict and server.log reading stay out of scope
