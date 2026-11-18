@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=6 class=stale
leaf-text: (`lint` · `screenshot` · `advance`)
source: ctr-nca5otqa «The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read and a list is opened.»
replace-with: (`lint` · `screenshot` · `advance` · `graph` · `actions` · `layout`)
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=6 class=omission
leaf-text: which the pipe reads only to count
source: dec-5cy5sipd «The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only, and never prints or copies it.»
replace-with: which the pipe reads by span name only — the `tool_call` span's `tool` and `ending` and the `command` span's `cause` word — to count
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=18 class=omission
leaf-text: newlines escaped, string values Debug-quoted
source: ctr-7gmbly6m «Newlines and carriage returns in values are escaped as \n and \r, and string values are Debug-quoted.»
replace-with: newlines and carriage returns escaped as `\n` and `\r`, string values Debug-quoted
