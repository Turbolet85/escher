@task_prose part=session
A test-only sweep: every 0.3.0 capability is now pinned to a named witness that the stand-repeat CI leg runs, through a new 0.3.0 witness table, and the leg check reads the stand-repeat arm alone.
A new stand check proves the value mask holds on the newer carriers: the graph, actions and layout answers, a kept check and its replay, every explored path's kept check, and a file input's value read over MCP, in both layout modes.
The browser app's document loader gained five unit tests over the document config it builds (root shell provider, HTML parser provider, font context, the passed abort signal, parallel style); the config itself is unchanged.
No production code, public symbol, verb, carrier field, dependency, CI script or config changed; the per-crate coverage floors held (escher-driver 97.68 %, dioxus-native-dom 90.79 %, blitz-test-harness 95.70 % on CI).
The coverage CI job went red once on a corrupt coverage profile at the merge step and green on its re-run of the same tree; no cause measured.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand requirement sweep check, which pins each 0.2.0 and 0.3.0 requirement to a witness test inside the stand-repeat CI leg and holds the list of named stand targets that leg runs; edited
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: a new stand check of the password and file input value mask on the graph answer, actions answer, layout answer, kept checks with replay, every explored path's kept check and the MCP tool result for a file input; added
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each document's config (root shell provider, HTML parser provider, font context, abort signal, parallel style); edited in its unit tests alone
@task_item n=1 part=touches
decision/mask-holds-on-every-carrier: its guard names stand_act_filled and stand_mcp_masked but not the new stand_masks_on_new_answers check, which now proves the mask on the graph, actions and layout answers, kept checks and explored paths' checks; the guard cell is stale.
@task_item n=2 part=touches
The masked-input rows (decision/masked-input-values, decision/password-file-value-masked, decision/recorded-type-masked): a file input's value now proven to read MASKED_VALUE over MCP and a kept type to a masked control to read MASKED_STAND_IN; a guard cell naming only older checks may now take the new one.
@task_item n=3 part=touches
Contracts holding the mask stand-ins (contract/snapshot-value/masked-value, contract/masked-input-value/masked-stand-in, contract/recorded-check/masked-stand-in, contract/recorded-check/masked, contract/mcp-result/masked-control-values, contract/driver-session-reply/masked-control-values): newly witnessed on the graph, actions and layout answers and over MCP; their values are unchanged.
@task_item n=4 part=touches
The answer-crossing decisions for the graph, actions and layout answers (decision/graph-answer-crossing, decision/actions-answer-crossing, decision/session-layout-read-in-process): the mask over these answers now has a stand witness in both layout modes.
@task_item n=5 part=touches
Kept check data-protection rows (decision/kept-check-graph-answer, decision/kept-check-actions-answer, decision/kept-check-layout-answer, decision/kept-check-answer-whole, decision/exploration-report-closed-content): a kept check and every explored path's kept check are now proven to hold the typed password and file path zero times.
@task_item n=6 part=touches
decision/caused-text-crossing: the mask check states a caused item outside the mask by ratified exception, read in-process, since no served task logs or panics.
@task_item n=7 part=touches
decision/document-abort-signal-cancels-fetches: the document config is now unit-tested to carry the passed abort signal; the behaviour is unchanged.
@task_item n=8 part=touches
decision/per-crate-coverage-floors: the three crates' floors held locally and on CI; any row holding a coverage figure reads the figures above, and the floor values themselves did not move.
@task_item n=9 part=touches
Rows whose guard names the stand-repeat leg or the stand requirement sweep: the sweep now reads only the stand-repeat arm of the CI leg script and its named target list grew from 10 to 17.
@task_fact n=1 slug=input-validation
fact: The value mask on the graph, actions and layout answers, on kept checks with their replay, on every explored path's kept check and on a file input's value over MCP is proven by one stand check in both layout modes, and a caused item stands outside the mask by ratified exception.
@task_item n=1 part=untouched
Authentication and authorization rows, CI token, publish and secret-management rows: no workflow, secret or permission changed.
@task_item n=2 part=untouched
Dependency-security rows (audit, licence gate, locked builds, pins, coverage tool): no dependency, manifest or tool changed.
@task_item n=3 part=untouched
API-security rows on the session socket, MCP stdio server, idle expiry, load waits and outbound HTTP limits: no listener, bound or request path changed.
@task_item n=4 part=untouched
Error-handling rows (refusal causes, session errors, MCP protocol errors, exit codes): no refusal, error table or exit code changed.
@task_item n=5 part=untouched
Logging and telemetry sink rows (allowlists, span lines, content scrubbing): no sink, target or span changed; the new check prints nothing.
@task_item n=6 part=untouched
Select list, range, stable element id, accessibility, CSS and HTML parsing rows: no engine or driver behaviour changed.
@task_item n=7 part=untouched
Cold-agent and agent-run harness rows: the harness scripts are unchanged; only their run counts moved and no row holds them.
