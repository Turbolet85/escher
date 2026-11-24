@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: nor the driver's layout read, which opens no span and emits no event of its own
source: dec-nwhdhtlo «The canvas read opens no span — The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs inside that call's command span.»
replace-with: nor the driver's canvas read, which opens no span of its own and, reached by the `canvas` verb, runs inside that call's `command` span, nor the driver's layout read, which opens no span and emits no event of its own
@leaf_edit n=2 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: a `snapshot`, `screenshot`, `lint`, `graph`, `actions` or `layout` call's
source: ctr-5cyzlcp6 «Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout, canvas and a part read record verb alone, and a refusal records verb and cause alone.»
replace-with: a `snapshot`, `screenshot`, `lint`, `graph`, `actions`, `layout` or `canvas` call's
@leaf_edit n=3 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: nor a part's number or the part count.
source: ctr-b6sutxng «A canvas's line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag, part number or anything of its answer.»
replace-with: nor a part's number or the part count, nor a canvas's id, full flag or anything of its answer.
@leaf_edit n=4 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: show no layout-answer key needle (`laid_out`, `placed`, `shifts`);
source: dec-5zkib22n «A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel, layout-answer key or canvas needle in what they write.»
replace-with: show no layout-answer key needle (`laid_out`, `placed`, `shifts`); a canvas call's `command` line, a part read's included, `verb="canvas"`, holds no id, full flag, part number or anything of its answer — `host_log` and `mcp_log` read at `trace` under both builds show no canvas needle;
