@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=57
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=14 class=omission
leaf-text: every boot but `stand::boot_load`, which replaces it with the stand's `LoadNet`, answering `http://stand.test/load.css` from a thread of its own after 200 ms)
source: dec-6mytiutr «and the error-text check's failing page, whose FailingNet stand-in fails its fetch.»
replace-with: every boot but `stand::boot_load`, which replaces it with the stand's `LoadNet`, answering `http://stand.test/load.css` from a thread of its own after 200 ms, and the error-text check's failing page, whose `FailingNet` stand-in fails its fetch)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=46 class=omission
leaf-text: must pass — fmt, clippy and the workspace rustdoc gate green
source: dec-pe4jrwxz «Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block. · why: The fast leg runs no rustdoc»
replace-with: must pass — fmt, clippy and the workspace rustdoc gate green; the fast leg runs no rustdoc, so a chunk that edits a public doc comment lists the doc leg in its local gate block
