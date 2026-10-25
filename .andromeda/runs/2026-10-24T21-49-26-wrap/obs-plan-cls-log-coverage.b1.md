# Classified — obs-plan · log-coverage.b1

## Answers

log-coverage#1.1 · no · its closed-span line format is escher-telemetry's, and no file of that crate is reached or changed
log-coverage#2.1 · no · its verdict fields sit in scripts/cold-agent.sh, which the session neither reaches nor runs
log-coverage#3.1 · no · its apps/browser log sites sit in a crate the session does not reach
log-coverage#4.1 · no · its example output and escher-session endings sit in examples and escher-driver, neither reached, and the session adds no print
log-coverage#5.1 · no · its escher-driver span and emitter lines sit in packages/escher-driver, which the session leaves unchanged
log-coverage#6.1 · no · its blitz-dom log sites sit in packages/blitz-dom, which the session leaves unchanged
log-coverage#6.2 · no · its blitz-dom events and util log sites sit in packages/blitz-dom, which the session leaves unchanged
log-coverage#6.3 · no · its blitz-dom layout log sites sit in packages/blitz-dom, which the session leaves unchanged
log-coverage#6.4 · no · its blitz-dom node log sites sit in packages/blitz-dom, which the session leaves unchanged
log-coverage#6.5 · no · its blitz-net log sites sit in packages/blitz-net, which the session does not reach
log-coverage#6.6 · no · its blitz-html parse-error log sits in packages/blitz-html, which the session does not reach
log-coverage#6.7 · no · its blitz-shell and blitz-paint log sites sit in packages, which the session leaves unchanged
log-coverage#6.8 · no · its blitz and blitz-vibey-script log sites sit in packages, which the session leaves unchanged
log-coverage#6.9 · no · its dioxus-native and dioxus-native-dom log sites sit in packages, which the session leaves unchanged
log-coverage#6.10 · no · its two cited test files are outside the reached set, and the session adds no print to any test
log-coverage#6.11 · no · its wpt/runner log sites sit in wpt/runner, which the session does not reach
log-coverage#6.12 · no · its publish-build CARGO_LOG lines sit in publish-browser.yml, which the session does not reach, and ci.yml is byte-identical
log-coverage#7.1 · no · its console-call search covers the 21 s02 files, and the session's changed lines (the a11y arm's target names, the pin's words) carry no console call
log-coverage#7.2 · no · its structured-logging search covers the examples slice, which the session does not change
log-coverage#7.3 · yes · add-to · F1: the eight checks joining the a11y leg (accessibility_states through stand_act_select) install no subscriber, read no env var and print nothing, a silence census reading 0 over them, which joins this record's census
