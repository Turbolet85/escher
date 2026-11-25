@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=24 class=omission
leaf-text: findings follow document order, then the defect order)
source: ctr-cb2qijrq «contract/lint-findings/order · Lint findings · order — Lint findings come in document order, then in the stated order of the defects; a canvas element reads `truncated` once per label, those findings in paint order.»
replace-with: findings follow document order, then the defect order, a canvas element reading `truncated` once per label, those findings in paint order)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=24 class=omission
leaf-text: `Session::lint`'s readings over a held instance covered by `stand_lint`,
source: dec-eeyz3rvo «The driver's executor verbs, its refusals and the session lint's readings over a held instance are tested by integration checks, not unit tests; the canvas label reader, which reads recorded paint commands»
replace-with: `Session::lint`'s readings over a held instance covered by `stand_lint`, the canvas label reader, which reads recorded paint commands and no held instance, unit-tested,
