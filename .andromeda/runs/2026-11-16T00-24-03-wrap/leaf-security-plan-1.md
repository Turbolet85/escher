@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=18 class=stale
leaf-text: admits a run selection only as `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem
source: dec-je2iskva «a selection other than the fixed words `stand`, `all` or `session`, matched before the file-stem arm, or an existing test-file stem matching `^[a-z0-9_]+$`.»
replace-with: admits a run selection only as the fixed words `stand`, `all` or `session` (matched before the file-stem arm) or an existing `^[a-z0-9_]+$` blitz-tests file stem
