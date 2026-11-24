@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: (its layout read opening no span and emitting no event of its own: read in process it writes no `escher_` line at TRACE, and reached by the `layout` verb it runs inside that call's `command` span)
source: dec-nwhdhtlo «decision/canvas-read-opens-no-span · The canvas read opens no span — The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs»
replace-with: (its layout read opening no span and emitting no event of its own: read in process it writes no `escher_` line at TRACE, and reached by the `layout` verb it runs inside that call's `command` span; its canvas read likewise opening no span of its own, reached by the `canvas` verb inside that call's `command` span)
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=33 class=omission
leaf-text: holds nothing of its answer, the layout read the call reaches opening no span of its own.
source: ctr-b6sutxng «contract/command-span/canvas · The driver's command span · canvas — A canvas's line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag»
replace-with: holds nothing of its answer, the layout read the call reaches opening no span of its own; a canvas's command-span line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag, part number or anything of its answer, the canvas read the call reaches opening no span of its own.
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=42 class=omission
leaf-text: no layout-answer key (`laid_out` · `placed` · `shifts`) and no shot's image bytes on stderr, under both builds; `cargo test -p seven_guis --test mcp_log` — the same binary's `mcp` role at `trace`: 0 typed-sentinel occurrences, 0 id needles, 0 name needles, 0 layout-answer key needles,
source: dec-5zkib22n «decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel, layout-answer»
replace-with: no layout-answer key (`laid_out` · `placed` · `shifts`), no canvas needle and no shot's image bytes on stderr, under both builds; `cargo test -p seven_guis --test mcp_log` — the same binary's `mcp` role at `trace`: 0 typed-sentinel occurrences, 0 id needles, 0 name needles, 0 layout-answer key needles, 0 canvas needles,
