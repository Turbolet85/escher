@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: (`stand_mcp_tools` installs a hand-written recorder of its own, not this sink)
source: dec-64bjtz2h «except: The MCP tools and layout stand checks install their own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.»
replace-with: (`stand_mcp_tools` and the layout stand check `stand_layout` each install a recording subscriber of their own, not this sink)
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: the session library emits one span per command under the target `escher_driver`
source: dec-zhnzewyk «The driver's in-process layout read opens no span and emits no event, runs outside the command span, and writes no escher_ line at TRACE.»
replace-with: the session library emits one span per command its run path runs under the target `escher_driver` (its in-process layout read, outside the run path, opening none and writing no `escher_` line at TRACE)
