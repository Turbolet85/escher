# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the closed-span line format lives in packages/escher-telemetry, which the session did not reach, and no span or field moved
log-coverage#2.1 · yes · add-to · the verdict gains seven fields and three reasons (F3, F4), span_counts' server.log counts (F5), a kept check the pipe never prints (F6) and tasks_met's paged join (F7)
log-coverage#3.1 · no · apps/browser is not reached and the session changes no browser logging
log-coverage#4.1 · yes · stale · session_host.rs gained lines above main, so its citation session_host.rs:31 moved
log-coverage#5.1 · yes · stale · host_log.rs shifted after line 134 with its fourth test, so the citation host_log.rs:398-411 moved, and host_log now drives a second part read at trace
log-coverage#6.1 · no · docs/capabilities.md is not reached and the emitter reach and scrub limits it restates are unchanged
log-coverage#7.1 · no · blitz-dom's log sites live in packages/blitz-dom, which the session did not reach
log-coverage#7.2 · no · blitz-dom's events and util log sites live in packages/blitz-dom, which the session did not reach
log-coverage#7.3 · no · blitz-dom's layout log sites live in packages/blitz-dom, which the session did not reach
log-coverage#7.4 · no · blitz-dom's node log sites and print_tree live in packages/blitz-dom, which the session did not reach
log-coverage#7.5 · no · blitz-net's log sites live in packages/blitz-net, which the session did not reach, and the stand's LoadNet uses no blitz_net
log-coverage#7.6 · no · blitz-html's parse-error log lives in packages/blitz-html, which the session did not reach
log-coverage#7.7 · no · blitz-shell and blitz-paint log sites live in unreached packages and the headless stand adds none
log-coverage#7.8 · no · blitz and blitz-vibey-script log sites live in unreached packages and the session adds no JavaScript
log-coverage#7.9 · no · dioxus-native and dioxus-native-dom log sites live in unreached packages and the session adds no DOM log site
log-coverage#7.10 · no · its two cited test files, br_trailing_line.rs and paint_tree_bench.rs, are not reached and no feature or manifest changed
log-coverage#7.11 · no · wpt/runner is not reached and the session touches no WPT code
log-coverage#7.12 · no · publish-browser.yml is not reached and no CI step changed
log-coverage#8.1 · no · its console. census is over JavaScript files, no reached file is JavaScript and the session adds no console call
log-coverage#8.2 · yes · add-to · the examples slice gained the load page's LoadNet in stand.rs (F2), which has no print, env read, socket or listener and reads 0 in the census
log-coverage#8.3 · yes · add-to · cells_needles in the seven_guis tests shared module joins these test additions (F8), its messages naming no needle
log-coverage#8.4 · yes · stale · tests/blitz-tests/tests/common/mod.rs is a reached file, so the silence census over it must be re-read on the changed file
