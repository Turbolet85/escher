@row_none n=1 rec=1 type=7
@row_none n=2 rec=2 type=7
@row_none n=3 rec=3 type=7
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=5
@row_none n=9 rec=9 type=5
@row_none n=10 rec=10 type=5
@row_none n=11 rec=11 type=5
@row_none n=12 rec=12 type=5
@row_none n=13 rec=13 type=5
@row_none n=14 rec=14 type=5
@row_none n=15 rec=15 type=5
@row_none n=16 rec=16 type=5
@row_add n=1 row=decision/wpt-scores-upstream-only area=metric-coverage rec=17
name: WPT scores published from upstream only
rule: WPT scores are computed and published to Pages only on the upstream DioxusLabs/blitz repository, the wpt job being repository-guarded.
@row_add n=2 row=absence/metrics-exporters area=metric-coverage rec=18,24
capability: Metrics exporters or telemetry backends (OpenTelemetry, Prometheus, Sentry)
@row_add n=3 row=absence/metric-emission area=metric-coverage rec=19,20,21,22,23
capability: Metric emission: counters, gauges or histograms
@row_add n=4 row=decision/session-logs-carry-no-page-content area=pii-scrubbing-compliance rec=25
name: Session logs carry no page content
rule: The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build.
guard: host_log, mcp_log
@row_add n=5 row=decision/shot-image-leaves-only-in-answer area=pii-scrubbing-compliance rec=25
name: Shot image leaves only in its answer
rule: A shot's image is screen content that leaves only in the command's answer, never on a log line.
guard: host_log, mcp_log
@row_add n=6 row=decision/open-list-shot-carries-labels area=pii-scrubbing-compliance rec=25
name: Open list shot carries labels as pixels
rule: A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved.
why: a boundary widening, ratified
@row_add n=7 row=contract/graph-answer/content area=pii-scrubbing-compliance rec=26
boundary: the UI graph answer (graph)
item: content
promise: Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified.
@row_add n=8 row=contract/actions-answer/shape area=pii-scrubbing-compliance rec=27
boundary: the actions answer (actions)
item: shape
promise: Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
@row_add n=9 row=contract/actions-answer/owner area=pii-scrubbing-compliance rec=27
boundary: the actions answer (actions)
item: owner
promise: Left out when the node has no owner.
@row_add n=10 row=contract/actions-answer/cause area=pii-scrubbing-compliance rec=27
boundary: the actions answer (actions)
item: cause
promise: A closed cause word, left out when absent.
@row_add n=11 row=contract/actions-answer/content area=pii-scrubbing-compliance rec=27
boundary: the actions answer (actions)
item: content
promise: Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
@row_add n=12 row=contract/actions-answer/logging area=pii-scrubbing-compliance rec=27
boundary: the actions answer (actions)
item: logging
promise: The answer is kept whole in a recorded check and is never logged or fielded.
@row_add n=13 row=contract/coverage-line/shape area=pii-scrubbing-compliance rec=28
boundary: the coverage line (escher-session coverage)
item: shape
promise: Written as {"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}.
@row_add n=14 row=contract/coverage-line/owner area=pii-scrubbing-compliance rec=28
boundary: the coverage line (escher-session coverage)
item: owner
promise: Left out when the handler's node has no owner.
@row_add n=15 row=contract/coverage-line/content area=pii-scrubbing-compliance rec=28
boundary: the coverage line (escher-session coverage)
item: content
promise: Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a boundary widening, ratified.
@row_add n=16 row=contract/coverage-line/logging area=pii-scrubbing-compliance rec=28
boundary: the coverage line (escher-session coverage)
item: logging
promise: The line is a command answer, never logged or fielded.
@row_add n=17 row=contract/multiple-select-form-entries/chosen-option area=pii-scrubbing-compliance rec=29
boundary: multiple select form entries
item: chosen option
promise: On input and change, one (name, FormValue::Text(value)) entry per chosen option.
@row_add n=18 row=contract/snapshot/multiple-select-value area=pii-scrubbing-compliance rec=29
boundary: the snapshot
item: value of a multiple select
promise: The first chosen option in tree order.
@row_add n=19 row=decision/select-chosen-set-never-logged area=pii-scrubbing-compliance rec=29
name: Select chosen set never logged
rule: A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field.
@row_add n=20 row=decision/select-size-attribute-read area=pii-scrubbing-compliance rec=30
name: Select size attribute read
rule: The size attribute on a multiple select is read as a whole number capped at 65534, carrying no text out and moving no redaction shape, for every document.
why: an engine change for every document and a boundary widening, ratified
@row_add n=21 row=decision/optgroup-label-crossing area=pii-scrubbing-compliance rec=31
name: Optgroup label as group name and painted header
rule: An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field.
why: inside the ratified name crossing; a boundary widening, ratified
@row_add n=22 row=contract/snapshot-diff/open-list-row-bounds area=pii-scrubbing-compliance rec=32
boundary: the snapshot and the diff
item: open list row bounds
promise: An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.
@row_add n=23 row=contract/snapshot-node/clickable area=pii-scrubbing-compliance rec=33
boundary: the snapshot node
item: clickable
promise: A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
@row_add n=24 row=contract/snapshot-text/clickable area=pii-scrubbing-compliance rec=33
boundary: the snapshot text
item: clickable
promise: The token clickable stands after focused.
@row_add n=25 row=decision/select-list-label-never-logged area=pii-scrubbing-compliance rec=34
name: Select list row label never logged
rule: A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
@row_add n=26 row=contract/page-events/checkbox-change area=pii-scrubbing-compliance rec=35
boundary: page script events
item: change on a checkbox
promise: The engine always sends change after input with the same value, carrying the value class input carries.
@row_add n=27 row=contract/page-events/radio-change area=pii-scrubbing-compliance rec=35
boundary: page script events
item: change on a radio
promise: The engine sends change after input with the same value only where the radio was not checked before the click.
@row_add n=28 row=contract/page-events/single-change area=pii-scrubbing-compliance rec=35
boundary: page script events
item: change count
promise: A page's script hears one change, from the engine, the script runtime synthesising none.
@row_none n=17 rec=36 type=4
relied: Page scripts never see the native search-params parser, which is deleted from globalThis before any page script runs.
@row_add n=29 row=decision/runtime-probe-leaks-no-read-value area=pii-scrubbing-compliance rec=37
name: Runtime probe leaks no read value
rule: The dioxus runtime probe lets no read value leave it: its failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id.
@row_add n=30 row=decision/mutation-buffer-memory-only area=pii-scrubbing-compliance rec=38
name: Mutation buffer contents stay in memory
rule: The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents.
@row_add n=31 row=decision/state-registry-holds-no-values area=pii-scrubbing-compliance rec=39
name: State registry holds no values
rule: The state registry holds each registered state's name, declarations, write records and dispatch chain, never a state's value; its Debug prints counts alone and it holds no log site, print or env read.
@row_add n=32 row=decision/sink-target-allowlist area=pii-scrubbing-compliance rec=40,42
name: Sink target allowlist
rule: escher's sink admits only events whose target starts with an engine prefix or escher_, and drops an event from any other target whole before anything is written, at every level and whatever RUST_LOG names.
why: the trade is that a third-party WARN or ERROR no longer reaches stderr at any setting
except: the upstream apps' fmt stdout subscribers and the WPT runner's env_logger stay unscrubbed
@row_add n=33 row=decision/engine-target-safe-fields area=pii-scrubbing-compliance rec=40
name: Engine targets print safe fields only
rule: Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
@row_add n=34 row=decision/escher-target-content-named-set area=pii-scrubbing-compliance rec=40
name: Escher targets redact the content-named set
rule: Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
@row_add n=35 row=decision/closed-span-lines-judged-alike area=pii-scrubbing-compliance rec=40
name: Closed span lines judged by the same rule
rule: A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
@row_add n=36 row=decision/content-named-span-values-not-stored area=pii-scrubbing-compliance rec=40
name: Content-named span values never stored
rule: The value of a content-named span field is never stored, its marker written at print time.
@row_add n=37 row=decision/driver-span-fields-print area=pii-scrubbing-compliance rec=40
name: Driver span fields tool and ending print
rule: The tool_call span's fields tool and ending are not in the content-named set and print.
why: both are closed words; the closed-span line was ratified
@row_none n=18 rec=41 type=4
relied: A failed load's error text reaches only the Resource load failed line, where the sink redacts url, error and the message.
@row_add n=38 row=decision/cold-agent-events-carry-no-user-content area=pii-scrubbing-compliance rec=43
name: Cold-agent events carry no user content
rule: No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
@row_add n=39 row=decision/cold-agent-artifacts-never-printed area=pii-scrubbing-compliance rec=43
name: Cold-agent server log and check never printed
rule: The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
@row_add n=40 row=decision/cold-agent-transcript-unscrubbed area=pii-scrubbing-compliance rec=43
name: Cold-agent transcript unscrubbed
rule: The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target.
why: unscrubbed by design, the provider crossing ratified
@row_add n=41 row=decision/committed-transcript-copies-masked area=pii-scrubbing-compliance rec=43
name: Committed transcript copies masked
rule: Each committed copy of a cold-agent transcript masks the session dir and the home prefix.
@row_none n=19 rec=44 type=7
@row_none n=20 rec=45 type=7
@row_none n=21 rec=46 type=7
@row_none n=22 rec=47 type=7
@row_none n=23 rec=48 type=7
@row_none n=24 rec=49 type=7
@row_none n=25 rec=50 type=7
@row_none n=26 rec=51 type=7
@row_none n=27 rec=52 type=7
@row_none n=28 rec=53 type=7
@row_none n=29 rec=54 type=7
@row_none n=30 rec=55 type=7
