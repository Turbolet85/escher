@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=16 class=omission
leaf-text: with its fourteen verbs
source: dec-vnrtz2ue «The cold-agent pipe's verb list holds the fourteen driver verbs other than `layout`.»
replace-with: with its fourteen verbs, the driver's verbs other than `layout`
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: a layout read changing nothing
source: ctr-afdxnw5f «A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied»
replace-with: a layout read, in process or through the `layout` verb, changing nothing
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: read inclusively)
source: ctr-k7ugyh26 «The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read»
replace-with: read inclusively; the `layout` verb taking one required id and answering one line of id, bounds, laid_out, placed, shifts, width and height — ids, closed kebab-case words and numbers only — equal to the in-process layout read on every node of the six served screens in both layout modes)
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=24 class=stale
leaf-text: the fourteen verbs at their levels
source: ctr-cavzgdgu «Fifteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.»
replace-with: the fifteen verbs at their levels
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=24 class=stale
leaf-text: the listing as the fourteen verbs, `graph` after `lint` and `actions` after `graph`
source: ctr-cthm7xgj «tools/list offers the fifteen verbs in order, `layout` after `actions` and `help` last.»
replace-with: the listing as the fifteen verbs, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last
@leaf_edit n=6 leaf=.claude/docs/tests-summary.md line=24 class=stale
leaf-text: offering the fourteen verbs, `graph` after `lint` and `actions` after `graph`
source: ctr-dr63nnol «`tools/list` offers the fifteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last.»
replace-with: offering the fifteen verbs, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last
