@leaf_group group=added-5
@leaf_read leaf=.claude/rules/testing.md lines=77
last: - 2026-11-22: Any cargo run on the shared `target/` without
@leaf_edit n=1 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, the duplication reading not held against them.
source: dec-nh5z5mxh «Escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them.»
replace-with: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, the duplication reading not held against them, and escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them (a split moves the pinned target layout the CI legs read).
