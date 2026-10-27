# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · escher-telemetry is byte-unchanged at the take per the sink re-check, so the closed-span line format it cites holds
log-coverage#2.1 · no · the verdict fields it lists sit in scripts/cold-agent.sh, which the take did not touch
log-coverage#3.1 · no · span_counts sits in scripts/cold-agent.sh, which the take did not touch
log-coverage#3.2 · no · the live child's recording directory and kept checks sit in scripts/cold-agent.sh, which the take did not touch
log-coverage#3.3 · no · tasks_met and the replay's expect match sit in scripts/cold-agent.sh, which the take did not touch
log-coverage#4.1 · no · its cited apps/browser files (document_loader.rs, toolbar.rs, persistence/src/lib.rs, browser_history.rs) are unreached and the take adds no log site
log-coverage#5.1 · no · its cited example files and the seven_guis escher-session lines are unreached and the seven_guis package is unchanged
log-coverage#6.1 · yes · add-to · F2: the take leaves escher-driver byte-unchanged and host_log and mcp_log green under both builds, a re-measure this emitter record now carries
log-coverage#7.1 · no · the capability statement's limits still hold, since the bridged js_console record keeps its target and print rules (only its file moved) and the upstream apps' fmt::init is unmoved at main.rs:73-74
log-coverage#8.1 · yes · stale · blitz-dom mutator.rs, a conflict surface, gained 8 lines at 368-371 and 520-523 that move its cited sites at :1337-1357, :1379-1383 and :1406-1407
log-coverage#8.2 · no · its cited events/ime.rs, events/pointer.rs, util.rs and stylo_to_cursor_icon.rs are unreached and the IME-on-construct change adds no log site
log-coverage#8.3 · yes · stale · layout/construct.rs, a merge surface, gained 5 lines at 313-317 that move its cited SVG parse-failed warning at :461-467
log-coverage#8.4 · yes · stale · node/element.rs, a merge surface, gained 10 lines at 488-489 and 543-550 that move its cited sites at :807-822 and :873-876
log-coverage#8.5 · no · blitz-net's cited lib.rs is unreached and the take adds no log site
log-coverage#8.6 · no · blitz-html's cited html_sink.rs is unreached and the take adds no log site
log-coverage#8.7 · yes · stale · blitz-shell window.rs, a merge surface, gained 2 lines at 193-194 that move its cited site at :850-854
log-coverage#8.8 · yes · stale · blitz-vibey-script document.rs gained 11 lines above its cited :264-267, and the JS console mapping and the uncaught-JS log moved out of runtime.rs (151 lines now) into the runtime/ modules, report_js_error at runtime/events.rs:21
log-coverage#8.9 · no · its cited mutation_writer.rs, assets.rs, link_handler.rs and dioxus_application.rs are unreached and the dioxus_document.rs change is an added test that adds no log site
log-coverage#8.10 · no · its cited paint_tree_bench.rs and br_trailing_line.rs are unreached and the new pseudo_element_flow_classification target prints nothing
log-coverage#8.11 · no · wpt/runner's cited files are unreached and WPT_COMMIT pins the suite commit without holding a log site
log-coverage#8.12 · yes · stale · publish-browser.yml, a merge surface, gained 15 lines at 126 and 153-166 that move its cited :157, while :33 and :37 stand
log-coverage#9.1 · yes · stale · the take grew the blitz-vibey-script slice by 13 Rust modules and 9 JS bootstrap files, merge surfaces, so the 21-file count this console search names is stale until re-read
log-coverage#9.2 · no · the examples slice's 32 files are unreached and the seven_guis package is unchanged
log-coverage#9.3 · no · its 0.2.0 test-addition files are unreached and the new pseudo_element_flow_classification target sits outside that record
log-coverage#9.4 · no · the a11y leg's eight checks and their two shared modules are unreached, so the census over those ten files holds
