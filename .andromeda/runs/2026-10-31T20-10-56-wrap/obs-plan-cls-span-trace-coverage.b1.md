# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · examples test edits add no span or tracing token (added lines clean, silence census 0 over the popup checks)
span-trace-coverage#1.2 · no · the apps slice holds no reached file; apps/browser is outside the change
span-trace-coverage#1.3 · no · the change adds no span, and the added packages/*.rs lines hold no tracing token (gate entry 4)
span-trace-coverage#1.4 · no · listed s06 files are fixed and unreached by a span; no span added, added packages lines hold no tracing token
span-trace-coverage#1.5 · no · layout slice takes no new span; added lines hold no tracing token
span-trace-coverage#1.6 · no · listed files, only the doc comment matches; no span added and added lines hold no tracing token
span-trace-coverage#1.7 · no · listed s09 files; no span added, added lines clean
span-trace-coverage#1.8 · no · the blitz slice's added lines hold no span or instrument token, so the absence stands
span-trace-coverage#1.9 · no · listed s11 files; no span added, added lines clean
span-trace-coverage#1.10 · yes · stale · the three new test-crate files (select_popup, select_popup_painted, stand_act_popup) join the slice and move its 61 count, though none holds a tracing token
span-trace-coverage#1.11 · no · listed wpt runner files, outside the change; no span added
span-trace-coverage#2.1 · no · re-read and unmoved (two info_span sites, eight fields, causes 8, VERBS 12) though execute.rs was edited; cause returns unchanged
span-trace-coverage#2.2 · yes · stale · host_log.rs gains lines at 17-20, before the cited 137-280 range, moving the fourth test's citation by four
span-trace-coverage#3.1 · yes · stale · execute.rs was edited, so "the executor unchanged" no longer reads as written; the tool_call nesting and its two fields hold
span-trace-coverage#3.2 · no · tool_call's two fields are unchanged and none added; wrong_calls' definition does not move and scripts/cold-agent.sh is unreached
