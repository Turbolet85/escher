# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the closed-span line class and its pair-by-pair scrub under the span's target are the sink rule that host_log and mcp_log measure at trace
log-coverage#2.1 · no · the cold-agent verdict fields belong to v020-11, a later entry out of this chunk, and scripts/cold-agent.sh is not reached
log-coverage#3.1 · no · apps/browser's own logging is on the Not-needed list, the sweep reaching only seven_guis and the blitz-tests checks
log-coverage#4.1 · yes · escher-session's own stderr endings (its one eprintln, the fixed usage, session-error and recording messages, and the empty stderr of an accepted run with RUST_LOG unset) are what host_log and mcp_log read; wasm_hello and the examples' println! output are Not-needed
log-coverage#5.1 · yes · the command span's verb and cause and the tool_call span's tool and ending, with the lint, screenshot and part-read lines and one tool_call line per MCP call, are what the sweep's log checks assert
log-coverage#6.1 · no · blitz-dom's per-site emitters and their as-is values (resource URLs, debug_log_node) are Not-needed and judged only through the sink's allowlist, and document.rs, resolve.rs, mutator.rs and net.rs are not reached
log-coverage#6.2 · no · blitz-dom's events and util emitters (the IME node_id, the link href and attribute dumps) are Not-needed inventory judged only through the sink's allowlist, and events/ and util.rs are not reached
log-coverage#6.3 · no · blitz-dom's layout emitters (the SVG parse html field, image URLs) are Not-needed inventory judged only through the sink's allowlist, and no layout file is reached
log-coverage#6.4 · no · the node property warnings and Node::print_tree's stdout print are an engine inventory on the Not-needed list, the sweep adds no print, and node.rs is not reached
log-coverage#6.5 · no · blitz-net's cache-dir and fetch logs with URL fields are Not-needed inventory judged only through the sink's allowlist, and blitz-net is not reached
log-coverage#6.6 · no · blitz-html's collected parse-error logs are Not-needed inventory judged only through the sink's allowlist, and html_sink.rs is not reached
log-coverage#6.7 · no · blitz-shell's windowed event-conversion errors sit outside the headless sweep and blitz-paint's layer warnings are Not-needed inventory, and neither file is reached
log-coverage#6.8 · no · blitz's launch_url info and blitz-vibey-script's console and JS-error logs are Not-needed (no 0.2.0 requirement runs script), and neither crate is reached
log-coverage#6.9 · no · dioxus-native and dioxus-native-dom's debug and per-mutation trace logs are Not-needed inventory (the sweep adds no engine log site), judged only through the sink's allowlist, and no mutation or asset file is reached
log-coverage#6.10 · no · tests/blitz-tests' println! diagnostics and the log-phase-times feature are metric coverage or existing checks outside the sweep, the silence rule covering only new or extended checks
log-coverage#6.11 · no · the WPT runner's logging is Not-needed, since no WPT target is a 0.2.0 witness
log-coverage#6.12 · no · upstream-only publish builds at CARGO_LOG info are Not-needed, publish-browser.yml is not reached, and the chunk touches no upstream-only CI
log-coverage#7.1 · no · the console-call absence over the 21 s02 files is not reached, since the chunk reaches no JavaScript file and adds no console call
log-coverage#7.2 · no · a re-check every change owes: the sweep adds no tracing, log or env_logger call to the examples slice, and seven_guis_native's sink is Not-needed
