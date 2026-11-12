@row_add n=1 row=absence/telemetry-backend area=2-telemetry-strategy rec=1,2,3,4
capability: A telemetry backend or OTel SDK (Sentry, OpenTelemetry, Prometheus or a metrics exporter)
@row_add n=2 row=decision/sink-target-allowlist area=6-log-coverage rec=5,6,7,8,9,10,11
name: Sink target allowlist
rule: escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names.
why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting.
guard: telemetry_drop
except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
@row_add n=3 row=contract/agent-run-test-event/file area=6-log-coverage rec=12,13
boundary: The agent-run harness test event
item: file
promise: The target binary's stem with its hash stripped, or doc:{crate}.
@row_add n=4 row=contract/cold-agent-verdict/counts-agree area=6-log-coverage rec=14,15
boundary: The cold-agent verdict
item: counts_agree
promise: Ties the server's span lines to the transcript: the tool_call lines equal the escher tool uses, and its refused and error lines equal the escher results marked is_error.
@row_add n=5 row=contract/cold-agent-verdict/wrong-calls area=6-log-coverage rec=14,15,16
boundary: The cold-agent verdict
item: wrong_calls
promise: The sum of refused, error and foreign tool uses and permission denials, the refused and error counts read from the live server child's server.log, deciding the verdict against wrong_call_bar_pct.
@row_add n=6 row=contract/cold-agent-verdict/lint-calls area=6-log-coverage rec=14,16
boundary: The cold-agent verdict
item: lint_calls
promise: The count of accepted tool_call span lines of the lint verb in the live server child's server.log.
@row_add n=7 row=contract/cold-agent-verdict/screenshot-calls area=6-log-coverage rec=14,16
boundary: The cold-agent verdict
item: screenshot_calls
promise: The count of accepted tool_call span lines of the screenshot verb in the live server child's server.log.
@row_add n=8 row=contract/cold-agent-verdict/advance-calls area=6-log-coverage rec=14,16
boundary: The cold-agent verdict
item: advance_calls
promise: The count of accepted tool_call span lines of the advance verb in the live server child's server.log.
@row_add n=9 row=contract/cold-agent-verdict/wrong-call-bar-pct area=6-log-coverage rec=14,15
boundary: The cold-agent verdict
item: wrong_call_bar_pct
promise: The bar on wrong calls, 5; the reason wrong-calls-over-bar is added when wrong_calls × 100 > 5 × tool_calls.
@row_add n=10 row=absence/error-reporting-service area=7-error-capture-reporting rec=17,18,19,20,21,22,23
capability: An error-reporting client or error-capture service (such as Sentry)
@row_add n=11 row=contract/command-span/verb area=log-coverage rec=24,25
boundary: The driver's command span
item: verb
promise: The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
