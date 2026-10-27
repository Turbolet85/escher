# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · the closed-span line format, target allowlist and scrub of escher-telemetry's sink, which the chunk does not change (the task's Not needed names the line format)
log-coverage#2.1 · no · the cold-agent pipe's verdict fields, a consumer the chunk does not touch (Not needed: the cold-agent pipe)
log-coverage#3.1 · no · the pipe's reading of tool_call lines by span name; the chunk neither renames nor adds a field to them and the pipe is not touched
log-coverage#3.2 · no · the cold-agent pipe's live child, its recording directory and cleanup, untouched by the chunk (Not needed: session recording's kept checks and the pipe)
log-coverage#3.3 · no · the pipe's tasks_met and its paged-answer decoding, untouched by the chunk (Not needed: the cold-agent pipe)
log-coverage#4.1 · no · apps/browser's log sites, outside the chunk's reached files
log-coverage#5.1 · no · the examples' prints and escher-session's command-line endings, which the chunk does not edit; its lint path's silence and answer bytes are held by the task's Needs, not by these endings
log-coverage#6.1 · yes · the lint verb's command span records verb and, when refused, cause alone and holds no finding, id, bound or count; the factoring must add no span field (the task's Needs)
log-coverage#7.1 · no · docs/capabilities.md's statement of the span reach and scrub limits, which the chunk does not change; its pin reads none of these claims
log-coverage#8.1 · no · blitz-dom's document, resolve, mutator and net log sites, document.rs included (Not needed: the engine's log sites and println! dumps)
log-coverage#8.2 · no · blitz-dom's event and util log sites, outside the chunk's reached files
log-coverage#8.3 · no · blitz-dom's layout log sites, outside the chunk's reached files
log-coverage#8.4 · no · blitz-dom's node log sites and Node::print_tree, outside the chunk's reached files
log-coverage#8.5 · no · blitz-net's log sites, outside the chunk's reached files
log-coverage#8.6 · no · blitz-html's parse-error log, outside the chunk's reached files
log-coverage#8.7 · no · blitz-shell's and blitz-paint's log sites, outside the chunk's reached files
log-coverage#8.8 · no · the blitz launch and blitz-vibey-script logs, outside the chunk's reached files
log-coverage#8.9 · no · dioxus-native and dioxus-native-dom's mutation and asset logs, outside the chunk's reached files, and no subscriber runs in the touched or re-run tests
log-coverage#8.10 · no · the println! and eprintln! diagnostics and the log-phase-times feature sit in br_trailing_line.rs and paint_tree_bench.rs, which the chunk does not touch
log-coverage#8.11 · no · wpt/runner's log calls, env_logger and panic hook, outside the chunk's reached files (Not needed: the WPT runner)
log-coverage#8.12 · no · publish-build logging in publish-browser.yml, a CI job the chunk does not change (Not needed: CI publish builds)
log-coverage#9.1 · no · the console. census over the 21 s02 files, a pattern and file set the chunk neither reaches nor re-measures
log-coverage#9.2 · no · the examples slice's tracing and log census over files the chunk does not reach (the chunk's own census covers only its touched files)
log-coverage#9.3 · yes · the silence census of the 0.2.0 requirement test additions the chunk rewrites or re-runs: stand_requirement_sweep's WITNESSES_020 and its ci-leg.sh include_str! leg check, and stand_lint's reading_page and seventh test
log-coverage#9.4 · no · the a11y leg's eight checks and the common and fixtures modules, none in the chunk's reached files (the chunk's census covers only its touched files)
