# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the span line format and its capture are read from escher-telemetry, outside the change, and no span or field was added
log-coverage#2.1 · no · the verdict field list and the definition of wrong_calls hold; the session can move only the value, which this row does not state
log-coverage#3.1 · no · cites scripts/cold-agent.sh, outside the change; no tool_call line changed
log-coverage#3.2 · no · cites scripts/cold-agent.sh, outside the change; the recording directory is unchanged
log-coverage#3.3 · no · cites scripts/cold-agent.sh, outside the change; the snapshot answer reading is unchanged
log-coverage#4.1 · no · apps/browser is outside the change and every cited line is in it
log-coverage#5.1 · no · the cited session_host.rs and cli.rs are outside the change; the endings, VERBS and the six task names are unmoved
log-coverage#6.1 · yes · stale · host_log.rs:514-527 and mcp_log.rs:190-203 move under the added test lines (host_log 17-20 and 282-394, mcp_log 12-15 and 68-114); the span sites, fields and VERBS 12 hold; F3, F4, F5 and F8 join its needle and fixed-text records
log-coverage#7.1 · no · docs/capabilities.md is reached, but the report shows its span set, fields, stderr-only reach and scrub limits unchanged (no span or field added, escher-telemetry untouched)
log-coverage#8.1 · yes · stale · document.rs:1807-1818, :1917-1921 and :2237-2238 move under the 340 added document.rs lines, while resolve.rs:45 and document.rs:2401 hold; F7 joins the select-readers row and F9 leaves the focussed-node count unsettled
log-coverage#8.2 · yes · stale · pointer.rs:1025 and :1030 move under the 131 added pointer.rs lines
log-coverage#8.3 · no · the cited layout files are outside the change
log-coverage#8.4 · yes · add-to · F2: blitz-dom's new node::select_list holds no tracing or log token; the cited element.rs and node.rs lines hold
log-coverage#8.5 · no · blitz-net is outside the change
log-coverage#8.6 · no · blitz-html is outside the change
log-coverage#8.7 · yes · add-to · F2: blitz-paint's new render::select_list holds no warn or tracing token; the cited background.rs and mask.rs lines hold
log-coverage#8.8 · no · blitz and blitz-vibey-script are outside the change
log-coverage#8.9 · no · the cited dioxus-native files are outside the change, and is_select is a reader, not a mutation log
log-coverage#8.10 · no · the cited test files are outside the change, and the new popup tests print nothing
log-coverage#8.11 · no · wpt/runner is outside the change
log-coverage#8.12 · no · the publish workflows are outside the change
log-coverage#9.1 · no · the session adds no print or console call, and its census over the edited and new checks reads 0
log-coverage#9.2 · no · no tracing, log or env_logger token is added and the examples slice gains no file
log-coverage#9.3 · yes · stale · its "packages/ unchanged" basis no longer holds (blitz-dom and blitz-paint gained select_list modules) and ci-leg.sh's named words moved; the silence census still reads 0, and F1 joins it with the popup checks
log-coverage#9.4 · yes · stale · its "packages/ unchanged" basis no longer holds (blitz-dom and blitz-paint gained select_list modules); the silence census over its eight checks still reads 0, with select_interaction and stand_act_select edited
