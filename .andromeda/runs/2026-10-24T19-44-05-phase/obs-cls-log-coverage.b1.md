# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · the closed-span line format and its §8 scrub are Not needed (escher-telemetry's line format); the chunk installs no sink and adds no span or field
log-coverage#2.1 · no · the cold-agent verdict fields are Not needed (the cold-agent pipe is neither run nor changed), and scripts/cold-agent.sh is not a reached file
log-coverage#3.1 · no · the apps/browser loader, toolbar and persistence logs print only in a running browser, which the chunk never starts, and no reached file belongs to apps/browser
log-coverage#4.1 · no · the examples' println! sites and the wasm, seven_guis_native and escher-session logs are not reached (Not needed: the chunk spawns no escher-session and runs no example binary), and the headless stand boot prints no log line (Needs)
log-coverage#5.1 · no · the escher-driver command and tool_call spans and their field sets are Not needed; the chunk adds no verb or span field, and stand_act_select drives Session::run with no sink installed
log-coverage#6.1 · no · blitz-dom's tracing sites, the focus line on the Tab rows included, print nothing in a leg that installs no subscriber (Needs), and debug_log_node's println! runs only when called, which the silence probe (reads 0) rules out; the chunk edits no engine file
log-coverage#6.2 · no · blitz-dom's link-href, link-click and attribute-carrying lines are not reached by a stand check (Not needed: URL and attribute log lines), and its IME, WOFF and cursor sites print nothing with no subscriber (Needs)
log-coverage#6.3 · no · the image-URL lines are Not needed (URL log lines), and the rest of blitz-dom's layout logs print nothing in a leg with no subscriber (Needs); the chunk edits no layout code
log-coverage#6.4 · no · blitz-dom's unsupported-property warnings print nothing in a leg with no subscriber (Needs), and print_tree's println! runs only when called, which the silence probe (reads 0) rules out; the chunk edits no node code
log-coverage#6.5 · no · blitz-net's cache, fetch and HTTP-status logs are Not needed (fetch-error logging), and nothing in the leg installs a subscriber to print them (Needs)
log-coverage#6.6 · no · blitz-html's parse-error logs fire on sink finish and print nothing in a leg with no subscriber (Needs); the chunk edits no parser
log-coverage#6.7 · no · blitz-shell's event-conversion errors and blitz-paint's warnings sit on the windowed shell and the paint path, which no stand a11y check runs
log-coverage#6.8 · no · blitz's launch log and blitz-vibey-script's script-error and console logs are not reached: the chunk launches no URL and runs no script (Not needed: the JS error surface)
log-coverage#6.9 · no · dioxus-native-dom's mutation trace lines run on the stand's render path but print nothing in a leg with no subscriber (Needs), and dioxus-native's debug logs are Not needed
log-coverage#6.10 · no · the two test files that print via println! are not stand a11y checks, and the phase-timing print is Not needed (frame and phase timings); the leg's checks print nothing (Needs)
log-coverage#6.11 · no · the wpt/runner logs are not reached: the chunk runs no WPT runner, and its panic hook and env_logger are Not needed
log-coverage#6.12 · no · the publish builds' CARGO_LOG info sits in publish-browser.yml, which is not a reached file and whose workflow the chunk does not change (Not needed)
log-coverage#7.1 · no · the console census over the 21 s02 files is a re-check every change owes: no reached file is named in it, and the chunk adds no console call
log-coverage#7.2 · no · the structured-logging census over the 32 examples-slice files is a re-check every change owes: no reached file lies in the examples slice, and the chunk adds no log call
log-coverage#7.3 · yes · the widened leg runs select_interaction and stand_act_select, which read the fixtures module whose many_items and ITEMS this census pins silent, so the chunk's new rows must keep it at 0 (Needs), and stand_requirement_sweep's leg check reads the reached ci-leg.sh through include_str!
