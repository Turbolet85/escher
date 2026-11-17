@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=stale
leaf-text: (its in-process layout read, outside the run path, opening none and writing no `escher_` line at TRACE)
source: dec-zhnzewyk «The layout read opens no span — The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span.»
replace-with: (its layout read opening no span and emitting no event of its own: read in process it writes no `escher_` line at TRACE, and reached by the `layout` verb it runs inside that call's `command` span)
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=33 class=omission
leaf-text: 0 id, name and actions needles under both builds.
source: ctr-xldqcthd «contract/command-span/layout · The driver's command span · layout — A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.»
replace-with: 0 id, name and actions needles under both builds; a layout's command-span line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer, the layout read the call reaches opening no span of its own.
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=42 class=omission
leaf-text: no stable id, no accessible name, no typed sentinel and no shot's image bytes on stderr, under both builds; `cargo test -p seven_guis --test mcp_log` — the same binary's `mcp` role at `trace`: 0 typed-sentinel occurrences, 0 id needles, 0 name needles, no shot's image bytes
source: dec-5zkib22n «Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write.»
replace-with: no stable id, no accessible name, no typed sentinel, no layout-answer key (`laid_out` · `placed` · `shifts`) and no shot's image bytes on stderr, under both builds; `cargo test -p seven_guis --test mcp_log` — the same binary's `mcp` role at `trace`: 0 typed-sentinel occurrences, 0 id needles, 0 name needles, 0 layout-answer key needles, no shot's image bytes
