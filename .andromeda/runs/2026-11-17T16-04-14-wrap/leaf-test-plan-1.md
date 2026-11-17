@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=29 class=retired
leaf-text: `Stand checks repeated`'s the one that frees disk, pinned by `test_only_stand_repeat_frees_disk`
source: dec-2eaaxbw7 «The Stand checks repeated job and the default-features test job each free runner disk with a pinned free-disk action after the toolchain and before the cache»
replace-with: `Stand checks repeated` and the default-features test job each freeing disk, pinned by `test_free_disk_only_on_test_and_stand_repeat`
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=29 class=retired
leaf-text: only `Stand checks repeated` freeing disk (`test_only_stand_repeat_frees_disk`)
source: dec-2eaaxbw7 «The Stand checks repeated job and the default-features test job each free runner disk with a pinned free-disk action after the toolchain and before the cache»
replace-with: only `Stand checks repeated` and the default-features test job freeing disk (`test_free_disk_only_on_test_and_stand_repeat`)
