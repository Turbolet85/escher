@leaf_group group=added-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: the scoped runs read 137 mutants, 0 missed, 0 timeout (test-plan §10)
source: dec-la3dsw2u «The mutation tier does not run the rdme and blitz packages and owes them no test for it; churn, sizes and the other collectors keep both.»
replace-with: the scoped runs read 137 mutants, 0 missed, 0 timeout (test-plan §10); the tier does not run the rdme and blitz packages, upstream's demo shells, and owes them no test for it, churn, sizes and the other collectors keeping both
