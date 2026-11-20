@leaf_group group=test-plan-3
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: the scoped runs read 137 mutants, 0 missed, 0 timeout (test-plan §10);
source: dec-vbmban3p «Scoped mutation run filter anchored on the verb — A cargo-mutants run scoped to one function anchors its filter on the mutant name's verb, the replace verb then the function's name and a space, and its»
replace-with: the scoped runs read 137 mutants, 0 missed, 0 timeout (test-plan §10); a run scoped to one function anchors its filter on the mutant name's verb — the replace verb, then the function's name and a space, since the filter is an unanchored regex over the whole mutant name and a bare name matches others holding it — and its mutant listing is read first and must show exactly the mutants the run is meant to test;
