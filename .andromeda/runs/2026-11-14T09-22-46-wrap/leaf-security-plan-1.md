@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=20 class=omission
leaf-text: ends as on `stop`; unix only.
source: dec-m7xw3ssy «The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.»
replace-with: ends as on `stop`; the socket exists on unix only, while an in-process session runs on linux, windows and macOS alike.
