@leaf_group group=layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=28 class=omission
leaf-text: `scripts/agent-run.sh` (the agent-run test contract: usage on stderr, JSON lines only on stdout)
source: ctr-guhhzszg «agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all, session or a blitz-tests file name; session is a closed word»
replace-with: `scripts/agent-run.sh` (the agent-run test contract: usage on stderr, JSON lines only on stdout; run selections `stand` · `all` · `session` or a blitz-tests file name, `session` a closed word matched before a file name that runs the escher-session binary's flows in seven_guis)
