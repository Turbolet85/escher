# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the span side of the sink allowlist (a non-admitted span writes no byte, an admitted one passes the scrub under its target) is the format.rs re-read that host_log and mcp_log count
log-coverage#2.1 · no · the cold-agent verdict fields are Not needed, as the chunk runs no cold-agent pass
log-coverage#3.1 · no · the cold-agent server-log counting is Not needed, as the chunk runs no cold-agent pass
log-coverage#3.2 · no · the cold-agent live child's recording in target/cold-agent/record is Not needed, and escher-driver and the seven_guis host are not edited
log-coverage#3.3 · no · the cold-agent tasks_met reading is Not needed, as the chunk runs no cold-agent pass
log-coverage#4.1 · yes · apps/browser is reached whole, and the Needs re-read its logged events and upstream-only fmt::init where the take carries d63ed2c8c
log-coverage#5.1 · no · the examples' print sites and escher-session's endings are Not needed and outside the reach (session_host.rs and escher-driver's cli.rs are not edited)
log-coverage#6.1 · yes · escher-driver's command and tool_call span lines are the counts host_log and mcp_log re-run at trace under both builds, and its two info_span! sites must read unchanged
log-coverage#7.1 · yes · two scrub limits it records, the js_console log.file bridge and the browser's unscrubbed fmt::init, are re-read by the Needs, so the statement is checked by hand if either moves
log-coverage#8.1 · yes · document.rs (reached whole) and mutator.rs (conflict surface) keep their resource-load, URL and iframe lines behind the tracing feature through each resolution; resolve.rs, form.rs, net.rs and debug.rs are not reached
log-coverage#8.2 · yes · events/pointer.rs (reached whole) keeps its href and attribute lines at 901 and 906 behind the tracing feature, as the Needs name; ime.rs, util.rs and stylo_to_cursor_icon.rs are not reached
log-coverage#8.3 · yes · construct.rs (conflict surface, its SVG parse-failure event carrying outer HTML), layout/mod.rs and damage.rs (named in the Needs' layout census) keep their feature gate through each resolution
log-coverage#8.4 · yes · element.rs (reached whole, its invalid-property-value warning named in the Needs) and node.rs (conflict surface, print_tree's println!) bind under the feature-gate and no-println rule
log-coverage#8.5 · no · blitz-net's sources are not in the reach and no Need names them
log-coverage#8.6 · no · blitz-html's html_sink.rs is not in the reach and no Need names it
log-coverage#8.7 · yes · convert_events.rs and window.rs (reached whole) bind with the clipboard-failure site and blitz-shell's optional tracing dependency that the Needs name (527394724); blitz-paint's warn! lines are not reached
log-coverage#8.8 · yes · runtime.rs (conflict surface) holds the uncaught-JS-error site and the JS console bridge at 1104-1105 and 1253-1269, both named by the Needs; blitz/src/lib.rs and vibey-script's document.rs are not reached
log-coverage#8.9 · yes · dioxus-native-dom's mutation_writer.rs trace! lines and dioxus-native's asset log sites are named by the Needs, re-read where 7fc9ee3af edits the bridge and where a take of 286874cc1 replaces upstream's side
log-coverage#8.10 · no · its println!/eprintln! test sites are in paint_tree_bench.rs and br_trailing_line.rs, neither in the reach, and the take's own test targets are checked under S1
log-coverage#8.11 · no · the WPT runner's source is not in the reach (the take moves only WPT_COMMIT) and its env_logger, panic hook and report messages are Not needed
log-coverage#8.12 · yes · publish-browser.yml is reached whole, and the Needs re-read its CARGO_LOG info and --verbose --trace where the take carries a7ea9631b and b5b163450
log-coverage#9.1 · no · the console-call absence covers 21 s02 files that the reach names none of, the take's JS console bridge is log-coverage#8.8's, and the WASM console hook is Not needed
log-coverage#9.2 · no · on the two reached examples tests the chunk re-reads only the needle check, so this absence is a re-check every change owes, and the other examples are not reached
log-coverage#9.3 · no · standing S1 · test targets install no subscriber, read no env var and print nothing, confirmed by the silence census per file
log-coverage#9.4 · no · standing S1 · test targets install no subscriber, read no env var and print nothing, confirmed by the silence census per file
