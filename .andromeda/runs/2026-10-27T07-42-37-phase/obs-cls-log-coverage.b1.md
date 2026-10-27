# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the sink allowlist re-read in escher-telemetry's format.rs (reached) rules over this line's span rule: a span from a target outside the allowlist writes no byte, and every span pair passes the scrub under its target
log-coverage#2.1 · no · the cold-agent verdict fields; the task's Not needed names them, and the chunk runs no cold-agent pass or edits scripts/cold-agent.sh
log-coverage#3.1 · no · span_counts reads the cold-agent pipe's server.log; the task's Not needed excludes the pipe's server-log counting, and the chunk runs no cold-agent pass
log-coverage#3.2 · no · the cold-agent live server child's record directory; the task's Not needed excludes the pipe, and the chunk runs no cold-agent pass
log-coverage#3.3 · no · tasks_met, the cold-agent pipe's reading of snapshot answers; the task's Not needed excludes the pipe, and the chunk runs no cold-agent pass
log-coverage#4.1 · yes · apps/browser is reached whole, and the Needs re-read the browser app's logged events and its fmt::init subscriber where the take carries d63ed2c8c
log-coverage#5.1 · no · no reached file holds the example println!/eprintln! sites or wasm_hello's info line, and the Not needed excludes the WASM examples and the driver's command-line endings (escher-driver and seven_guis' host are not edited)
log-coverage#6.1 · yes · the Needs re-run host_log and mcp_log at trace under both builds (0 needles, tool_call and command span counts unchanged) and keep escher-driver's two info_span! sites and 0 events, the spans and counts this line states
log-coverage#7.1 · yes · the scrub limits it states, log.file on a bridged js_console record and the unscrubbed upstream apps, are what the Needs re-read (the js_console sites, d63ed2c8c); a moved limit is made in docs/capabilities.md by hand
log-coverage#8.1 · yes · the Needs name document.rs's resource-load warnings and mutator.rs's URL lines, and both files are reached (document.rs whole, mutator.rs a conflict surface)
log-coverage#8.2 · yes · events/pointer.rs is reached whole, and the Needs name its href and attribute lines (pointer.rs:901, :906) as kept behind the tracing feature through each resolution
log-coverage#8.3 · yes · the Needs name construct.rs's SVG parse-failure event carrying outer HTML (a conflict surface), and layout/mod.rs and damage.rs's image-fetch URL events in the same census
log-coverage#8.4 · yes · the Needs name node/element.rs's invalid-property-value warning, and node/node.rs (a conflict surface) holds print_tree's println!
log-coverage#8.5 · no · blitz-net's lib.rs is not in the reached files and no Needs bullet names it, so no take commit edits its fetch log sites
log-coverage#8.6 · no · blitz-html's html_sink.rs is not in the reached files and no Needs bullet names it
log-coverage#8.7 · yes · blitz-shell's convert_events.rs and window.rs are reached whole, and their tracing::error! sites sit under the optional tracing dependency and clipboard feature the Needs re-read (527394724)
log-coverage#8.8 · yes · runtime.rs (a conflict surface) holds the uncaught-JS-error site and the JS console bridge, which the Needs re-read after the module split (e6f4615ec)
log-coverage#8.9 · yes · the Needs name dioxus-native-dom's mutation_writer.rs debug sites (text-node contents, attribute values) and dioxus-native's asset log sites, re-read where 7fc9ee3af edits the bridge and 286874cc1 replaces upstream's side
log-coverage#8.10 · no · the take carries neither paint_tree_bench.rs nor br_trailing_line.rs, and no Needs bullet names the log-phase-times feature
log-coverage#8.11 · no · the take moves only the pinned WPT revision (WPT_COMMIT), not the runner's source, which the Not needed excludes
log-coverage#8.12 · yes · .github/workflows/publish-browser.yml is reached whole, and the Needs re-read its CARGO_LOG info with --verbose --trace where the take carries a7ea9631b and b5b163450
log-coverage#9.1 · yes · the take reaches blitz-vibey-script's JS runtime (runtime.rs and its js_console bridge) and tests/dom.rs, so a console. call the take adds there would break this absence
log-coverage#9.2 · no · the take edits no example; host_log.rs and mcp_log.rs are only re-read against their 0-needle check, which does not rule over tracing, log or env_logger use
log-coverage#9.3 · no · host_log.rs is the one named file the take reaches, re-read against its needles, which do not rule over the silence this line records; the other 0.2.0 test additions are not reached
log-coverage#9.4 · no · the take reaches none of the eight a11y-leg checks or their two shared modules, and the a11y leg's re-run is a behaviour check, not a silence census
