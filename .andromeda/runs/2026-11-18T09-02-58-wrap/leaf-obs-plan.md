@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=17 class=stale
leaf-text: the accepted lines of each counted verb (`lint` · `screenshot` · `advance`, `span_counts`, giving the verdict's `lint_calls` · `screenshot_calls` · `advance_calls`)
source: ctr-pzhtbn2z «contract/cold-agent-counted-verbs/actions · The cold-agent pipe's counted verbs · actions — A counted verb: its accepted `tool_call` span lines are counted from the server log.»
replace-with: the accepted lines of each counted verb (`lint` · `screenshot` · `advance` · `actions` · `graph` · `layout`, `span_counts`, giving the verdict's `lint_calls` · `screenshot_calls` · `advance_calls` · `actions_calls` · `graph_calls` · `layout_calls`)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=17 class=omission
leaf-text: never printing or copying it; a judged check is replayed
source: dec-5cy5sipd «decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only»
replace-with: and the `command` lines' `cause` word (`refused_by_cause`), never printing or copying it; a judged check is replayed
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=12 class=stale
leaf-text: which install the sink over an in-memory capture
source: dec-64bjtz2h «except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.»
replace-with: which install the sink over an in-memory writer
@leaf_edit n=4 leaf=.claude/rules/observability.md line=26 class=stale
leaf-text: (`lint` · `screenshot` · `advance`, `span_counts`)
source: ctr-pzhtbn2z «contract/cold-agent-counted-verbs/actions · The cold-agent pipe's counted verbs · actions — A counted verb: its accepted `tool_call` span lines are counted from the server log.»
replace-with: (`lint` · `screenshot` · `advance` · `actions` · `graph` · `layout`, `span_counts`)
@leaf_edit n=5 leaf=.claude/rules/observability.md line=26 class=omission
leaf-text: never printing or copying it; a judged check is replayed
source: dec-5cy5sipd «decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only»
replace-with: and the `command` lines' `cause` word (`refused_by_cause`), never printing or copying it; a judged check is replayed
@leaf_edit n=6 leaf=.claude/rules/observability.md line=26 class=omission
leaf-text: counts, booleans, closed words, the task,
source: dec-4milvibf «decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, an object of closed words to counts, the task, the client's tool names, version and model»
replace-with: counts, booleans, closed words, objects of closed words to counts, the task,
@leaf_edit n=7 leaf=.claude/rules/observability.md line=18 class=stale
leaf-text: which install the sink over an in-memory capture
source: dec-64bjtz2h «except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.»
replace-with: which install the sink over an in-memory writer
