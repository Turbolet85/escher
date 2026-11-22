@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=24 class=omission
leaf-text: `graph` answering one line of the UI graph;
source: ctr-amy33am2 «contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes `--part` with one value, a whole number 1 to 1000, anything else refused `malformed`; a whole answer is one line»
replace-with: `graph` answering one line of the UI graph — `graph` taking `--part` with one value, a whole number 1 to 1000, anything else refused `malformed`, a whole answer one line `{"nodes":[…],"states":[…]}` and a part one line `{"nodes":[…],"states":[…],"part":n,"parts":m}` with `states` in part 1 alone, status `0`;
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=29 class=stale
leaf-text: its membership the founder's «Плюс проверки дерева (Рекомендую)»
source: dec-br6fk74z «decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the files that each witness an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap.»
replace-with: its membership the files that each witness an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap, chosen over running the tier-0 witnesses alone
