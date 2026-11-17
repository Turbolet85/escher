@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: a log's scrubbed fields and the first reached handler
source: ctr-wkieu5ai «contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location»
replace-with: a log's fields, its `error` field kept, and the first reached handler
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: holding its message, location, boundary, cause chain and fields, a content-named field read as redacted: a ratified boundary widening to the error text
source: dec-vztg2sue «decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier when the step caused something: one item per panic, boundary error and kept record with»
replace-with: holding its message, location, boundary, cause chain and fields, an `error` field as written within the text bound, every other content-named field read as redacted: a ratified boundary widening to the error text
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=28 class=omission
leaf-text: No command's ending writes back anything the caller typed.
source: dec-x7drthne «decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error»
replace-with: No command's ending writes back anything the caller typed, but `caused` carries an app's own messages and fields as their author wrote them, `error` included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
