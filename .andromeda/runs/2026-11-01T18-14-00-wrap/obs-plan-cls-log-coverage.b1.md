# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · escher-telemetry has no diff in this session, and the span line format, allowlist and scrub it describes are unchanged
log-coverage#2.1 · no · the verdict fields live in scripts/cold-agent.sh, which is not reached and is not among the changed files
log-coverage#3.1 · no · span_counts reads the cold-agent server log through scripts/cold-agent.sh, which the session does not reach
log-coverage#3.2 · no · the recording directory is set in scripts/cold-agent.sh, which the session does not reach
log-coverage#3.3 · no · tasks_met is scripts/cold-agent.sh logic, which the session does not reach
log-coverage#4.1 · no · apps/browser files are not reached and the session adds no browser log site or change to the loader's sites
log-coverage#5.1 · no · no example file is reached, seven_guis is unedited, and its log checks were re-run and pass
log-coverage#6.1 · yes · add-to · F1: the take's silence, its 2 info_span!, the eight-field command span and 0 host_log and mcp_log needles at trace under both builds, joins the escher-driver no-site bullet
log-coverage#7.1 · no · docs/capabilities.md and its pin are not reached, and the span set and scrub limits it restates are unmoved
log-coverage#8.1 · yes · add-to · F2: the engine changes the silence covers (NetHandler::error and the failed-stylesheet path, the blend module, init_stylo_prefs, the deallocate_anonymous_block fix, and the rest) join the no-site bullet; the document.rs, mutator.rs and net.rs citations in this entity moved too
log-coverage#8.2 · no · pointer.rs, ime.rs, util.rs and stylo_to_cursor_icon.rs are not reached and the session changes none of their sites
log-coverage#8.3 · yes · stale · construct.rs and damage.rs are reached and their cited lines (construct.rs:466-472, damage.rs:603-619) sit below the inserted lines, so the citations moved
log-coverage#8.4 · yes · stale · node/element.rs and node/node.rs are merge surfaces reached whole, so their cited lines (element.rs:807-822 and :873-876, node.rs:1023-1037) can have moved and the report re-reads none
log-coverage#8.5 · no · blitz-net lib.rs is a merge surface, but the cited lines (57-58 through 307-312) sit above the new handler.error call at :313 and read unchanged, so the fetch errors are still logged
log-coverage#8.6 · no · html_sink.rs is not reached and the session changes none of its sites
log-coverage#8.7 · yes · stale · render/background.rs is reached and its warn lines at :202-221 can have moved with the blend layer change; mask.rs and the blitz-shell files are not reached
log-coverage#8.8 · yes · add-to · F4: URLSearchParams, URL.prototype.searchParams and the native parse_search_params hold no log site and no carrier for what a page passes them, joining the vibey-script log list
log-coverage#8.9 · no · the cited sites are in mutation_writer.rs, assets.rs, link_handler.rs and dioxus_application.rs, none reached, and the reached dioxus_document.rs holds none of them
log-coverage#8.10 · no · the cited test files paint_tree_bench.rs and br_trailing_line.rs are not reached and still print; the reached new tests add no print
log-coverage#8.11 · no · the wpt runner's log files are not reached, and timeout-quarantine.txt holds no log call
log-coverage#8.12 · no · publish-browser.yml is not reached, and the ci.yml edit removes a free-disk step and sets no CARGO_LOG
log-coverage#9.1 · no · the session adds no console call, and the new URLSearchParams script holds no log site (F4)
log-coverage#9.2 · no · the examples slice is unedited and the session adds no tracing or log site
log-coverage#9.3 · yes · add-to · F3: key_press_after_container_lost_its_box (3 tests) and url_search_params (4 tests) add no env read, print or tracing site, joining the test-additions absence record
log-coverage#9.4 · no · host_log's option-label check sits in examples/seven_guis, which is unedited, and its two log checks were re-run and pass
