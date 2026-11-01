# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the closed-span line format lives in escher-telemetry and escher-driver, neither reached, and the take adds no span, field or event site
log-coverage#2.1 · no · the cold-agent verdict fields in scripts/cold-agent.sh are not reached, and the cold-agent scripts are not among the changed files
log-coverage#3.1 · no · span_counts in scripts/cold-agent.sh is not reached
log-coverage#3.2 · no · the recording directory in scripts/cold-agent.sh is not reached
log-coverage#3.3 · no · tasks_met in scripts/cold-agent.sh is not reached
log-coverage#4.1 · no · apps/browser logs and their subscribers are not reached
log-coverage#5.1 · no · example println/eprintln sites and the escher-session endings are unchanged, examples/seven_guis is untouched and no example is reached
log-coverage#6.1 · yes · add-to · F1: the take's re-measure at 45c333f16 (2 info_span!, command span at eight fields, host_log and mcp_log at 0 needles at trace under both builds) joins this emitter record, with escher-driver itself unchanged
log-coverage#7.1 · no · docs/capabilities.md is not reached, and the span set, the scrub limits and the default-warn rule are unchanged by the session
log-coverage#8.1 · yes · stale · document.rs and mutator.rs are merge surfaces whose added lines moved the cited sites (document.rs :1829-1840 and :2269-2270, mutator.rs :1449-1469 and :1491-1495), and F2's engine changes join the no-site-added silence list
log-coverage#8.2 · no · events/ime.rs, events/pointer.rs, util.rs and stylo_to_cursor_icon.rs are not reached, so their claims stand
log-coverage#8.3 · yes · stale · construct.rs and damage.rs are merge surfaces whose added lines moved the SVG parse-failed warn and the image-fetch info sites (construct.rs :466-472, damage.rs :603-619)
log-coverage#8.4 · yes · stale · element.rs and node.rs are merge surfaces, and the warn sites at element.rs :807-822 and :873-876 and the print_tree site at node.rs :1023-1037 are not shown to hold on the changed files
log-coverage#8.5 · yes · add-to · F2: blitz-net's error arm now calls NetHandler::error at :313, so a failed stylesheet also reaches the handler, and the Error fetching line at :307-312 stands
log-coverage#8.6 · no · blitz-html html_sink.rs is not reached
log-coverage#8.7 · yes · stale · render/background.rs is reached though not a merge surface, and its warn at :202-221 is not shown to hold on the changed file, while the other cited files stand
log-coverage#8.8 · yes · add-to · F4: URLSearchParams, URL.prototype.searchParams and the native parse_search_params hold no log site and no carrier, joining this vibey-script log record
log-coverage#8.9 · no · mutation_writer.rs and the dioxus-native log sites are not reached
log-coverage#8.10 · no · the cited paint_tree_bench.rs and br_trailing_line.rs are not reached
log-coverage#8.11 · no · the wpt/runner sources are not reached, and timeout-quarantine.txt holds no log site
log-coverage#8.12 · no · publish-browser.yml is not reached
log-coverage#9.1 · no · the new url_search_params script and the bootstrap hold no console call (F4), so the console absence holds on the changed files
log-coverage#9.2 · no · the examples slice is untouched, so the structured-logging absence stands
log-coverage#9.3 · yes · add-to · F3: the key_press_after_container_lost_its_box and url_search_params test targets add no env read, print or tracing site, joining this test-additions absence record
log-coverage#9.4 · no · host_log.rs is not reached and the session adds no log site that could carry a select option's label
