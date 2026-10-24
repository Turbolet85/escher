# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · packages/escher-telemetry is byte-unchanged and the session adds no span, so the closed-span line format and its allowlist stand
log-coverage#2.1 · no · the cold-agent scripts are unchanged and no escher-driver tool_call span changed, so the verdict fields stand
log-coverage#3.1 · yes · stale · document_loader.rs gains lines at 3 and 62 and toolbar.rs at 3 and 203, so :120, :151, :235, :241-243 and toolbar.rs:130 move; the log levels stand
log-coverage#4.1 · yes · add-to · the new root example examples/multi_doc.rs prints its timings with one println! on stdout, so it joins the examples that use println!/eprintln!
log-coverage#5.1 · no · escher-driver is byte-unchanged and seven_guis_native runs none of its spans, so the spans, fields and cited measurements stand
log-coverage#6.1 · yes · stale · resolve.rs:45 moves to 47 (lines added at 29, 32) and mutator.rs:1321-1341, :1363-1367, :1390-1391 move by 8 (lines added at 375-378, 499-502); the log sites stand
log-coverage#6.2 · no · the session does not reach events/*.rs, util.rs or stylo_to_cursor_icon.rs, so their cited sites and lines stand
log-coverage#6.3 · yes · stale · construct.rs's SVG parse-failure warn moves from :480-486 to :483-489 (lines added at 6, 23, 29); its event and fields stand
log-coverage#6.4 · yes · stale · 17 lines added to element.rs (45-48, 56-61, 313, 422-426, 431) move both property warns, :780-795 and :846-849, down by 17; the warns stand
log-coverage#6.5 · no · nothing in the ten commits reaches blitz-net, so the cache, fetch and error sites stand
log-coverage#6.6 · no · blitz-html is outside the session's changes, so html_sink.rs:181-186 stands
log-coverage#6.7 · no · blitz-shell and blitz-paint are outside the session's changes, so the convert_events, window, background and mask sites stand
log-coverage#6.8 · no · packages/blitz and blitz-vibey-script are outside the session's changes, so the launch and script log sites stand
log-coverage#6.9 · no · dioxus-native-dom is unchanged and dioxus-native changed only in src/lib.rs, which none of these sites is in
log-coverage#6.10 · no · the new target hit_test_dropped_nodes prints nothing and the cited test files are unchanged, so the println! and phase-timing citations stand
log-coverage#6.11 · yes · stale · the #[global_allocator] lines at main.rs 1-3 move main.rs:470-475 and :295 down by 3; the runner's logging is unchanged
log-coverage#6.12 · no · no CI file changed and mimalloc is an allocator, not a log change, so the CARGO_LOG citations at publish-browser.yml:33, :37, :153 stand
log-coverage#7.1 · no · the session adds no console call, so the absence over the s02 files still holds
log-coverage#7.2 · yes · stale · the new root example examples/multi_doc.rs joins the examples slice (32 to 33 files) and holds no tracing::, log:: or env_logger token, so the absence stands
