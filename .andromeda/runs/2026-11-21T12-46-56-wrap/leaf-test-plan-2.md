@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=76
last: - 2026-11-21: A kept check read back by `Check::from_bytes`
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own
@leaf_edit n=1 leaf=.claude/rules/testing.md line=22 class=stale
leaf-text: a change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices
source: dec-vabbkkgs «A change of the driver's verb count or of a CI leg's file count sweeps the tests, examples and CI-script tests for pins of that count held as a literal, and a test rename sweeps the old name held»
replace-with: a change of the driver's verb count or of a CI leg's file count sweeps the tests, examples and CI-script tests for pins of that count held as a literal
@leaf_edit n=2 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: Assert first that the fixture produces the condition under test, then the behaviour;
source: dec-phg3jrc4 «A focus test pins the node focus lands on, never only that a hidden control is not focused.»
replace-with: Assert first that the fixture produces the condition under test, then the behaviour; a focus test pins the node focus lands on, never only that a hidden control is not focused (the post-layout focus fixup makes that hold with or without the change under test);
@leaf_edit n=3 leaf=.claude/rules/testing.md line=16 class=omission
leaf-text: `blitz_test_harness::Harness` over `HtmlDocument` / `DioxusDocument`;
source: dec-3e2k3bgk «A test task that waits for a later render on the Dioxus bridge leaves its waker for the producer to wake and never wakes its own waker while it waits.»
replace-with: `blitz_test_harness::Harness` over `HtmlDocument` / `DioxusDocument` (a test task that waits for a later render on the Dioxus bridge leaves its waker for the producer to wake and never wakes its own waker while it waits — doing so livelocks inside the buffered render);
