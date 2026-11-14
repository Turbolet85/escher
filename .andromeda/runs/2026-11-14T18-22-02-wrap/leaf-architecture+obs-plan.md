@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: one `tool_call` span per MCP tool call, its parent, printed only where a sink is installed at `info` or below;
source: dec-rlke2ua7 «Auto-exploration opens no span and emits no event of its own; each call it drives, re-reach steps included, runs through the session's run and leaves its own command span.»
replace-with: one `tool_call` span per MCP tool call, its parent, printed only where a sink is installed at `info` or below; an auto-exploration opens no span and emits no event of its own, each call it drives, re-reach steps included, running through the session's run and leaving its own `command` span;
