@leaf_group group=test-plan-4
@leaf_read leaf=.claude/rules/testing.md lines=74
last: - 2026-11-19: A check written over a path just measured pins
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=14 class=omission
leaf-text: some modules sit in files of their own, each declared `#[cfg(test)]` with `#[path]` at its parent
source: dec-7nt3wnew «Unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path at their parent, and fixture builders in a child module of that test module.»
replace-with: some modules sit out of line in a sibling file with the `_tests` suffix, each declared `#[cfg(test)]` with `#[path]` at its parent, fixture builders allowed in a child module of that test module
@leaf_edit n=2 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: Every other file is its own target:
source: dec-nh5z5mxh «Oversize integration-test targets kept whole — Escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them.»
replace-with: Every other file is its own target, one over 800 code lines kept whole, one file and one target, never split into several:
@leaf_edit n=3 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: so only splitting a function into module-level sibling items lowers its reading.
source: dec-whylzu6r «Upstream's duplicated test pairs kept as upstream's — Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, and the duplication reading is not held against them.»
replace-with: so only splitting a function into module-level sibling items lowers its reading. Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, the duplication reading not held against them.
