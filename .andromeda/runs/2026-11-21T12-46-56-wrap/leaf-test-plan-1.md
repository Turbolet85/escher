@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: a select's choice still moving past the dropped block
source: dec-6by3zqcu «Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.»
replace-with: the post-layout fixup giving focus up, and a select's choice moving past the dropped block only after a re-focus
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: it does not panic, focus stays and the key still acts
source: dec-6by3zqcu «Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.»
replace-with: it does not panic, the post-layout fixup gives focus up and the key acts only after a re-focus
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=37 class=omission
leaf-text: its Dioxus 0.8 move not taken
source: dec-3e2k3bgk «Render waits leave the waker to the producer — A test task that waits for a later render on the Dioxus bridge leaves its waker for the producer to wake and never wakes its own waker while it waits.»
replace-with: its Dioxus 0.8 move not taken, a test task waiting for a later render on the bridge leaving its waker for the producer to wake and never waking its own while it waits, which livelocks inside the buffered render, a limit asserted by no check
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=52 class=omission
leaf-text: NO RECORDED INTENT — conventions as built are in `.claude/rules/testing.md`.
source: dec-phg3jrc4 «Focus tests pin where focus lands — A focus test pins the node focus lands on, never only that a hidden control is not focused.»
replace-with: A focus test pins the node focus lands on, never only that a hidden control is not focused (the post-layout focus fixup makes the latter hold either way); other conventions as built are in `.claude/rules/testing.md`.
